use axum::Router;
use axum::routing::{get, post};
use crate::app_state::AppState;
use crate::handlers::user_handler::{create_user_handler, user_login_handler};

pub fn create_app_router() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/create", post(create_user_handler))
        .route("/user/login", post(user_login_handler))
}
