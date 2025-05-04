package com.kuramapommel.cqrs_es_example.domain.reservation

trait ReservationIdFactory:
  def create(): ReservationId

object ReservationIdFactory:
  def apply(): ReservationIdFactory =
    new ReservationIdFactory:
      override def create(): ReservationId =
        ReservationId(java.util.UUID.randomUUID().toString)
