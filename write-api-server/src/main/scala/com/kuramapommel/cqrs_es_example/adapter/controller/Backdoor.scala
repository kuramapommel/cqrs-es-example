package com.kuramapommel.cqrs_es_example.adapter.controller

import akka.http.scaladsl.model.StatusCodes
import akka.http.scaladsl.model.headers.HttpCookie
import akka.http.scaladsl.server.Directives.*
val routes = cors():
  // 動作確認用に必要なだけ
  pathPrefix("api" / "backdoor"):
    pathEnd:
      post:
        val userId = java.util.UUID.randomUUID().toString

        // HttpOnly な Cookie を作成
        val cookie = HttpCookie(
          name = "userId",
          value = userId,
          httpOnly = true,
          path = Some("/"),
          maxAge = Some(3600) // 秒（例: 1時間）
        )

        setCookie(cookie):
          complete((StatusCodes.OK, s"""{"userId":"$userId"}"""))
