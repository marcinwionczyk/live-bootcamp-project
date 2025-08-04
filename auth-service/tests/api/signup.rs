use serde::{Deserialize, Serialize};
use crate::helpers::{get_random_email, TestApp};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

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

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let response = app.post_signup(&serde_json::json!({
        "email": get_random_email(),
        "password": "Password",
        "requires2FA": false
    })).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse { message: "User created successfully!".to_owned() };
    assert_eq!(response.json::<SignupResponse>().await.expect("Could not deserialize body to UserBody"), expected_response);
}