// src/lib/domain/appstate.rs

// dependencies
use sqlx::PgPool;

// struct to represent the application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub api_key: String,
}
