pub mod config;
pub mod crypto;
pub mod logger;
pub mod metrics;
pub mod shutdown;
pub mod xmysql;
pub mod xpulsar;
pub mod xredis;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("ok");
    }
}
