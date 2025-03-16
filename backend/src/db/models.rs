use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Page {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
    pub indexed_at: DateTime<Utc>,
}
