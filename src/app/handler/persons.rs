use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::app::dto::person::{CreatePersonRequest, PersonResponse, UpdatePersonRequest};
use crate::app::state::AppState;
use crate::pkg::error::AppError;


pub async fn list_persons(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.person_service();
    let persons: Vec<PersonResponse> = svc.list_persons().await?;
    Ok((StatusCode::OK, Json(persons)))
}


pub async fn create_person(
    State(state): State<AppState>,
    Json(req): Json<CreatePersonRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.person_service();
    let person: PersonResponse = svc.create_person(req).await?;
    Ok((StatusCode::CREATED, Json(person)))
}


pub async fn get_person(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    println!("HIT get_person id={}", id);

    let svc = state.person_service();
    let person: PersonResponse = svc.get_person(id).await?;
    Ok((StatusCode::OK, Json(person)))
}


pub async fn update_person(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(req): Json<UpdatePersonRequest>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.person_service();
    let person: PersonResponse = svc.update_person(id, req).await?;
    Ok((StatusCode::OK, Json(person)))
}


pub async fn delete_person(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let svc = state.person_service();
    svc.delete_person(id).await?;
    Ok(StatusCode::NO_CONTENT)
}