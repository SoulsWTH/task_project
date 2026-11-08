use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    handlers::{auth, tasks},
    helpers::auth::require_auth,
    state::AppState,
};

pub fn app(state: AppState) -> Router {
    // These routes require authentication.
    let protected = Router::new()
        .route("/task", get(tasks::list).post(tasks::create))
        .route(
            "/task/{id}",
            get(tasks::get_one)
                .delete(tasks::delete)
                .patch(tasks::update),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // These routes are publicly accessible.
    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .merge(protected)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
