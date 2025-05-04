package com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation

import akka.actor.testkit.typed.scaladsl.ActorTestKit
import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation.ReservationActor.Command
import com.kuramapommel.cqrs_es_example.domain.reservation.Event
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationId
import org.scalatest.wordspec.AnyWordSpecLike

class ShardedReservationActorSpec
    extends ScalaTestWithActorTestKit(ActorTestKit("test-write-api-server"))
      with AnyWordSpecLike:

  "ShardedReservationActor" should {
    """予約コマンドを受けると, 予約確定イベントを発行すること""" in {
      val reservationId = ReservationId("test-reservation-id")
      val userId = "test-user-id"
      val tableId = "test-table-id"
      val actor = testKit.spawn(ShardedReservationActor())
      val probe = testKit.createTestProbe[Event]()

      actor ! Command.Make(reservationId, userId, tableId, probe.ref)
      probe.expectMessage(Event.Confirmed(reservationId, userId, tableId))
    }
  }
