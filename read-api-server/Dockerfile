FROM oven/bun:latest

RUN apt-get update -y && apt-get install -y openssl

COPY dist/bundle.js /app/bundle.js
COPY generated/prisma/libquery_engine-linux-arm64-openssl-3.0.x.so.node /app/generated/prisma/libquery_engine-linux-arm64-openssl-3.0.x.so.node

WORKDIR /app

CMD ["bun", "./bundle.js"]