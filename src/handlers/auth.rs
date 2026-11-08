use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{
    helpers::auth::{create_token, hash_password, verify_password},
    models::{CreateUser, TokenResponse, User, UserResponse},
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let username = user.username.trim().to_string();
    let password = user.password.trim().to_string();

    if username.len() < 3 || password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let password_hash = tokio::task::spawn_blocking(move || hash_password(&password).ok())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut users = state.users.lock().unwrap();

    if users.iter().any(|u| u.username == username) {
        return Err(StatusCode::CONFLICT);
    }

    let uuid = Uuid::new_v4();
    users.push(User {
        uuid,
        username: username.clone(),
        hashed_pasword: password_hash,
    });

    Ok((StatusCode::CREATED, Json(UserResponse { uuid, username })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let users = state.users.lock().unwrap();

    let user = users
        .iter()
        .find(|u| u.username == payload.username)
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    drop(users);

    let password = payload.password;
    let password_hash = user.hashed_pasword.clone();

    let correct = tokio::task::spawn_blocking(move || verify_password(&password, &password_hash))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !correct {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(user.uuid, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TokenResponse {
        access_token: token,
    }))
}
