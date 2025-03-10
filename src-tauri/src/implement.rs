use log::{error, info};
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::fs::File;
use std::process;
use std::path::PathBuf;

const MAX_TASKS: usize = 9;
const DATA_FILE: &str = "tasks.toml";
const LOG_FILE: &str = "app.log";
const APP_NAME: &str = "tauri-todo-list";

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

pub struct Manager {
    log_dir: PathBuf,
    task_data_dir: PathBuf,
}

impl Manager {
    pub fn new() -> Self {
        let storage_dir = Self::get_storage_dir();
        let log_dir = storage_dir.join(LOG_FILE);
        let task_data_dir = storage_dir.join(DATA_FILE);
        let manager = Manager {
            log_dir,
            task_data_dir,
        };
        manager.init_logger();
        manager
    }

    fn init_logger(&self) {
        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(&self.log_dir).unwrap(),
        )])
        .unwrap();
    }

    fn get_storage_dir() -> std::path::PathBuf {
        let app_data_dir = std::env::var("LOCALAPPDATA").unwrap();
        let storage_dir = std::path::Path::new(&app_data_dir).join(APP_NAME);
        storage_dir
    }

    pub fn write_data(&self, data: Data) {
        let toml_str = toml::to_string(&data).unwrap_or_else(|err| {
            error!("Failed to serialize data: {err}");
            process::exit(1);
        });
        std::fs::write(&self.task_data_dir, &toml_str).unwrap_or_else(|err| {
            error!("Failed to write data to file: {err}");
            process::exit(1);
        });
        info!("Data successfully written to file");
    }

    pub fn read_data(&self) -> Data {
        info!(
            "Reading data from file: {}",
            &self.task_data_dir.to_str().unwrap()
        );
        let data: Data = match std::fs::read_to_string(&self.task_data_dir) {
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
            }
            Err(err) => {
                error!("Failed to read data from file: {err}");
                info!("Return default data");
                Data::default()
            }
        };
        data
    }
}
