use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct AccounterCategory{
    pub category_id: i32,
    pub category_name: String,
    pub parent_id: Option<i32>,
    pub r#type : Option<i8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}