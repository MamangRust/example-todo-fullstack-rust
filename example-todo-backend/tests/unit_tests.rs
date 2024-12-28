use example_todo_backend::database::{create_pool, create_test_table};
use example_todo_backend::crud::{get_todos, create_todo, update_todo, delete_todo, get_todo_by_id};

#[tokio::test]
async fn test_create_todo() {
    let pool = create_pool().await;
    let title = "Test Todo".to_string();
    let description = "Test Description".to_string();

    create_test_table(&pool).await;

    let todo = create_todo(&pool, &title, &description).await.unwrap();
    assert_eq!(todo.title, title);
    assert_eq!(todo.description, description);
}

#[tokio::test]
async fn test_get_todos() {
    let pool = create_pool().await;
    create_test_table(&pool).await;

    let todos = get_todos(&pool).await.unwrap();
    assert!(todos.is_empty()); 
}

#[tokio::test]
async fn test_get_todo_by_id() {
    let pool = create_pool().await;
    let title = "Test Todo".to_string();
    let description = "Test Description".to_string();

    create_test_table(&pool).await;

    let mytodo = create_todo(&pool, &title, &description).await.unwrap();

    

    let fetched_todo = get_todo_by_id(&pool, mytodo.id.try_into().unwrap()).await.unwrap();
    assert_eq!(fetched_todo.unwrap().title, title);
}

#[tokio::test]
async fn test_update_todo() {
    let pool = create_pool().await;
    let title = "Old Title".to_string();
    let description = "Old Description".to_string();
    create_test_table(&pool).await;

    let mytodo = create_todo(&pool, &title, &description).await.unwrap();


    let new_title = "Updated Title".to_string();
    let new_description = "Updated Description".to_string();
    let result = update_todo(&pool, mytodo.id.try_into().unwrap(), &new_title, &new_description).await;

    assert!(result.is_ok());
    let updated_todo = get_todo_by_id(&pool, mytodo.id.try_into().unwrap()).await.unwrap().unwrap();
    assert_eq!(updated_todo.title, new_title);
    assert_eq!(updated_todo.description, new_description);
}

#[tokio::test]
async fn test_delete_todo() {
    let pool = create_pool().await;
    let title = "Delete Me".to_string();
    let description = "I should be deleted".to_string();
    
    create_test_table(&pool).await;

    let mytodo = create_todo(&pool, &title, &description).await.unwrap();


    let result = delete_todo(&pool, mytodo.id.try_into().unwrap()).await;
    assert!(result.is_ok());

    let deleted_todo = get_todo_by_id(&pool, mytodo.id.try_into().unwrap()).await.unwrap();
    assert!(deleted_todo.is_none());
}