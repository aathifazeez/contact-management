use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::dto::person::{CreatePersonRequest, PersonResponse, UpdatePersonRequest};
use crate::app::repository::email_repository::EmailRepository;
use crate::app::repository::mobile_repository::MobileRepository;
use crate::app::repository::person_repository::PersonRepository;
use crate::pkg::error::AppError;

pub struct PersonService {
    persons: Arc<Mutex<PersonRepository>>,
    mobiles: Arc<Mutex<MobileRepository>>,
    emails: Arc<Mutex<EmailRepository>>,
}

impl PersonService {
    pub fn new(
        persons: Arc<Mutex<PersonRepository>>,
        mobiles: Arc<Mutex<MobileRepository>>,
        emails: Arc<Mutex<EmailRepository>>,
    ) -> Self {
        Self { persons, mobiles, emails }
    }

    pub async fn list_persons(&self) -> Result<Vec<PersonResponse>, AppError> {
        let repo = self.persons.lock().await;
        Ok(repo.find_all())
    }

    pub async fn get_person(&self, id: u32) -> Result<PersonResponse, AppError> {
        let repo = self.persons.lock().await;
        println!("FIND id={} all={:?}", id, repo.find_all());

        repo.find_by_id(id)
            .ok_or_else(|| AppError::NotFound(format!("Person with id {} not found", id)))
    }

    
    pub async fn create_person(
        &self,
        req: CreatePersonRequest,
    ) -> Result<PersonResponse, AppError> {
        if req.name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Person name cannot be empty".to_string(),
            ));
        }

        let mut repo = self.persons.lock().await;
        Ok(repo.create(req))
    }

   
    pub async fn update_person(
        &self,
        id: u32,
        req: UpdatePersonRequest,
    ) -> Result<PersonResponse, AppError> {
        if let Some(ref name) = req.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Person name cannot be empty".to_string(),
                ));
            }
        }

        let mut repo = self.persons.lock().await;
        repo.update(id, req)
            .ok_or_else(|| AppError::NotFound(format!("Person with id {} not found", id)))
    }

   
    pub async fn delete_person(&self, id: u32) -> Result<(), AppError> {
        {
            let repo = self.persons.lock().await;
            if repo.find_by_id(id).is_none() {
                return Err(AppError::NotFound(format!(
                    "Person with id {} not found", id
                )));
            }
        } 

        {
            let mobile_repo = self.mobiles.lock().await;
            if mobile_repo.has_any_for_person(id) {
                return Err(AppError::Conflict(format!(
                    "Cannot delete person {}: they still have mobile numbers. \
                     Delete all mobiles first.",
                    id
                )));
            }
        } 

        {
            let email_repo = self.emails.lock().await;
            if email_repo.has_any_for_person(id) {
                return Err(AppError::Conflict(format!(
                    "Cannot delete person {}: they still have email addresses. \
                     Delete all emails first.",
                    id
                )));
            }
        } 

        let mut repo = self.persons.lock().await;
        repo.delete(id);
        Ok(())
    }
}