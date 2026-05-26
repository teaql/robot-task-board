use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

// Declare submodules
mod utils;
mod ui;
pub mod service;

// Import our decoupled submodules' types (no direct TeaQL references!)
use service::{TaskService, TaskModel, MoveResult};

const MAX_LOGS: usize = 40;

pub struct App {
    pub input: String,
    pub logs: VecDeque<String>,
    pub planned_tasks: Vec<TaskModel>,
    pub process_tasks: Vec<TaskModel>,
    pub done_tasks: Vec<TaskModel>,
    pub planned_count: usize,
    pub process_count: usize,
    pub done_count: usize,
    pub service: TaskService,
    pub search_term: Option<String>,
    pub should_quit: bool,
    pub cpu_model: String,
    pub mem_size: String,
}

impl App {
    pub fn new(service: TaskService) -> Self {
        let sys_info = utils::get_system_info();
        let mut app = Self {
            input: String::new(),
            logs: VecDeque::new(),
            planned_tasks: Vec::new(),
            process_tasks: Vec::new(),
            done_tasks: Vec::new(),
            planned_count: 0,
            process_count: 0,
            done_count: 0,
            service,
            search_term: None,
            should_quit: false,
            cpu_model: sys_info.cpu_model,
            mem_size: sys_info.mem_size,
        };
        app.add_log("System successfully initialized.");
        app.add_log("Pre-loaded SQLite database 'robot_kanban.db'.");
        app.add_log("TeaQL v0.9.3: Comment Propagation is fully active.");
        app
    }

    pub fn add_log(&mut self, msg: &str) {
        if self.logs.len() >= MAX_LOGS {
            self.logs.pop_front();
        }
        self.logs.push_back(msg.to_owned());
    }

    pub fn check_sql_logs(&mut self) {
        let new_logs = self.service.check_sql_logs();
        for log in new_logs {
            self.add_log(&log);
        }
    }

    pub async fn reload_data(&mut self) -> Result<(), Box<dyn Error>> {
        let res = self.service.reload_data(&self.search_term).await?;

        self.planned_tasks = res.planned_tasks;
        self.process_tasks = res.process_tasks;
        self.done_tasks = res.done_tasks;
        self.planned_count = res.planned_count;
        self.process_count = res.process_count;
        self.done_count = res.done_count;

        self.add_log(&res.query_trace);

        Ok(())
    }

    pub async fn execute_command(&mut self) -> Result<(), Box<dyn Error>> {
        let trimmed = self.input.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0].to_lowercase();
        let args = if parts.len() > 1 { parts[1].trim() } else { "" };

        match cmd.as_str() {
            "exit" | "quit" | "q" => {
                self.should_quit = true;
                self.add_log("Exiting application...");
            }
            "search" | "s" => {
                if args.is_empty() {
                    self.search_term = None;
                    self.add_log("Cleared active search query.");
                } else {
                    self.search_term = Some(args.to_owned());
                    self.add_log(&format!("Searching for tasks by keyword: '{}'", args));
                }
                self.reload_data().await?;
            }
            "add" => {
                if args.is_empty() {
                    self.add_log("Error: Task name cannot be empty. Usage: add <task name>");
                } else {
                    let next_id = self.service.add_task(args).await?;
                    self.add_log(&format!("Created task [ID: {}] '{}'", next_id, args));
                    self.reload_data().await?;
                }
            }
            "delete" | "del" => {
                if args.is_empty() {
                    self.add_log("Error: Missing task ID. Usage: delete <id>");
                } else if let Ok(id) = args.parse::<u64>() {
                    if self.service.delete_task(id).await? {
                        self.add_log(&format!("Deleted task [ID: {}]", id));
                        self.reload_data().await?;
                    } else {
                        self.add_log(&format!("Error: Task with ID {} not found", id));
                    }
                } else {
                    self.add_log(&format!("Error: Invalid task ID '{}'", args));
                }
            }
            "move" | "mv" => {
                if args.is_empty() {
                    self.add_log("Error: Missing arguments. Usage: move <id> [planned|process|done|next]");
                    self.input.clear();
                    return Ok(());
                }

                let move_parts: Vec<&str> = args.split_whitespace().collect();

                if let Ok(id) = move_parts[0].parse::<u64>() {
                    let target_status = if move_parts.len() > 1 {
                        move_parts[1].to_lowercase()
                    } else {
                        "".to_owned() // Triggers next transition
                    };

                    let res = self.service.move_task(id, &target_status).await?;
                    match res {
                        MoveResult::Moved { status_name, query_trace } => {
                            self.add_log(&query_trace);
                            self.add_log(&format!("Moved task {} to '{}' (DDD transition)", id, status_name));
                            self.reload_data().await?;
                        }
                        MoveResult::AlreadyDone { query_trace } => {
                            self.add_log(&query_trace);
                            self.add_log(&format!("Task {} is already in 'Done' status", id));
                        }
                        MoveResult::Error { err_msg, query_trace } => {
                            self.add_log(&query_trace);
                            self.add_log(&format!("Error: {}", err_msg));
                        }
                        MoveResult::NotFound { query_trace } => {
                            self.add_log(&query_trace);
                            self.add_log(&format!("Error: Task with ID {} not found", id));
                        }
                    }
                } else {
                    self.add_log(&format!("Error: Invalid task ID '{}'", move_parts[0]));
                }
            }
            _ => {
                self.add_log(&format!("Unknown command: '{}'. Valid commands: add, delete (del), move (mv), search (s), exit (q)", cmd));
            }
        }

        self.input.clear();
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Initialize SQLite Database, Schema & seed initial values
    let service = TaskService::new("robot_kanban.db").await?;

    // 2. Initialize terminal and ratatui backend
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::cursor::Show,
        crossterm::cursor::EnableBlinking
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.show_cursor()?;

    // 3. Initialize app state
    let mut app = App::new(service);
    app.reload_data().await?;

    // 4. Main application loop
    let loop_res = run_app(&mut terminal, &mut app).await;

    // 5. Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = loop_res {
        println!("Application error: {}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        app.check_sql_logs();
        terminal.draw(|f| ui::ui(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => {
                            if let Err(e) = app.execute_command().await {
                                app.add_log(&format!("Error: {}", e));
                            }
                        }
                        KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_comment_propagation() -> Result<(), Box<dyn std::error::Error>> {
        // 1. Delete old test files if present
        let _ = std::fs::remove_file("test_propagation.db");
        let _ = std::fs::remove_file("app.log");

        // 2. Initialize service directly
        let db = TaskService::new("test_propagation.db").await?;

        // 3. Reload data with None (triggering Get active tasks)
        let _ = db.reload_data(&None).await?;

        // 4. Retrieve logged entries returned by check_sql_logs()
        let formatted_logs = db.check_sql_logs();
        
        println!("=== Captured Formatted Logs ===");
        let mut found_facet_status_query = false;
        let mut found_facet_task_query = false;
        for log in &formatted_logs {
            println!("{}", log);
            if log.contains("Get active tasks->status_stats->Count status") {
                if log.contains("task_status_data") {
                    found_facet_status_query = true;
                }
                if log.contains("task_data") && log.contains("COUNT(*)") {
                    found_facet_task_query = true;
                }
            }
        }

        // Cleanup test db and log
        let _ = std::fs::remove_file("test_propagation.db");
        let _ = std::fs::remove_file("app.log");

        assert!(
            found_facet_status_query,
            "Comment propagation chain [Get active tasks->status_stats->Count status] not found in task_status_data subquery log!"
        );
        assert!(
            found_facet_task_query,
            "Comment propagation chain [Get active tasks->status_stats->Count status] not found in task_data relation aggregate subquery log!"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_add_and_delete_task() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_add_delete.db";
        let _ = std::fs::remove_file(db_file);

        // 1. Initialize Service and add a task
        let db = TaskService::new(db_file).await?;
        let task_id = db.add_task("Verify Task Flow").await?;
        assert!(task_id > 0, "Task ID should be greater than 0");

        // 2. Reload data and verify task presence
        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 1, "Should have exactly 1 planned task");
        assert_eq!(reloaded.planned_tasks[0].name, "Verify Task Flow");

        // 3. Delete task and verify it's gone
        let deleted = db.delete_task(task_id).await?;
        assert!(deleted, "Task deletion should be successful");

        let reloaded_after = db.reload_data(&None).await?;
        assert_eq!(reloaded_after.planned_tasks.len(), 0, "Planned task list should be empty after deletion");

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }

    #[tokio::test]
    async fn test_move_task_ddd() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_move_task.db";
        let _ = std::fs::remove_file(db_file);

        // 1. Initialize and add a task
        let db = TaskService::new(db_file).await?;
        let task_id = db.add_task("DDD Aggregates Transition").await?;

        // Verify initial status (Planned = 1)
        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 1);
        assert_eq!(reloaded.process_tasks.len(), 0);

        // 2. Move next (empty command moves Planned -> Process)
        let res = db.move_task(task_id, "").await?;
        match res {
            MoveResult::Moved { status_name, .. } => {
                assert_eq!(status_name, "Process");
            }
            _ => panic!("Expected task to be moved"),
        }

        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 0);
        assert_eq!(reloaded.process_tasks.len(), 1);

        // 3. Move directly to Done
        let res2 = db.move_task(task_id, "done").await?;
        match res2 {
            MoveResult::Moved { status_name, .. } => {
                assert_eq!(status_name, "Done");
            }
            _ => panic!("Expected task to be moved to Done"),
        }

        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.process_tasks.len(), 0);
        assert_eq!(reloaded.done_tasks.len(), 1);

        // 4. Test invalid status move
        let res3 = db.move_task(task_id, "invalid_status").await?;
        match res3 {
            MoveResult::Error { err_msg, .. } => {
                assert!(err_msg.contains("Invalid status"));
            }
            _ => panic!("Expected move to fail with invalid status"),
        }

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }
}
