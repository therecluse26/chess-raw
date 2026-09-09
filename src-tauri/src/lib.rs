use std::sync::Mutex;

use tauri::State;

#[tauri::command]
fn increment_counter(state: State<'_, Mutex<AppState>>) -> u32 {
    let mut guard = state.lock().unwrap();
    guard.counter += 1;
    guard.counter
}

#[derive(Default)]
struct AppState {
    counter: u32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![increment_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
