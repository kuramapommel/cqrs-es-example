# ベースイメージとして軽量なJava 11のイメージを使用
FROM adoptopenjdk:11-jre-hotspot

# アプリケーションJARをコピー
COPY ./target/scala-3.6.4/cqrs-es-example-write-api-server.jar /app/cqrs-es-example-write-api-server.jar

# 下記を参考に ENTRYPOINT でシェル形式を指定しても警告が出ないように明示的に使用するシェルを指定
# https://docs.docker.com/reference/build-checks/json-args-recommended/#explicitly-specify-the-shell
SHELL ["/bin/bash", "-c"]

# アプリケーションをENTRYPOINTとして実行
ENTRYPOINT exec java -jar /app/cqrs-es-example-write-api-server.jar