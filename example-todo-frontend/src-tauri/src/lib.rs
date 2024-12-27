use reqwest;
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Default)]
struct ApiClient {
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoRequest {
    title: String,
    description: String,
}

const API_URL: &str = "http://localhost:3000";

#[tauri::command]
async fn fetch_todos(_state: State<'_, ApiClient>) -> Result<Vec<Todo>, String> {
    let response = reqwest::get(format!("{}/todos", API_URL))
        .await
        .map_err(|e| e.to_string())?;
        
    response.json::<Vec<Todo>>()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_todo(_state: State<'_, ApiClient>, todo: TodoRequest) -> Result<Todo, String> {
    let client = reqwest::Client::new();
    let response = client.post(format!("{}/todos", API_URL))
        .json(&todo)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    response.json::<Todo>()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_todo(_state: State<'_, ApiClient>, id: i32, todo: TodoRequest) -> Result<(), String> {
    let client = reqwest::Client::new();
    client.put(format!("{}/todos/{}", API_URL, id))
        .json(&todo)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    Ok(())
}

#[tauri::command]
async fn delete_todo(_state: State<'_, ApiClient>, id: i32) -> Result<(), String> {
    let client = reqwest::Client::new();
    client.delete(format!("{}/todos/{}", API_URL, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ApiClient::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_todos, create_todo, update_todo, delete_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
