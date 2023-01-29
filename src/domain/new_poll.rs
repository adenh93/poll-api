use super::new_poll_choice::NewPollChoice;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct NewPoll {
    pub name: String,
    pub end_date: DateTime<Utc>,
    #[validate(length(min = 2))]
    pub choices: Vec<NewPollChoice>,
}
