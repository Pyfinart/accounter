use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Currency{
    pub currency_id: i32,
    pub currency_code: String,
    pub currency_name: String,
    pub currency_symbol: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}