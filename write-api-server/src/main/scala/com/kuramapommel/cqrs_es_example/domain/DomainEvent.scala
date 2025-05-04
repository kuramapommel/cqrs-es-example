package com.kuramapommel.cqrs_es_example.domain

trait DomainEvent

object DomainEvent:
  final case class DomainError(runtimeException: RuntimeException) extends DomainEvent:
    override def toString: String = runtimeException.getMessage
