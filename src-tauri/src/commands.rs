use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::db::get_connection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    pub id: i64,
    pub word: String,
    pub translate: String,
    pub correct: i64,
    pub total: i64,
    pub created_at: String,
    pub group_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub interval_minutes: u32,
    pub direction: String,
    pub active_group_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub word_count: i64,
}

#[tauri::command]
pub fn get_words(app_handle: AppHandle) -> Result<Vec<Word>, String> {
    let conn = get_connection(&app_handle)?;
    let mut stmt = conn
        .prepare("SELECT id, word, translate, correct, total, created_at, group_id FROM words ORDER BY id DESC")
        .map_err(|e| e.to_string())?;

    let words = stmt
        .query_map([], |row| {
            Ok(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                translate: row.get(2)?,
                correct: row.get(3)?,
                total: row.get(4)?,
                created_at: row.get(5)?,
                group_id: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(words)
}

#[tauri::command]
pub fn add_word(
    app_handle: AppHandle,
    word: String,
    translate: String,
    group_id: i64,
) -> Result<Word, String> {
    let conn = get_connection(&app_handle)?;
    conn.execute(
        "INSERT INTO words (word, translate, group_id) VALUES (?1, ?2, ?3)",
        params![word, translate, group_id],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    // Fetch the inserted word to return it
    let mut stmt = conn
        .prepare("SELECT id, word, translate, correct, total, created_at, group_id FROM words WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![id], |row| {
        Ok(Word {
            id: row.get(0)?,
            word: row.get(1)?,
            translate: row.get(2)?,
            correct: row.get(3)?,
            total: row.get(4)?,
            created_at: row.get(5)?,
            group_id: row.get(6)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_word(
    app_handle: AppHandle,
    id: i64,
    word: String,
    translate: String,
    group_id: i64,
) -> Result<Word, String> {
    let conn = get_connection(&app_handle)?;
    conn.execute(
        "UPDATE words SET word = ?1, translate = ?2, group_id = ?3 WHERE id = ?4",
        params![word, translate, group_id, id],
    )
    .map_err(|e| e.to_string())?;

    // Fetch the updated word
    let mut stmt = conn
        .prepare("SELECT id, word, translate, correct, total, created_at, group_id FROM words WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![id], |row| {
        Ok(Word {
            id: row.get(0)?,
            word: row.get(1)?,
            translate: row.get(2)?,
            correct: row.get(3)?,
            total: row.get(4)?,
            created_at: row.get(5)?,
            group_id: row.get(6)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_word(app_handle: AppHandle, id: i64) -> Result<bool, String> {
    let conn = get_connection(&app_handle)?;
    let rows_affected = conn
        .execute("DELETE FROM words WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(rows_affected > 0)
}

#[tauri::command]
pub fn record_answer(app_handle: AppHandle, id: i64, known: bool) -> Result<Word, String> {
    let conn = get_connection(&app_handle)?;
    let increment_correct = if known { 1 } else { 0 };

    conn.execute(
        "UPDATE words SET correct = correct + ?1, total = total + 1 WHERE id = ?2",
        params![increment_correct, id],
    )
    .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, word, translate, correct, total, created_at, group_id FROM words WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![id], |row| {
        Ok(Word {
            id: row.get(0)?,
            word: row.get(1)?,
            translate: row.get(2)?,
            correct: row.get(3)?,
            total: row.get(4)?,
            created_at: row.get(5)?,
            group_id: row.get(6)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let conn = get_connection(&app_handle)?;
    
    let query_setting = |key: &str, default: &str| -> Result<String, String> {
        let val: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(val.unwrap_or_else(|| default.to_string()))
    };

    let interval_minutes_str = query_setting("interval_minutes", "5")?;
    let interval_minutes: u32 = interval_minutes_str.parse().unwrap_or(5);
    
    let direction = query_setting("direction", "native_to_foreign")?;
    let active_group_id = query_setting("active_group_id", "all")?;

    Ok(Settings {
        interval_minutes,
        direction,
        active_group_id,
    })
}

#[tauri::command]
pub fn save_settings(
    app_handle: AppHandle,
    interval_minutes: u32,
    direction: String,
    active_group_id: String,
) -> Result<bool, String> {
    let conn = get_connection(&app_handle)?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('interval_minutes', ?1)",
        params![interval_minutes.to_string()],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('direction', ?1)",
        params![direction],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('active_group_id', ?1)",
        params![active_group_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub fn get_groups(app_handle: AppHandle) -> Result<Vec<Group>, String> {
    let conn = get_connection(&app_handle)?;
    let mut stmt = conn
        .prepare(
            "SELECT g.id, g.name, g.created_at, COUNT(w.id) as word_count 
             FROM groups g 
             LEFT JOIN words w ON g.id = w.group_id 
             GROUP BY g.id 
             ORDER BY g.id ASC",
        )
        .map_err(|e| e.to_string())?;

    let groups = stmt
        .query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                word_count: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(groups)
}

#[tauri::command]
pub fn add_group(app_handle: AppHandle, name: String) -> Result<Group, String> {
    let conn = get_connection(&app_handle)?;
    conn.execute("INSERT INTO groups (name) VALUES (?1)", params![name])
        .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    let mut stmt = conn
        .prepare("SELECT id, name, created_at, 0 as word_count FROM groups WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![id], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            word_count: row.get(3)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_group(app_handle: AppHandle, id: i64, name: String) -> Result<Group, String> {
    let conn = get_connection(&app_handle)?;
    conn.execute("UPDATE groups SET name = ?1 WHERE id = ?2", params![name, id])
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT g.id, g.name, g.created_at, COUNT(w.id) as word_count 
             FROM groups g 
             LEFT JOIN words w ON g.id = w.group_id 
             WHERE g.id = ?1
             GROUP BY g.id",
        )
        .map_err(|e| e.to_string())?;

    stmt.query_row(params![id], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            word_count: row.get(3)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_group(app_handle: AppHandle, id: i64) -> Result<bool, String> {
    if id == 1 {
        return Err("Cannot delete default group".to_string());
    }
    let conn = get_connection(&app_handle)?;
    let rows_affected = conn
        .execute("DELETE FROM groups WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(rows_affected > 0)
}

#[tauri::command]
pub fn reset_all_data(app_handle: AppHandle) -> Result<(), String> {
    let conn = get_connection(&app_handle)?;
    conn.execute("DELETE FROM words", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM groups WHERE id != 1", []).map_err(|e| e.to_string())?;
    conn.execute("UPDATE groups SET name = 'Default' WHERE id = 1", []).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn log_test(msg: String) {
    println!("TEST_JS_OUTPUT: {}", msg);
}

#[tauri::command]
pub fn open_game_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("game") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn close_game_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("game") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn open_main_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    if let Some(window) = app_handle.get_webview_window("game") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn set_tray_active(app_handle: AppHandle) -> Result<(), String> {
    if let Some(tray) = app_handle.tray_by_id("tray") {
        let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon-active.png"))
            .map_err(|e| e.to_string())?;
        tray.set_icon(Some(icon)).map_err(|e| e.to_string())?;
        let _ = tray.set_icon_as_template(false);
    }
    Ok(())
}

#[tauri::command]
pub fn set_tray_normal(app_handle: AppHandle) -> Result<(), String> {
    if let Some(tray) = app_handle.tray_by_id("tray") {
        let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon-normal.png"))
            .map_err(|e| e.to_string())?;
        tray.set_icon(Some(icon)).map_err(|e| e.to_string())?;
        let _ = tray.set_icon_as_template(true);
    }
    Ok(())
}

#[tauri::command]
pub fn update_timer_interval(
    state: tauri::State<'_, std::sync::Arc<crate::TrayTimerState>>,
    app_handle: AppHandle,
    minutes: u32,
) -> Result<(), String> {
    *state.interval_minutes.lock().unwrap() = minutes;
    *state.last_activity.lock().unwrap() = std::time::Instant::now();
    set_tray_normal(app_handle)?;
    Ok(())
}
