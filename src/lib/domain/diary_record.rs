// src/lib/domain/data.rs

// dependencies
use chrono::prelude::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// struct which embodies the response data provided by the /data/diary endpoint
#[derive(Clone, Debug, Default, Deserialize, FromRow, Serialize, Validate)]
pub struct DiaryRecord {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(skip)]
    pub created_at: DateTime<Utc>,
    #[garde(skip)]
    pub updated_at: Option<DateTime<Utc>>,
    #[garde(alphanumeric)]
    pub band: String,
    #[garde(alphanumeric)]
    pub album: String,
    #[garde(alphanumeric)]
    pub thoughts: String,
}
