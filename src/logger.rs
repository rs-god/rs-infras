// 引入了chrono::Local模块，主要用于日志记录时间格式的自定义
use chrono::Local;
// 引入Write trait，主要用于env_logger自定义日志写入格式
use env_logger::Target;
use std::io::Write;

pub struct Logger {
    is_custom: bool, // 日志初始化是否自定义
}

impl Logger {
    pub fn new() -> Self {
        Self { is_custom: false }
    }

    // 如果开启了自定义模式，输出结果如下
    // 2025-11-08 17:24:18 INFO [logger::tests:72] info message
    // 2025-11-08 17:24:18 ERROR [logger::tests:73] error message:invalid key
    pub fn with_custom(mut self) -> Self {
        self.is_custom = true;
        self
    }

    // 日志初始化
    // 其中日志level优先级从高到低：error > warn > info > debug > trace
    // 程序启动时可以通过 RUST_LOG=info 设置日志级别
    pub fn init(&self) {
        if !self.is_custom {
            // 如果你不关注日志时区的话，可以直接使用eng_logger默认方式初始化
            env_logger::builder().target(Target::Stdout).init();
            return;
        }

        // env_logger env settings
        env_logger::builder()
            .target(Target::Stdout)
            .format(|buf, record| {
                let level = record.level();
                writeln!(
                    buf,
                    "{} {} [{}:{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"), // 时间格式
                    level,                                    // 日志级别
                    record.module_path().unwrap_or("unnamed"), // 模块名
                    record.line().unwrap_or(0),               // 行号
                    &record.args()                            // 日志message body内容
                )
            })
            .init();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{error, info};
    use std::env;
    #[test]
    fn test_logger() {
        // 通过环境变量 RUST_LOG 设置日志级别
        // 一般来说，程序启动时，可以先设置 RUST_LOG=info 而不需要通过unsafe方式来设置
        // 如果开启了自定义模式，输出结果如下
        // 2025-11-08 17:24:18 INFO [logger::tests:72] info message
        // 2025-11-08 17:24:18 ERROR [logger::tests:73] error message:invalid key
        //
        // 否则就不包含函数行号等信息
        // [2025-11-08T09:15:21Z INFO  logger::tests] info message
        // [2025-11-08T09:15:21Z ERROR logger::tests] error message:invalid key
        unsafe {
            env::set_var("RUST_LOG", "info");
        }

        let logger = Logger::new().with_custom();
        // let logger = Logger::new();
        logger.init();

        info!("info message");
        error!("error message:{}", "invalid key");
    }
}
