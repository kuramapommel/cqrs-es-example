package com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation

import akka.actor.typed.ActorRef
import akka.actor.typed.ActorSystem
import akka.actor.typed.Behavior
import akka.actor.typed.scaladsl.Behaviors
import akka.cluster.sharding.typed.ShardingEnvelope
import akka.cluster.sharding.typed.scaladsl.ClusterSharding
import akka.cluster.sharding.typed.scaladsl.Entity
import akka.persistence.typed.PersistenceId

object ShardedReservationActor:
  def apply()(using system: ActorSystem[?]): Behavior[ReservationActor.Command] =
    val sharding = ClusterSharding(system)
    val shardregion: ActorRef[ShardingEnvelope[ReservationActor.Command]] =
      sharding.init(
        Entity(ReservationActor.typeKey)(createBehavior =
          entityContext =>
            ReservationActor(() => PersistenceId(entityContext.entityTypeKey.name, entityContext.entityId))
        )
      )

    Behaviors.receiveMessage:
      case command =>
        shardregion ! ShardingEnvelope(command.reservationId.value, command)
        Behaviors.same
