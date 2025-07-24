use auth_service::Application;
use reqwest::{self, header};
use serde::{Deserialize, Serialize};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2_fa: bool,
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename(serialize = "loginAttemptId"))]
    pub login_attempt_id: String,
    #[serde(rename(serialize = "2FACode"))]
    pub two_factor_code: String,
}

#[derive(Serialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new(); // Create a Reqwest http client instance

        // Create new `TestApp` instance and return it
        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)
    pub async fn post_signup(&self, signup_request: SignupRequest) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(&signup_request)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login(&self, login_request: LoginRequest) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&login_request)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self, verify_2fa_request: Verify2FARequest) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&verify_2fa_request)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self, jwt_token: String) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .header(header::COOKIE, format!("jwt={}", jwt_token))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token(
        &self,
        verify_token_request: VerifyTokenRequest,
    ) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(&verify_token_request)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
