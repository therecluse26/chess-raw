mod game;
mod pieces;

use crate::game::{setup_app_state, AppState};
use std::sync::Mutex;

use tauri::State;

#[tauri::command]
fn show_game_state(state: State<'_, Mutex<AppState>>) -> serde_json::Value {
    let guard = state.lock().unwrap();
    serde_json::to_value(&guard.game).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(setup_app_state())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![show_game_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
