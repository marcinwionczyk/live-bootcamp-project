use crate::services::hasmap_user_store::HashmapUserStore;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

pub mod domain;
pub mod routes;
pub mod services;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}

impl Application {
    pub async fn build(
        app_state: AppState,
        address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-token", post(routes::verify_token))
            .route("/verify-2fa", post(routes::verify2fa))
            .with_state(app_state);
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        Ok(Self { server, address })
    }
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Listening on {}", &self.address);
        self.server.await
    }
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
