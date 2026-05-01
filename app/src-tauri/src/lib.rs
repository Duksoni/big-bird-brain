mod config;
mod setup;

use tauri::Manager;
use crate::setup::{init_db, init_tracing};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_tracing("big-bird-brain-app");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            tauri::async_runtime::block_on(async move {
                let app_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");
                if !app_dir.exists() {
                    std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
                }
                
                let db_path = app_dir.join("big-bird-brain.db");
                let connection_string = format!("sqlite:{}", db_path.display());
                
                let pool = init_db(&connection_string).await.expect("failed to init db");
                
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("failed to run migrations");
                
                app_handle.manage(pool);
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
