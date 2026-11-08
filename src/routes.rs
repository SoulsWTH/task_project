use axum::{Router, routing::get};

use crate::handlers::tasks;
use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/task", get(tasks::list).post(tasks::create))
        .route(
            "/task/{id}",
            get(tasks::get_one)
                .delete(tasks::delete)
                .patch(tasks::update),
        )
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
