package com.kuramapommel.cqrs_es_example.domain

import akka.serialization.jackson.JsonSerializable
import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonValue

package object reservation:

  sealed trait Event extends JsonSerializable with DomainEvent:
    def reservationId: ReservationId

  object Event:
    final case class Confirmed(
        reservationId: ReservationId,
        userId: String,
        tableId: String
    ) extends Event
    final case class Cancelled(reservationId: ReservationId) extends Event

  case class ReservationId @JsonCreator() (value: String) extends AnyVal:
    @JsonValue def getValue: String = value
