# write-api-server

## Docker build

```sh
docker build -t cqrs-es-example-write-api-server .
```

## Preparing for Kubernetes

```sh
docker tag cqrs-es-example-write-api-server:latest kuramapommel/cqrs-es-example-write-api-server:latest
docker push kuramapommel/cqrs-es-example-write-api-server:latest
```
