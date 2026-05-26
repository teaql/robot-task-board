use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Terminal;

use rusqlite::Connection;
use teaql_core::{DeleteCommand, Expr, UpdateCommand, TeaqlEntity};
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteDialect, RusqliteIdSpaceGenerator,
    RusqliteMutationExecutor, RusqliteProviderExt,
};
use teaql_runtime::{
    GraphTransactionBoundary, InMemoryMetadataStore, InMemoryRepositoryRegistry,
    QueryExecutor, UserContext,
};

// Import generated entities
use robot_kanban::{Platform, Task, TaskStatus, TeaqlRuntimeContext};

#[derive(Clone)]
struct LoggingExecutor {
    inner: RusqliteMutationExecutor,
}

impl QueryExecutor for LoggingExecutor {
    type Error = teaql_provider_rusqlite::MutationExecutorError;

    fn fetch_all(
        &self,
        query: &teaql_sql::CompiledQuery,
    ) -> Result<Vec<teaql_core::Record>, Self::Error> {
        QueryExecutor::fetch_all(&self.inner, query)
    }

    fn execute(&self, query: &teaql_sql::CompiledQuery) -> Result<u64, Self::Error> {
        QueryExecutor::execute(&self.inner, query)
    }

    fn begin_transaction(&self) -> Result<GraphTransactionBoundary, Self::Error> {
        QueryExecutor::begin_transaction(&self.inner)
    }

    fn commit_transaction(&self) -> Result<(), Self::Error> {
        QueryExecutor::commit_transaction(&self.inner)
    }

    fn rollback_transaction(&self) -> Result<(), Self::Error> {
        QueryExecutor::rollback_transaction(&self.inner)
    }
}

struct TransitionCommand {
    pub target_status: String,
}

#[derive(Debug, Clone)]
struct DomainTask {
    pub task: Task,
}

impl teaql_core::TeaqlEntity for DomainTask {
    fn entity_descriptor() -> teaql_core::EntityDescriptor {
        Task::entity_descriptor()
    }
}

impl teaql_core::Entity for DomainTask {
    fn from_record(record: teaql_core::Record) -> Result<Self, teaql_core::EntityError> {
        let task = Task::from_record(record)?;
        Ok(Self { task })
    }

    fn into_record(self) -> teaql_core::Record {
        self.task.into_record()
    }
}

impl DomainTask {
    /// Domain behavior method showing DDD Aggregate Root logic.
    /// Transitions task status based on a TransitionCommand object.
    /// If target status is empty, it automatically moves to the next phase.
    pub fn transition_status(&mut self, cmd: &TransitionCommand) -> Result<Option<u64>, String> {
        let current_status = self.task.status_id();
        let target = cmd.target_status.trim().to_lowercase();

        let next_status_id = if target.is_empty() {
            // Planned -> Process -> Done
            if current_status == 1 {
                Some(2_u64)
            } else if current_status == 2 {
                Some(3_u64)
            } else {
                None
            }
        } else {
            match target.as_str() {
                "planned" => Some(1_u64),
                "process" => Some(2_u64),
                "done" => Some(3_u64),
                _ => return Err(format!("Invalid status '{}'. Use planned, process, done, or empty to move next.", cmd.target_status)),
            }
        };

        Ok(next_status_id)
    }
}

struct App {
    input: String,
    logs: Vec<String>,
    planned_tasks: Vec<Task>,
    process_tasks: Vec<Task>,
    done_tasks: Vec<Task>,
    planned_count: usize,
    process_count: usize,
    done_count: usize,
    ctx: TeaqlRuntimeContext<RusqliteDialect, LoggingExecutor>,
    inner_executor: RusqliteMutationExecutor,
    last_log_index: usize,
    search_term: Option<String>,
    should_quit: bool,
}

impl App {
    fn new(
        ctx: TeaqlRuntimeContext<RusqliteDialect, LoggingExecutor>,
        inner_executor: RusqliteMutationExecutor,
    ) -> Self {
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
        };
        app.add_log("System successfully initialized.");
        app.add_log("Pre-loaded SQLite database 'robot_kanban.db'.");
        app
    }

    fn add_log(&mut self, msg: &str) {
        self.logs.push(msg.to_owned());
    }

    fn check_sql_logs(&mut self) {
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

    fn build_search_json(&self) -> String {
        if let Some(ref term) = self.search_term {
            format!(r#"{{"name": "{}"}}"#, term)
        } else {
            r#"{}"#.to_owned()
        }
    }

    fn build_search_comment(&self) -> &'static str {
        if self.search_term.is_some() {
            "Get filtered tasks by keyword"
        } else {
            "Get active tasks"
        }
    }

    async fn reload_data(&mut self) -> Result<(), Box<dyn Error>> {
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

    async fn execute_command(&mut self) -> Result<(), Box<dyn Error>> {
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
                self.add_log(&format!("Error: Unknown command '{}'", cmd));
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
        terminal.draw(|f| ui(f, app))?;

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

fn parse_log_line(line: &str) -> Line<'_> {
    if line.len() < 23 || !line.starts_with("202") {
        return Line::from(line.cyan());
    }

    let mut spans = Vec::new();
    
    // 1. Timestamp (Slate gray)
    let timestamp = &line[..23];
    spans.push(Span::styled(timestamp, Style::default().fg(Color::Indexed(244))));

    let mut rest = &line[23..];

    // 2. User ID bracket e.g. -[philip@pid-xxx.tid-x] (Neon Violet)
    if rest.starts_with("-[") {
        if let Some(end) = rest.find(']') {
            let user_part = &rest[..end+1];
            spans.push(Span::styled(user_part, Style::default().fg(Color::Rgb(155, 89, 182)).add_modifier(Modifier::BOLD)));
            rest = &rest[end+1..];
        }
    }

    // 3. Severity e.g. --DEBUG - SqlLogEntry
    if rest.starts_with("--DEBUG - SqlLogEntry") {
        spans.push(Span::styled("--DEBUG - SqlLogEntry", Style::default().fg(Color::Indexed(242))));
        rest = &rest[21..];
    }

    // 4. Comment part and Result summary part
    // Dynamically distinguish between log entries that have comments vs. those that do not.
    if rest.starts_with(" - [") {
        if let Some(end) = rest[4..].find(']') {
            let first_segment = &rest[..end+5];
            let after_first = &rest[end+5..];
            
            if after_first.starts_with(" - [") {
                // If there is another " - [" immediately following, then the first one is the comment!
                spans.push(Span::styled(first_segment, Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD)));
                rest = after_first;
                
                // Now parse the second one as the result summary
                if let Some(end2) = rest[4..].find(']') {
                    let result_part = &rest[..end2+5];
                    spans.push(Span::styled(result_part, Style::default().fg(Color::Rgb(52, 152, 219))));
                    rest = &rest[end2+5..];
                }
            } else {
                // If there is no " - [" following, then this first segment is the result summary (no comment exists)!
                spans.push(Span::styled(first_segment, Style::default().fg(Color::Rgb(52, 152, 219))));
                rest = after_first;
            }
        }
    }

    // 6. SQL statement and elapsed time
    if let Some(took_idx) = rest.rfind(" (took ") {
        let sql = &rest[..took_idx];
        let took = &rest[took_idx..];
        spans.push(Span::styled(sql, Style::default().fg(Color::White)));
        spans.push(Span::styled(took, Style::default().fg(Color::Rgb(231, 76, 60))));
    } else {
        spans.push(Span::styled(rest, Style::default().fg(Color::White)));
    }

    Line::from(spans)
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // 1. Log area
            Constraint::Length(3),      // 2. Status statistics
            Constraint::Min(5),         // 3. Columns (Planned, Process, Done)
            Constraint::Length(3),      // 4. Command Line Area
            Constraint::Length(9),      // 5. Command Help Area
        ])
        .split(f.size());

    // 1. Render Log Area (Keeps beautiful syntax-highlighted logs)
    let log_height = chunks[0].height as usize - 2; // Subtract borders
    let logs_to_show = if app.logs.len() > log_height {
        &app.logs[app.logs.len() - log_height..]
    } else {
        &app.logs
    };
    let log_paragraph = Paragraph::new(
        logs_to_show
            .iter()
            .map(|l| parse_log_line(l))
            .collect::<Vec<Line>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Action Logs & Executed SQL ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::Indexed(240))),
    );
    f.render_widget(log_paragraph, chunks[0]);

    // 2. Render Status Statistics Area (3 equal columns - plain white borders)
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[1]);

    let planned_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Planned tasks count: "),
        Span::styled(
            format!("{}", app.planned_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(planned_stat, stats_chunks[0]);

    let process_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Tasks in Process count: "),
        Span::styled(
            format!("{}", app.process_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(process_stat, stats_chunks[1]);

    let done_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Completed Tasks count: "),
        Span::styled(
            format!("{}", app.done_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(done_stat, stats_chunks[2]);

    // 3. Render task list columns (Planned, Process, Done - plain white borders)
    let col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[2]);

    // Planned Tasks column
    let planned_lines = app
        .planned_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::Indexed(243))),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let planned_list = Paragraph::new(planned_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" PLANNED ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(planned_list, col_chunks[0]);

    // Process Tasks column
    let process_lines = app
        .process_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::Indexed(243))),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let process_list = Paragraph::new(process_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" PROCESS ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(process_list, col_chunks[1]);

    // Done Tasks column
    let done_lines = app
        .done_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::Indexed(243))),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let done_list = Paragraph::new(done_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" DONE ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(done_list, col_chunks[2]);

    // 4. Render Command Line Area (plain white borders)
    let prompt_line = Line::from(vec![
        Span::styled("  >  ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(app.input.clone(), Style::default().fg(Color::White)),
    ]);
    let cmd_input = Paragraph::new(prompt_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Command Line Area ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(cmd_input, chunks[3]);

    // Position and show the cursor inside the input area (adjusted +6: 1 for left border + 5 for prompt arrow)
    f.set_cursor(
        chunks[3].x + 6 + app.input.chars().count() as u16,
        chunks[3].y + 1,
    );

    let help_text = vec![
        Line::from(vec![
            Span::raw("  "),
            Span::styled("add <name>           ", Style::default().fg(Color::White)),
            Span::raw("- Create a new task in Planned status"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("move <id> [s]        ", Style::default().fg(Color::White)),
            Span::raw("- Change status to planned/process/done (default next)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("delete <id>          ", Style::default().fg(Color::White)),
            Span::raw("- Permanently delete task from database"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("search <kw>          ", Style::default().fg(Color::White)),
            Span::raw("- Search tasks by keyword using JSON dynamic EXPR"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("s <kw>               ", Style::default().fg(Color::White)),
            Span::raw("- Shortcut for search (empty keyword to clear search)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("exit | quit          ", Style::default().fg(Color::White)),
            Span::raw("- Quit the application dashboard"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("ESC                  ", Style::default().fg(Color::White)),
            Span::raw("- Immediate escape"),
        ]),
    ];
    let help_box = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Command Help Area ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(help_box, chunks[4]);
}
