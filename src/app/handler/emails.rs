use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::app::dto::email::{CreateEmailRequest, EmailResponse, UpdateEmailRequest};
use crate::app::state::AppState;
use crate::pkg::error::AppError;

pub async fn list_emails(
    Path(person_id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.email_service();
    let emails: Vec<EmailResponse> = svc.list_emails_for_person(person_id).await?;
    Ok((StatusCode::OK, Json(emails)))
}

pub async fn create_email(
    Path(person_id): Path<u32>,
    State(state): State<AppState>,
    Json(req): Json<CreateEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.email_service();
    let email: EmailResponse = svc.create_email(person_id, req).await?;
    Ok((StatusCode::CREATED, Json(email)))
}

pub async fn get_email(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.email_service();
    let email: EmailResponse = svc.get_email(id).await?;
    Ok((StatusCode::OK, Json(email)))
}

pub async fn update_email(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(req): Json<UpdateEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.email_service();
    let email: EmailResponse = svc.update_email(id, req).await?;
    Ok((StatusCode::OK, Json(email)))
}

pub async fn delete_email(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.email_service();
    svc.delete_email(id).await?;
    Ok(StatusCode::NO_CONTENT)
}