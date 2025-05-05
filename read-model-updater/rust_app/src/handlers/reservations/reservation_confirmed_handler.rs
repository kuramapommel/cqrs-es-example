use crate::application_services::reservations::reservation_make_service::ReservationMakeService;
use crate::daos::reservation::ReservationDaoForMySQL;
use crate::event_deserializers::deserializer::{DomainEvent, ReservationEvent};
use crate::handlers::HandlerResult;
use crate::{Error, Response};

use crate::handlers::event_handler::EventHandler;
use std::sync::Arc;

pub struct ReservationConfirmedHandler {
    service: Arc<ReservationMakeService<ReservationDaoForMySQL>>,
}

impl ReservationConfirmedHandler {
    pub fn new(service: ReservationMakeService<ReservationDaoForMySQL>) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}

impl EventHandler for ReservationConfirmedHandler {
    fn handle_or_none(&self, event: Arc<dyn DomainEvent>) -> Option<HandlerResult> {
        let (reservation_id, user_id, table_id) =
            match event.as_any().downcast_ref::<ReservationEvent>()?.clone() {
                ReservationEvent::ReservationConfirmed {
                    reservation_id,
                    user_id,
                    table_id,
                } => (reservation_id, user_id, table_id),
                _ => return None,
            };

        tracing::info!(
            "reservation_id: {}, user_id: {}, table_id: {}",
            reservation_id,
            user_id,
            table_id
        );

        let service = Arc::clone(&self.service);

        Some(Box::pin(async move {
            match service
                .on_reservation_confirmed(&reservation_id, &user_id, &table_id)
                .await
            {
                Ok(reservation_id) => Ok(Response {
                    status_code: 200,
                    body: reservation_id,
                }),
                Err(err) => Err(Error::from(err)),
            }
        }))
    }
}
