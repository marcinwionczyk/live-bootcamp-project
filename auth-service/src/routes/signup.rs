use axum::{extract::State, http::StatusCode, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{AppState, domain::{AuthAPIError, User}};
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

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;
    if !email.contains('@') || !email.contains('.') {
        return Err(AuthAPIError::InvalidCredentials);
    }
    if password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }
    let user = User::new(email.as_str(), password.as_str(), request.requires_2fa);
    let mut user_store = state.user_store.write().await;
    if user_store.get_user(email.as_str()).is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    user_store.add_user(user).map_err(|_| AuthAPIError::UnexpectedError)?;
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    Ok((StatusCode::CREATED, response))
}

