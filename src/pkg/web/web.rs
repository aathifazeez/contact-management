use axum::Router;
use tokio::net::TcpListener;

use crate::pkg::config::app_config::AppConfig;

pub async fn init_web(config: AppConfig, router: Router){
    let addr = config.address();
    let listener: TcpListener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address: {}", addr));
        
    println!("Contact Management Service running on http://{}", addr);
    println!("Health check → http://{}/livez", addr);
    println!("API base     → http://{}/api/v1", addr);

    
    axum::serve(listener, router).await.expect("Server crashed unexpectedly!");
    
}