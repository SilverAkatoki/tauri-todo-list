use std::fs::File;
use std::io::Read;

use serde::{Serialize, Deserialize};


const MAX_TASKS: usize = 9;
const FILE_PATH: &str = "tasks.toml";

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub is_completed: bool,
}

pub fn read_tasks() -> Result<Vec<Task>, String> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut contents = String::new();
        if let Ok(_) = file.read_to_string(&mut contents) {
            match toml::de::from_str::<Vec<Task>>(&contents) {
                Ok(tasks) => return Ok(tasks),
                Err(e) => return Err(format!("解析 TOML 文件失败：{}", e)),
            }
        }
    }
    Ok(Vec::with_capacity(MAX_TASKS))
}

#[allow(dead_code, unused_variables)]
pub fn write_tasks(tasks: Vec<Task>) {
    // Write tasks to a file
}
