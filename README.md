# rs-infras
- rust infras includes the packaging of mysql(sqlx),redis,kafka and other basic libraries
- which facilitates rust development without worrying about the underlying details and enables rust developers to focus more on the development of business logic.

# reference crates
- https://crates.io/crates/redis
- https://crates.io/crates/r2d2
- https://crates.io/crates/sqlx
- https://crates.io/crates/pulsar
- https://crates.io/crates/tokio
- https://crates.io/crates/serde
- https://crates.io/crates/serde_yaml
- https://crates.io/crates/log
- https://crates.io/crates/env_logger

# dependencies
[Cargo.toml](https://github.com/rs-god/rs-infras/blob/main/Cargo.toml)

```toml
[dependencies]
# redis use tokio async
redis = { version = "0.32.7", features = ["r2d2", "tokio-comp", "cluster","cluster-async", "json"] }
r2d2 = "0.8.10"

sqlx = { version = "0.8.6", features = ["runtime-tokio", "mysql", "chrono", "time"] }

tokio = { version = "1.48.0", features = ["full"] }
futures = "0.3.31"
chrono = "0.4.42"

# pulsar
pulsar = "6.5.0"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"

# serde yaml for config read
serde_yaml = "0.9.33"

# crypto
aes = "0.8.4"
cbc = "0.1.2"
rand = "0.9.2"
base64 = "0.22.1"
```

# license
    MIT
