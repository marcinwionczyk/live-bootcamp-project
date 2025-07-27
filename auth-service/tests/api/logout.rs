use crate::helpers::{TestApp, TOKEN};

#[tokio::test]
async fn logout_return_200() {
    let app = TestApp::new().await;
    let response = app.post_logout(TOKEN.to_string()).await;
    assert_eq!(response.status().as_u16(), 200);
}