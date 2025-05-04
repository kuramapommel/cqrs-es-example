package com.kuramapommel.cqrs_es_example.adapter.aggregate.controller

import akka.actor.ActorSystem
import akka.actor.testkit.typed.scaladsl.ActorTestKit
import akka.http.scaladsl.marshallers.sprayjson.SprayJsonSupport.*
import akka.http.scaladsl.marshalling.Marshal
import akka.http.scaladsl.model.*
import akka.http.scaladsl.model.headers.Cookie
import akka.http.scaladsl.testkit.ScalatestRouteTest
import akka.util.Timeout
import com.kuramapommel.cqrs_es_example.adapter.controller.ReservationRoutes
import com.kuramapommel.cqrs_es_example.domain.DomainEvent.DomainError
import com.kuramapommel.cqrs_es_example.domain.reservation.Event
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationId
import com.kuramapommel.cqrs_es_example.usecase.ServiceContext
import com.kuramapommel.cqrs_es_example.usecase.reservation.ReservationUseCase
import org.scalatest.concurrent.ScalaFutures
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec
import scala.concurrent.Future

class ReservationRoutesSpec extends AnyWordSpec with Matchers with ScalaFutures with ScalatestRouteTest:
  lazy val testKit = ActorTestKit("test-write-api-server")
  given Timeout =
    Timeout.create(testKit.system.settings.config.getDuration("cqrs-es-example.routes.ask-timeout"))
  override def createActorSystem(): ActorSystem =
    testKit.system.classicSystem

  "ReservationRoutes" should {
    "顧客は予約することができる（POST: /api/reservation）" in {
      val userId = "test-user-id"
      val reservationId = ReservationId("test-reservation-id")

      val routes = new ReservationRoutes(
        new ReservationUseCase:
          override def execute(tableId: String)(using ctx: ServiceContext) =
            Future.successful(Event.Confirmed(reservationId, ctx.userId, tableId))
      ).routes

      val reservationRequest = ReservationRoutes.ReservationRequest(
        tableId = "test-table-id"
      )
      val request = Post("/api/reservation").withEntity(Marshal(reservationRequest).to[MessageEntity].futureValue)
      request ~> Cookie("userId" -> "test-user") ~> routes ~> check:
        status should ===(StatusCodes.OK)
        responseAs[String] should ===(s"""{"reservation_id":"${reservationId.value}"}""")
    }

    "DomainError が発生したとき, 500 を返す" in {
      val routes = new ReservationRoutes(
        new ReservationUseCase:
          override def execute(tableId: String)(using ctx: ServiceContext) =
            Future.successful(DomainError(new RuntimeException("test error")))
      ).routes

      val reservationRequest = ReservationRoutes.ReservationRequest(
        tableId = "test-table-id"
      )
      val request = Post("/api/reservation").withEntity(Marshal(reservationRequest).to[MessageEntity].futureValue)
      request ~> Cookie("userId" -> "test-user") ~> routes ~> check:
        status should ===(StatusCodes.InternalServerError)
        responseAs[String] should ===(s"""{"message":"test error"}""")
    }
  }
