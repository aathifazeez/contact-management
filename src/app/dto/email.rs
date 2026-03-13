use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailResponse {
    pub id: u32,
    pub person_id: u32,
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEmailRequest {
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEmailRequest {
    pub address: Option<String>,
    pub label: Option<String>,
}
