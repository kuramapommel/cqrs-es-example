use crate::daos::table::{TableDao, UpsertTable};

pub struct ReservationConfirmService<Dao> {
    dao: Dao,
}

impl<Dao> ReservationConfirmService<Dao>
where
    Dao: TableDao,
{
    pub fn new(dao: Dao) -> Self {
        Self { dao }
    }

    pub async fn on_reservation_confirmed(
        &self,
        table_id: &str,
        user_id: &str,
        reservation_id: &str,
    ) -> Result<String, String> {
        let table_entity = self
            .dao
            .upsert(UpsertTable::new(table_id, user_id, reservation_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to upsert table: {}", e);
                "Failed to upsert table".to_string()
            })?;

        Ok(table_entity.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daos::table::tests::TableDaoForMemory;

    #[tokio::test]
    async fn 予約確定を受け取ったらテーブル情報を更新する() {
        let dao = TableDaoForMemory::new();

        let reservation_confirm_service = ReservationConfirmService::new(dao);
        let table_id = "test_table_id";
        let user_id = "test_user_id";
        let reservation_id = "test_reservation_id";

        let result = reservation_confirm_service
            .on_reservation_confirmed(table_id, user_id, reservation_id)
            .await;

        assert!(result.is_ok());
        let result_table_id: String = result.unwrap();
        assert_eq!(result_table_id, "test_table_id");
    }
}
