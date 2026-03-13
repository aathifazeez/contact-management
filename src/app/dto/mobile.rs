use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MobileResponse {
    pub id: u32,
    pub person_id: u32,
    pub number: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMobileRequest {
    pub number: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMobileRequest {
    pub number: Option<String>,
    pub label: Option<String>,
}