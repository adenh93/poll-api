use crate::helpers::{generate_poll, TestApp};
use poll_api::domain::{CreatedPoll, Poll};
use uuid::Uuid;

#[tokio::test]
async fn fails_to_get_non_existent_poll() {
    let app = TestApp::new().await;
    let uuid = Uuid::new_v4();

    let response = app.get_poll(&uuid).await;
    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn gets_existing_election() {
    let app = TestApp::new().await;

    let new_poll = generate_poll(3, false);
    let response = app.post_poll(&new_poll).await;
    let created_poll = response.json::<CreatedPoll>().await.unwrap();

    let response = app.get_poll(&created_poll.id).await;
    assert!(response.status().is_success());

    let poll = response.json::<Poll>().await.unwrap();
    assert_eq!(poll.id, created_poll.id);
}
