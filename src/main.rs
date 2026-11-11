mod handlers;
mod helpers;
mod models;
mod routes;
mod state;

use std::env;

use tokio::net::TcpListener;

use crate::{routes::app, state::AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let jwt_secret = env_var_to_get(
        get_env(vec!["JWT_SECRET".to_string()]),
        "JWT_SECRET".to_string(),
    );

    println!("jwt_secret: {jwt_secret}");

    let app_state = AppState::new(jwt_secret);

    let app = app(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("taskr running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

fn get_env(env_variables_names: Vec<String>) -> Vec<String> {
    env_variables_names
        .iter()
        .map(|variable| {
            format!(
                "{variable} {}",
                env::var(variable).expect("vaiable not set")
            )
        })
        .collect()
}

fn env_var_to_get(env_variables: Vec<String>, env_variable: String) -> String {
    env_variables
        .iter()
        .find(|env| env.split_whitespace().collect::<Vec<&str>>()[0] == env_variable.as_str())
        .expect("Variable not found")
        .to_string()
}
