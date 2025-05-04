package com.kuramapommel.cqrs_es_example.domain.table_management

final case class Table(tableId: TableId, reservationOpt: Option[Reservation]):

  def reserve(reservationId: String, userId: String): Table =
    copy(reservationOpt = Some(Reservation(reservationId, userId)))

final case class Reservation(reservationId: String, userId: String)
