use std::collections::HashMap;
use crate::app::dto::email::{CreateEmailRequest, EmailResponse, UpdateEmailRequest};


#[derive(Debug, Default)]
pub struct EmailRepository {
    store: HashMap<u32, EmailResponse>,
    by_person: HashMap<u32, Vec<u32>>,
    next_id: u32,
}

impl EmailRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            by_person: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn find_by_person_id(&self, person_id: u32) -> Vec<EmailResponse> {
        self.by_person
            .get(&person_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.store.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn find_by_id(&self, id: u32) -> Option<EmailResponse> {
        self.store.get(&id).cloned()
    }

    pub fn create(&mut self, person_id: u32, req: CreateEmailRequest) -> EmailResponse {
        let id = self.next_id;
        self.next_id += 1;

        let email = EmailResponse {
            id,
            person_id,
            address: req.address,
            label: req.label,
        };

        self.store.insert(id, email.clone());
        self.by_person
            .entry(person_id)
            .or_insert_with(Vec::new)
            .push(id);

        email
    }

    pub fn update(&mut self, id: u32, req: UpdateEmailRequest) -> Option<EmailResponse> {
        let email = self.store.get_mut(&id)?;

        if let Some(address) = req.address {
            email.address = address;
        }
        if let Some(label) = req.label {
            email.label = Some(label);
        }

        Some(email.clone())
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(email) = self.store.remove(&id) {
            if let Some(ids) = self.by_person.get_mut(&email.person_id) {
                ids.retain(|&eid| eid != id);
            }
            true
        } else {
            false
        }
    }

    pub fn has_any_for_person(&self, person_id: u32) -> bool {
        self.by_person
            .get(&person_id)
            .map(|ids| !ids.is_empty())
            .unwrap_or(false)
    }
}