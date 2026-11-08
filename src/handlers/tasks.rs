use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::{CreateTask, Task, UpdateTask},
    state::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    Json(payloaod): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let mut tasks = state.tasks.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let task = Task {
        id: *next_id,
        tittle: payloaod.tittle,
        done: false,
    };

    *next_id += 1;

    tasks.push(task.clone());

    (StatusCode::CREATED, Json(task))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();

    tasks
        .iter()
        .find(|t| t.id == id)
        .map(|t| Json(t.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payloaod): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    if payloaod.tittle.is_none() && payloaod.done.is_none() {
        return Err(StatusCode::NOT_MODIFIED);
    }

    let mut tasks = state.tasks.lock().unwrap();

    let Some(task) = tasks.iter_mut().find(|t| t.id == id) else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(tittle) = payloaod.tittle {
        task.tittle = tittle;
    }

    if let Some(done) = payloaod.done {
        task.done = done;
    }

    Ok(Json(task.clone()))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let mut tasks = state.tasks.lock().unwrap();

    let Some(index) = tasks
        .iter()
        .enumerate()
        .find(|(_, t)| t.id == id)
        .map(|result| result.0)
    else {
        return StatusCode::NOT_FOUND;
    };

    tasks.remove(index);

    StatusCode::OK
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<Task>> {
    Json(state.tasks.lock().unwrap().to_vec())
}
