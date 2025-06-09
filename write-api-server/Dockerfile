# --- Stage 1: Build the fat JAR using sbt ------------------
FROM sbtscala/scala-sbt:graalvm-ce-22.3.3-b1-java17_1.11.0_3.7.0 as builder

# 作業ディレクトリを作成
WORKDIR /build

# sbt のキャッシュを有効にするために先にプロジェクト構成をコピー
COPY project ./project
COPY build.sbt .
COPY ./src ./src

# fat JAR を生成
RUN sbt assembly

# --- Stage 2: Run JAR ------------------
# ベースイメージとして amazon corretto の java17 のイメージを使用
FROM amazoncorretto:17-alpine3.21

# 作業ディレクトリ
WORKDIR /app

# アプリケーションJARをコピー
COPY --from=builder /build/target/scala-3.6.4/cqrs-es-example-write-api-server.jar .

# 下記を参考に ENTRYPOINT でシェル形式を指定しても警告が出ないように明示的に使用するシェルを指定
# https://docs.docker.com/reference/build-checks/json-args-recommended/#explicitly-specify-the-shell
SHELL ["/bin/sh", "-c"]

# アプリケーションをENTRYPOINTとして実行
ENTRYPOINT exec java -jar cqrs-es-example-write-api-server.jar