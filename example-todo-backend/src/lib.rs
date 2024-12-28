pub mod database;
pub mod crud;
pub mod models;
pub mod handler;
pub mod state;

use axum::{Router, routing::get};
use database::{
    create_pool, create_table
};
use handler::{
    get_all_todos,
    create_new_todo,
    update_todo_endpoint,
    delete_todo_endpoint,
    get_todo_by_id_endpoint
};
use state::AppState;




pub async fn build_app() -> Router {
    let pool = create_pool().await;
    create_table(&pool).await;

    let app_state = AppState { pool };

    Router::new()
        .route("/todos", get(get_all_todos).post(create_new_todo))
        .route(
            "/todos/:id",
            get(get_todo_by_id_endpoint)
                .put(update_todo_endpoint)
                .delete(delete_todo_endpoint),
        )
        .with_state(app_state)
}