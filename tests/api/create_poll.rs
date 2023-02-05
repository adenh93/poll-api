use crate::helpers::{generate_poll, TestApp};
use actix_web::http::StatusCode;
use chrono::Utc;
use fake::faker::chrono::en::DateTimeBefore;
use fake::Fake;
use poll_api::domain::CreatedPoll;
use poll_api::errors::ValidationErrorResponse;

#[tokio::test]
async fn fails_if_name_is_too_short() {
    let app = TestApp::new().await;

    let mut poll = generate_poll(5, false);
    poll.name = (0..4).fake::<String>();

    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());

    let body = response.json::<ValidationErrorResponse>().await.unwrap();
    let first_error = body.field_errors.first().unwrap();

    assert_eq!(body.field_errors.len(), 1);
    assert_eq!(first_error.field, "name");
}

#[tokio::test]
async fn fails_if_name_is_too_long() {
    let app = TestApp::new().await;

    let mut poll = generate_poll(5, false);
    poll.name = (101).fake::<String>();

    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());

    let body = response.json::<ValidationErrorResponse>().await.unwrap();
    let first_error = body.field_errors.first().unwrap();

    assert_eq!(body.field_errors.len(), 1);
    assert_eq!(first_error.field, "name");
}

#[tokio::test]
async fn fails_if_no_choices_provided() {
    let app = TestApp::new().await;

    let poll = generate_poll(0, false);
    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());

    let body = response.json::<ValidationErrorResponse>().await.unwrap();
    let first_error = body.field_errors.first().unwrap();

    assert_eq!(body.field_errors.len(), 1);
    assert_eq!(first_error.field, "choices");
}

#[tokio::test]
async fn fails_if_two_few_choices_provided() {
    let app = TestApp::new().await;

    let poll = generate_poll(1, false);
    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());

    let body = response.json::<ValidationErrorResponse>().await.unwrap();
    let first_error = body.field_errors.first().unwrap();

    assert_eq!(body.field_errors.len(), 1);
    assert_eq!(first_error.field, "choices");
}

#[tokio::test]
async fn fails_if_too_many_choices_provided() {
    let app = TestApp::new().await;

    let poll = generate_poll(21, false);
    let response = app.post_poll(&poll).await;

    assert!(response.status().is_client_error());

    let body = response.json::<ValidationErrorResponse>().await.unwrap();
    let first_error = body.field_errors.first().unwrap();

    assert_eq!(body.field_errors.len(), 1);
    assert_eq!(first_error.field, "choices");
}

#[tokio::test]
async fn fails_if_end_date_in_past() {
    let app = TestApp::new().await;

    let mut poll = generate_poll(3, false);
    poll.end_date = DateTimeBefore(Utc::now()).fake();

    let response = app.post_poll(&poll).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.json::<ValidationErrorResponse>().await.unwrap();
    let first_error = body.field_errors.first().unwrap();

    assert_eq!(body.field_errors.len(), 1);
    assert_eq!(first_error.field, "end_date");
}

#[tokio::test]
async fn successfully_create_a_poll() {
    let app = TestApp::new().await;

    let poll = generate_poll(3, false);
    let response = app.post_poll(&poll).await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn returns_a_128_bit_uuid() {
    let app = TestApp::new().await;

    let poll = generate_poll(3, false);
    let response = app.post_poll(&poll).await;
    let created_poll = response.json::<CreatedPoll>().await.unwrap();

    assert_eq!(created_poll.id.to_string().len(), 36);
}
