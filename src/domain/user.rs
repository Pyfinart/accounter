use sqlx::types::time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}