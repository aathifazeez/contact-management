use std::collections::HashMap;
use crate::app::dto::person::{CreatePersonRequest, PersonResponse, UpdatePersonRequest};

#[derive(Debug, Default)]
pub struct PersonRepository {
    store: HashMap<u32, PersonResponse>,
    next_id: u32,
}

impl PersonRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn find_all(&self) -> Vec<PersonResponse> {
        self.store.values().cloned().collect()
    }

    pub fn find_by_id(&self, id: u32) -> Option<PersonResponse> {
        self.store.get(&id).cloned()
    }


    pub fn create(&mut self, req: CreatePersonRequest) -> PersonResponse {
        let id = self.next_id;
        self.next_id += 1;

        let person = PersonResponse {
            id,
            name: req.name,
            display_name: req.display_name,
        };

        self.store.insert(id, person.clone());
        person
    }

    pub fn update(&mut self, id: u32, req: UpdatePersonRequest) -> Option<PersonResponse> {
        let person = self.store.get_mut(&id)?;
        if let Some(name) = req.name {
            person.name = name;
        }
        if let Some(display_name) = req.display_name {
            person.display_name = Some(display_name);
        }
        Some(person.clone())
    }

    pub fn delete(&mut self, id: u32) -> bool {
        self.store.remove(&id).is_some()
    }

}