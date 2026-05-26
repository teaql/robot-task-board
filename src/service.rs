use std::error::Error;
use std::sync::Mutex;
use robot_kanban::{Q, Task, TeaqlRuntimeContext, TeaqlRuntime};
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteDialect, RusqliteIdSpaceGenerator,
    RusqliteMutationExecutor, RusqliteProviderExt, MutationExecutorError,
};
use teaql_runtime::{
    UserContext, QueryExecutor, GraphTransactionBoundary,
};
use teaql_core::{Entity, EntityDescriptor, EntityError, DeleteCommand, Record, TeaqlEntity};
use teaql_sql::CompiledQuery;

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

pub struct TransitionCommand {
    pub target_status: String,
}

pub struct CreateTaskCommand {
    pub name: String,
}

pub struct DeleteTaskCommand;

#[derive(Debug, Clone)]
pub struct DomainTask {
    pub task: Task,
}

impl TeaqlEntity for DomainTask {
    fn entity_descriptor() -> EntityDescriptor {
        Task::entity_descriptor()
    }
}

impl Entity for DomainTask {
    fn from_record(record: Record) -> Result<Self, EntityError> {
        let task = Task::from_record(record)?;
        Ok(Self { task })
    }

    fn into_record(self) -> Record {
        self.task.into_record()
    }
}

impl DomainTask {
    /// Domain factory method to create a new DomainTask.
    pub fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &TeaqlRuntimeContext<RusqliteDialect, LoggingExecutor>) -> Result<Self, String> {
        if cmd.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_owned());
        }
        let mut task = Q::tasks().new_entity(ctx);
        task.update_id(next_id)
            .update_name(cmd.name.clone())
            .update_version(1_i64)
            .update_status_id(1_u64) // Default status: Planned (1)
            .update_platform_id(1_u64);
        Ok(Self { task })
    }

    /// Domain method to execute aggregate deletion validation.
    pub fn delete(&self, _cmd: &DeleteTaskCommand) -> Result<(), String> {
        // Validation logic can be added here if needed in the future
        Ok(())
    }

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
    ctx: TeaqlRuntimeContext<RusqliteDialect, LoggingExecutor>,
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

        // Register synchronous executors
        ctx.use_rusqlite_provider(inner_executor.clone());
        ctx.insert_resource(logging_executor.clone());

        // Build Schema & seed initial values if missing
        ensure_rusqlite_schema_for(&ctx)?;

        let rt_ctx = TeaqlRuntimeContext::<RusqliteDialect, LoggingExecutor>::new(ctx);

        Ok(Self {
            ctx: rt_ctx,
            inner_executor,
            last_log_index: Mutex::new(0),
        })
    }

    pub fn context(&self) -> &UserContext {
        self.ctx.context()
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
            format!(r#"{{"name": "{}"}}"#, term)
        } else {
            r#"{}"#.to_owned()
        };

        let select = Q::tasks()
            .comment(search_comment)
            .find_with_json(&search_json)
            .facet_by_status_as("status_stats", Q::task_status().comment("Count status").count_tasks());

        let query_trace = format!(
            "Q: Q::tasks().comment(\"{}\").find_with_json(\"{}\").facet_by_status_as(\"status_stats\", Q::task_status().comment(\"Count status\").count_tasks())",
            search_comment,
            search_json.replace('"', "\\\"")
        );

        let all_tasks = select.execute_for_list(&self.ctx).await?;

        let mut planned_count = 0;
        let mut process_count = 0;
        let mut done_count = 0;

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
                    1 => planned_count = count,
                    2 => process_count = count,
                    3 => done_count = count,
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
                1 => planned_tasks.push(task_model),
                2 => process_tasks.push(task_model),
                3 => done_tasks.push(task_model),
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
        let id_gen = RusqliteIdSpaceGenerator::from_executor(self.inner_executor.clone());
        let next_id = id_gen.next_id("Task")?;

        let cmd = CreateTaskCommand {
            name: name.to_owned(),
        };
        let domain_task = DomainTask::create(&cmd, next_id, &self.ctx)?;
        domain_task.task.save(&self.ctx).await?;

        Ok(next_id)
    }

    pub async fn delete_task(&self, id: u64) -> Result<bool, Box<dyn Error>> {
        let select = Q::tasks()
            .filter_by_id(id)
            .return_type::<DomainTask>();

        let found_tasks = select.execute_for_list(&self.ctx).await?;

        if let Some(domain_task) = found_tasks.into_iter().next() {
            let cmd = DeleteTaskCommand;
            domain_task.delete(&cmd)?;

            let repo = self.ctx.task_repository()?;
            repo.delete(&DeleteCommand::new("Task", id).expected_version(domain_task.task.version()))?;
            Ok(true)
        } else {
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
            .filter_by_id(id)
            .return_type::<DomainTask>();

        let query_trace = format!(
            "Q: Q::tasks().comment(\"Get task for DDD\").filter_by_id({}).return_type::<DomainTask>()",
            id
        );

        let found_tasks = select.execute_for_list(&self.ctx).await?;

        if let Some(mut domain_task) = found_tasks.into_iter().next() {
            let cmd_obj = TransitionCommand {
                target_status: target_status.to_owned(),
            };
            let transition_result = domain_task.transition_status(&cmd_obj);

            match transition_result {
                Ok(Some(new_status)) => {
                    domain_task.task.update_status_id(new_status);
                    domain_task.task.save(&self.ctx).await?;

                    let status_name = match new_status {
                        1 => "Planned",
                        2 => "Process",
                        3 => "Done",
                        _ => "Unknown",
                    };

                    Ok(MoveResult::Moved {
                        status_name: status_name.to_owned(),
                        query_trace,
                    })
                }
                Ok(None) => Ok(MoveResult::AlreadyDone { query_trace }),
                Err(err_msg) => Ok(MoveResult::Error { err_msg, query_trace }),
            }
        } else {
            Ok(MoveResult::NotFound { query_trace })
        }
    }

    pub fn check_sql_logs(&self) -> Vec<String> {
        let mut new_logs = Vec::new();
        let sql_logs = self.ctx.context().sql_logs();
        if let Ok(mut last_log) = self.last_log_index.lock() {
            if sql_logs.len() > *last_log {
                for entry in &sql_logs[*last_log..] {
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
                    new_logs.push(log_line);
                }
                *last_log = sql_logs.len();
            }
        }
        new_logs
    }
}
