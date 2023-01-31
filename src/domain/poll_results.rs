use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PollResults {
    pub poll_id: Uuid,
    pub choice_id: Uuid,
    pub vote_count: Option<i64>,
}
