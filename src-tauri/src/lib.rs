use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

struct GamePopupState {
    last_hidden: AtomicI64,
}

pub struct TrayTimerState {
    pub last_activity: std::sync::Mutex<std::time::Instant>,
    pub interval_minutes: std::sync::Mutex<u32>,
}

mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(GamePopupState {
                last_hidden: AtomicI64::new(0),
            });

            use tauri::Listener;
            
            let settings = commands::get_settings(app.handle().clone())
                .unwrap_or(commands::Settings { interval_minutes: 5, direction: "native_to_foreign".to_string() });

            let timer_state = std::sync::Arc::new(TrayTimerState {
                last_activity: std::sync::Mutex::new(std::time::Instant::now()),
                interval_minutes: std::sync::Mutex::new(settings.interval_minutes),
            });
            app.manage(timer_state.clone());

            let app_handle_ev = app.handle().clone();
            let timer_state_ev = timer_state.clone();
            app.listen("answer-recorded", move |_| {
                *timer_state_ev.last_activity.lock().unwrap() = std::time::Instant::now();
                let _ = commands::set_tray_normal(app_handle_ev.clone());
            });

            let app_handle_loop = app.handle().clone();
            let timer_state_loop = timer_state.clone();
            tauri::async_runtime::spawn(async move {
                let mut is_active = false;
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    
                    let elapsed = {
                        let last = timer_state_loop.last_activity.lock().unwrap();
                        last.elapsed().as_secs()
                    };
                    let interval_secs = {
                        let mins = *timer_state_loop.interval_minutes.lock().unwrap();
                        if mins == 0 {
                            // If interval is somewhat 0, disable timer maybe?
                            10000000 // arbitrarily large
                        } else {
                            (mins as u64) * 60
                        }
                    };

                    if elapsed >= interval_secs {
                        if !is_active {
                            let _ = commands::set_tray_active(app_handle_loop.clone());
                            is_active = true;
                        }
                    } else {
                        if is_active {
                            let _ = commands::set_tray_normal(app_handle_loop.clone());
                            is_active = false;
                        }
                    }
                }
            });

            // Build the tray menu
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&settings_i, &quit_i])?;

            // Create tray icon
            let _tray = TrayIconBuilder::with_id("tray")
                .icon(tauri::image::Image::from_bytes(include_bytes!(
                    "../icons/icon-normal.png"
                ))?)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        rect,
                        ..
                    } = event
                    {
                        let app_handle = tray.app_handle();
                        
                        // Prevent instant reopen if it was just closed by clicking the tray
                        let state = app_handle.state::<GamePopupState>();
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
                        let last_hidden = state.last_hidden.load(Ordering::SeqCst);
                        
                        // If it was hidden less than 200ms ago, it means the focus loss from clicking the tray
                        // just closed it. Don't reopen it.
                        if now - last_hidden < 200 {
                            return;
                        }

                        if let Some(window) = app_handle.get_webview_window("game") {
                            // strictly position under the icon, independent of cursor
                            let (icon_x, icon_y) = match rect.position {
                                tauri::Position::Physical(p) => (p.x as f64, p.y as f64),
                                tauri::Position::Logical(p) => (p.x, p.y),
                            };
                            let (icon_w, icon_h) = match rect.size {
                                tauri::Size::Physical(s) => (s.width as f64, s.height as f64),
                                tauri::Size::Logical(s) => (s.width, s.height),
                            };

                            let icon_center_x = icon_x + (icon_w / 2.0);
                            
                            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x: (icon_center_x - 160.0) as i32, // 320 / 2 = 160
                                y: (icon_y + icon_h) as i32, // right under the icon
                            }));
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                if window.label() == "main" {
                    window.hide().unwrap();
                    api.prevent_close();
                }
            }
            WindowEvent::Focused(false) => {
                if window.label() == "game" {
                    let app_handle = window.app_handle();
                    let state = app_handle.state::<GamePopupState>();
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
                    state.last_hidden.store(now, Ordering::SeqCst);
                    let _ = window.hide();
                }
            }
            _ => {}
        })
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
            commands::log_test,
            commands::open_game_window,
            commands::close_game_window,
            commands::set_tray_active,
            commands::set_tray_normal,
            commands::update_timer_interval
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
