# read-model-updater

## docker build

```sh
# observer
docker build -t cqrs-es-example-dynamodb-observer -f Dockerfile.observer .
```

```sh
# lambda
SAM_BUILD_MODE=debug sam build --beta-features
docker build --platform linux/arm64 -t cqrs-es-example-read-model-updater -f Dockerfile.lambda .
```

## Preparing for Kubernetes

```sh
docker tag cqrs-es-example-read-model-updater:latest kuramapommel/cqrs-es-example-read-model-updater:latest
docker push kuramapommel/cqrs-es-example-read-model-updater:latest
docker tag cqrs-es-example-dynamodb-observer:latest kuramapommel/cqrs-es-example-dynamodb-observer:latest
docker push kuramapommel/cqrs-es-example-dynamodb-observer:latest
```
