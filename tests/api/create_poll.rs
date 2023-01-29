use crate::helpers::{generate_poll, TestApp};
use chrono::Utc;
use fake::faker::chrono::en::DateTimeBefore;
use fake::Fake;
use poll_api::domain::CreatedPoll;

#[tokio::test]
async fn fails_if_no_choices_provided() {
    let app = TestApp::new().await;

    let poll = generate_poll(0);
    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn fails_if_end_date_in_past() {
    let app = TestApp::new().await;

    let mut poll = generate_poll(3);
    poll.end_date = DateTimeBefore(Utc::now()).fake();

    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn successfully_create_a_poll() {
    let app = TestApp::new().await;

    let poll = generate_poll(3);
    let response = app.post_poll(&poll).await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn returns_a_128_bit_uuid() {
    let app = TestApp::new().await;

    let poll = generate_poll(3);
    let response = app.post_poll(&poll).await;
    let created_poll = response.json::<CreatedPoll>().await.unwrap();

    assert_eq!(created_poll.id.to_string().len(), 36);
}
