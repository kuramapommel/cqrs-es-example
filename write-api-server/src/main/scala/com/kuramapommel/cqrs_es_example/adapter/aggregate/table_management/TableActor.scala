package com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management

import akka.actor.typed.ActorRef
import akka.actor.typed.Behavior
import akka.cluster.sharding.typed.scaladsl.EntityTypeKey
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.SnapshotSelectionCriteria
import akka.persistence.typed.scaladsl.Effect
import akka.persistence.typed.scaladsl.EventSourcedBehavior
import akka.serialization.jackson.CborSerializable
import com.kuramapommel.cqrs_es_example.domain.table_management.Event
import com.kuramapommel.cqrs_es_example.domain.table_management.Table
import com.kuramapommel.cqrs_es_example.domain.table_management.TableId

object TableActor:
  val typeKey: EntityTypeKey[Command] =
    EntityTypeKey[Command]("table")

  trait Command() extends CborSerializable:
    def tableId: TableId

  object Command:
    final case class ConfirmReservation(
        tableId: TableId,
        userId: String,
        reservationId: String,
        replyTo: ActorRef[Event]
    ) extends Command()

  def apply(id: TableId, createPersistenceId: () => PersistenceId): Behavior[Command] =
    val persistenceId = createPersistenceId()
    EventSourcedBehavior[Command, Event, Table](
      persistenceId = persistenceId,
      emptyState = Table(id, None),
      commandHandler = {
        case (state, Command.ConfirmReservation(_, userId, reservationId, replyTo)) =>
          val event = Event.ReservationConfirmed(state.tableId, userId, reservationId)
          replyTo ! event
          Effect.persist(event)
        case (_, _) =>
          Effect.unhandled
      },
      eventHandler = { case (state, Event.ReservationConfirmed(tableId, userId, reservationId)) =>
        state.reserve(userId, reservationId)
      }
    ).snapshotWhen:
      case (_, _, _) => true
    .withSnapshotSelectionCriteria(
        SnapshotSelectionCriteria.latest // 最新のスナップショットから復元
      )
