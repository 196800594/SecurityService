#[macro_use]
extern crate rust_i18n;
i18n!("langs", fallback = "en-US");
pub mod routes;
pub mod handlers;
pub mod services;
pub mod models;
pub mod utils;
mod errors;

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
        .max_lifetime(std::time::Duration::from_secs(1800))
        .idle_timeout(std::time::Duration::from_secs(600))
        .acquire_timeout(std::time::Duration::from_secs(5))
        .test_before_acquire(true)
        .connect(&database_url)
        .await?;
    let state = AppState{ pool };
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = routes::app_routes(state.clone()).with_state(state);
    axum::serve(listener, app).await?;

    Ok(())
}