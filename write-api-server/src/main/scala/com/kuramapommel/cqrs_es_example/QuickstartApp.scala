package com.kuramapommel.cqrs_es_example

import akka.actor.typed.ActorSystem
import akka.actor.typed.scaladsl.Behaviors
import akka.http.scaladsl.Http
import akka.http.scaladsl.server.Route
import akka.management.cluster.bootstrap.ClusterBootstrap
import akka.management.scaladsl.AkkaManagement
import akka.util.Timeout
import com.kuramapommel.cqrs_es_example.adapter.aggregate.reservation.ShardedReservationActor
import com.kuramapommel.cqrs_es_example.adapter.aggregate.table_management.ShardedTableActor
import com.kuramapommel.cqrs_es_example.adapter.controller.ReservationRoutes
import com.kuramapommel.cqrs_es_example.adapter.usecase.reservation.ReservationUseCaseImpl
import com.kuramapommel.cqrs_es_example.domain.reservation.ReservationIdFactory
import scala.util.Failure
import scala.util.Success

//#main-class
object QuickstartApp:
  // #start-http-server
  private def startHttpServer(routes: Route)(implicit system: ActorSystem[?]): Unit =
    // Akka HTTP still needs a classic ActorSystem to start
    import system.executionContext

    val port = system.settings.config.getInt("my-app.server.port")
    val futureBinding = Http().newServerAt("0.0.0.0", port).bind(routes)
    futureBinding.onComplete:
      case Success(binding) =>
        val address = binding.localAddress
        system.log.info(s"Server online at http://${address.getHostString}:${address.getPort}/")
      case Failure(ex) =>
        system.log.error("Failed to bind HTTP endpoint, terminating system", ex)
        system.terminate()
  // #start-http-server
  def main(args: Array[String]): Unit =
    // #server-bootstrapping
    val rootBehavior = Behaviors.setup[Nothing]: context =>
      import context.system
      given Timeout = Timeout.create(context.system.settings.config.getDuration("cqrs-es-example.routes.ask-timeout"))

      val classicSystem = context.system.classicSystem

      val reservationActor = context.spawn(ShardedReservationActor(), "reservation-actor")
      val tableManagementActor = context.spawn(ShardedTableActor(), "table-actor")

      val reservationUseCase =
        new ReservationUseCaseImpl(ReservationIdFactory(), reservationActor, tableManagementActor)

      val reservationRoutes = new ReservationRoutes(reservationUseCase)

      startHttpServer(reservationRoutes.routes)

      if context.system.settings.config
          .getBoolean("cqrs-es-example.use-akka-management")
      then
        AkkaManagement.get(classicSystem).start()
        ClusterBootstrap.get(classicSystem).start()

      Behaviors.empty
    val system = ActorSystem[Nothing](rootBehavior, "write-api-server")
    // #server-bootstrapping
//#main-class
