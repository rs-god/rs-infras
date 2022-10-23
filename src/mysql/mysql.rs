use sqlx;
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

// mysql 配置信息
#[derive(Default, Debug)]
pub struct MysqlConf<'a> {
    // 结构体中包含引用类型的字段，如果编译器无法推断生命周期，需要手动标注
    dsn: &'a str,              // dsn &str 引用类型
    max_connections: u32,      // 最大连接数，默认100个
    min_connections: u32,      // 最小连接数，默认10个
    max_lifetime: Duration,    // 最大生命周期，默认1800s
    idle_timeout: Duration,    // 空闲连接的生命周期，默认600s
    connect_timeout: Duration, // 连接超时,默认10s
}

impl<'a> MysqlConf<'a> {
    pub fn new(dsn: &'a str) -> Self {
        if dsn.is_empty() {
            panic!("mysql dsn is empty");
        }

        Self {
            dsn,
            max_connections: 100,
            min_connections: 10,
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(600),
            connect_timeout: Duration::from_secs(10),
        }
    }

    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    pub fn with_min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
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
        self.connect_timeout = timeout;
        self
    }

    // 初始化MYSQL pool
    pub async fn init_pool(&self) -> Result<sqlx::MySqlPool, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(self.max_connections) // 最大连接数
            .min_connections(self.min_connections) // 最小连接数
            .max_lifetime(self.max_lifetime) // 最大生命周期
            .idle_timeout(self.idle_timeout) // 空闲连接的生命周期
            .connect_timeout(self.connect_timeout) // 连接超时
            .connect(&self.dsn)
            .await?;
        Ok(pool)
    }
}

/*
CREATE TABLE `student` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(128) NOT NULL,
  `age` int(11) NOT NULL,
  `id_card` varchar(128) NOT NULL,
  `last_update` date NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
 */
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use futures::TryStreamExt;
    use sqlx::Row;

    #[derive(Debug, sqlx::FromRow)]
    struct Stu {
        id: i64,
        name: String,
        age: i32,
        id_card: String,
        last_update: NaiveDate, // 时间格式
    }

    #[tokio::test]
    async fn test_mysql() -> Result<(), sqlx::Error> {
        let dsn = "mysql://root:root1234@localhost/test";
        let mysql_conf = MysqlConf::new(dsn).with_max_connections(10);
        let pool = mysql_conf.init_pool().await.unwrap();
        let row: (i64,) = sqlx::query_as("select ?")
            .bind(120i64)
            .fetch_one(&pool)
            .await?;

        println!("res: {}", row.0);
        assert_eq!(row.0, 120);

        // 1、使用fetch，获取cursor游标，自己处理row
        let sql = "select * from student where id >= ?";
        let mut rows = sqlx::query(sql).bind(1).fetch(&pool);
        while let Some(row) = rows.try_next().await? {
            let stu = Stu {
                id: row.get("id"),
                name: row.get("name"),
                age: row.get("age"),
                id_card: row.get("id_card"),
                last_update: row.get("last_update"),
            };

            // println!("stu = {:?}", stu);
            println!(
                "id: {},name: {} age: {} id_card: {} last_update: {}",
                stu.id, stu.name, stu.age, stu.id_card, stu.last_update
            );
        }

        Ok(())
    }
}
