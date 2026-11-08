mod handlers;
mod helpers;
mod models;
mod routes;
mod state;

use tokio::net::TcpListener;

use crate::{routes::app, state::AppState};

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    let app = app(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap(); // it claims a port 

    println!("taskr runnig on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
