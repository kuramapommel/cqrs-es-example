use serde::Deserialize;

use crate::application_services::reservations::reservation_cancel_service::ReservationCancelService;
use crate::daos::reservation::ReservationDao;
use crate::handlers::HandlerResult;
use crate::{Error, Response};

use crate::handlers::event_handler::EventHandler;
use std::sync::Arc;

pub struct ReservationCancelledHandler<Dao> {
    service: Arc<ReservationCancelService<Dao>>,
}

impl<Dao> ReservationCancelledHandler<Dao> {
    pub fn new(service: ReservationCancelService<Dao>) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(rename = "reservationId")]
    reservation_id: String,
}

impl<Dao> EventHandler for ReservationCancelledHandler<Dao>
where
    Dao: ReservationDao + Send + Sync + 'static,
{
    fn handle_or_none(&self, payload: &str, manifest: &str) -> Option<HandlerResult> {
        let reservation_id = match manifest {
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Cancelled" => {
                serde_json::from_str(payload)
                    .map(|Payload { reservation_id }| reservation_id)
                    .expect("Failed to deserialize event payload")
            }
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

#[cfg(test)]
mod tests {
    use crate::daos::reservation::tests::ReservationDaoForMemory;

    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn マニフェストが_reservation_package_Event_Cancelled_の場合ハンドリングする() {
        let manifest =
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Cancelled"
                .to_string();
        let payload = r#"{
                            "reservationId":"test-reservation-id"
                        }"#
        .to_string();

        let handler = ReservationCancelledHandler::new(ReservationCancelService::new(
            ReservationDaoForMemory::new(),
        ));

        match handler.handle_or_none(&payload, &manifest) {
            Some(_) => assert!(true),
            None => assert!(false, "Expected to handle event"),
        }
    }
}
