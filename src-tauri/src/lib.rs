mod implement;

use implement::Data;

#[tauri::command]
fn read_data() -> Data {
    implement::read_data()
}

#[tauri::command]
fn write_data(data: Data) {
    implement::write_data(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_data, write_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

