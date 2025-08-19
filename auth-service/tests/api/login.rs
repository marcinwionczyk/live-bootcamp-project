use crate::helpers::{get_random_email, SignupRequest, TestApp};
use auth_service::routes::LoginRequest;
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let test_set = vec![
        (get_random_email(), "Password", true),
        (get_random_email(), "2short", false),
        ("wrongemail".to_string(), "Password", false),
        ("wrongemail".to_string(), "2short", false),
    ];
    for (email, password, credentials_are_ok) in test_set {
        if credentials_are_ok {
            let response = app
                .post_signup(&SignupRequest {
                    email: email.clone(),
                    password: password.to_string(),
                    requires_2_fa: false,
                })
                .await;
            assert_eq!(response.status().as_u16(), 201);
        }
        let response = app
            .post_login(&LoginRequest {
                email,
                password: password.to_string(),
            })
            .await;
        if credentials_are_ok {
            assert_eq!(response.status().as_u16(), 200);
        } else {
            assert_eq!(response.status().as_u16(), 422);
            assert_eq!(
                response
                    .json::<ErrorResponse>()
                    .await
                    .expect("Could not deserialize response body to ErrorResponse")
                    .error,
                "Unprocessable content".to_owned()
            );
        }
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_credentials() {
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = "Password".to_string();
    let wrong_password = "SomeOtherWord".to_string();
    // create a user with a valid email and password
    let response = app
        .post_signup(&SignupRequest {
            email: email.clone(),
            password: password.clone(),
            requires_2_fa: false,
        })
        .await;
    assert_eq!(response.status().as_u16(), 201);
    // login a user with a valid email and wrong password
    let response = app
        .post_login(&LoginRequest {
            email,
            password: wrong_password.clone(),
        })
        .await;
    assert_eq!(response.status().as_u16(), 401);
}
