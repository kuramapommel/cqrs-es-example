package com.kuramapommel.cqrs_es_example.adapter.usecase.reservation

import akka.actor.Scheduler
import akka.actor.typed.ActorRef
import akka.actor.typed.ActorSystem
import akka.actor.typed.scaladsl.AskPattern.*
import akka.pattern.retry
import akka.stream.scaladsl.Flow
import akka.stream.scaladsl.Sink
import akka.stream.scaladsl.Source
import akka.util.Timeout
import com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation.ReservationActor.Command as ReservationCommand
import com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management.TableActor.Command as TableCommand
import com.kuramapommel.cqrs_es_example.domain.reservation.Event
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationId
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationIdFactory
import com.kuramapommel.cqrs_es_example.domain.table_management.TableId
import com.kuramapommel.cqrs_es_example.usecase.ServiceContext
import com.kuramapommel.cqrs_es_example.usecase.reservation.ReservationUseCase
import scala.concurrent.ExecutionContext
import scala.concurrent.Future
import scala.concurrent.duration.*
import scala.util.control.NonFatal

class ReservationUseCaseImpl(
    reservationIdFactory: ReservationIdFactory,
    reservationActor: ActorRef[ReservationCommand],
    tableActor: ActorRef[TableCommand]
)(using system: ActorSystem[?])
    extends ReservationUseCase:
  override def execute(tableId: String)(using ctx: ServiceContext): Timeout ?=> Future[Event] =
    given ExecutionContext = system.executionContext
    given Scheduler = system.classicSystem.scheduler

    val makeReservationFlow: Flow[ReservationId, Event, ?] =
      Flow[ReservationId]
        .mapAsync(1): reservationId =>
          reservationActor.ask(
            ReservationCommand.Make(reservationId, ctx.userId, tableId, _)
          )

    val confirmReservationFlow: Flow[Event, Event, ?] =
      Flow[Event]
        .mapAsync(1):
          case event @ Event.Confirmed(reservationId, userId, tableId) =>
            retry(
              attempt =
                () => tableActor.ask(TableCommand.ConfirmReservation(TableId(tableId), userId, reservationId.value, _)),
              attempts = 3,
              delay = 300.millis
            ).map(_ => event)
              .recoverWith:
                case NonFatal(e) =>
                  reservationActor.ask(ReservationCommand.Cancel(reservationId, _))
          case _ =>
            Future.failed(new RuntimeException("Invalid event"))

    Source
      .single(reservationIdFactory.create())
      .via(makeReservationFlow)
      .via(confirmReservationFlow)
      .runWith(Sink.head[Event])
