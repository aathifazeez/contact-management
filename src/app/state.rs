use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::repository::{email_repository::EmailRepository, mobile_repository::MobileRepository, person_repository::PersonRepository};
use crate::app::service::person_service::PersonService;
use crate::app::service::mobile_service::MobileService;
use crate::app::service::email_service::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub persons: Arc<Mutex<PersonRepository>>,
    pub mobiles: Arc<Mutex<MobileRepository>>,
    pub emails: Arc<Mutex<EmailRepository>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            persons: Arc::new(Mutex::new(PersonRepository::new())),
            mobiles: Arc::new(Mutex::new(MobileRepository::new())),
            emails: Arc::new(Mutex::new(EmailRepository::new())),
        }
    }

    pub fn person_service(&self) -> PersonService {
        PersonService::new(
            self.persons.clone(),
            self.mobiles.clone(),
            self.emails.clone(),
        )
    }

    pub fn mobile_service(&self) -> MobileService {
        MobileService::new(
            self.mobiles.clone(),
            self.persons.clone(),
        )
    }

    pub fn email_service(&self) -> EmailService {
        EmailService::new(
            self.emails.clone(),
            self.persons.clone(),
        )
    }
}
