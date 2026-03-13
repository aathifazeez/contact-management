use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::dto::email::{CreateEmailRequest, EmailResponse, UpdateEmailRequest};
use crate::app::repository::email_repository::EmailRepository;
use crate::app::repository::person_repository::PersonRepository;
use crate::pkg::error::AppError;


pub struct EmailService {
    emails: Arc<Mutex<EmailRepository>>,
    persons: Arc<Mutex<PersonRepository>>,
}

impl EmailService {
    pub fn new(
        emails: Arc<Mutex<EmailRepository>>,
        persons: Arc<Mutex<PersonRepository>>,
    ) -> Self {
        Self { emails, persons }
    }
    pub async fn list_emails_for_person(
        &self,
        person_id: u32,
    ) -> Result<Vec<EmailResponse>, AppError> {
        {
            let person_repo = self.persons.lock().await;
            if person_repo.find_by_id(person_id).is_none() {
                return Err(AppError::NotFound(format!(
                    "Person with id {} not found", person_id
                )));
            }
        }

        let repo = self.emails.lock().await;
        Ok(repo.find_by_person_id(person_id))
    }

    pub async fn get_email(&self, id: u32) -> Result<EmailResponse, AppError> {
        let repo = self.emails.lock().await;
        repo.find_by_id(id)
            .ok_or_else(|| AppError::NotFound(format!("Email with id {} not found", id)))
    }

    pub async fn create_email(
        &self,
        person_id: u32,
        req: CreateEmailRequest,
    ) -> Result<EmailResponse, AppError> {
        {
            let person_repo = self.persons.lock().await;
            if person_repo.find_by_id(person_id).is_none() {
                return Err(AppError::NotFound(format!(
                    "Person with id {} not found", person_id
                )));
            }
        }

        if req.address.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Email address cannot be empty".to_string(),
            ));
        }

        if !req.address.contains('@') {
            return Err(AppError::ValidationError(format!(
                "'{}' is not a valid email address (missing '@')",
                req.address
            )));
        }

        let mut repo = self.emails.lock().await;
        Ok(repo.create(person_id, req))
    }

   
    pub async fn update_email(
        &self,
        id: u32,
        req: UpdateEmailRequest,
    ) -> Result<EmailResponse, AppError> {
        if let Some(ref address) = req.address {
            if address.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Email address cannot be empty".to_string(),
                ));
            }
            if !address.contains('@') {
                return Err(AppError::ValidationError(format!(
                    "'{}' is not a valid email address (missing '@')",
                    address
                )));
            }
        }

        let mut repo = self.emails.lock().await;
        repo.update(id, req)
            .ok_or_else(|| AppError::NotFound(format!("Email with id {} not found", id)))
    }

    pub async fn delete_email(&self, id: u32) -> Result<(), AppError> {
        let mut repo = self.emails.lock().await;
        if repo.delete(id) {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Email with id {} not found", id)))
        }
    }
}