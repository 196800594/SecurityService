use sqlx::MySqlPool;
#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}