use serde::{Deserialize, Serialize};
use log::{info, error};
use std::process;


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
    info!("Reading data from file");
    let data: Data = match std::fs::read_to_string(FILE_PATH) {
        Ok(data) => {
            info!("Data successfully read from file");
            match toml::from_str(&data) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to deserialize data: {err}");
                    info!("Return default data");
                    Data::default()
                }
            }
        },
        Err(err) => {
            error!("Failed to read data from file: {err}");
            info!("Return default data");
            Data::default()
        }
    };
    data
}

pub fn write_data(data: Data) {
    let toml_str = toml::to_string(&data).unwrap_or_else(|err| {
        error!("Failed to serialize data: {err}");
        process::exit(1);
    });
    std::fs::write(FILE_PATH, &toml_str).unwrap_or_else(|err| {
        error!("Failed to write data to file: {err}");
        process::exit(1);
    });
    info!("Data successfully written to file");
}
