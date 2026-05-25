use anyhow::Context;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;
use tauri::{App, Manager, async_runtime::Mutex};

use crate::rule_engine::RuleEngine;

pub async fn init_db(connection_string: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(connection_string)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .context("Failed to connect to DB")?;
    Ok(pool)
}

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();

    tauri::async_runtime::block_on(async move {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
        }

        let db_path = app_dir.join("data.db");
        let connection_string = format!("sqlite:{}", db_path.display());

        let pool = init_db(&connection_string)
            .await
            .expect("failed to init db");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        app_handle.manage(pool);
        app_handle.manage(Mutex::new(RuleEngine::new_draft().unwrap()));
    });

    Ok(())
}
