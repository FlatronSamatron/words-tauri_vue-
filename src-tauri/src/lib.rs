mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(db::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_words,
            commands::add_word,
            commands::update_word,
            commands::delete_word,
            commands::record_answer,
            commands::get_settings,
            commands::save_settings,
            commands::log_test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
