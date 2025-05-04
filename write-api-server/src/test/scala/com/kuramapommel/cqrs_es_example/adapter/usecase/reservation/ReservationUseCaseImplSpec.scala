package com.kuramapommel.cqrs_es_example.adapter.usecase.reservation

import akka.actor.testkit.typed.scaladsl.ActorTestKit
import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation.ReservationActor.Command as ReservationCommand
import com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management.TableActor.Command as TableCommand
import com.kuramapommel.cqrs_es_example.domain.reservation.Event as ReservationEvent
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationId
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationIdFactory
import com.kuramapommel.cqrs_es_example.domain.table_management.Event
import com.kuramapommel.cqrs_es_example.usecase.ServiceContext
import org.scalatest.wordspec.AnyWordSpecLike

class ReservationUseCaseImplSpec
    extends ScalaTestWithActorTestKit(ActorTestKit("test-write-api-server"))
      with AnyWordSpecLike:

  "ReservationUseCaseImpl" should {
    "予約に成功すると, テーブルを確保する" in {
      given ctx: ServiceContext = ServiceContext("test-user-id")

      val reservationId = ReservationId("test-reservation-id")
      val tableId = "test-table-id"

      val reservationProbe = testKit.createTestProbe[ReservationCommand]()
      val tableProbe = testKit.createTestProbe[TableCommand]()

      val reservationUseCase = new ReservationUseCaseImpl(
        reservationIdFactory = new ReservationIdFactory:
          override def create(): ReservationId = reservationId
        ,
        reservationActor = reservationProbe.ref,
        tableActor = tableProbe.ref
      )

      val resultFut = reservationUseCase.execute(tableId)

      val actualEvent = ReservationEvent.Confirmed(reservationId, ctx.userId, tableId)
      reservationProbe.expectMessageType[ReservationCommand.Make] match
        case ReservationCommand.Make(_reservationId, _userId, _tableId, replyTo) =>
          replyTo ! ReservationEvent.Confirmed(_reservationId, _userId, _tableId)

      tableProbe.expectMessageType[TableCommand.ConfirmReservation] match
        case TableCommand.ConfirmReservation(_tableId, _userId, _reservationId, replyTo) =>
          replyTo ! Event.ReservationConfirmed(_tableId, _userId, _reservationId)

      whenReady(resultFut): result =>
        result shouldBe actualEvent
    }

    "予約に成功したあとテーブルの確保に失敗すると, 予約を取り消す" in {
      import scala.concurrent.duration.*
      val maxDuration = 100000.millis

      given ctx: ServiceContext = ServiceContext("test-user-id")

      val reservationId = ReservationId("test-reservation-id")
      val tableId = "test-table-id"

      val reservationProbe = testKit.createTestProbe[ReservationCommand]()
      val tableProbe = testKit.createTestProbe[TableCommand]()

      val reservationUseCase = new ReservationUseCaseImpl(
        reservationIdFactory = new ReservationIdFactory:
          override def create(): ReservationId = reservationId
        ,
        reservationActor = reservationProbe.ref,
        tableActor = tableProbe.ref
      )

      val resultFut = reservationUseCase.execute(tableId)

      val actualEvent = ReservationEvent.Cancelled(reservationId)
      reservationProbe.expectMessageType[ReservationCommand.Make] match
        case ReservationCommand.Make(_reservationId, _userId, _tableId, replyTo) =>
          replyTo ! ReservationEvent.Confirmed(_reservationId, _userId, _tableId)

      // リトライ回数分のタイムアウトを発生させる
      tableProbe.expectMessageType[TableCommand.ConfirmReservation] match
        case TableCommand.ConfirmReservation(_tableId, _userId, _reservationId, replyTo) =>
          println("タイムアウト発生させるため返信しない１")

      tableProbe.expectMessageType[TableCommand.ConfirmReservation](maxDuration) match
        case TableCommand.ConfirmReservation(_tableId, _userId, _reservationId, replyTo) =>
          println("タイムアウト発生させるため返信しない２")

      tableProbe.expectMessageType[TableCommand.ConfirmReservation](maxDuration) match
        case TableCommand.ConfirmReservation(_tableId, _userId, _reservationId, replyTo) =>
          println("タイムアウト発生させるため返信しない３")

      reservationProbe.expectMessageType[ReservationCommand.Cancel](maxDuration) match
        case ReservationCommand.Cancel(_reservationId, replyTo) =>
          replyTo ! ReservationEvent.Cancelled(_reservationId)

      whenReady(resultFut): result =>
        result shouldBe actualEvent
    }
  }
