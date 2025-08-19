use crate::domain::{AuthAPIError, Email, Password};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email(request.email).parse()?;
    let password = Password(request.password).parse()?;
    let user_store = &state.user_store.read().await;
    match user_store.get_user(&Email(email)).await {
        Ok(user) => {
            if user.password.parse()? != password {
                Err(AuthAPIError::Unauthorized)
            } else {
                Ok(StatusCode::OK)
            }
        }
        Err(e) => Err(AuthAPIError::from(e)),
    }
}
