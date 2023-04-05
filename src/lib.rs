pub mod mysql;
pub mod redis;
pub mod rpulsar;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("ok");
    }
}
