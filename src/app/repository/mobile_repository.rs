use std::collections::HashMap;
use crate::app::dto::mobile::{CreateMobileRequest, MobileResponse, UpdateMobileRequest};


#[derive(Debug, Default)]
pub struct MobileRepository {
    store: HashMap<u32, MobileResponse>,     
    by_person: HashMap<u32, Vec<u32>>,        
    next_id: u32,
}

impl MobileRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            by_person: HashMap::new(),
            next_id: 1,
        }
    }

  
    pub fn find_by_person_id(&self, person_id: u32) -> Vec<MobileResponse> {
        self.by_person
            .get(&person_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.store.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    
    pub fn find_by_id(&self, id: u32) -> Option<MobileResponse> {
        self.store.get(&id).cloned()
    }

    pub fn create(&mut self, person_id: u32, req: CreateMobileRequest) -> MobileResponse {
        let id = self.next_id;
        self.next_id += 1;

        let mobile = MobileResponse {
            id,
            person_id,
            number: req.number,
            label: req.label,
        };

        self.store.insert(id, mobile.clone());

        self.by_person
            .entry(person_id)
            .or_insert_with(Vec::new)
            .push(id);

        mobile
    }

    pub fn update(&mut self, id: u32, req: UpdateMobileRequest) -> Option<MobileResponse> {
        let mobile = self.store.get_mut(&id)?;

        if let Some(number) = req.number {
            mobile.number = number;
        }
        if let Some(label) = req.label {
            mobile.label = Some(label);
        }

        Some(mobile.clone())
    }

    pub fn delete(&mut self, id: u32) -> bool {
        if let Some(mobile) = self.store.remove(&id) {
            if let Some(ids) = self.by_person.get_mut(&mobile.person_id) {
                ids.retain(|&mid| mid != id);
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