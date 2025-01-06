use crate::app_state::AppState;
use crate::errors::errs::AppError;
use crate::models::resp_req::{ApiResponse, CreateUserRequest, CreateUserResponse, EmptyResponse, LoginRequest};
use axum::extract::State;
use axum::Json;

#[axum_macros::debug_handler]
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<CreateUserResponse>>, AppError> {
    let user_service = &state.user_service.lock().await;
    let result = user_service.create_user(req.username, req.password).await;
    match result {
        Ok(id) => {
            Ok(Json(ApiResponse::success(CreateUserResponse { data: id, message: "success".to_string() })))
        }
        Err(msg) => Err(AppError::BadRequest(msg))
    }
}


#[axum_macros::debug_handler]
pub async fn user_login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<EmptyResponse>>, AppError> {
    let user_service = &state.user_service.lock().await;
    let result = user_service.user_login(req.username, req.password).await;
    match result {
        Ok(_) => {
            Ok(Json(ApiResponse::success(EmptyResponse)))
        }
        Err(msg) => Err(AppError::BadRequest(msg))
    }
}
