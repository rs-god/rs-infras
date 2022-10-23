use r2d2::Pool;
use redis::{self, cluster::ClusterClient};
use std::time::Duration;

// 推荐使用&'a str方式，这样with_dsn就可以传递&str作为参数
// 对刚接触 Rust 的程序员来说，辨别&str 和 String 的应用场景会存在一些困惑。最佳的
// 做法是尽可能使用带有&str 类型的 API，因为当字符串已经分配到某处时，只需引用该字
// 符串就可以节省复制和分配的成本。在程序中传递&str 几乎是零成本的：它几乎不会产生
// 分配成本，也不会复制内存。
// 在内部，&String 会自动被强制转换为&str，因为 String 为 str 类型实现了类型强制性特征 Deref，该特征确保了
// &String 到&str 的转换。
// 字符串切片是一个用途广泛的输入型参数，
// 不仅适用于实际的字符串切片引用，还适用于 String 引用。所以再强调一遍：如果你需要
// 将一个字符串传递给你的函数，那么请使用字符串切片&str
//
// 当然也可采用String设计
// 但那样就会将dsn String所有权传递给dsn，要么就是dsn参数显示调用clone复制一个string那样是拷贝成本的。

#[derive(Default, Debug)]
pub struct RedisConf<'a> {
    dsn: &'a str,
    cluster_nodes: Vec<&'a str>,
    max_size: u32,
    min_idle: u32,
    max_lifetime: Duration,
    idle_timeout: Duration,
    connection_timeout: Duration,
}

impl<'a> RedisConf<'a> {
    pub fn builder() -> Self {
        Self {
            max_size: 20,
            min_idle: 3,
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(300),
            connection_timeout: Duration::from_secs(10),
            ..Default::default()
        }
    }

    pub fn with_dsn(mut self, dsn: &'a str) -> Self {
        self.dsn = dsn;
        self
    }

    pub fn with_cluster_nodes(mut self, nodes: Vec<&'a str>) -> Self {
        self.cluster_nodes = nodes;
        self
    }

    pub fn with_max_size(mut self, max: u32) -> Self {
        self.max_size = max;
        self
    }

    pub fn with_min_idle(mut self, min: u32) -> Self {
        self.min_idle = min;
        self
    }

    pub fn with_max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = lifetime;
        self
    }

    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    pub fn init_pool(&self) -> Pool<redis::Client> {
        if self.dsn.is_empty() {
            panic!("redis dsn is empty");
        }

        let client = redis::Client::open(self.dsn).unwrap();
        let pool = self.set_pool(client);
        pool
    }

    pub fn init_cluster_pool(&self) -> Pool<ClusterClient> {
        if self.cluster_nodes.is_empty() {
            panic!("cluster nodes is empty");
        }

        let mut nodes = Vec::new();
        for node in self.cluster_nodes.iter() {
            nodes.push(node.as_ref())
        }

        let client = ClusterClient::new(nodes).unwrap();
        let pool = self.set_pool(client);
        pool
    }

    // 由于redis client和redis cluster client创建pool只是build的client不一样
    // 所以这里可以采用泛型方法创建pool,这个泛型参数T满足ManageConnection trait特征就可以
    fn set_pool<T: r2d2::ManageConnection>(&self, client: T) -> Pool<T> {
        let pool = Pool::builder()
            .max_size(self.max_size)
            .max_lifetime(Some(self.max_lifetime))
            .idle_timeout(Some(self.idle_timeout))
            .min_idle(Some(self.min_idle))
            .connection_timeout(self.connection_timeout)
            .build(client)
            .unwrap();
        pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::{self, Commands};

    #[test]
    fn test_redis() {
        let dsn = "redis://:@127.0.0.1:6379/0";
        let pool = RedisConf::builder().with_dsn(dsn).init_pool();
        let mut conn = pool.get().unwrap(); // 默认超时是 connection_timeout 参数

        // 设置单个pool timeout
        // let mut conn = pool.get_timeout(Duration::from_secs(2)).unwrap();
        let res: redis::RedisResult<String> = conn.set("my_user", "daheige");
        if res.is_err() {
            println!("redis set error:{}", res.err().unwrap().to_string());
        } else {
            println!("set success");
        }
    }

    #[test]
    fn test_redis_cluster() {
        let nodes = vec![
            "redis://:@127.0.0.1:6381/0",
            "redis://:@127.0.0.1:6382/0",
            "redis://:@127.0.0.1:6383/0",
            "redis://:@127.0.0.1:6384/0",
            "redis://:@127.0.0.1:6385/0",
            "redis://:@127.0.0.1:6386/0",
        ];

        let pool = RedisConf::builder()
            .with_cluster_nodes(nodes)
            .init_cluster_pool();
        let mut conn = pool.get().unwrap();

        let res: redis::RedisResult<String> = conn.set("my_user", "daheige");
        if res.is_err() {
            println!("redis set error:{}", res.err().unwrap().to_string());
        } else {
            println!("set success");
        }
    }
}
