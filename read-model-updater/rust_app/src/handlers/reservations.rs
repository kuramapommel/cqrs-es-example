use std::sync::Arc;

use reservation_cancelled_handler::ReservationCancelledHandler;
use reservation_confirmed_handler::ReservationConfirmedHandler;
use sqlx::MySqlPool;

use crate::{
    application_services::reservations::{
        reservation_cancel_service::ReservationCancelService,
        reservation_make_service::ReservationMakeService,
    },
    daos::reservation::ReservationDaoForMySQL,
};

use super::event_handler::EventHandler;

pub mod reservation_cancelled_handler;
pub mod reservation_confirmed_handler;

pub fn create_reservation_handlers(pool: &MySqlPool) -> Vec<Arc<dyn EventHandler>> {
    vec![
        // 予約コンテキスト
        Arc::new(ReservationConfirmedHandler::new(
            ReservationMakeService::new(ReservationDaoForMySQL::new(pool.clone())),
        )),
        Arc::new(ReservationCancelledHandler::new(
            ReservationCancelService::new(ReservationDaoForMySQL::new(pool.clone())),
        )),
    ]
}
