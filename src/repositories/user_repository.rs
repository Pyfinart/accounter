use crate::domain::user::User;
use anyhow::{Error, Result};
use sea_orm::prelude::async_trait;
use sqlx::MySqlPool;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_user_by_id(&self, id: i64) -> Result<User>;
    async fn create_user(&self, user: &User) -> Result<u64, String>;
    async fn find_user_by_username(&self, username: &String) -> Result<User>;
    async fn check_username_exists(&self, username: &String) -> Result<bool>;
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

    async fn find_user_by_username(&self, username: &String) -> Result<User> {
        let res = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = ?",
            username
        )
            .fetch_one(&self.pool)
            .await;
        match res {
            Ok(queryRes) => {
                let user = User {
                    user_id: queryRes.user_id,
                    username: queryRes.username,
                    password_hash: queryRes.password_hash,
                    create_time: queryRes.create_time,
                    update_time: queryRes.update_time,
                };
                Ok(user)
            }
            Err(err) => Err(Error::from(err))
        }
    }

    async fn check_username_exists(&self, username: &String) -> Result<bool> {
        let res = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE username = ?",
            username
        )
            .fetch_one(&self.pool)
            .await;
        match res {
            Ok(queryRes) => {
                let count = queryRes.count;
                Ok(count > 0)
            }
            Err(err) => Err(Error::from(err))
        }
    }
}