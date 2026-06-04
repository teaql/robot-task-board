use std::error::Error;
use std::sync::Mutex;
use robot_kanban::{Q, Task, TaskExecutionLog};
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteIdSpaceGenerator,
    RusqliteMutationExecutor, RusqliteProviderExt,
};
use teaql_runtime::{
    UserContext, UnifiedLogBuffer, LogPayload,
};
use teaql_core::TeaqlEntity;

use crate::logging::{is_bootstrap_message, AppAuditSink};
use crate::models::{TaskModel, ReloadedData, MoveResult};

pub trait UserContextExt {
    fn next_id_for<T: TeaqlEntity>(&self) -> Result<u64, Box<dyn Error>>;
}

impl UserContextExt for UserContext {
    fn next_id_for<T: TeaqlEntity>(&self) -> Result<u64, Box<dyn Error>> {
        self.generate_id(&T::entity_descriptor().name)?
            .ok_or_else(|| "ID generator not configured for UserContext".into())
    }
}

pub trait TaskDomainBehavior {
    fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &UserContext) -> Result<Self, String> where Self: Sized;
    fn transition_status(&self, cmd: &TransitionCommand) -> Result<Option<u64>, String>;
    fn generate_execution_log(&self, action: &str, detail: &str, ctx: &UserContext) -> TaskExecutionLog;
}

pub struct TransitionCommand {
    pub target_status: String,
}

pub struct CreateTaskCommand {
    pub name: String,
}

impl TaskDomainBehavior for Task {
    fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &UserContext) -> Result<Self, String> {
        if cmd.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_owned());
        }
        
        let comment = format!("Create task '{}'", cmd.name);
        crate::logging::emit_ui_message(ctx, &format!("Execute TeaQL - Q::tasks().comment({:?}).new_entity(ctx)", comment));
        let mut task = Q::tasks().comment(&comment).new_entity(ctx);
        task.update_id(next_id)
            .update_name(cmd.name.clone())
            .update_version(0_i64)
            .update_status_to_planned()
            .update_platform_id(1_u64);
        Ok(task)
    }

    fn generate_execution_log(&self, action: &str, detail: &str, ctx: &UserContext) -> TaskExecutionLog {
        let comment = format!("Generate execution log for action '{}'", action);
        crate::logging::emit_ui_message(ctx, &format!("Execute TeaQL - Q::task_execution_logs().comment({:?}).new_entity(ctx)", comment));
        let mut log = Q::task_execution_logs().comment(&comment).new_entity(ctx);
        teaql_core::Entity::set_comment(&mut log, comment);
        log.update_action(action.to_owned())
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
            // Planned -> Ready -> Executing -> Verified
            match current_status {
                1001 => Some(1002_u64),
                1002 => Some(1003_u64),
                1003 => Some(1004_u64),
                _ => None,
            }
        } else {
            match target.as_str() {
                "planned" => Some(1001_u64),
                "ready" => Some(1002_u64),
                "executing" => Some(1003_u64),
                "verified" => Some(1004_u64),
                _ => return Err(format!("Invalid status '{}'. Use planned, ready, executing, verified, or empty to move next.", cmd.target_status)),
            }
        };

        Ok(next_status_id)
    }
}

pub struct TaskService {
    ctx: UserContext,
    #[allow(dead_code)]
    inner_executor: RusqliteMutationExecutor,
    last_log_index: Mutex<usize>,
    pub status_cache: std::collections::HashMap<u64, String>,
}

impl TaskService {
    /// Initializes SQLite database, creates/updates schemas, seeds initial data,
    /// constructs the thread-safe UserContext, and returns the fully configured TaskService.
    ///
    /// Bootstrap progress is observable through `EntityEventSink`:
    /// - `SchemaCreated` events are fired for each table created
    /// - `DataSeeded` events are fired for each entity type seeded
    pub async fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let conn = rusqlite::Connection::open(db_path)?;
        let inner_executor = RusqliteMutationExecutor::new(conn);


        let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();

        // Register custom TUI log buffer, audit event sink, and env-driven audit configuration.
        // The known_tables list below is project-specific. In a generated project,
        // the code generator produces this list automatically.
        let log_buffer = UnifiedLogBuffer::default();
        ctx.insert_resource(log_buffer);
        ctx.set_event_sink(AppAuditSink);
        let env_config = teaql_tool_core::audit_config_from_env(&[
            "task", "task_status", "task_execution_log",
        ]);
        ctx.insert_resource(env_config.config.clone());
        ctx.insert_resource(env_config);

        // Register synchronous executors
        ctx.use_rusqlite_provider(inner_executor.clone());
        ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(inner_executor.clone()));

        // Also register ServiceRuntimeExecutor for the generated repository lookups
        let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(inner_executor.clone());
        ctx.insert_resource(service_runtime_executor);

        // Build Schema & seed initial values if missing.
        // This now fires SchemaCreated and DataSeeded events through the EntityEventSink,
        // which are captured in the UnifiedLogBuffer for the startup screen to observe.
        ensure_rusqlite_schema_for(&ctx)?;

        let mut status_cache = std::collections::HashMap::new();
        crate::logging::emit_ui_message(&ctx, "Execute TeaQL - Q::task_status().comment(\"Load task statuses for cache\").execute_for_list(&ctx)");
        let statuses = robot_kanban::Q::task_status()
            .comment("Load task statuses for cache")
            .execute_for_list(&ctx)
            .await?.data;
        for status in statuses {
            status_cache.insert(status.id(), status.code().to_string());
        }

        Ok(Self {
            ctx,
            inner_executor,
            last_log_index: Mutex::new(0),
            status_cache,
        })
    }

    /// Returns bootstrap events captured in the UnifiedLogBuffer during initialization,
    /// with real per-step elapsed times. Each entry is (message, elapsed_ms).
    /// Then clears them from the buffer.
    pub fn drain_bootstrap_events(&self) -> Vec<(String, f64)> {
        let Some(buf) = self.ctx.get_resource::<UnifiedLogBuffer>() else {
            return Vec::new();
        };
        let Ok(mut entries) = buf.entries.lock() else {
            return Vec::new();
        };

        // Collect bootstrap entries with their timestamps
        let boot_entries: Vec<(String, std::time::SystemTime)> = entries
            .iter()
            .filter_map(|entry| {
                if let LogPayload::Info(info) = &entry.payload {
                    if is_bootstrap_message(&info.message) {
                        return Some((info.message.clone(), entry.timestamp));
                    }
                }
                None
            })
            .collect();

        // Compute per-step elapsed times from consecutive timestamps
        let mut results = Vec::new();
        for i in 0..boot_entries.len() {
            let elapsed_ms = if i == 0 {
                0.0
            } else {
                boot_entries[i].1
                    .duration_since(boot_entries[i - 1].1)
                    .unwrap_or_default()
                    .as_secs_f64() * 1000.0
            };
            results.push((boot_entries[i].0.clone(), elapsed_ms));
        }

        // Remove bootstrap entries from the buffer so they don't appear in the TUI logs
        entries.retain(|entry| {
            if let LogPayload::Info(info) = &entry.payload {
                !is_bootstrap_message(&info.message)
            } else {
                true
            }
        });
        results
    }

    pub fn context(&self) -> &UserContext {
        &self.ctx
    }

    pub fn emit_ui_message(&self, message: &str) {
        crate::logging::emit_ui_message(&self.ctx, message);
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

        let query = robot_kanban::Q::tasks()
            .comment(search_comment)
            .facet_by_status_as("status_stats", robot_kanban::Q::task_status().comment("Count status").count_tasks());

        // Unified logging: Log the query trace before running the query
        self.emit_ui_message(&format!("Starting query: {}", search_comment));
        
        let teaql_code = if let Some(kw) = search_term {
            format!("Q::tasks().comment({:?}).with_name_like(\"%{}%\").facet_by_status_as(\"status_stats\", Q::task_status().comment(\"Count status\").count_tasks())", search_comment, kw)
        } else {
            format!("Q::tasks().comment({:?}).facet_by_status_as(\"status_stats\", Q::task_status().comment(\"Count status\").count_tasks())", search_comment)
        };
        self.emit_ui_message(&format!("Execute TeaQL - {}", teaql_code));

        let list_result = query.execute_for_list(&self.ctx).await?;

        let mut planned_count = 0;
        let mut ready_count = 0;
        let mut executing_count = 0;
        let mut verified_count = 0;

        if let Some(facet_list) = list_result.facet("status_stats") {
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
                    1002 => ready_count = count,
                    1003 => executing_count = count,
                    1004 => verified_count = count,
                    _ => {}
                }
            }
        }

        let mut planned_tasks = Vec::new();
        let mut ready_tasks = Vec::new();
        let mut executing_tasks = Vec::new();
        let mut verified_tasks = Vec::new();

        let mut all_tasks = list_result.data;
        all_tasks.sort_by_key(|t| t.id());

        for task in all_tasks {
            if let Some(term) = search_term {
                if !task.name().to_lowercase().contains(&term.to_lowercase()) {
                    continue;
                }
            }
            let task_model = TaskModel {
                id: task.id(),
                name: task.name().to_string(),
            };
            match task.status_id() {
                1001 => {
                    planned_tasks.push(task_model);
                }
                1002 => {
                    ready_tasks.push(task_model);
                }
                1003 => {
                    executing_tasks.push(task_model);
                }
                1004 => {
                    verified_tasks.push(task_model);
                }
                _ => {}
            }
        }

        self.emit_ui_message(&format!("Finished query: {}", search_comment));

        Ok(ReloadedData {
            planned_tasks,
            ready_tasks,
            executing_tasks,
            verified_tasks,
            planned_count,
            ready_count,
            executing_count,
            verified_count,
        })
    }

    pub async fn add_task(&self, name: &str) -> Result<u64, Box<dyn Error>> {
        self.emit_ui_message(&format!("Starting business action: Create task '{}'", name));
        let next_id = self.ctx.next_id_for::<Task>()?;
        let cmd = CreateTaskCommand { name: name.to_owned() };
        let mut task = Task::create(&cmd, next_id, &self.ctx)?;

        let log = task.generate_execution_log("CREATED", &format!("Task '{}' created.", name), &self.ctx);

        let comment = format!("Create task '{}'", name);
        
        task.task_execution_log_list_mut().push(log);
        task.set_comment(&comment);
        
        task.save(&self.ctx).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.emit_ui_message(&format!("Finished business action: Create task '{}'", name));
        Ok(next_id)
    }

    pub async fn delete_task(&self, id: u64) -> Result<bool, Box<dyn Error>> {
        self.emit_ui_message(&format!("Starting business action: Delete task ID {}", id));
        self.emit_ui_message(&format!("Execute TeaQL - Q::tasks().with_id_is({}).comment(\"Load task {} for deletion\").execute_for_one(&self.ctx)", id, id));
        
        let task_opt = robot_kanban::Q::tasks()
            .with_id_is(id)
            .comment(&format!("Load task {} for deletion", id))
            .execute_for_one(&self.ctx).await?;

        if let Some(mut task) = task_opt {
            let task_name = task.name().to_string();
            let comment = format!("Delete task '{}'", task_name);
            
            task.set_comment(&comment);
            task.mark_as_delete();
            task.save(&self.ctx).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

            self.emit_ui_message(&format!("Finished business action: Delete task ID {}", id));
            Ok(true)
        } else {
            self.emit_ui_message(&format!("Finished business action: Error: Task with ID {} not found", id));
            Ok(false)
        }
    }

    pub async fn move_task(
        &self,
        id: u64,
        target_status: &str,
    ) -> Result<MoveResult, Box<dyn Error>> {
        let trimmed_status = target_status.trim();

        self.emit_ui_message(&format!("Execute TeaQL - Q::tasks().with_id_is({}).comment(\"Load task {} for status transition\").execute_for_one(&self.ctx)", id, id));
        let task_opt = robot_kanban::Q::tasks()
            .with_id_is(id)
            .comment(&format!("Load task {} for status transition", id))
            .execute_for_one(&self.ctx).await?;

        if let Some(mut task) = task_opt {
            let cmd_obj = TransitionCommand {
                target_status: trimmed_status.to_owned(),
            };
            let transition_result = task.transition_status(&cmd_obj);

            match transition_result {
                Ok(Some(new_status)) => {
                    let old_status_id = task.status_id();
                    match new_status {
                        1001 => { task.update_status_to_planned(); }
                        1002 => { task.update_status_to_ready(); }
                        1003 => { task.update_status_to_executing(); }
                        1004 => { task.update_status_to_verified(); }
                        _ => {}
                    }
                    let status_name = self.status_cache.get(&new_status).cloned().unwrap_or_else(|| "Unknown".to_owned());
                    let old_status_name = self.status_cache.get(&old_status_id).cloned().unwrap_or_else(|| "Unknown".to_owned());
                    
                    let task_name = task.name().to_string();
                    
                    self.emit_ui_message(&format!("Starting business action: Move '{}' {} => {}", task_name, old_status_name, status_name));

                    let detail = format!("Status changed from {} to {}.", old_status_name, status_name);
                    
                    let log = task.generate_execution_log("STATUS_CHANGED", &detail, &self.ctx);

                    let comment = format!("DOMAIN: Move '{}' {} => {}", task_name, old_status_name, status_name);
                    
                    // Attach the log to the task's execution log list to establish the graph relation
                    task.task_execution_log_list_mut().push(log);

                    // Set the comment on the aggregate root
                    task.set_comment(&comment);

                    // Save the aggregate root, which implicitly saves the child execution logs
                    task.save(&self.ctx).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

                    self.emit_ui_message(&format!("Finished business action: Moved task {} to '{}' (DDD transition)", id, status_name));

                    Ok(MoveResult::Moved {
                        status_name,
                    })
                }
                Ok(None) => {
                    self.emit_ui_message(&format!("Action failed: Task {} already in final status", id));
                    Ok(MoveResult::AlreadyFinal)
                }
                Err(e) => {
                    let err_msg = format!("Transition error: {}", e);
                    self.emit_ui_message(&format!("Action failed: {}", err_msg));
                    Ok(MoveResult::Error { err_msg })
                }
            }
        } else {
            self.emit_ui_message(&format!("Action failed: Task {} not found", id));
            Ok(MoveResult::NotFound)
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
                                    let elapsed_us = (sql_entry.elapsed.as_secs_f64() * 1_000_000.0).round() as u64;
                                    let line1 = format!("[{}]-[{}]-[{:>5}µs]-[DEBUG]-SqlLogEntry{} - [{}]", ts, uid, elapsed_us, trace, sql_entry.result_summary);
                                    let line2 = format!("          {}", sql_entry.pretty_sql.replace("\n", " "));
                                    new_logs.push(line1);
                                    new_logs.push(line2);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[tokio::test]
    async fn test_core_flow() -> Result<(), Box<dyn Error>> {
        let db_path = "test_core_flow.db";
        let _ = std::fs::remove_file(db_path);

        let service = TaskService::new(db_path).await?;
        
        let task_id = service.add_task("Test Task").await?;

        // Restart service to simulate user's flow
        let service2 = TaskService::new(db_path).await?;

        service2.move_task(task_id, "Ready").await?;
        
        Ok(())
    }
}
