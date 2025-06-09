use crate::daos::reservation::{ReservationDao, UpsertReservation};

pub struct ReservationMakeService<Dao> {
    dao: Dao,
}

impl<Dao> ReservationMakeService<Dao>
where
    Dao: ReservationDao,
{
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub async fn on_reservation_confirmed(
        &self,
        reservation_id: &str,
        user_id: &str,
        table_id: &str,
    ) -> Result<String, String> {
        let reservation_entity = self
            .dao
            .upsert(UpsertReservation::new(reservation_id, user_id, table_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to upsert reservation: {}", e);
                "Failed to upsert reservation".to_string()
            })?;

        Ok(reservation_entity.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daos::reservation::tests::ReservationDaoForMemory;

    #[tokio::test]
    async fn 予約を受け取ったら保存する() {
        let dao = ReservationDaoForMemory::new();

        let reservation_make_service = ReservationMakeService::new(dao);
        let reservation_id = "test_reservation_id";
        let user_id = "test_user_id";
        let table_id = "test_table_id";

        let result = reservation_make_service
            .on_reservation_confirmed(reservation_id, user_id, table_id)
            .await;

        assert!(result.is_ok());
        let result_reservation_id: String = result.unwrap();
        assert_eq!(result_reservation_id, "test_reservation_id");
    }
}
