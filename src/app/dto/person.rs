use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonResponse {
    pub id: u32,
    pub name: String,
    pub display_name: Option<String>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct CreatePersonRequest {
    pub name: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePersonRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
}
