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
