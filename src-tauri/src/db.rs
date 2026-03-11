use rusqlite::Connection;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R, std::option::Option<tauri_plugin_sql::PluginConfig>> {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_tables",
            sql: "
            CREATE TABLE IF NOT EXISTS words (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              word TEXT NOT NULL,
              translate TEXT NOT NULL,
              correct INTEGER DEFAULT 0,
              total INTEGER DEFAULT 0,
              created_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS settings (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL
            );

            INSERT OR IGNORE INTO settings VALUES ('interval_minutes', '5');
            INSERT OR IGNORE INTO settings VALUES ('direction', 'native_to_foreign');
            ",
            kind: MigrationKind::Up,
        }
    ];

    tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:words.db", migrations)
        .build()
}

pub fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    // In Tauri v2, tauri-plugin-sql resolves "sqlite:filename.db" inside app_config_dir or app_data_dir.
    // Try app_config_dir first as per standard plugin-sql behavior, fallback to app_data_dir.
    let path_resolver = app_handle.path();
    let config_dir = path_resolver.app_config_dir().map_err(|e| e.to_string())?;
    // By default in Tauri 2, plugin-sql uses appConfigDir unless specified otherwise
    Ok(config_dir.join("words.db"))
}

pub fn get_connection(app_handle: &AppHandle) -> Result<Connection, String> {
    let db_path = get_db_path(app_handle)?;
    
    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    Connection::open(db_path).map_err(|e| e.to_string())
}
