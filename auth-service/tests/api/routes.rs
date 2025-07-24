use crate::helpers::{LoginRequest, SignupRequest, TestApp, Verify2FARequest, VerifyTokenRequest};

static TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaXNTb2NpYWwiOnRyDWV9.4pcPyMD09olPSyXnrXCjTwXyr4BsezdI1AVTmud2fU4";

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

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

#[tokio::test]
async fn logout_return_200() {
    let app = TestApp::new().await;
    let response = app.post_logout(TOKEN.to_string()).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify2fa_return_200() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_2fa(Verify2FARequest{
            email: "some@email.com".to_string(),
            login_attempt_id: "1500100900".to_string(),
            two_factor_code: "123456".to_string(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_return_200() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_token(VerifyTokenRequest{
            token: TOKEN.to_string(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
}


