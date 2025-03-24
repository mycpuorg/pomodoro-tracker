mod db;

use db::{PomodoroSession, init_database, save_session, get_sessions_by_date_range};
use tauri::{State, Manager};
use std::sync::Mutex;

struct AppState {
    db_connection: Mutex<rusqlite::Connection>,
}

#[tauri::command]
fn save_pomodoro_session(
    state: State<AppState>,
    task_name: String,
    category: String,
    start_time: i64,
    end_time: i64,
) -> Result<i64, String> {
    let conn = state.db_connection.lock().unwrap();
    let session = PomodoroSession {
        id: None,
        task_name,
        category,
        start_time,
        end_time,
    };
    
    save_session(&conn, session).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_sessions(
    state: State<AppState>,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<Vec<PomodoroSession>, String> {
    let conn = state.db_connection.lock().unwrap();
    get_sessions_by_date_range(&conn, start_timestamp, end_timestamp)
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let conn = init_database(&app_handle).expect("Database initialization failed");
            
            app.manage(AppState {
                db_connection: Mutex::new(conn),
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_pomodoro_session,
            get_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
