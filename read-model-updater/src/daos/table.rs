use std::future::Future;

use sqlx::{prelude::FromRow, MySqlPool};

pub trait TableDao {
    fn upsert(
        &self,
        payload: UpsertTable<'_>,
    ) -> impl Future<Output = anyhow::Result<TableEntity>> + Send;
}

pub struct TableDaoForMySQL {
    pool: MySqlPool,
}

impl TableDaoForMySQL {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl TableDao for TableDaoForMySQL {
    async fn upsert(&self, payload: UpsertTable<'_>) -> anyhow::Result<TableEntity> {
        let tx = self.pool.begin().await?;
        sqlx::query(
            r#"
            INSERT INTO tables (id, user_id, reservation_id)
            VALUES (?, ?, ?) AS new
            ON DUPLICATE KEY UPDATE
            reservation_id = new.reservation_id, reservation_id = new.reservation_id;
            "#,
        )
        .bind(payload.id)
        .bind(payload.user_id)
        .bind(payload.reservation_id)
        .execute(&self.pool)
        .await?;

        let entity = sqlx::query_as::<_, TableEntity>(
            r#"
            SELECT id, user_id, reservation_id
            FROM tables
            WHERE id = ?;
            "#,
        )
        .bind(payload.id)
        .fetch_one(&self.pool)
        .await?;

        tx.commit().await?;

        Ok(entity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct TableEntity {
    pub id: String,
    user_id: String,
    reservation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpsertTable<'a> {
    id: &'a str,
    user_id: &'a str,
    reservation_id: &'a str,
}

impl<'a> UpsertTable<'a> {
    pub fn new(id: &'a str, user_id: &'a str, table_id: &'a str) -> Self {
        Self {
            id,
            user_id,
            reservation_id: table_id,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockWriteGuard},
    };

    use super::*;

    impl TableEntity {
        pub fn new(id: &str, user_id: &str, table_id: &str) -> Self {
            Self {
                id: id.to_string(),
                user_id: user_id.to_string(),
                reservation_id: table_id.to_string(),
            }
        }

        pub fn answer(self, table_id: &str) -> Self {
            Self {
                reservation_id: table_id.to_string(),
                ..self
            }
        }
    }

    type InvitationDatas = HashMap<String, TableEntity>;

    pub struct TableDaoForMemory {
        store: Arc<RwLock<InvitationDatas>>,
    }

    impl TableDaoForMemory {
        pub fn new() -> Self {
            Self {
                store: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<InvitationDatas> {
            self.store.write().unwrap()
        }
    }

    impl TableDao for TableDaoForMemory {
        async fn upsert(&self, payload: UpsertTable<'_>) -> anyhow::Result<TableEntity> {
            let mut store = self.write_store_ref();
            let id = String::from(payload.id);
            let invitation = store
                .get(&id)
                .cloned()
                .map(|invitation| invitation.answer(payload.reservation_id))
                .unwrap_or_else(|| {
                    TableEntity::new(payload.id, payload.user_id, payload.reservation_id)
                });
            store.insert(id, invitation.clone());
            Ok(invitation)
        }
    }
}
