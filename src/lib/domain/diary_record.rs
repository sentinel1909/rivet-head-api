// src/lib/domain/data.rs

// dependencies
use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// struct which embodies the response data provided by the /data/diary endpoint
#[derive(Clone, Debug, Default, Deserialize, FromRow, Serialize)]
pub struct DiaryRecord {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub band: String,
    pub album: String,
    pub thoughts: String,
}
