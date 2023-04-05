# rs-infras
- rust infras includes the packaging of mysql(sqlx),redis,kafka and other basic libraries
- which facilitates rust development without worrying about the underlying details and enables rust developers to focus more on the development of business logic.

# reference crates
- https://crates.io/crates/redis
- https://crates.io/crates/r2d2
- https://crates.io/crates/sqlx

# pulsar in docker
doc: https://pulsar.apache.org/docs/2.11.x/getting-started-docker/
```shell    
docker run -dit \
  --name pulsar-sever \
  -p 6650:6650 \
  -p 8080:8080 \
  --mount source=pulsardata,target=/pulsar/data \
  --mount source=pulsarconf,target=/pulsar/conf \
  apachepulsar/pulsar:2.9.4 \
  bin/pulsar standalone

# license
    MIT
