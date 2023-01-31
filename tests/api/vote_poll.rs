use crate::helpers::{generate_poll, pick_random_choice, TestApp};
use poll_api::domain::{CreatedPoll, Poll};
use uuid::Uuid;

#[tokio::test]
async fn vote_poll_works_properly() {
    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, false);
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

    let generated_poll = generate_poll(5, false);
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

#[tokio::test]
async fn fails_if_attempting_to_vote_in_expired_poll() {
    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, true);
    let uuid = app.add_past_election(&generated_poll).await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    let choice = pick_random_choice(&poll.choices);
    let response = app.vote_poll(&poll.id, &choice).await;

    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn fails_if_voting_for_choice_not_belonging_to_poll() {
    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, false);
    let response = app.post_poll(&generated_poll).await;
    let uuid = response.json::<CreatedPoll>().await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    let choice = Uuid::new_v4();
    let response = app.vote_poll(&poll.id, &choice).await;

    assert!(response.status().is_client_error());
}
