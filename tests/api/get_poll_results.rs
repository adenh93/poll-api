use crate::helpers::{generate_poll, TestApp};
use poll_api::domain::{CreatedPoll, Poll, PollResults};

#[tokio::test]
async fn gets_poll_results() {
    const VOTE_COUNT: usize = 100;

    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, true);
    let uuid = app.add_past_election(&generated_poll).await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    app.simulate_poll(&poll, VOTE_COUNT).await.unwrap();

    let response = app.get_poll_results(&uuid).await;
    assert!(response.status().is_success());
}

#[tokio::test]
async fn poll_results_add_up_to_vote_count() {
    const VOTE_COUNT: usize = 100;

    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, true);
    let uuid = app.add_past_election(&generated_poll).await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    app.simulate_poll(&poll, VOTE_COUNT).await.unwrap();

    let response = app.get_poll_results(&uuid).await;
    let results = response.json::<Vec<PollResults>>().await.unwrap();

    let vote_count: usize = results
        .iter()
        .fold(0, |acc, next| acc + next.vote_count.unwrap() as usize);

    assert_eq!(vote_count, VOTE_COUNT);
}

#[tokio::test]
async fn poll_results_are_in_desc_order_by_vote_count() {
    const VOTE_COUNT: usize = 1_000;

    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, true);
    let uuid = app.add_past_election(&generated_poll).await.unwrap().id;

    let response = app.get_poll(&uuid).await;
    let poll = response.json::<Poll>().await.unwrap();

    app.simulate_poll(&poll, VOTE_COUNT).await.unwrap();

    let response = app.get_poll_results(&uuid).await;
    let results = response.json::<Vec<PollResults>>().await.unwrap();
    let vote_counts: Vec<i64> = results.iter().map(|res| res.vote_count.unwrap()).collect();
    let is_in_desc_order = vote_counts.iter().is_sorted_by(|a, b| b.partial_cmp(a));

    assert!(is_in_desc_order);
}

#[tokio::test]
async fn fails_to_get_results_for_current_poll() {
    const VOTE_COUNT: usize = 10;

    let app = TestApp::new().await;

    let generated_poll = generate_poll(5, false);
    let response = app.post_poll(&generated_poll).await;
    let uuid = response.json::<CreatedPoll>().await.unwrap().id;

    let response = app.get_poll_results(&uuid).await;

    assert!(response.status().is_client_error());
}
