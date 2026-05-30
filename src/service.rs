use std::error::Error;
use std::sync::Mutex;
use robot_kanban::{Q, Task, TaskExecutionLog, TeaqlEntityRepository, TeaqlRuntime};
use teaql_core::EntityGraph;
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteIdSpaceGenerator,
    RusqliteMutationExecutor, RusqliteProviderExt, MutationExecutorError,
};
use teaql_runtime::{
    UserContext, QueryExecutor, GraphTransactionBoundary, EntityEvent,
    EntityEventKind, EntityEventSink, RuntimeError, UnifiedLogEntry, UnifiedLogBuffer, LogPayload,
};
use teaql_core::{Record, Value, TeaqlEntity};
use teaql_sql::CompiledQuery;

pub trait UserContextExt {
    fn next_id_for<T: TeaqlEntity>(&self) -> Result<u64, Box<dyn Error>>;
}

impl UserContextExt for UserContext {
    fn next_id_for<T: TeaqlEntity>(&self) -> Result<u64, Box<dyn Error>> {
        self.generate_id(&T::entity_descriptor().name)?
            .ok_or_else(|| "ID generator not configured for UserContext".into())
    }
}

/// Extract just the OS username from the full user identifier (e.g. "philip@pid-123.tid-1" → "philip")
fn short_user(ctx: &UserContext) -> String {
    let full = ctx.user_identifier().unwrap_or("unknown");
    full.split('@').next().unwrap_or(full).to_owned()
}

/// Post-process a log line from teaql-runtime:
/// 1. Shorten [philip@pid-xxx.tid-x] → [philip]
/// 2. Move (took X.XXXms) from end to after username: [philip]-[0.231ms]
fn reformat_log_line(line: &str) -> String {
    let mut result = line.to_owned();

    // 1. Shorten user identifier: [philip@pid-xxx.tid-x] → [philip]
    if let Some(start) = result.find("]-[") {
        let user_start = start + 3; // skip "]-["
        if let Some(at_pos) = result[user_start..].find('@') {
            let at_abs = user_start + at_pos;
            if let Some(end_bracket) = result[at_abs..].find(']') {
                let end_abs = at_abs + end_bracket;
                // Extract the took time from end of line
                let took_str = if let Some(took_start) = result.rfind("(took ") {
                    if result.ends_with(')') {
                        let took = &result[took_start+6..result.len()-1]; // "0.231ms"
                        let took_val = took.to_owned();
                        result = format!("{}{}", &result[..took_start].trim_end(), "");
                        Some(took_val)
                    } else {
                        None
                    }
                } else {
                    None
                };
                // Replace @pid-xxx.tid-x with took time or nothing
                let replacement = if let Some(ref took) = took_str {
                    format!("]-[{}", took)
                } else {
                    String::new()
                };
                result = format!("{}{}{}", &result[..at_abs], replacement, &result[end_abs..]);
            }
        }
    }

    result
}

fn format_val_helper(val: &Option<Value>) -> String {
    match val {
        Some(Value::Null) | None => "NULL".to_owned(),
        Some(Value::Text(s)) => format!("'{}'", s),
        Some(Value::I64(n)) => n.to_string(),
        Some(Value::U64(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Timestamp(t)) => t.format("%Y-%m-%d %H:%M:%S").to_string(),
        Some(other) => format!("{:?}", other),
    }
}


pub struct AppAuditSink;

impl EntityEventSink for AppAuditSink {
    fn on_event(&self, ctx: &UserContext, event: &EntityEvent) -> Result<(), RuntimeError> {
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f").to_string();
        let user = short_user(ctx);

        let action_name = match event.kind {
            EntityEventKind::Created => "CREATED",
            EntityEventKind::Updated => "UPDATED",
            EntityEventKind::Deleted => "DELETED",
            EntityEventKind::Recovered => "RECOVERED",
        };

        let entity_id_str = match event.values.get("id") {
            Some(id_val) => match id_val {
                Value::Text(s) => s.clone(),
                Value::I64(n) => n.to_string(),
                Value::U64(n) => n.to_string(),
                Value::Null => "NULL".to_owned(),
                other => format!("{:?}", other),
            },
            None => "UNKNOWN".to_owned(),
        };
        let entity_identity = format!("{}({})", event.entity, entity_id_str);

        let comment_part = if event.trace_chain.is_empty() {
            "".to_owned()
        } else {
            let trace = event.trace_chain.iter().map(|n| n.comment.clone()).collect::<Vec<_>>().join(" -> ");
            format!(" [{}]", trace)
        };

        // Build compact single-line audit for TUI and app.log
        let mut field_changes = Vec::new();
        for change in &event.changes {
            let old_str = format_val_helper(&change.old_value);
            let new_str = format_val_helper(&change.new_value);
            if old_str != new_str {
                field_changes.push(format!("{}: [{} ➔ {}]", change.field, old_str, new_str));
            }
        }
        let fields_part = if field_changes.is_empty() {
            String::new()
        } else {
            format!(" {{{}}}", field_changes.join(",  "))
        };

        let line = format!(
            "[{}]-[{}]-[AUDIT]-Entity [{}] was {}.{}{}",
            timestamp, user, entity_identity, action_name, comment_part, fields_part
        );

        // Write to app.log
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("app.log")
        {
            use std::io::Write;
            let _ = writeln!(file, "{}", line);
        }

        // Write to TUI buffer
        if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
            if let Ok(mut entries) = buf.entries.lock() {
                entries.push(UnifiedLogEntry {
                    timestamp: std::time::SystemTime::now(),
                    user_identifier: Some(user.clone()),
                    trace_chain: event.trace_chain.clone(),
                    payload: LogPayload::Info(teaql_runtime::InfoLogEntry {
                        message: line.clone(),
                    }),
                });
            }
        }

        // Write to audit.log with the long format
        let timestamp_with_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let audit_header = format!(
            "[{}] - [{}] - [AUDIT] Entity [{}] was {}.{}",
            timestamp_with_date, user, entity_identity, action_name, comment_part
        );
        let mut audit_lines = vec![audit_header];
        for change in &event.changes {
            let old_str = format_val_helper(&change.old_value);
            let new_str = format_val_helper(&change.new_value);
            if old_str != new_str {
                let detail = format!(
                    "[{}] - [{}] - [AUDIT]   -> Field [{}]: {} ➔ {}",
                    timestamp_with_date, user, change.field, old_str, new_str
                );
                audit_lines.push(detail);
            }
        }
        audit_lines.push(format!("[{}] - [{}] - [AUDIT] ------------------------------------------------------------", timestamp_with_date, user));
        
        for line in &audit_lines {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("audit.log")
            {
                use std::io::Write;
                let _ = writeln!(file, "{}", line);
            }
        }

        Ok(())
    }
}


#[derive(Clone)]
pub struct LoggingExecutor {
    pub inner: RusqliteMutationExecutor,
}

impl QueryExecutor for LoggingExecutor {
    type Error = MutationExecutorError;

    fn fetch_all(
        &self,
        query: &CompiledQuery,
    ) -> Result<Vec<Record>, Self::Error> {
        QueryExecutor::fetch_all(&self.inner, query)
    }

    fn execute(&self, query: &CompiledQuery) -> Result<u64, Self::Error> {
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

pub trait TaskDomainBehavior {
    fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &UserContext) -> Result<Self, String> where Self: Sized;
    fn transition_status(&self, cmd: &TransitionCommand) -> Result<Option<u64>, String>;
    fn generate_execution_log(&self, log_id: u64, action: &str, detail: &str, ctx: &UserContext) -> TaskExecutionLog;
}

pub struct TransitionCommand {
    pub target_status: String,
}

pub struct CreateTaskCommand {
    pub name: String,
}

pub struct DeleteTaskCommand;

impl TaskDomainBehavior for Task {
    fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &UserContext) -> Result<Self, String> {
        if cmd.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_owned());
        }
        let mut task = Q::tasks().with_id_is(next_id).new_entity(ctx);
        task.update_id(next_id)
            .update_name(cmd.name.clone())
            .update_version(0_i64)
            .update_status_to_planned()
            .update_platform_id(1_u64);
        Ok(task)
    }

    fn generate_execution_log(&self, log_id: u64, action: &str, detail: &str, ctx: &UserContext) -> TaskExecutionLog {
        let mut log = Q::task_execution_logs().with_id_is(log_id).new_entity(ctx);
        log.update_id(log_id)
            .update_action(action.to_owned())
            .update_detail(detail.to_owned())
            .update_version(0_i64)
            .update_task_id(self.id());
        log
    }





    /// Domain behavior method showing DDD Aggregate Root logic.
    /// Transitions task status based on a TransitionCommand object.
    /// If target status is empty, it automatically moves to the next phase.
    fn transition_status(&self, cmd: &TransitionCommand) -> Result<Option<u64>, String> {
        let current_status = self.status_id();
        let target = cmd.target_status.trim().to_lowercase();

        let next_status_id = if target.is_empty() {
            // Planned -> Process -> Done
            if current_status == 1001 {
                Some(1002_u64)
            } else if current_status == 1002 {
                Some(1003_u64)
            } else {
                None
            }
        } else {
            match target.as_str() {
                "planned" => Some(1001_u64),
                "process" => Some(1002_u64),
                "done" => Some(1003_u64),
                _ => return Err(format!("Invalid status '{}'. Use planned, process, done, or empty to move next.", cmd.target_status)),
            }
        };

        Ok(next_status_id)
    }
}

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

pub struct ReloadedData {
    pub planned_tasks: Vec<TaskModel>,
    pub process_tasks: Vec<TaskModel>,
    pub done_tasks: Vec<TaskModel>,
    pub planned_count: usize,
    pub process_count: usize,
    pub done_count: usize,
    pub query_trace: String,
}

pub enum MoveResult {
    Moved { status_name: String, query_trace: String },
    AlreadyDone { query_trace: String },
    Error { err_msg: String, query_trace: String },
    NotFound { query_trace: String },
}

pub struct TaskService {
    ctx: UserContext,
    inner_executor: RusqliteMutationExecutor,
    last_log_index: Mutex<usize>,
}

impl TaskService {
    /// Initializes SQLite database, creates/updates schemas, seeds initial data,
    /// constructs the thread-safe UserContext, and returns the fully configured TaskService.
    pub async fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let conn = rusqlite::Connection::open(db_path)?;
        let inner_executor = RusqliteMutationExecutor::new(conn);

        let logging_executor = LoggingExecutor {
            inner: inner_executor.clone(),
        };

        let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();

        // Register custom TUI log buffer and audit event sink
        let log_buffer = UnifiedLogBuffer::default();
        ctx.insert_resource(log_buffer);
        ctx.set_event_sink(AppAuditSink);

        // Register synchronous executors
        ctx.use_rusqlite_provider(inner_executor.clone());
        ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(inner_executor.clone()));
        ctx.insert_resource(logging_executor.clone());
        
        // Also register ServiceRuntimeExecutor for the generated repository lookups
        let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(inner_executor.clone());
        ctx.insert_resource(service_runtime_executor);

        // Build Schema & seed initial values if missing
        ensure_rusqlite_schema_for(&ctx)?;

        Ok(Self {
            ctx,
            inner_executor,
            last_log_index: Mutex::new(0),
        })
    }

    pub fn context(&self) -> &UserContext {
        &self.ctx
    }

    pub fn log_info(&self, message: &str) {
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f").to_string();
        let user = short_user(&self.ctx);
        let log_line = format!("[{}]-[{}]-[INFO]-{}", timestamp, user, message);
        
        // Write to TUI buffer
        if let Some(buf) = self.ctx.get_resource::<UnifiedLogBuffer>() {
            if let Ok(mut entries) = buf.entries.lock() {
                entries.push(UnifiedLogEntry {
                    timestamp: std::time::SystemTime::now(),
                    user_identifier: Some(user.clone()),
                    trace_chain: Vec::new(),
                    payload: LogPayload::Info(teaql_runtime::InfoLogEntry {
                        message: log_line.clone(),
                    }),
                });
            }
        }

        // Also write to app.log for completeness
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("app.log")
        {
            use std::io::Write;
            let _ = writeln!(file, "{}", log_line);
        }
    }

    pub async fn reload_data(
        &self,
        search_term: &Option<String>,
    ) -> Result<ReloadedData, Box<dyn Error>> {
        let search_comment = if search_term.is_some() {
            "Get filtered tasks by keyword"
        } else {
            "Get active tasks"
        };

        let search_json = if let Some(ref term) = search_term {
            let escaped_name = serde_json::Value::String(term.clone());
            format!(r#"{{"name": {}}}"#, escaped_name)
        } else {
            r#"{}"#.to_owned()
        };

        let select = Q::tasks()
            .comment(search_comment)
            .filter_with_json(&search_json)
            .facet_by_status_as("status_stats", Q::task_status().comment("Count status").count_tasks());

        let query_trace = format!(
            "Q: Q::tasks().comment(\"{}\").filter_with_json(\"{}\").facet_by_status_as(\"status_stats\", Q::task_status().comment(\"Count status\").count_tasks())",
            search_comment,
            search_json.replace('"', "\\\"")
        );

        // Unified logging: Log the query trace before running the query
        self.log_info(&query_trace);

        let all_tasks = select.execute_for_list(&self.ctx).await?;

        let mut planned_count = 0;
        let mut process_count = 0;
        let mut done_count = 0;

        if let Some(facet_list) = all_tasks.facet("status_stats") {
            for record in facet_list.iter() {
                let status_id = match record.get("id") {
                    Some(&teaql_core::Value::U64(id)) => id,
                    Some(&teaql_core::Value::I64(id)) => id as u64,
                    _ => 0,
                };
                let count = match record.get("count_tasks") {
                    Some(&teaql_core::Value::U64(c)) => c as usize,
                    Some(&teaql_core::Value::I64(c)) => c as usize,
                    _ => 0,
                };
                match status_id {
                    1001 => planned_count = count,
                    1002 => process_count = count,
                    1003 => done_count = count,
                    _ => {}
                }
            }
        }

        let mut planned_tasks = Vec::new();
        let mut process_tasks = Vec::new();
        let mut done_tasks = Vec::new();

        for task in all_tasks.data {
            let task_model = TaskModel {
                id: task.id(),
                name: task.name().to_string(),
            };
            match task.status_id() {
                1001 => planned_tasks.push(task_model),
                1002 => process_tasks.push(task_model),
                1003 => done_tasks.push(task_model),
                _ => {}
            }
        }

        Ok(ReloadedData {
            planned_tasks,
            process_tasks,
            done_tasks,
            planned_count,
            process_count,
            done_count,
            query_trace,
        })
    }

    pub async fn add_task(&self, name: &str) -> Result<u64, Box<dyn Error>> {
        let next_id = self.ctx.next_id_for::<Task>()?;
        let cmd = CreateTaskCommand { name: name.to_owned() };
        let task = Task::create(&cmd, next_id, &self.ctx)?;

        let log_id = self.ctx.next_id_for::<TaskExecutionLog>()?;
        let log = task.generate_execution_log(log_id, "CREATED", &format!("Task '{}' created.", name), &self.ctx);

        let comment = format!("Create task '{}'", name);
        let repo = self.ctx.task_repository().map_err(|e| Box::new(e) as Box<dyn Error>)?;
        TeaqlEntityRepository::save_entity_graph_from(&repo,
            EntityGraph::new(task)
                .comment(&comment)
                .child("task_execution_log_list",
                    EntityGraph::new(log).comment(&comment))
                .build()
        ).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.log_info(&format!("Created task [ID: {}] '{}'", next_id, name));
        Ok(next_id)
    }

    pub async fn delete_task(&self, id: u64) -> Result<bool, Box<dyn Error>> {
        let select = Q::tasks()
            .comment(&format!("Get task {} for deletion", id))
            .with_id_is(id);

        self.log_info(&format!(
            "Q: Q::tasks().comment(\"Get task {} for deletion\").with_id_is({})",
            id, id
        ));

        let task_opt = select.execute_for_one(&self.ctx).await?;

        if let Some(task) = task_opt {
            let task_name = task.name().to_string();
            let comment = format!("Delete task '{}'", task_name);
            let repo = self.ctx.task_repository().map_err(|e| Box::new(e) as Box<dyn Error>)?;
            TeaqlEntityRepository::save_entity_graph_from(&repo,
                EntityGraph::new(task)
                    .comment(&comment)
                    .delete()
                    .build()
            ).map_err(|e| Box::new(e) as Box<dyn Error>)?;

            self.log_info(&format!("Deleted task [ID: {}]", id));
            Ok(true)
        } else {
            self.log_info(&format!("Error: Task with ID {} not found", id));
            Ok(false)
        }
    }

    pub async fn move_task(
        &self,
        id: u64,
        target_status: &str,
    ) -> Result<MoveResult, Box<dyn Error>> {
        let select = Q::tasks()
            .comment("Get task for DDD")
            .with_id_is(id);

        let query_trace = format!(
            "Q: Q::tasks().comment(\"Get task for DDD\").with_id_is({})",
            id
        );

        self.log_info(&query_trace);

        let task_opt = select.execute_for_one(&self.ctx).await?;

        if let Some(mut task) = task_opt {
            let cmd_obj = TransitionCommand {
                target_status: target_status.to_owned(),
            };
            let transition_result = task.transition_status(&cmd_obj);

            match transition_result {
                Ok(Some(new_status)) => {
                    let old_status_id = task.status_id();
                    match new_status {
                        1001 => { task.update_status_to_planned(); }
                        1002 => { task.update_status_to_process(); }
                        1003 => { task.update_status_to_done(); }
                        _ => {}
                    }
                    let status_name = match new_status {
                        1001 => "Planned",
                        1002 => "Process",
                        1003 => "Done",
                        _ => "Unknown",
                    };
                    let old_status_name = match old_status_id {
                        1001 => "Planned",
                        1002 => "Process",
                        1003 => "Done",
                        _ => "Unknown",
                    };
                    let detail = format!("Status changed from {} to {}.", old_status_name, status_name);
                    
                    let log_id = self.ctx.next_id_for::<TaskExecutionLog>()?;
                    let log = task.generate_execution_log(log_id, "STATUS_CHANGED", &detail, &self.ctx);
                    let task_name = task.name().to_string();

                    let comment = format!("Move task '{}' to {}", task_name, status_name);
                    let repo = self.ctx.task_repository().map_err(|e| Box::new(e) as Box<dyn Error>)?;
                    TeaqlEntityRepository::save_entity_graph_from(&repo,
                        EntityGraph::new(task)
                            .comment(&comment)
                            .child("task_execution_log_list",
                                EntityGraph::new(log).comment(&comment))
                            .build()
                    ).map_err(|e| Box::new(e) as Box<dyn Error>)?;
                    self.log_info(&format!("Moved task {} to '{}' (DDD transition)", id, status_name));

                    Ok(MoveResult::Moved {
                        status_name: status_name.to_owned(),
                        query_trace,
                    })
                }
                Ok(None) => {
                    self.log_info(&format!("Task {} is already in 'Done' status", id));
                    Ok(MoveResult::AlreadyDone { query_trace })
                }
                Err(err_msg) => {
                    self.log_info(&format!("Error: {}", err_msg));
                    Ok(MoveResult::Error { err_msg, query_trace })
                }
            }
        } else {
            self.log_info(&format!("Error: Task with ID {} not found", id));
            Ok(MoveResult::NotFound { query_trace })
        }
    }

    pub fn check_sql_logs(&self) -> Vec<String> {
        let mut new_logs = Vec::new();
        if let Some(buf) = self.ctx.get_resource::<UnifiedLogBuffer>() {
            if let Ok(mut last_log) = self.last_log_index.lock() {
                if let Ok(entries) = buf.entries.lock() {
                    if entries.len() > *last_log {
                        for entry in &entries[*last_log..] {
                            match &entry.payload {
                                LogPayload::Sql(sql_entry) => {
                                    // Manually format a line similar to what reformat_log_line expected
                                    let local_time: chrono::DateTime<chrono::Local> = entry.timestamp.into();
                                    let ts = local_time.format("%H:%M:%S%.3f");
                                    let uid = entry.user_identifier.as_deref().unwrap_or("").split('@').next().unwrap_or("");
                                    let trace = if entry.trace_chain.is_empty() {
                                        "".to_owned()
                                    } else {
                                        format!(" - [{}]", entry.trace_chain.iter().map(|n| n.comment.clone()).collect::<Vec<_>>().join(" -> "))
                                    };
                                    let line = format!("[{}]-[{}]-[DEBUG]-SqlLogEntry{} - [{}] {} - [{:.3}ms]", ts, uid, trace, sql_entry.result_summary, sql_entry.pretty_sql.replace("\n", " "), sql_entry.elapsed.as_secs_f64() * 1000.0);
                                    new_logs.push(line);
                                }
                                LogPayload::Info(info) => {
                                    new_logs.push(info.message.clone());
                                }
                            }
                        }
                        *last_log = entries.len();
                    }
                }
            }
        }
        new_logs
    }
}
