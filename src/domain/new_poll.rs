use super::new_poll_choice::NewPollChoice;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct NewPoll {
    #[validate(length(
        min = 5,
        max = 100,
        message = "Poll name must be between 5 and 100 characters."
    ))]
    pub name: String,
    pub description: Option<String>,
    #[validate(custom(
        function = "end_date_greater_than_utc_now",
        message = "End date must be in the future."
    ))]
    pub end_date: DateTime<Utc>,
    #[validate(length(min = 2, max = 20, message = "Must provide between 2 to 20 choices."))]
    pub choices: Vec<NewPollChoice>,
}

fn end_date_greater_than_utc_now(end_date: &DateTime<Utc>) -> Result<(), ValidationError> {
    if *end_date < Utc::now() {
        return Err(ValidationError::new("end_date"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::chrono::en::DateTimeBefore;
    use fake::faker::lorem::en::Sentence;
    use fake::Fake;

    #[test]
    fn fails_to_validate_if_end_date_in_past() {
        let fake_sentence = Sentence(5..8);

        let choices = (0..3)
            .map(|_| NewPollChoice {
                name: fake_sentence.fake(),
            })
            .collect();

        let new_poll = NewPoll {
            name: fake_sentence.fake(),
            description: fake_sentence.fake(),
            end_date: DateTimeBefore(Utc::now()).fake(),
            choices,
        };

        assert!(new_poll.validate().is_err());
    }
}
