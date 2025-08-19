use crate::helpers::{TestApp, Verify2FARequest};

#[tokio::test]
async fn verify2fa_return_200() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_2fa(Verify2FARequest {
            email: "some@email.com".to_string(),
            login_attempt_id: "1500100900".to_string(),
            two_factor_code: "123456".to_string(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 200);
}
