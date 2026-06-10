use crate::helpers::TestApp;

#[tokio::test]
async fn health_check_should_return_200() {
    let mut app = TestApp::new().await;

    let response = app.get_health_check().await;

    assert_eq!(response.status().as_u16(), 200);
    app.clean_up().await;
}
