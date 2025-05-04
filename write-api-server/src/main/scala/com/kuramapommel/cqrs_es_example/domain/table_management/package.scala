package com.kuramapommel.cqrs_es_example.domain

import akka.serialization.jackson.JsonSerializable
import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonValue

package object table_management:
  sealed trait Event extends JsonSerializable with DomainEvent:
    def tableId: TableId

  object Event:
    final case class ReservationConfirmed(tableId: TableId, userId: String, reservationId: String) extends Event

  case class TableId @JsonCreator() (value: String) extends AnyVal:
    @JsonValue def getValue: String = value
