package com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management

import akka.actor.typed.ActorRef
import akka.actor.typed.ActorSystem
import akka.actor.typed.Behavior
import akka.actor.typed.scaladsl.Behaviors
import akka.cluster.sharding.typed.ShardingEnvelope
import akka.cluster.sharding.typed.scaladsl.ClusterSharding
import akka.cluster.sharding.typed.scaladsl.Entity
import akka.persistence.typed.PersistenceId
import com.kuramapommel.cqrs_es_example.domain.table_management.TableId

object ShardedTableActor:
  def apply()(using system: ActorSystem[?]): Behavior[TableActor.Command] =
    val sharding = ClusterSharding(system)
    val shardregion: ActorRef[ShardingEnvelope[TableActor.Command]] =
      sharding.init(
        Entity(TableActor.typeKey)(createBehavior =
          entityContext =>
            TableActor(
              TableId(entityContext.entityId),
              () => PersistenceId(entityContext.entityTypeKey.name, entityContext.entityId)
            )
        )
      )

    Behaviors.receiveMessage:
      case command =>
        shardregion ! ShardingEnvelope(command.tableId.value, command)
        Behaviors.same
