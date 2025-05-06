package com.kuramapommel.cqrs_es_example.adapter.controller

import akka.http.scaladsl.marshallers.sprayjson.SprayJsonSupport.*
import akka.http.scaladsl.model.StatusCodes
import akka.http.scaladsl.server.Directives.*
import akka.util.Timeout
import com.kuramapommel.cqrs_es_example.domain.reservation.Event
import com.kuramapommel.cqrs_es_example.usecase.ServiceContext
import com.kuramapommel.cqrs_es_example.usecase.reservation.ReservationUseCase
import spray.json.DefaultJsonProtocol.*
import spray.json.RootJsonFormat

object ReservationRoutes:
  final case class ReservationRequest(
      tableId: String
  )

  given reservationRequestJsonFormat: RootJsonFormat[ReservationRequest] = jsonFormat1(ReservationRequest.apply)

class ReservationRoutes(
    reservationUseCase: ReservationUseCase
)(using Timeout):
  import ReservationRoutes.*

  val routes =
    cors():
      cookie("userId"): userId =>
        given ServiceContext = ServiceContext(userId.value)

        pathPrefix("api" / "reservation"):
          pathEnd:
            post:
              entity(as[ReservationRequest]): reservationRequest =>
                onSuccess(reservationUseCase.execute(reservationRequest.tableId)):
                  case Event.Confirmed(reservationId, _, _) =>
                    complete((StatusCodes.OK, s"""{"reservation_id":"${reservationId.getValue}"}"""))
                  case error =>
                    complete((StatusCodes.InternalServerError, s"""{"message":"${error.toString()}"}"""))
