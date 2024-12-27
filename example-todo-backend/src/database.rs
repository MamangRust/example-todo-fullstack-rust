use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

pub async fn create_pool() -> Pool<Sqlite> {
    let database_url = "sqlite:sqlite.db"; // Atur path database SQLite sesuai kebutuhan
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool.");
    pool
}

pub async fn create_table(pool: &Pool<Sqlite>) {
    let query = r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL
        );
    "#;

    sqlx::query(query)
        .execute(pool)
        .await
        .expect("Failed to create table.");
}
