use axum::{
    extract::{Path, State}, http::StatusCode,  Json
};
use serde::{Deserialize, Serialize};


use crate::state::AppState;
use crate::models::Todo;
use crate::crud::{create_todo, delete_todo, get_todos, get_todo_by_id, update_todo};


#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub title: String,
    pub description: String,
}

pub async fn get_all_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    get_todos(&state.pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_new_todo(
    State(state): State<AppState>,
    Json(payload): Json<TodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    create_todo(&state.pool, &payload.title, &payload.description)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_todo_by_id_endpoint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<Todo>>, StatusCode> {
    get_todo_by_id(&state.pool, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}


pub async fn update_todo_endpoint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<TodoRequest>,
) -> Result<(), StatusCode> {
    update_todo(&state.pool, id, &payload.title, &payload.description)
        .await
        .map(|_| ())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}


pub async fn delete_todo_endpoint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    delete_todo(&state.pool, id)
        .await
        .map(|_| ())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}