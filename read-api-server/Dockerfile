# -------- STAGE 1: Build Phase --------
FROM oven/bun:latest AS builder

WORKDIR /app

# 必要なファイルをコピー（不要なファイルは除く）
COPY prisma ./prisma
COPY src ./src
COPY bun.lock package.json tsconfig.json ./

# 依存解決 & バンドル
RUN bun install \
    && bun prisma:b \ 
    && bun bundle

# -------- STAGE 2: Runtime Phase --------
FROM oven/bun:latest

RUN apt-get update -y && apt-get install -y openssl

WORKDIR /app

COPY --from=builder /app/dist/bundle.js /app/bundle.js
COPY --from=builder /app/generated/prisma/libquery_engine-linux-arm64-openssl-3.0.x.so.node /app/generated/prisma/libquery_engine-linux-arm64-openssl-3.0.x.so.node

CMD ["bun", "./bundle.js"]