use crate::daos::reservation::{DeleteReservation, ReservationDao};

pub struct ReservationCancelService<Dao> {
    dao: Dao,
}

impl<Dao> ReservationCancelService<Dao>
where
    Dao: ReservationDao,
{
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub async fn on_reservation_cancel(&self, reservation_id: &str) -> Result<String, String> {
        let reservation_id = self
            .dao
            .delete(DeleteReservation::new(reservation_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete reservation: {}", e);
                "Failed to delete reservation".to_string()
            })?;

        Ok(reservation_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daos::reservation::tests::ReservationDaoForMemory;

    #[tokio::test]
    async fn 予約キャンセル受け取ったら削除する() {
        let dao = ReservationDaoForMemory::new();

        let reservation_canecl_service = ReservationCancelService::new(dao);
        let reservation_id = "test_reservation_id";

        let result = reservation_canecl_service
            .on_reservation_cancel(reservation_id)
            .await;

        assert!(result.is_ok());
        let result_reservation_id: String = result.unwrap();
        assert_eq!(result_reservation_id, "test_reservation_id");
    }
}
