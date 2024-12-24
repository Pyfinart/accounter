use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use crate::models::resp_req::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    UnprocessableEntity(String),
    UnsupportedMediaType(String),
    TooManyRequests(String),
    ServiceUnavailable(String),
    GatewayTimeout(String),
    NotImplemented(String),
    MethodNotAllowed(String),
    NotAcceptable(String),
    RequestTimeout(String),
    LengthRequired(String),
    PreconditionFailed(String),
    PayloadTooLarge(String),
    UriTooLong(String),
    DatabaseErr(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, msg) = match self {
            AppError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, 1001, msg),
            AppError::InternalServerError(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, 1002, msg,),
            AppError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, 1003, msg),
            AppError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, 1004, msg),
            AppError::Forbidden(msg) => (axum::http::StatusCode::FORBIDDEN, 1005, msg),
            AppError::Conflict(msg) => (axum::http::StatusCode::CONFLICT, 1006, msg),
            AppError::UnprocessableEntity(msg) => (axum::http::StatusCode::UNPROCESSABLE_ENTITY, 1007, msg),
            AppError::UnsupportedMediaType(msg) => (axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE, 1008, msg),
            AppError::TooManyRequests(msg) => (axum::http::StatusCode::TOO_MANY_REQUESTS, 1009, msg),
            AppError::ServiceUnavailable(msg) => (axum::http::StatusCode::SERVICE_UNAVAILABLE, 1010, msg),
            AppError::GatewayTimeout(msg) => (axum::http::StatusCode::GATEWAY_TIMEOUT, 1011, msg),
            AppError::NotImplemented(msg) => (axum::http::StatusCode::NOT_IMPLEMENTED, 1012, msg),
            AppError::MethodNotAllowed(msg) => (axum::http::StatusCode::METHOD_NOT_ALLOWED, 1013, msg),
            AppError::NotAcceptable(msg) => (axum::http::StatusCode::NOT_ACCEPTABLE, 1014, msg),
            AppError::RequestTimeout(msg) => (axum::http::StatusCode::REQUEST_TIMEOUT, 1015, msg),
            AppError::LengthRequired(msg) => (axum::http::StatusCode::LENGTH_REQUIRED, 1016, msg),
            AppError::PreconditionFailed(msg) => (axum::http::StatusCode::PRECONDITION_FAILED, 1017, msg),
            AppError::PayloadTooLarge(msg) => (axum::http::StatusCode::PAYLOAD_TOO_LARGE, 1018, msg),
            AppError::UriTooLong(msg) => (axum::http::StatusCode::URI_TOO_LONG, 1019, msg),
            AppError::DatabaseErr(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, 1020, msg),
        };

        let err_resp = ApiResponse::<()>::error(code, &msg);
        (status, Json(err_resp)).into_response()
    }
}

// From trait，能够自动将其他错误类型（如 SQLx 错误）转换为AppError，简化错误处理
// 在Handler中遇到sqlx::Error直接使用 ? 操作符即可
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Record not found".into()),
            _ => AppError::DatabaseErr(format!("Database error: {:?}", err)),
        }
    }
}