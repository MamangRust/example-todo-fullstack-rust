use example_todo_backend::build_app;
use reqwest::Client;
use serde_json::json;
use tokio::task;


#[tokio::test]
async fn test_crud_operations() {
    let addr = "127.0.0.1:3001";
    let app = build_app().await;


    let server = task::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();
        axum::serve(listener, app).await.unwrap();
    });

   
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = Client::new();

   
    let todo_request = json!({
        "title": "Integration Test Todo",
        "description": "Test Description"
    });

    let resp = client
        .post(format!("http://{}/todos", addr))
        .json(&todo_request)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);

    let created_todo: serde_json::Value = resp.json().await.unwrap();
    let created_todo_id = created_todo["id"].as_i64().unwrap();

   
    let resp = client
        .get(format!("http://{}/todos", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);

    let todos: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert!(todos.iter().any(|todo| todo["id"] == created_todo["id"]));

    
    let resp = client
        .get(format!("http://{}/todos/{}", addr, created_todo_id))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);

    let fetched_todo: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(fetched_todo["title"], todo_request["title"]);

    
    let update_request = json!({
        "title": "Updated Title",
        "description": "Updated Description"
    });
    let resp = client
        .put(format!("http://{}/todos/{}", addr, created_todo_id))
        .json(&update_request)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);

    
    let resp = client
        .delete(format!("http://{}/todos/{}", addr, created_todo_id))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);

    
    let resp = client
        .get(format!("http://{}/todos/{}", addr, created_todo_id))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);

   
    server.abort();
}
