use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::dto::mobile::{CreateMobileRequest, MobileResponse, UpdateMobileRequest};
use crate::app::repository::mobile_repository::MobileRepository;
use crate::app::repository::person_repository::PersonRepository;
use crate::pkg::error::AppError;


pub struct MobileService {
    mobiles: Arc<Mutex<MobileRepository>>,
    persons: Arc<Mutex<PersonRepository>>,
}

impl MobileService {
    pub fn new(
        mobiles: Arc<Mutex<MobileRepository>>,
        persons: Arc<Mutex<PersonRepository>>,
    ) -> Self {
        Self { mobiles, persons }
    }

    pub async fn list_mobiles_for_person(
        &self,
        person_id: u32,
    ) -> Result<Vec<MobileResponse>, AppError> {
        {
            let person_repo = self.persons.lock().await;
            if person_repo.find_by_id(person_id).is_none() {
                return Err(AppError::NotFound(format!(
                    "Person with id {} not found", person_id
                )));
            }
        }

        let repo = self.mobiles.lock().await;
        Ok(repo.find_by_person_id(person_id))
    }


    pub async fn get_mobile(&self, id: u32) -> Result<MobileResponse, AppError> {
        let repo = self.mobiles.lock().await;
        repo.find_by_id(id)
            .ok_or_else(|| AppError::NotFound(format!("Mobile with id {} not found", id)))
    }

    pub async fn create_mobile(
        &self,
        person_id: u32,
        req: CreateMobileRequest,
    ) -> Result<MobileResponse, AppError> {
        {
            let person_repo = self.persons.lock().await;
            if person_repo.find_by_id(person_id).is_none() {
                return Err(AppError::NotFound(format!(
                    "Person with id {} not found", person_id
                )));
            }
        }

        if req.number.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Mobile number cannot be empty".to_string(),
            ));
        }

        let mut repo = self.mobiles.lock().await;
        Ok(repo.create(person_id, req))
    }


    pub async fn update_mobile(
        &self,
        id: u32,
        req: UpdateMobileRequest,
    ) -> Result<MobileResponse, AppError> {
        if let Some(ref number) = req.number {
            if number.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Mobile number cannot be empty".to_string(),
                ));
            }
        }

        let mut repo = self.mobiles.lock().await;
        repo.update(id, req)
            .ok_or_else(|| AppError::NotFound(format!("Mobile with id {} not found", id)))
    }

    pub async fn delete_mobile(&self, id: u32) -> Result<(), AppError> {
        let mut repo = self.mobiles.lock().await;
        if repo.delete(id) {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Mobile with id {} not found", id)))
        }
    }
}