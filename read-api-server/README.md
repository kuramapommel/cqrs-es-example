# read-api-server

## docker build

```sh
bun install
bun prisma:b
bun bundle
docker build -t cqrs-es-example-read-api-server .
```

## Preparing for Kubernetes

```sh
docker tag cqrs-es-example-read-api-server:latest kuramapommel/cqrs-es-example-read-api-server:latest
docker push kuramapommel/cqrs-es-example-read-api-server:latest
```
