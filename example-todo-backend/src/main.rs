use example_todo_backend::build_app;

use serde::{Deserialize, Serialize};
use tokio;


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
    let app = build_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}