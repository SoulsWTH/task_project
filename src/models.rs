use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)] // serialize converts struct into json
pub struct Task {
    pub id: u32,
    pub user_id: Uuid,
    pub tittle: String,
    pub done: bool,
}

#[derive(Clone, Deserialize)] // deserialize converts json to struct
pub struct CreateTask {
    pub tittle: String,
}

#[derive(Clone, Deserialize)] // deserialize converts json to struct
pub struct UpdateTask {
    pub tittle: Option<String>,
    pub done: Option<bool>,
}

#[derive(Clone, Deserialize)] // deserialize converts json to struct
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub hashed_pasword: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
}
