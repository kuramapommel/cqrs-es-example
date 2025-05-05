use serde::Deserialize;

use crate::application_services::table_managements::reservation_confirm_service::ReservationConfirmService;
use crate::daos::table::TableDao;
use crate::handlers::HandlerResult;
use crate::{Error, Response};

use crate::handlers::event_handler::EventHandler;
use std::sync::Arc;

pub struct ReservationConfirmedHandler<Dao> {
    service: Arc<ReservationConfirmService<Dao>>,
}

impl<Dao> ReservationConfirmedHandler<Dao> {
    pub fn new(service: ReservationConfirmService<Dao>) -> Self {
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
    Dao: TableDao + Send + Sync + 'static,
{
    fn handle_or_none(&self, payload: &str, manifest: &str) -> Option<HandlerResult> {
        let (table_id, user_id, reservation_id) = match manifest {
            "com.kuramapommel.cqrs_es_example.domain.table_management.package$Event$ReservationConfirmed" => {
                serde_json::from_str(payload)
                    .map(
                        |Payload {
                            table_id,
                            user_id,
                            reservation_id,
                         }| (table_id, user_id, reservation_id),
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
    use crate::daos::table::tests::TableDaoForMemory;

    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn マニフェストが_tablemanagement_package_Event_ReservationConfirmed_の場合ハンドリングする() {
        let manifest =
            "com.kuramapommel.cqrs_es_example.domain.table_management.package$Event$ReservationConfirmed"
                .to_string();
        let payload = r#"{
                            "tableId":"test-table-id",
                            "userId":"test-user-id",
                            "reservationId":"test-reservation-id"
                        }"#
        .to_string();

        let handler = ReservationConfirmedHandler::new(ReservationConfirmService::new(
            TableDaoForMemory::new(),
        ));

        match handler.handle_or_none(&payload, &manifest) {
            Some(_) => assert!(true),
            None => assert!(false, "Expected to handle event"),
        }
    }
}
