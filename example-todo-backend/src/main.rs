use axum::{
    extract::{Extension, Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use crate::database::{create_pool, create_table};
use crate::crud::{create_todo, get_todos, get_todo_by_id, update_todo, delete_todo};
use crate::models::Todo;
use serde::{Deserialize, Serialize};
use tokio;

mod database;
mod crud;
mod models;

#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub title: String,
    pub description: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await;
    create_table(&pool).await;
   
    let app_state = AppState { pool };

    let app = Router::new()
        .route("/todos", get(get_all_todos).post(create_new_todo))
        .route("/todos/:id", 
            get(get_todo_by_id_endpoint)
            .put(update_todo_endpoint)
            .delete(delete_todo_endpoint))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn get_all_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    get_todos(&state.pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Handler to create a new todo
async fn create_new_todo(
    State(state): State<AppState>,
    Json(payload): Json<TodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    create_todo(&state.pool, &payload.title, &payload.description)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Handler to get a todo by ID
async fn get_todo_by_id_endpoint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<Todo>>, StatusCode> {
    get_todo_by_id(&state.pool, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Handler to update a todo by ID
async fn update_todo_endpoint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<TodoRequest>,
) -> Result<(), StatusCode> {
    update_todo(&state.pool, id, &payload.title, &payload.description)
        .await
        .map(|_| ())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Handler to delete a todo by ID
async fn delete_todo_endpoint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), StatusCode> {
    delete_todo(&state.pool, id)
        .await
        .map(|_| ())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}