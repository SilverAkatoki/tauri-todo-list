mod implement;

use implement::Task;

#[tauri::command]
fn read_tasks() -> Result<Vec<Task>, String> {
    implement::read_tasks()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
