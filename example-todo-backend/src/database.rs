use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn create_pool() -> Pool<Sqlite> {
    let database_url = "sqlite::memory:"; 
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


pub async fn create_test_table(pool: &sqlx::Pool<sqlx::Sqlite>) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await
    .unwrap();
}
