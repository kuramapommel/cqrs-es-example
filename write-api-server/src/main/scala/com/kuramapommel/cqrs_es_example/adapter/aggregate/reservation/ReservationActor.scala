package com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation

import akka.actor.typed.ActorRef
import akka.actor.typed.Behavior
import akka.cluster.sharding.typed.scaladsl.EntityTypeKey
import akka.persistence.typed.PersistenceId
import akka.persistence.typed.SnapshotSelectionCriteria
import akka.persistence.typed.scaladsl.Effect
import akka.persistence.typed.scaladsl.EventSourcedBehavior
import akka.serialization.jackson.CborSerializable
import com.kuramapommel.cqrs_es_example.domain.reservation.Event
import com.kuramapommel.cqrs_es_example.domain.reservation.Reservation
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationId

object ReservationActor:
  val typeKey: EntityTypeKey[Command] =
    EntityTypeKey[Command]("reservation")

  trait Command extends CborSerializable:
    def reservationId: ReservationId

  object Command:
    final case class Make(
        reservationId: ReservationId,
        userId: String,
        tableId: String,
        replyTo: ActorRef[Event]
    ) extends Command
    final case class Cancel(
        reservationId: ReservationId,
        replyTo: ActorRef[Event]
    ) extends Command

  def apply(createPersistenceId: () => PersistenceId): Behavior[Command] =
    val persistenceId = createPersistenceId()
    EventSourcedBehavior[Command, Event, Option[Reservation]](
      persistenceId = persistenceId,
      emptyState = None,
      commandHandler = {
        case (None, Command.Make(id, userId, tableId, replyTo)) =>
          val event = Event.Confirmed(id, userId, tableId)
          replyTo ! event
          Effect.persist(event)
        case (Some(reservation), Command.Cancel(id, replyTo)) =>
          val event = Event.Cancelled(id)
          replyTo ! event
          Effect.persist(event)
        case (_, _) =>
          Effect.unhandled
      },
      eventHandler = {
        case (state, Event.Confirmed(_, userId, tableId)) =>
          state.map(
            _.copy(userId = userId, tableId = tableId)
          )
        case (_, Event.Cancelled(_)) =>
          None
      }
    ).snapshotWhen:
      case (_, _, _) => true
    .withSnapshotSelectionCriteria(
        SnapshotSelectionCriteria.latest // 最新のスナップショットから復元
      )
