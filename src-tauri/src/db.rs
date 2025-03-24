use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::api::path::app_data_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct PomodoroSession {
    pub id: Option<i64>,
    pub task_name: String,
    pub category: String,
    pub start_time: i64,
    pub end_time: i64,
}

pub fn init_database(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let app_data_dir = app_data_dir(&app_handle.config()).expect("Failed to get app data dir");
    fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    
    let db_path = app_data_dir.join("pomodoro.db");
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pomodoro_sessions (
            id INTEGER PRIMARY KEY,
            task_name TEXT NOT NULL,
            category TEXT NOT NULL,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn save_session(conn: &Connection, session: PomodoroSession) -> Result<i64> {
    conn.execute(
        "INSERT INTO pomodoro_sessions (task_name, category, start_time, end_time) 
         VALUES (?1, ?2, ?3, ?4)",
        [
            &session.task_name,
            &session.category,
            &session.start_time.to_string(),
            &session.end_time.to_string(),
        ],
    )?;
    
    Ok(conn.last_insert_rowid())
}

pub fn get_sessions_by_date_range(
    conn: &Connection,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<Vec<PomodoroSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_name, category, start_time, end_time 
         FROM pomodoro_sessions 
         WHERE start_time >= ?1 AND start_time <= ?2
         ORDER BY start_time DESC"
    )?;
    
    let sessions = stmt.query_map([start_timestamp.to_string(), end_timestamp.to_string()], |row| {
        Ok(PomodoroSession {
            id: Some(row.get(0)?),
            task_name: row.get(1)?,
            category: row.get(2)?,
            start_time: row.get(3)?,
            end_time: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(sessions)
}
