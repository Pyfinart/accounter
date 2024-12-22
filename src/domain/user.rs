use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}