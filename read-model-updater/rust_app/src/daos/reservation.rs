use std::future::Future;

use sqlx::{prelude::FromRow, MySqlPool};

pub trait ReservationDao {
    fn upsert(
        &self,
        payload: UpsertReservation<'_>,
    ) -> impl Future<Output = anyhow::Result<ReservationEntity>> + Send;
    fn delete(
        &self,
        payload: DeleteReservation<'_>,
    ) -> impl Future<Output = anyhow::Result<String>> + Send;
}

pub struct ReservationDaoForMySQL {
    pool: MySqlPool,
}

impl ReservationDaoForMySQL {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl ReservationDao for ReservationDaoForMySQL {
    async fn upsert(&self, payload: UpsertReservation<'_>) -> anyhow::Result<ReservationEntity> {
        let tx = self.pool.begin().await?;
        sqlx::query(
            r#"
            INSERT INTO reservations (id, user_id, table_id)
            VALUES (?, ?, ?) AS new
            ON DUPLICATE KEY UPDATE
            table_id = new.table_id;
            "#,
        )
        .bind(payload.id)
        .bind(payload.user_id)
        .bind(payload.table_id)
        .execute(&self.pool)
        .await?;

        let entity = sqlx::query_as::<_, ReservationEntity>(
            r#"
            SELECT id, user_id, table_id
            FROM reservations
            WHERE id = ?;
            "#,
        )
        .bind(payload.id)
        .fetch_one(&self.pool)
        .await?;

        tx.commit().await?;

        Ok(entity)
    }

    async fn delete(&self, payload: DeleteReservation<'_>) -> anyhow::Result<String> {
        let tx = self.pool.begin().await?;
        sqlx::query(
            r#"
            DELETE FROM reservations
            WHERE id = ?;
            "#,
        )
        .bind(payload.id)
        .execute(&self.pool)
        .await?;

        tx.commit().await?;

        Ok(String::from(payload.id))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct ReservationEntity {
    pub id: String,
    user_id: String,
    table_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpsertReservation<'a> {
    id: &'a str,
    user_id: &'a str,
    table_id: &'a str,
}

impl<'a> UpsertReservation<'a> {
    pub fn new(id: &'a str, user_id: &'a str, table_id: &'a str) -> Self {
        Self {
            id,
            user_id,
            table_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteReservation<'a> {
    id: &'a str,
}

impl<'a> DeleteReservation<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockWriteGuard},
    };

    use super::*;

    impl ReservationEntity {
        pub fn new(id: &str, user_id: &str, table_id: &str) -> Self {
            Self {
                id: id.to_string(),
                user_id: user_id.to_string(),
                table_id: table_id.to_string(),
            }
        }

        pub fn answer(self, table_id: &str) -> Self {
            Self {
                table_id: table_id.to_string(),
                ..self
            }
        }
    }

    type InvitationDatas = HashMap<String, ReservationEntity>;

    pub struct ReservationDaoForMemory {
        store: Arc<RwLock<InvitationDatas>>,
    }

    impl ReservationDaoForMemory {
        pub fn new() -> Self {
            Self {
                store: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<InvitationDatas> {
            self.store.write().unwrap()
        }
    }

    impl ReservationDao for ReservationDaoForMemory {
        async fn upsert(
            &self,
            payload: UpsertReservation<'_>,
        ) -> anyhow::Result<ReservationEntity> {
            let mut store = self.write_store_ref();
            let id = String::from(payload.id);
            let invitation = store
                .get(&id)
                .cloned()
                .map(|invitation| invitation.answer(payload.table_id))
                .unwrap_or_else(|| {
                    ReservationEntity::new(payload.id, payload.user_id, payload.table_id)
                });
            store.insert(id, invitation.clone());
            Ok(invitation)
        }

        async fn delete(&self, payload: DeleteReservation<'_>) -> anyhow::Result<String> {
            let mut store = self.write_store_ref();
            store.remove(payload.id);
            Ok(String::from(payload.id))
        }
    }
}
