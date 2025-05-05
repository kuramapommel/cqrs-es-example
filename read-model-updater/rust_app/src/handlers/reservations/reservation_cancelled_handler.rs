use crate::application_services::reservations::reservation_cancel_service::ReservationCancelService;
use crate::daos::reservation::ReservationDaoForMySQL;
use crate::event_deserializers::deserializer::{DomainEvent, ReservationEvent};
use crate::handlers::HandlerResult;
use crate::{Error, Response};

use crate::handlers::event_handler::EventHandler;
use std::sync::Arc;

pub struct ReservationCancelledHandler {
    service: Arc<ReservationCancelService<ReservationDaoForMySQL>>,
}

impl ReservationCancelledHandler {
    pub fn new(service: ReservationCancelService<ReservationDaoForMySQL>) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}

impl EventHandler for ReservationCancelledHandler {
    fn handle_or_none(&self, event: Arc<dyn DomainEvent>) -> Option<HandlerResult> {
        let reservation_id = match event.as_any().downcast_ref::<ReservationEvent>()?.clone() {
            ReservationEvent::ReservationCancelled { reservation_id } => reservation_id,
            _ => return None,
        };

        tracing::info!("reservation_id: {}", reservation_id);

        let service = Arc::clone(&self.service);

        Some(Box::pin(async move {
            match service.on_reservation_cancel(&reservation_id).await {
                Ok(reservation_id) => Ok(Response {
                    status_code: 200,
                    body: reservation_id,
                }),
                Err(err) => Err(Error::from(err)),
            }
        }))
    }
}
