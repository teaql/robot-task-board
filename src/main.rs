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
mod startup;
pub mod service;

// Import our decoupled submodules' types (no direct TeaQL references!)
use service::{TaskService, TaskModel, MoveResult};

const MAX_LOGS: usize = 1000;

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
    pub log_scroll_offset: usize,
    pub hide_logs: bool,
}

impl App {
    pub fn new(service: TaskService) -> Self {
        let sys_info = utils::get_system_info();
        let app = Self {
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
            log_scroll_offset: 0,
            hide_logs: std::env::args().any(|arg| arg == "-c"),
        };
        app.service.log_info("TeaQL traces one business request into generated SQL, facets, and audit records.");
        app.service.log_info("System successfully initialized.");
        app.service.log_info("Pre-loaded SQLite database 'robot_kanban.db'.");
        app.service.log_info("TeaQL v0.9.9: Comment Propagation is fully active.");
        app
    }

    pub fn add_log(&mut self, msg: &str) {
        let was_scrolled = self.log_scroll_offset > 0;
        if self.logs.len() >= MAX_LOGS {
            self.logs.pop_front();
            if self.log_scroll_offset > 0 {
                self.log_scroll_offset -= 1;
            }
        }
        self.logs.push_back(msg.to_owned());
        if was_scrolled {
            self.log_scroll_offset += 1;
        }
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

        Ok(())
    }

    pub async fn execute_command(&mut self) -> Result<(), Box<dyn Error>> {
        self.log_scroll_offset = 0;
        let trimmed = self.input.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        // Slash-prefixed commands; bare input defaults to add task
        if trimmed.starts_with('/') {
            let without_slash = &trimmed[1..];
            let parts: Vec<&str> = without_slash.splitn(2, ' ').collect();
            let cmd = parts[0].to_lowercase();
            let args = if parts.len() > 1 { parts[1].trim() } else { "" };

            match cmd.as_str() {
                "exit" | "quit" | "q" => {
                    self.should_quit = true;
                    self.service.log_info("Exiting application...");
                }
                "search" | "s" => {
                    if args.is_empty() {
                        self.search_term = None;
                        self.service.log_info("Cleared active search query.");
                    } else {
                        self.search_term = Some(args.to_owned());
                        self.service.log_info(&format!("Searching for tasks by keyword: '{}'", args));
                    }
                    self.reload_data().await?;
                }
                "add" => {
                    if args.is_empty() {
                        self.service.log_info("Error: Task name cannot be empty. Usage: /add <task name>");
                    } else {
                        let _next_id = self.service.add_task(args).await?;
                        self.reload_data().await?;
                    }
                }
                "delete" | "del" => {
                    if args.is_empty() {
                        self.service.log_info("Error: Missing task ID. Usage: /del <id>");
                    } else if let Ok(id) = args.parse::<u64>() {
                        if self.service.delete_task(id).await? {
                            self.reload_data().await?;
                        }
                    } else {
                        self.service.log_info(&format!("Error: Invalid task ID '{}'", args));
                    }
                }
                "move" | "mv" => {
                    if args.is_empty() {
                        self.service.log_info("Error: Missing arguments. Usage: /mv <id> [planned|process|done|next]");
                        self.input.clear();
                        return Ok(());
                    }

                    let move_parts: Vec<&str> = args.split_whitespace().collect();

                    if let Ok(id) = move_parts[0].parse::<u64>() {
                        let target_status = if move_parts.len() > 1 {
                            move_parts[1].to_lowercase()
                        } else {
                            "".to_owned()
                        };

                        let res = self.service.move_task(id, &target_status).await?;
                        match res {
                            MoveResult::Moved { .. } => {
                                self.reload_data().await?;
                            }
                            _ => {}
                        }
                    } else {
                        self.service.log_info(&format!("Error: Invalid task ID '{}'", move_parts[0]));
                    }
                }
                _ => {
                    self.service.log_info(&format!("Unknown command: '/{}'. Type a task name directly or use /mv, /del, /s, /q", cmd));
                }
            }
        } else {
            // Default: bare input = add task
            let _next_id = self.service.add_task(trimmed).await?;
            self.reload_data().await?;
        }

        self.input.clear();
        self.check_sql_logs();
        self.add_log("--------------------------------------------------------------------------------");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Initialize terminal and ratatui backend (needed for startup screens)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::cursor::Hide,
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Screen 1: Welcome
    startup::draw_welcome(&mut terminal)?;
    startup::wait_for_key()?;

    // 3. Bootstrap: show "Bootstrapping..." while initializing
    use std::time::Instant;
    let boot_start = Instant::now();

    // Show a minimal bootstrap screen while the service initializes
    startup::draw_bootstrap(&mut terminal, &[
        startup::BootstrapStep { label: "Open SQLite database", completed: false, elapsed_ms: None },
    ], false, None, None)?;

    // 4. Initialize service — real events are fired through ctx.send_event()
    //    and captured in the UnifiedLogBuffer.
    let db_open_start = Instant::now();
    let service = TaskService::new("robot_kanban.db").await?;
    let total_elapsed = boot_start.elapsed();

    // 5. Collect real bootstrap events and build the step list dynamically
    let bootstrap_events = service.drain_bootstrap_events();
    let db_open_ms = db_open_start.elapsed().as_secs_f64() * 1000.0;

    // Compute summary counts from event messages
    let mut tables_created = 0usize;
    let mut tables_verified = 0usize;
    let mut fields_added = 0usize;
    let mut seeds = 0usize;
    for (msg, _) in &bootstrap_events {
        if msg.starts_with("Create ") {
            tables_created += 1;
        } else if msg.starts_with("Verified ") {
            tables_verified += 1;
        } else if msg.starts_with("  + field ") {
            fields_added += 1;
        } else if msg.starts_with("Seed ") {
            seeds += 1;
        }
    }
    let entity_count = tables_created + tables_verified;
    let mut summary_parts = Vec::new();
    summary_parts.push(format!("{} entities", entity_count));
    if tables_created > 0 {
        summary_parts.push(format!("{} tables created", tables_created));
    }
    if tables_verified > 0 {
        summary_parts.push(format!("{} tables verified", tables_verified));
    }
    if fields_added > 0 {
        summary_parts.push(format!("{} fields added", fields_added));
    }
    if seeds > 0 {
        summary_parts.push(format!("{} seeds", seeds));
    }
    let summary = summary_parts.join(", ");

    // Build steps: "Open SQLite database" + "N entities discovered" + events + "Startup complete"
    let mut final_steps: Vec<startup::BootstrapStep> = Vec::new();
    final_steps.push(startup::BootstrapStep {
        label: "Open SQLite database",
        completed: true,
        elapsed_ms: Some(db_open_ms),
    });

    // Insert "N entities discovered" step
    let discovered_label: &'static str = Box::leak(
        format!("{} entities discovered", entity_count).into_boxed_str()
    );
    final_steps.push(startup::BootstrapStep {
        label: discovered_label,
        completed: true,
        elapsed_ms: None,
    });

    // Leak the strings so we can use &'static str in BootstrapStep
    for (event_msg, elapsed_ms) in &bootstrap_events {
        let label: &'static str = Box::leak(event_msg.clone().into_boxed_str());
        final_steps.push(startup::BootstrapStep {
            label,
            completed: true,
            elapsed_ms: Some(*elapsed_ms),
        });
    }
    final_steps.push(startup::BootstrapStep {
        label: "TeaQL Runtime ready",
        completed: true,
        elapsed_ms: None,
    });

    // 6. Animate the bootstrap steps with checkmarks one by one
    for i in 0..final_steps.len() {
        let display_steps: Vec<startup::BootstrapStep> = final_steps
            .iter()
            .enumerate()
            .map(|(j, s)| startup::BootstrapStep {
                label: s.label,
                completed: j <= i,
                elapsed_ms: if j <= i { s.elapsed_ms } else { None },
            })
            .collect();
        let all_done = i == final_steps.len() - 1;
        startup::draw_bootstrap(
            &mut terminal,
            &display_steps,
            all_done,
            if all_done { Some(total_elapsed) } else { None },
            if all_done { Some(&summary) } else { None },
        )?;
        if !all_done {
            std::thread::sleep(Duration::from_millis(80));
        }
    }
    startup::wait_for_key()?;

    // 7. Transition: show cursor for TUI input
    execute!(
        terminal.backend_mut(),
        crossterm::cursor::Show,
        crossterm::cursor::EnableBlinking
    )?;

    // 8. Initialize app state
    let mut app = App::new(service);
    app.reload_data().await?;
    app.service.log_info("=================================================================================================");

    // 9. Main application loop
    let loop_res = run_app(&mut terminal, &mut app).await;

    // 10. Cleanup terminal
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
                        KeyCode::Up => {
                            let log_height = if let Ok(size) = terminal.size() {
                                ((size.height / 2) as usize).saturating_sub(2)
                            } else {
                                10
                            };
                            let max_scroll = app.logs.len().saturating_sub(log_height);
                            app.log_scroll_offset = (app.log_scroll_offset + 1).min(max_scroll);
                        }
                        KeyCode::PageUp => {
                            let log_height = if let Ok(size) = terminal.size() {
                                ((size.height / 2) as usize).saturating_sub(2)
                            } else {
                                10
                            };
                            let max_scroll = app.logs.len().saturating_sub(log_height);
                            app.log_scroll_offset = (app.log_scroll_offset + 10).min(max_scroll);
                        }
                        KeyCode::Down => {
                            app.log_scroll_offset = app.log_scroll_offset.saturating_sub(1);
                        }
                        KeyCode::PageDown => {
                            app.log_scroll_offset = app.log_scroll_offset.saturating_sub(10);
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

    #[tokio::test]
    async fn test_task_execution_log_lineage() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_execution_log_lineage.db";
        let _ = std::fs::remove_file(db_file);

        let db = TaskService::new(db_file).await?;
        let task_id = db.add_task("Lineage Test Task").await?;

        // Move task to Process
        let _ = db.move_task(task_id, "process").await?;

        // Delete task (which explicitly cascade soft-deletes the related logs)
        let _ = db.delete_task(task_id).await?;

        // Retrieve SQL logs from the context to check for the lineage comment
        let sql_logs = db.context().sql_logs();
        
        println!("=== SQL Logs for Lineage Test ===");
        let mut found_created_log_lineage = false;
        let mut found_status_changed_log_lineage = false;

        for entry in &sql_logs {
            let sql = entry.debug_sql.to_lowercase();
            println!("SQL: {}", sql);
            if sql.contains("insert into task_execution_log_data") {
                // EntityGraph produces hierarchical trace chains embedded as SQL comments:
                // "Task(1): Create task '...' -> TaskExecutionLog(1): Create task '...'"
                if sql.contains("taskexecutionlog(1)") && sql.contains("create task") {
                    found_created_log_lineage = true;
                }
                if sql.contains("taskexecutionlog(2)") && sql.contains("move task") {
                    found_status_changed_log_lineage = true;
                }
            }
        }

        let _ = std::fs::remove_file(db_file);

        assert!(
            found_created_log_lineage,
            "Hierarchical lineage comment not propagated to SQL insert log for task creation execution log!"
        );
        assert!(
            found_status_changed_log_lineage,
            "Hierarchical lineage comment not propagated to SQL insert log for status transition execution log!"
        );
        // Verify task is actually deleted (functional check instead of SQL log check)
        let reloaded = db.reload_data(&None).await?;
        assert_eq!(
            reloaded.planned_tasks.len() + reloaded.process_tasks.len() + reloaded.done_tasks.len(),
            0,
            "Task should be soft-deleted and not visible after reload"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_unified_log_order() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_unified_log_order.db";
        let _ = std::fs::remove_file(db_file);

        let db = TaskService::new(db_file).await?;

        // 1. Initial reload (simulates app startup)
        let _ = db.reload_data(&None).await?;
        let startup_logs = db.check_sql_logs();
        println!("=== STARTUP RELOAD ===");
        for (i, log) in startup_logs.iter().enumerate() {
            println!("{:02}: {}", i, log);
        }

        // 2. Add a task + reload
        let _ = db.add_task("My New Task").await?;
        let _ = db.reload_data(&None).await?;
        let add_logs = db.check_sql_logs();
        println!("\n=== ADD TASK + RELOAD ===");
        for (i, log) in add_logs.iter().enumerate() {
            println!("{:02}: {}", i, log);
        }

        // 3. Move task + reload
        let _ = db.move_task(1, "Process").await?;
        let _ = db.reload_data(&None).await?;
        let move_logs = db.check_sql_logs();
        println!("\n=== MOVE TASK + RELOAD ===");
        for (i, log) in move_logs.iter().enumerate() {
            println!("{:02}: {}", i, log);
        }

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }
}

