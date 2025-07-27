use crate::helpers::{TestApp, VerifyTokenRequest, TOKEN};

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
