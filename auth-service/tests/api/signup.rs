use crate::helpers::{SignupRequest, TestApp};

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.
#[tokio::test]
async fn signup_return_200() {
    let app = TestApp::new().await;

    let response = app
        .post_signup(SignupRequest {
            email: "some@email.com".to_string(),
            password: "Password".to_string(),
            requires_2_fa: false,
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
}