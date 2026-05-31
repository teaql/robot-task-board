/// A lightweight view model for tasks displayed in the TUI.
#[derive(Debug, Clone)]
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
pub struct ReloadedData {
    pub planned_tasks: Vec<TaskModel>,
    pub ready_tasks: Vec<TaskModel>,
    pub executing_tasks: Vec<TaskModel>,
    pub verified_tasks: Vec<TaskModel>,
    pub planned_count: usize,
    pub ready_count: usize,
    pub executing_count: usize,
    pub verified_count: usize,
    pub query_trace: String,
}

/// Result of a move-task operation.
pub enum MoveResult {
    Moved { status_name: String, query_trace: String },
    AlreadyFinal { query_trace: String },
    Error { err_msg: String, query_trace: String },
    NotFound { query_trace: String },
}
