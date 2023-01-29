use crate::helpers::TestApp;
use uuid::Uuid;

#[tokio::test]
async fn fails_to_get_non_existent_poll() {
    let app = TestApp::new().await;
    let uuid = Uuid::new_v4();

    let response = app.get_poll(&uuid).await;
    assert_eq!(response.status().as_u16(), 404);
}
