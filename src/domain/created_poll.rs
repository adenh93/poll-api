use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatedPoll {
    pub id: Uuid,
    pub name: String,
}
