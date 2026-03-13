use contact_management::app::route::build_router;
use contact_management::app::state::AppState;
use contact_management::pkg::config::app_config::AppConfig;
use contact_management::pkg::web::web::init_web;

#[tokio::main]
async fn main() {

    let config = AppConfig::new();

    let state = AppState::new();

    let app = build_router(state);

    init_web(config, app).await;

}