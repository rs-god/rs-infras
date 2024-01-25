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

# dependencies
```toml
[dependencies]
# tokio-comp for tokio async support
redis = { version ="0.24.0",features = ["r2d2","tokio-comp","cluster","cluster-async","json"]}
r2d2 = "0.8.10"
sqlx = { version = "0.7.3", features = [ "runtime-tokio-rustls" , "mysql","chrono"] }
tokio = { version = "1.35.1", features = ["full"] }
futures = "0.3.30"
chrono = "0.4.33"

# pulsar
pulsar = "6.1.0"
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"

#serde yaml for config read
serde_yaml = "0.9.30"
```

# license
    MIT
