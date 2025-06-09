use std::sync::Arc;

use crate::handlers::table_managements::reservation_confirmed_handler::ReservationConfirmedHandler;
use sqlx::MySqlPool;

use crate::{
    application_services::table_managements::reservation_confirm_service::ReservationConfirmService,
    daos::table::TableDaoForMySQL,
};

pub mod reservation_confirmed_handler;

pub fn create_table_management_handlers(
    pool: &MySqlPool,
) -> Vec<Arc<dyn crate::handlers::event_handler::EventHandler>> {
    vec![Arc::new(ReservationConfirmedHandler::new(
        ReservationConfirmService::new(TableDaoForMySQL::new(pool.clone())),
    ))]
}
