use serde::{Deserialize, Serialize};

pub const MAX_TASKS: usize = 9;
const MAX_CLIPBOARDS: usize = 3;

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
pub struct Clipboard {
    pub title: String,
    pub tasks: Vec<Task>,
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
    pub clipboards: Vec<Clipboard>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            clipboards: vec![Clipboard::default(); MAX_CLIPBOARDS],
        }
    }
}
