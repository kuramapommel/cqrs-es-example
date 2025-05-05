use crate::application_services::table_managements::reservation_confirm_service::ReservationConfirmService;
use crate::daos::table::TableDaoForMySQL;
use crate::event_deserializers::deserializer::{DomainEvent, TableManagementEvent};
use crate::handlers::HandlerResult;
use crate::{Error, Response};

use crate::handlers::event_handler::EventHandler;
use std::sync::Arc;

pub struct ReservationConfirmedHandler {
    service: Arc<ReservationConfirmService<TableDaoForMySQL>>,
}

impl ReservationConfirmedHandler {
    pub fn new(service: ReservationConfirmService<TableDaoForMySQL>) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}

impl EventHandler for ReservationConfirmedHandler {
    fn handle_or_none(&self, event: Arc<dyn DomainEvent>) -> Option<HandlerResult> {
        let table_management_event = event
            .as_any()
            .downcast_ref::<TableManagementEvent>()?
            .clone();

        let TableManagementEvent::ReservationConfirmed {
            table_id,
            user_id,
            reservation_id,
        } = table_management_event;

        tracing::info!("table_id: {}", table_id);

        let service = Arc::clone(&self.service);

        Some(Box::pin(async move {
            match service
                .on_reservation_confirmed(&table_id, &user_id, &reservation_id)
                .await
            {
                Ok(table_id) => Ok(Response {
                    status_code: 200,
                    body: table_id,
                }),
                Err(err) => Err(Error::from(err)),
            }
        }))
    }
}
