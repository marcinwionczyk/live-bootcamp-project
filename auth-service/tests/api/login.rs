use crate::helpers::{LoginRequest, TestApp};

#[tokio::test]
async fn login_return_200() {
    let app = TestApp::new().await;
    let response = app
        .post_login(LoginRequest {
            email: "some@email.com".to_string(),
            password: "Password".to_string(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
}