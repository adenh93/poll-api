use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct PollVote {
    pub id: Uuid,
    pub poll_id: Uuid,
    pub choice_id: Uuid,
    pub created_at: DateTime<Utc>,
}
