use std::sync::{Arc, Mutex};

use crate::models::{Task, User};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
    pub users: Arc<Mutex<Vec<User>>>,
    pub jwt_secret: Arc<String>,
}

impl AppState {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
            users: Arc::new(Mutex::new(Vec::new())),
            jwt_secret: Arc::new(jwt_secret),
        }
    }
}
