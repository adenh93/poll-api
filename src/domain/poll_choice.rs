use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Type)]
pub struct PollChoice {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
