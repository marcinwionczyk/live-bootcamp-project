use axum::{extract::State, http::StatusCode, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{AppState, domain::User};
#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}


#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(state: State<AppState>, Json(request): Json<SignupRequest>) -> impl IntoResponse {
    let user = User { email: request.email, password: request.password, requires_2fa: request.requires_2fa };
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user).unwrap();
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    (StatusCode::CREATED, response)
}

