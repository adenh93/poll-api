use crate::helpers::{generate_poll, pick_random_choice, TestApp};
use poll_api::domain::{CreatedPoll, Poll};

#[tokio::test]
async fn vote_poll_works_properly() {
    let app = TestApp::new().await;

    let generated_poll = generate_poll(5);
    let response = app.post_poll(&generated_poll).await;
    let uuid = response.json::<CreatedPoll>().await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    let choice = pick_random_choice(&poll.choices);
    let response = app.vote_poll(&poll.id, &choice).await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn fails_if_client_has_already_voted_in_election() {
    let app = TestApp::new().await;

    let generated_poll = generate_poll(5);
    let response = app.post_poll(&generated_poll).await;
    let uuid = response.json::<CreatedPoll>().await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    let choice = pick_random_choice(&poll.choices);
    let response = app.vote_poll(&poll.id, &choice).await;

    assert!(response.status().is_success());

    let response = app.vote_poll(&poll.id, &choice).await;

    assert!(response.status().is_client_error());
}
