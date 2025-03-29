use log::{error, info};
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::fs::File;
use std::path::PathBuf;
use std::process;

const MAX_TASKS: usize = 9;
const MAX_CLIPBOARDS: usize = 3;
const DATA_FILE: &str = "tasks.toml";
const LOG_FILE: &str = "app.log";

// 记得改 .gitignore 里面的忽略目录
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

#[derive(Serialize, Deserialize, Clone)]
struct Clipboard {
    title: String,
    tasks: Vec<Task>,
}

impl Default for Clipboard {
    fn default() -> Self {
        Clipboard {
            title: String::new(),
            tasks: vec![Task::default(); MAX_TASKS],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    clipboards: Vec<Clipboard>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            clipboards: vec![Clipboard::default(); MAX_CLIPBOARDS],
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

        if !storage_dir.exists() {
            std::fs::create_dir_all(&storage_dir).unwrap_or_else(|err| {
                error!("Failed to create storage directory: {err}");
                process::exit(1);
            });
            info!("Storage directory created at: {}", storage_dir.display());
        }

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
        let base_dir = if cfg!(debug_assertions) {
            std::env::current_dir().unwrap()
        } else {
            std::path::PathBuf::from(std::env::var("LOCALAPPDATA").unwrap())
        };
        base_dir.join(APP_NAME)
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

    pub fn remove_done_tasks(&self, clipboard_index: usize) {
        let mut data = self.read_data();
        let clipboard = &mut data.clipboards[clipboard_index];
        clipboard
            .tasks
            .retain(|task| !task.is_completed && !task.description.is_empty());
        if clipboard.tasks.len() < MAX_TASKS {
            clipboard
                .tasks
                .extend((0..(MAX_TASKS - clipboard.tasks.len())).map(|_| Task::default()));
        }
        self.write_data(data);
    }
}
