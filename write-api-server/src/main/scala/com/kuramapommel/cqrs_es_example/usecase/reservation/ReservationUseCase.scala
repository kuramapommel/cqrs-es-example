package com.kuramapommel.cqrs_es_example.usecase.reservation

import akka.util.Timeout
import com.kuramapommel.cqrs_es_example.domain.reservation.Event
import com.kuramapommel.cqrs_es_example.usecase.ServiceContext
import scala.concurrent.Future

trait ReservationUseCase:
  def execute(tableId: String)(using ctx: ServiceContext): Timeout ?=> Future[Event]
