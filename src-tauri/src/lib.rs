mod implement;

use std::sync::Mutex;

use implement::{Manager, Data};
use tauri::State;


#[tauri::command]
fn read_data(manager: State<'_, Mutex<Manager>>) -> Data {
    let manager = manager.lock().unwrap();
    manager.read_data()
}

#[tauri::command]
fn write_data(manager: State<'_, Mutex<Manager>>, data: Data) {
    let manager = manager.lock().unwrap();
    manager.write_data(data)
}

#[tauri::command]
fn remove_done_tasks(manager: State<'_, Mutex<Manager>>) {
    let manager = manager.lock().unwrap();
    manager.remove_done_tasks()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Manager::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_data, write_data, remove_done_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

