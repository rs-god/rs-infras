use redis::Commands;
use redis::RedisResult;
use rs_infras::redis::RedisConf;

fn main() {
    let dsn = "redis://:@127.0.0.1:6379/0";
    let pool = RedisConf::builder().with_dsn(dsn).init_pool();
    let mut conn = pool.get().unwrap();

    // 设置单个pool timeout
    // let mut conn = pool.get_timeout(Duration::from_secs(2)).unwrap();
    let res: RedisResult<String> = conn.set("my_user", "daheige");
    if res.is_err() {
        println!("redis set error:{}", res.err().unwrap().to_string());
    } else {
        println!("set success");
    }
}
