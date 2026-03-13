pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}