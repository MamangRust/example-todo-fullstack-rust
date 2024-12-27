use sqlx::{query, Pool, Sqlite};
use crate::models::Todo;
use sqlx::query_as;

pub async fn create_todo(
    pool: &Pool<Sqlite>,
    title: &str,
    description: &str,
) -> Result<Todo, sqlx::Error> {
    let row = query!(
        r#"
        INSERT INTO todos (title, description)
        VALUES (?, ?)
        "#,
        title,
        description
    )
    .execute(pool)
    .await?;

    let todo = Todo {
        id: row.last_insert_rowid(),
        title: title.to_string(),
        description: description.to_string(),
    };

    Ok(todo)
}

pub async fn get_todos(pool: &Pool<Sqlite>) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = query_as!(
        Todo,
        r#"
        SELECT id, title, description
        FROM todos
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn get_todo_by_id(
    pool: &Pool<Sqlite>,
    id: i32,
) -> Result<Option<Todo>, sqlx::Error> {
    let todo = query_as!(
        Todo,
        r#"
        SELECT id, title, description
        FROM todos
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(todo)
}

pub async fn update_todo(
    pool: &Pool<Sqlite>,
    id: i32,
    title: &str,
    description: &str,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
        UPDATE todos
        SET title = ?, description = ?
        WHERE id = ?
        "#,
        title,
        description,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_todo(pool: &Pool<Sqlite>, id: i32) -> Result<(), sqlx::Error> {
    query!(
        r#"
        DELETE FROM todos
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
