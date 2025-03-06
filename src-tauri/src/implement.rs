use serde::{Deserialize, Serialize};

const MAX_TASKS: usize = 9;
const FILE_PATH: &str = "tasks.toml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub is_completed: bool,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            description: String::new(),
            is_completed: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            title: String::new(),
            tasks: vec![Task::default(); MAX_TASKS],
        }
    }
}

pub fn read_data() -> Data {
    let data: Data = match std::fs::read_to_string(FILE_PATH) {
        Ok(data) => toml::from_str(&data).unwrap_or_default(),
        Err(_) => Data::default()
    };
    data
}

pub fn write_data(data: Data) {
    let toml_str = toml::to_string(&data).unwrap();
    std::fs::write(FILE_PATH, &toml_str).unwrap();
}
