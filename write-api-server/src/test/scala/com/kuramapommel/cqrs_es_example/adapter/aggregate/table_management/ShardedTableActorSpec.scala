package com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management

import akka.actor.testkit.typed.scaladsl.ActorTestKit
import akka.actor.testkit.typed.scaladsl.ScalaTestWithActorTestKit
import com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management.TableActor.Command
import com.kuramapommel.cqrs_es_example.domain.table_management.Event
import com.kuramapommel.cqrs_es_example.domain.table_management.TableId
import org.scalatest.wordspec.AnyWordSpecLike

class ShardedTableActorSpec
    extends ScalaTestWithActorTestKit(ActorTestKit("test-write-api-server"))
      with AnyWordSpecLike:

  "ShardedTableActor" should {
    """テーブル予約コマンドを受けると, 予約確定イベントを発行すること""" in {
      val tableId = TableId("test-table-id")
      val userId = "test-user-id"
      val reservationId = "test-reservation-id"
      val actor = testKit.spawn(ShardedTableActor())
      val probe = testKit.createTestProbe[Event]()

      actor ! Command.ConfirmReservation(tableId, userId, reservationId, probe.ref)
      probe.expectMessage(Event.ReservationConfirmed(tableId, userId, reservationId))
    }
  }
