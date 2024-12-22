use std::sync::Arc;
use sqlx::{MySqlPool};
use tokio::sync::{Mutex};
use crate::repositories::user_repository::UserRepositoryImpl;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<Mutex<UserService>>,
    pub mysql_pool: MySqlPool,
}

impl AppState {
    pub fn new(db_pool: MySqlPool) -> Self {
        let user_service = UserService::new(UserRepositoryImpl { pool: db_pool.clone() });
        Self { user_service: Arc::new(Mutex::new(user_service)), mysql_pool: db_pool }
    }
}