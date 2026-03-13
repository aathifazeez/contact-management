use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::app::dto::mobile::{CreateMobileRequest, MobileResponse, UpdateMobileRequest};
use crate::app::state::AppState;
use crate::pkg::error::AppError;

pub async fn list_mobiles(
    Path(person_id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.mobile_service();
    let mobiles: Vec<MobileResponse> = svc.list_mobiles_for_person(person_id).await?;
    Ok((StatusCode::OK, Json(mobiles)))
}

pub async fn create_mobile(
    Path(person_id): Path<u32>,
    State(state): State<AppState>,
    Json(req): Json<CreateMobileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.mobile_service();
    let mobile: MobileResponse = svc.create_mobile(person_id, req).await?;
    Ok((StatusCode::CREATED, Json(mobile)))
}

/// GET /mobiles/{id}
/// Returns a single mobile by its own ID (not person_id).
pub async fn get_mobile(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.mobile_service();
    let mobile: MobileResponse = svc.get_mobile(id).await?;
    Ok((StatusCode::OK, Json(mobile)))
}

pub async fn update_mobile(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(req): Json<UpdateMobileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.mobile_service();
    let mobile: MobileResponse = svc.update_mobile(id, req).await?;
    Ok((StatusCode::OK, Json(mobile)))
}

pub async fn delete_mobile(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.mobile_service();
    svc.delete_mobile(id).await?;
    Ok(StatusCode::NO_CONTENT)
}