use rusqlite::{Connection, params};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    // In Tauri v2, tauri-plugin-sql resolves "sqlite:filename.db" inside app_config_dir or app_data_dir.
    // Try app_config_dir first as per standard plugin-sql behavior, fallback to app_data_dir.
    let path_resolver = app_handle.path();
    let config_dir = path_resolver.app_config_dir().map_err(|e| e.to_string())?;
    // By default in Tauri 2, plugin-sql uses appConfigDir unless specified otherwise
    Ok(config_dir.join("words.db"))
}

pub fn ensure_migrated(app_handle: &AppHandle) -> Result<(), String> {
    let mut conn = get_connection(app_handle)?;
    
    // Check if groups table exists
    let table_exists: bool = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='groups'",
        [],
        |row| row.get(0),
    ).unwrap_or(0) > 0;

    if !table_exists {
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        
        tx.execute(
            "CREATE TABLE IF NOT EXISTS groups (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              name TEXT NOT NULL UNIQUE,
              created_at TEXT DEFAULT (datetime('now'))
            );",
            [],
        ).map_err(|e| e.to_string())?;

        tx.execute("INSERT OR IGNORE INTO groups (id, name) VALUES (1, 'Default');", []).map_err(|e| e.to_string())?;
        
        // Add group_id to words if not exists
        let has_column: bool = tx.query_row(
            "SELECT count(*) FROM pragma_table_info('words') WHERE name='group_id'",
            [],
            |row| row.get(0),
        ).unwrap_or(0) > 0;

        if !has_column {
            tx.execute("ALTER TABLE words ADD COLUMN group_id INTEGER DEFAULT 1;", []).map_err(|e| e.to_string())?;
        }

        tx.execute("INSERT OR IGNORE INTO settings (key, value) VALUES ('active_group_id', 'all');", []).map_err(|e| e.to_string())?;

        // Add 5 groups with 20 words each as requested
        let test_groups = vec!["Travel", "Food", "Work", "Leisure", "Daily"];
        for (i, &name) in test_groups.iter().enumerate() {
            let group_id = (i + 2) as i64;
            tx.execute("INSERT OR IGNORE INTO groups (id, name) VALUES (?1, ?2)", params![group_id, name]).map_err(|e| e.to_string())?;
            
            for j in 1..=20 {
                let word = format!("{}_word_{}", name, j);
                let trans = format!("{}_перевод_{}", name, j);
                tx.execute(
                    "INSERT INTO words (word, translate, group_id) VALUES (?1, ?2, ?3)",
                    params![word, trans, group_id]
                ).map_err(|e| e.to_string())?;
            }
        }

        tx.commit().map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

pub fn get_connection(app_handle: &AppHandle) -> Result<Connection, String> {
    let db_path = get_db_path(app_handle)?;
    
    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    Connection::open(db_path).map_err(|e| e.to_string())
}
