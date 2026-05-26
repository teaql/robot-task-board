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

use rusqlite::Connection;
use teaql_core::{DeleteCommand, Expr, UpdateCommand, TeaqlEntity};
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteDialect, RusqliteIdSpaceGenerator,
    RusqliteMutationExecutor, RusqliteProviderExt,
};
use teaql_runtime::{
    InMemoryMetadataStore, InMemoryRepositoryRegistry,
    UserContext,
};

// Import generated entities
use robot_kanban::{Platform, Task, TaskStatus, TeaqlRuntimeContext};

// Declare submodules
mod utils;
mod model;
mod ui;

// Import our new submodules' types
use model::{LoggingExecutor, TransitionCommand, DomainTask};

pub struct App {
    pub input: String,
    pub logs: Vec<String>,
    pub planned_tasks: Vec<Task>,
    pub process_tasks: Vec<Task>,
    pub done_tasks: Vec<Task>,
    pub planned_count: usize,
    pub process_count: usize,
    pub done_count: usize,
    pub ctx: TeaqlRuntimeContext<RusqliteDialect, LoggingExecutor>,
    pub inner_executor: RusqliteMutationExecutor,
    pub last_log_index: usize,
    pub search_term: Option<String>,
    pub should_quit: bool,
    pub cpu_model: String,
    pub mem_size: String,
}

impl App {
    pub fn new(
        ctx: TeaqlRuntimeContext<RusqliteDialect, LoggingExecutor>,
        inner_executor: RusqliteMutationExecutor,
    ) -> Self {
        let sys_info = utils::get_system_info();
        let mut app = Self {
            input: String::new(),
            logs: Vec::new(),
            planned_tasks: Vec::new(),
            process_tasks: Vec::new(),
            done_tasks: Vec::new(),
            planned_count: 0,
            process_count: 0,
            done_count: 0,
            ctx,
            inner_executor,
            last_log_index: 0,
            search_term: None,
            should_quit: false,
            cpu_model: sys_info.cpu_model,
            mem_size: sys_info.mem_size,
        };
        app.add_log("System successfully initialized.");
        app.add_log("Pre-loaded SQLite database 'robot_kanban.db'.");
        app
    }

    pub fn add_log(&mut self, msg: &str) {
        self.logs.push(msg.to_owned());
    }

    pub fn check_sql_logs(&mut self) {
        let sql_logs = self.ctx.context().sql_logs();
        if sql_logs.len() > self.last_log_index {
            for entry in &sql_logs[self.last_log_index..] {
                let local_time: chrono::DateTime<chrono::Local> = entry.started_at.into();
                let timestamp_str = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                let user_id_str = entry.user_identifier.as_deref().unwrap_or("");
                let comment_part = if let Some(ref c) = entry.comment {
                    format!(" - [{c}]")
                } else {
                    "".to_owned()
                };
                let elapsed_ms = entry.elapsed.as_secs_f64() * 1000.0;
                let log_line = format!(
                    "{timestamp_str}-[{user_id_str}]--DEBUG - SqlLogEntry{} - [{}] {} (took {:.3}ms)",
                    comment_part, entry.result_summary, entry.debug_sql, elapsed_ms
                );
                self.add_log(&log_line);
            }
            self.last_log_index = sql_logs.len();
        }
    }

    pub fn build_search_json(&self) -> String {
        if let Some(ref term) = self.search_term {
            format!(r#"{{"name": "{}"}}"#, term)
        } else {
            r#"{}"#.to_owned()
        }
    }

    pub fn build_search_comment(&self) -> &'static str {
        if self.search_term.is_some() {
            "Get filtered tasks by keyword"
        } else {
            "Get active tasks"
        }
    }

    pub async fn reload_data(&mut self) -> Result<(), Box<dyn Error>> {
        use robot_kanban::Q;

        let select = Q::tasks()
            .comment(self.build_search_comment())
            .find_with_json(self.build_search_json())
            .facet_by_status_as("status_stats", Q::task_status().comment("Count status").count_tasks());

        self.add_log(&format!(
            "Q: Q::tasks().comment(\"{}\").find_with_json(\"{}\").facet_by_status_as(\"status_stats\", Q::task_status().comment(\"Count status\").count_tasks())",
            self.build_search_comment(),
            self.build_search_json().replace('"', "\\\"")
        ));

        let all_tasks = select.execute_for_list(&self.ctx).await?;

        // Retrieve status statistics directly from the query facets
        self.planned_count = 0;
        self.process_count = 0;
        self.done_count = 0;

        if let Some(facet_list) = all_tasks.facet("status_stats") {
            for record in facet_list.iter() {
                let status_id = match record.get("id") {
                    Some(teaql_core::Value::U64(id)) => *id,
                    Some(teaql_core::Value::I64(id)) => *id as u64,
                    _ => 0,
                };
                let count = match record.get("count_tasks") {
                    Some(teaql_core::Value::U64(c)) => *c as usize,
                    Some(teaql_core::Value::I64(c)) => *c as usize,
                    _ => 0,
                };
                match status_id {
                    1 => self.planned_count = count,
                    2 => self.process_count = count,
                    3 => self.done_count = count,
                    _ => {}
                }
            }
        }

        self.planned_tasks.clear();
        self.process_tasks.clear();
        self.done_tasks.clear();

        for task in all_tasks.data {
            match task.status_id() {
                1 => self.planned_tasks.push(task),
                2 => self.process_tasks.push(task),
                3 => self.done_tasks.push(task),
                _ => {}
            }
        }

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
                    let id_gen = RusqliteIdSpaceGenerator::from_executor(self.inner_executor.clone());
                    let next_id = id_gen.next_id("Task")?;

                    let repo = self
                        .ctx
                        .context()
                        .resolve_repository::<RusqliteDialect, LoggingExecutor>("Task")?;

                    repo.insert(
                        &repo
                            .insert_command()
                            .value("id", next_id)
                            .value("version", 1_i64)
                            .value("name", args.to_owned())
                            .value("status_id", 1_u64) // Planned status ID
                            .value("platform_id", 1_u64), // Platform ID
                    )?;

                    self.add_log(&format!("Created task [ID: {}] '{}'", next_id, args));
                    self.reload_data().await?;
                }
            }
            "delete" | "del" => {
                if args.is_empty() {
                    self.add_log("Error: Missing task ID. Usage: delete <id>");
                } else if let Ok(id) = args.parse::<u64>() {
                    let repo = self
                        .ctx
                        .context()
                        .resolve_repository::<RusqliteDialect, LoggingExecutor>("Task")?;

                    // Let's find the task to get its version
                    let select = repo.select().project("version").filter(Expr::eq("id", id));
                    let found_tasks = repo.fetch_entities::<Task>(&select)?;

                    if let Some(task) = found_tasks.first() {
                        repo.delete(&DeleteCommand::new("Task", id).expected_version(task.version()))?;
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
                if move_parts.is_empty() {
                    self.add_log("Error: Missing task ID. Usage: move <id> [planned|process|done|next]");
                    self.input.clear();
                    return Ok(());
                }

                if let Ok(id) = move_parts[0].parse::<u64>() {
                    let target_status = if move_parts.len() > 1 {
                        move_parts[1].to_lowercase()
                    } else {
                        "next".to_owned()
                    };

                    let target_arg = if target_status == "next" {
                        "".to_owned() // Pass empty string to trigger domain next transition logic!
                    } else {
                        target_status
                    };

                    use robot_kanban::Q;

                    let select = Q::tasks()
                        .comment("Get task for DDD")
                        .filter(Expr::eq("id", id))
                        .return_type::<DomainTask>();

                    self.add_log(&format!(
                        "Q: Q::tasks().comment(\"Get task for DDD\").filter(Expr::eq(\"id\", {})).return_type::<DomainTask>()",
                        id
                    ));

                    let found_tasks = select.execute_for_list(&self.ctx).await?;

                    if let Some(mut domain_task) = found_tasks.into_iter().next() {
                        let cmd_obj = TransitionCommand {
                            target_status: target_arg,
                        };
                        let transition_result = domain_task.transition_status(&cmd_obj);

                        match transition_result {
                            Ok(Some(new_status)) => {
                                let repo = self
                                    .ctx
                                    .context()
                                    .resolve_repository::<RusqliteDialect, LoggingExecutor>("Task")?;

                                repo.update(
                                    &UpdateCommand::new("Task", id)
                                        .expected_version(domain_task.task.version())
                                        .value("status_id", new_status),
                                )?;

                                let status_name = match new_status {
                                    1 => "Planned",
                                    2 => "Process",
                                    3 => "Done",
                                    _ => "Unknown",
                                };

                                self.add_log(&format!("Moved task {} to '{}' (DDD transition)", id, status_name));
                                self.reload_data().await?;
                            }
                            Ok(None) => {
                                self.add_log(&format!("Task {} is already in 'Done' status", id));
                            }
                            Err(err_msg) => {
                                self.add_log(&format!("Error: {}", err_msg));
                            }
                        }
                    } else {
                        self.add_log(&format!("Error: Task with ID {} not found", id));
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
    // 1. Initialize SQLite Database & context
    let conn = Connection::open("robot_kanban.db")?;
    let inner_executor = RusqliteMutationExecutor::new(conn);

    let logging_executor = LoggingExecutor {
        inner: inner_executor.clone(),
    };

    let mut ctx = UserContext::new()
        .with_metadata(
            InMemoryMetadataStore::new()
                .with_entity(Platform::entity_descriptor())
                .with_entity(TaskStatus::entity_descriptor())
                .with_entity(Task::entity_descriptor()),
        )
        .with_repository_registry(
            InMemoryRepositoryRegistry::new()
                .with_entity("Platform")
                .with_entity("TaskStatus")
                .with_entity("Task"),
        );

    // Register our synchronous executors
    ctx.use_rusqlite_provider(inner_executor.clone());
    ctx.insert_resource(logging_executor.clone());

    // 2. Build Schema & seed initial values if missing
    ensure_rusqlite_schema_for(&ctx)?;

    // Seed initial Platform if empty
    let platform_repo = ctx
        .resolve_repository::<RusqliteDialect, LoggingExecutor>("Platform")?;
    let plat_select = platform_repo.select().project("id");
    let platforms = platform_repo.fetch_entities::<Platform>(&plat_select)?;
    if platforms.is_empty() {
        platform_repo.insert(
            &platform_repo
                .insert_command()
                .value("id", 1_u64)
                .value("name", "Robot System".to_owned())
                .value("founded", chrono::Utc::now())
                .value("version", 1_i64),
        )?;
    }

    // Seed initial task statuses if empty
    let status_repo = ctx
        .resolve_repository::<RusqliteDialect, LoggingExecutor>("TaskStatus")?;
    let stat_select = status_repo.select().project("id");
    let statuses = status_repo.fetch_entities::<TaskStatus>(&stat_select)?;
    if statuses.is_empty() {
        status_repo.insert(
            &status_repo
                .insert_command()
                .value("id", 1_u64)
                .value("name", "Planned".to_owned())
                .value("code", "PLANNED".to_owned())
                .value("version", 1_i64)
                .value("platform_id", 1_u64),
        )?;
        status_repo.insert(
            &status_repo
                .insert_command()
                .value("id", 2_u64)
                .value("name", "Process".to_owned())
                .value("code", "PROCESS".to_owned())
                .value("version", 1_i64)
                .value("platform_id", 1_u64),
        )?;
        status_repo.insert(
            &status_repo
                .insert_command()
                .value("id", 3_u64)
                .value("name", "Done".to_owned())
                .value("code", "DONE".to_owned())
                .value("version", 1_i64)
                .value("platform_id", 1_u64),
        )?;
    }

    // 3. Initialize terminal and ratatui backend
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

    // 4. Initialize app state
    let rt_ctx = TeaqlRuntimeContext::<RusqliteDialect, LoggingExecutor>::new(ctx);
    let mut app = App::new(rt_ctx, inner_executor);
    app.reload_data().await?;

    // 5. Main application loop
    let loop_res = run_app(&mut terminal, &mut app).await;

    // 6. Cleanup terminal
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
