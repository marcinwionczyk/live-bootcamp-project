use crate::helpers::{get_random_email, TestApp};


#[tokio::test]
async fn signup_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({ "email": get_random_email(), "password": "Password" }),
        serde_json::json!({ "email": get_random_email(), "requires2FA": "Nah. Nope."}),
        serde_json::json!({ "email": get_random_email(), "password": 21345235, "requires2FA": true}),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}