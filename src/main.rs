mod handlers;
mod helpers;
mod models;
mod routes;
mod state;

use tokio::net::TcpListener;

use crate::{routes::app, state::AppState};

#[tokio::main]
async fn main() {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be configured");

    assert!(
        jwt_secret.len() >= 32,
        "JWT_SECRET must contain at least 32 bytes"
    );

    let app_state = AppState::new(jwt_secret);

    let app = app(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("taskr running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
