#!/bin/sh
set -o errexit -o nounset -o pipefail

# bashをインストール
apk add --no-cache bash

sleep 5

while ! nc -z "$LOCALSTACK_HOST" "$DDB_PORT"; do sleep 3; done

export DYNAMODB_ENDPOINT="http://${LOCALSTACK_HOST}:${DDB_PORT}"

echo "=== \$AWS_REGION = $AWS_REGION ==="
echo "=== \$AWS_DEFAULT_REGION = $AWS_DEFAULT_REGION ==="
echo "=== \$LOCALSTACK_HOST = $LOCALSTACK_HOST ==="
echo "=== \$DDB_PORT = $DDB_PORT ==="
echo "=== \$DYNAMODB_ENDPOINT = $DYNAMODB_ENDPOINT ==="
echo "=== \$AWS_ACCESS_KEY_ID = $AWS_ACCESS_KEY_ID ==="
echo "=== \$AWS_SECRET_ACCESS_KEY = $AWS_SECRET_ACCESS_KEY ==="

DIR=$(dirname "$0")

"$DIR"/create-tables.sh
