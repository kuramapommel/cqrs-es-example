use serde::Deserialize;

use crate::application_services::reservations::reservation_make_service::ReservationMakeService;
use crate::daos::reservation::ReservationDao;
use crate::handlers::HandlerResult;
use crate::{Error, Response};

use crate::handlers::event_handler::EventHandler;
use std::sync::Arc;

pub struct ReservationConfirmedHandler<Dao> {
    service: Arc<ReservationMakeService<Dao>>,
}

impl<Dao> ReservationConfirmedHandler<Dao> {
    pub fn new(service: ReservationMakeService<Dao>) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(rename = "reservationId")]
    reservation_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "tableId")]
    table_id: String,
}

impl<Dao> EventHandler for ReservationConfirmedHandler<Dao>
where
    Dao: ReservationDao + Send + Sync + 'static,
{
    fn handle_or_none(&self, payload: &str, manifest: &str) -> Option<HandlerResult> {
        let (reservation_id, user_id, table_id) = match manifest {
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Confirmed" => {
                serde_json::from_str(payload)
                    .map(
                        |Payload {
                             reservation_id,
                             user_id,
                             table_id,
                         }| (reservation_id, user_id, table_id),
                    )
                    .expect("Failed to deserialize event payload")
            }
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

#[cfg(test)]
mod tests {
    use crate::daos::reservation::tests::ReservationDaoForMemory;

    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn マニフェストが_reservation_package_Event_Confirmed_の場合ハンドリングする() {
        let manifest =
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Confirmed"
                .to_string();
        let payload = r#"{
                            "reservationId":"test-reservation-id",
                            "userId":"test-user-id",
                            "tableId":"test-table-id"
                        }"#
        .to_string();

        let handler = ReservationConfirmedHandler::new(ReservationMakeService::new(
            ReservationDaoForMemory::new(),
        ));

        match handler.handle_or_none(&payload, &manifest) {
            Some(_) => assert!(true),
            None => assert!(false, "Expected to handle event"),
        }
    }
}
