use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use axum::{
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

// Data contained inside a JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
    iss: String,
    aud: String,
}

// The authenticated user for one request
#[derive(Clone, Copy)]
pub struct AuthUser(pub Uuid);

// Hash a password

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let hash = Argon2::default().hash_password(password.as_bytes())?;

    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// Create an authentication token
pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: user_id,
        exp: (now + 3600) as usize,
        iss: "taskr".to_string(),
        aud: "taskr-api".to_string(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

// Middleware that protects authenticated routes
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut validation = Validation::new(Algorithm::HS256);

    validation.set_issuer(&["taskr"]);
    validation.set_audience(&["taskr-api"]);
    validation.set_required_spec_claims(&["exp", "sub", "iss", "aud"]);
    validation.leeway = 0;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    request.extensions_mut().insert(AuthUser(claims.sub));

    Ok(next.run(request).await)
}
