use rust_decimal::Decimal;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct AccounterTransaction {
    pub transaction_id: i64,
    pub user_id: i64,
    pub category_id: i32,
    pub currency_id: i32,
    pub transaction_type: i8,
    pub amount: Decimal,
    pub transaction_date: DateTime<Utc>,
    pub note: Option<String>,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}