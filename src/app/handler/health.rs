use axum::{http::StatusCode, response::IntoResponse};

pub async fn livez() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn readyz() -> impl IntoResponse {
    StatusCode::OK
}
