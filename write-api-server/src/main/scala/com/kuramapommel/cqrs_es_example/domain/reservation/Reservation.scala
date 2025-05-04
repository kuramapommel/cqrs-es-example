package com.kuramapommel.cqrs_es_example.domain.reservation

case class Reservation(id: ReservationId, userId: String, tableId: String)
