use crate::domain::user::User;
use anyhow::Result;
use sea_orm::prelude::async_trait;
use sqlx::MySqlPool;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_user_by_id(&self, id: i64) -> Result<User>;
    async fn create_user(&self, user: &User) -> Result<u64, String>;
}

pub struct UserRepositoryImpl {
    pub(crate) pool: MySqlPool,
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_user_by_id(&self, id: i64) -> Result<User> {
        Err(anyhow::Error::msg("[find_user_by_id] not implemented"))
    }

    async fn create_user(&self, user: &User) -> Result<u64, String> {
        let res = sqlx::query!(
            "INSERT INTO users (username, password_hash) VALUES (?, ?)",
            user.username,
            user.password_hash
        )
            .execute(&self.pool)
            .await;
        match res {
            Ok(queryRes) => {
                let id = queryRes.last_insert_id();
                let affected_rows = queryRes.rows_affected();
                if affected_rows == 0 {
                    return Err("[create_user] insert failed".to_string());
                }
                Ok(id)
            }
            Err(err) => Err(format!("[create_user] insert failed, err: {err}"))
        }
    }
}