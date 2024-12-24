use crate::app_state::AppState;
use dotenvy::dotenv;
use sqlx::MySqlPool;
use std::env;
use std::net::SocketAddr;

mod routes;
mod app_state;
mod repositories;
mod services;
mod handlers;
mod domain;
mod errors;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db = MySqlPool::connect(&database_url).await?;
    let app_state = AppState::new(db);
    let app = routes::create_app_router().with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8900));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
    Ok(())
}
