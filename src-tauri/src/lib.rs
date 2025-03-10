mod implement;

use std::sync::Mutex;

use lazy_static::lazy_static;
use implement::{Manager, Data};

lazy_static! {
    static ref TASK_MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}

#[tauri::command]
fn read_data() -> Data {
    let manager = TASK_MANAGER.lock().unwrap();
    manager.read_data()
}

#[tauri::command]
fn write_data(data: Data) {
    let manager = TASK_MANAGER.lock().unwrap();
    manager.write_data(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_data, write_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

