use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use tower_http::services::ServeDir;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup));
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

// Example route handler.
// For now we will simply return a 200 (OK) status code.
async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
