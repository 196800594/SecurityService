pub mod routes;
pub mod handlers;
pub mod services;
pub mod models;
pub mod utils;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use dotenvy::dotenv;
use std::env;
use models::pool::AppState;

#[tokio::main]
pub async fn main() ->Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .max_lifetime(std::time::Duration::from_secs(3600))
        .idle_timeout(std::time::Duration::from_secs(600))
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await?;
    let state = AppState{ pool };
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = routes::app_routes(state.clone()).with_state(state);
    axum::serve(listener, app).await?;

    Ok(())
}