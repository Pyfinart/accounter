use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub data: u64,
    pub message: String,
}

#[axum_macros::debug_handler]
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<CreateUserResponse>) {
    let user_service = &state.user_service.lock().await;
    let result = user_service.create_user(req.username, req.password).await;
    match result {
        Ok(id) => {
            (StatusCode::CREATED, Json(CreateUserResponse { data: id, message: "success".to_string() }))
        }
        Err(msg) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateUserResponse { data: 0, message: msg }))
        }
    }
}