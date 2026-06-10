use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

mod logging;
mod models;
pub mod service;
mod web_api;

use service::TaskService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let service = TaskService::new().await?;
    let shared_service = Arc::new(service);

    let app = web_api::router(shared_service.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("API server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
