use serde::{Deserialize, Serialize};

/// A lightweight view model for tasks displayed in the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskModel {
    pub id: u64,
    pub name: String,
}

impl TaskModel {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Data returned from a reload query.
#[derive(Serialize, Deserialize)]
pub struct ReloadedData {
    pub planned_tasks: Vec<TaskModel>,
    pub ready_tasks: Vec<TaskModel>,
    pub executing_tasks: Vec<TaskModel>,
    pub verified_tasks: Vec<TaskModel>,
    pub planned_count: usize,
    pub ready_count: usize,
    pub executing_count: usize,
    pub verified_count: usize,
}

/// Result of a move-task operation.
pub enum MoveResult {
    Moved { status_name: String },
    AlreadyFinal,
    Error { err_msg: String },
    NotFound,
}
