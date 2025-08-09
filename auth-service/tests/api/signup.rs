use serde::{Deserialize, Serialize};
use auth_service::ErrorResponse;
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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    let app = TestApp::new().await;
    let input = [
        serde_json::json!({
        "email": "IncorrectEmailFormat",
        "password": "ThisIsALongPasswordThatIsMoreThanEightCharactersLong",
        "requires2FA": false
        }),
        serde_json::json!({
        "email": get_random_email(),
        "password": "pwd",
        "requires2FA": false
        })];
    for i in input.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400);
        assert_eq!(response.json::<ErrorResponse>().await.expect("Could not deserialize response body to ErrorResponse").error, "Invalid credentials".to_owned());
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code
    let app = TestApp::new().await;
    let email = get_random_email();
    let mut response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": "ThisIsALongPasswordThatIsMoreThanEightCharactersLong",
        "requires2FA": false
    })).await;
    assert_eq!(response.status().as_u16(), 201);
    response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": "ThisIsALongPasswordThatIsMoreThanEightCharactersLong",
        "requires2FA": false
    })).await;
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}