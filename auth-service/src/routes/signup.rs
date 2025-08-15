use crate::domain::{Email, Password};
use crate::{
    domain::{AuthAPIError, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

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

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email(request.email)
        .parse()
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password(request.password)
        .parse()
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let user = User::new(email.as_str(), password.as_str(), request.requires_2fa);
    let mut user_store = state.user_store.write().await;
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    user_store
        .add_user(user)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    Ok((StatusCode::CREATED, response))
}
