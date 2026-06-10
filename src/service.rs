use robot_kanban::{AuditedSave, Platform, Task, TaskExecutionLog, Q};
use std::error::Error;
use std::sync::Mutex;
use teaql_core::{Entity, TeaqlEntity};
use teaql_provider_sqlite::{
    ensure_sqlite_schema_for, SqliteMutationExecutor, SqliteProviderExt,
};
use rusqlite::Connection;
use std::sync::Arc;
use teaql_runtime::{LogPayload, UnifiedLogBuffer, UserContext};

use crate::logging::{is_bootstrap_message, AppAuditSink};
use crate::models::{MoveResult, ReloadedData, TaskModel};

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
    fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &UserContext) -> Result<Self, String>
    where
        Self: Sized;
    fn transition_status(&self, cmd: &TransitionCommand) -> Result<Option<u64>, String>;
    fn generate_execution_log(
        &self,
        action: &str,
        detail: &str,
        ctx: &UserContext,
    ) -> TaskExecutionLog;
}

pub struct TransitionCommand {
    pub target_status: String,
}

pub struct CreateTaskCommand {
    pub name: String,
    pub tenant_id: u64,
}

impl TaskDomainBehavior for Task {
    fn create(cmd: &CreateTaskCommand, next_id: u64, ctx: &UserContext) -> Result<Self, String> {
        if cmd.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_owned());
        }

        let comment = format!("Create task '{}'", cmd.name);
        let mut task = Q::tasks()
            .comment(&comment)
            .purpose("Create new task")
            .new_entity(ctx);
        task.update_id(next_id)
            .update_name(cmd.name.clone())
            .update_version(0_i64)
            .update_status_to_planned()
            .update_tenant_id(cmd.tenant_id);
        Ok(task)
    }

    fn generate_execution_log(
        &self,
        action: &str,
        detail: &str,
        ctx: &UserContext,
    ) -> TaskExecutionLog {
        let comment = format!("Generate execution log for action '{}'", action);
        let mut log = Q::task_execution_logs()
            .comment(&comment)
            .purpose("Create execution log")
            .new_entity(ctx);
        teaql_core::Entity::set_comment(&mut log, comment);
        log.update_action(action.to_owned())
            .update_detail(detail.to_owned())
            .update_version(0_i64)
            .update_task_id(self.id());
        log
    }

    fn transition_status(&self, cmd: &TransitionCommand) -> Result<Option<u64>, String> {
        let current_status = self.status_id();
        let target = cmd.target_status.trim().to_lowercase();

        let next_status_id = if target.is_empty() {
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
    inner_executor: SqliteMutationExecutor,
    last_log_index: Mutex<usize>,
    pub status_cache: std::collections::HashMap<u64, String>,
}

impl TaskService {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open("teaql_data.db").map_err(|e| e.to_string())?;
        let inner_executor = SqliteMutationExecutor::new(Arc::new(Mutex::new(conn)));

        let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();

        let log_buffer = UnifiedLogBuffer::default();
        ctx.insert_resource(log_buffer);
        ctx.set_custom_event_sink(AppAuditSink);

        if let Ok(val) = std::env::var("TEAQL_SQL_LOG") {
            if val == "off" || val == "false" || val == "0" {
                ctx.set_sql_log_options(teaql_runtime::SqlLogOptions::disabled());
            }
        }

        ctx.use_sqlite_provider(inner_executor.clone());
        ctx.set_internal_id_generator(teaql_provider_sqlite::SqliteIdSpaceGenerator::from_executor(inner_executor.clone()));

        let service_runtime_executor =
            robot_kanban::ServiceRuntimeExecutor::new(inner_executor.clone());
        ctx.insert_resource(service_runtime_executor);

        ensure_sqlite_schema_for(&ctx).map_err(|e| e.to_string())?;

        let mut status_cache = std::collections::HashMap::new();
        let statuses = robot_kanban::Q::task_status()
            .comment("Load task statuses for cache")
            .purpose("Load task statuses for cache")
            .execute_for_list(&ctx)
            .await?
            .data;
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

    pub fn drain_bootstrap_events(&self) -> Vec<(String, f64)> {
        Vec::new() // No longer heavily used in Web API
    }

    pub fn context(&self) -> &UserContext {
        &self.ctx
    }

    pub fn log_info(&self, message: &str) {
        let user = crate::logging::short_user(&self.ctx);
        let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f").to_string();
        let log_line = format!("[{}]-[{}]-[INFO]-{}", timestamp, user, message);

        if let Some(buf) = self.ctx.get_resource::<UnifiedLogBuffer>() {
            if let Ok(mut entries) = buf.entries.lock() {
                entries.push(teaql_runtime::UnifiedLogEntry {
                    timestamp: std::time::SystemTime::now(),
                    user_identifier: Some(user),
                    trace_chain: Vec::new(),
                    payload: LogPayload::Info(teaql_runtime::InfoLogEntry { message: log_line }),
                });
            }
        }
    }

    pub async fn get_or_create_global_platform(&self) -> Result<u64, Box<dyn Error>> {
        let existing = Q::platforms()
            .with_name_is("Robot System".to_string())
            .comment("Find global platform")
            .purpose("Initialize")
            .execute_for_one(&self.ctx)
            .await?;
        if let Some(p) = existing {
            return Ok(p.id());
        }
        let next_id = self.ctx.next_id_for::<Platform>()?;
        let mut p = Q::platforms()
            .comment("Create global platform")
            .purpose("Initialize")
            .new_entity(&self.ctx);
        p.update_id(next_id).update_name("Robot System".to_string());
        teaql_core::Entity::set_comment(&mut p, "Init platform".to_string());
        p.save(&self.ctx).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(next_id)
    }

    pub async fn get_or_create_tenant(&self, session_id: &str) -> Result<u64, Box<dyn Error>> {
        let existing = Q::tenants()
            .with_name_is(format!("Session {}", session_id))
            .comment("Find platform by session")
            .purpose("Get current tenant")
            .execute_for_one(&self.ctx)
            .await?;

        if let Some(p) = existing {
            return Ok(p.id());
        }

        let platform_id = self.get_or_create_global_platform().await?;

        let next_id = self.ctx.next_id_for::<robot_kanban::Tenant>()?;
        let mut p = Q::tenants()
            .comment("Create platform")
            .purpose("Init tenant")
            .new_entity(&self.ctx);
        p.update_id(next_id)
            .update_name(format!("Session {}", session_id))
            .update_platform_id(platform_id);

        teaql_core::Entity::set_comment(&mut p, "New user session".to_string());
        p.save(&self.ctx)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(next_id)
    }

    pub async fn reload_data(
        &self,
        session_id: &str,
        search_term: &Option<String>,
    ) -> Result<ReloadedData, Box<dyn Error>> {
        let search_comment = if search_term.is_some() {
            "Get filtered tasks by keyword"
        } else {
            "Get active tasks"
        };

        let query = robot_kanban::Q::tasks()
            .with_tenant_matching(Q::tenants().with_name_is(format!("Session {}", session_id)))
            .comment(search_comment)
            .facet_by_status_as(
                "status_stats",
                robot_kanban::Q::task_status()
                    .comment("Count status")
                    .count_tasks(),
            );

        self.log_info(&format!("Starting query: {}", search_comment));
        let list_result = query
            .purpose("List tasks")
            .execute_for_list(&self.ctx)
            .await?;

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

    pub async fn add_task(&self, session_id: &str, name: &str) -> Result<u64, Box<dyn Error>> {
        let tenant_id = self.get_or_create_tenant(session_id).await?;

        self.log_info(&format!("Starting business action: Create task '{}'", name));
        let next_id = self.ctx.next_id_for::<Task>()?;
        let cmd = CreateTaskCommand {
            name: name.to_owned(),
            tenant_id,
        };
        let mut task = Task::create(&cmd, next_id, &self.ctx)?;

        let log =
            task.generate_execution_log("CREATED", &format!("Task '{}' created.", name), &self.ctx);
        let comment = format!("Create task '{}'", name);

        task.task_execution_log_list_mut().push(log);
        teaql_core::Entity::set_comment(&mut task, comment.to_string());
        task.save(&self.ctx)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(next_id)
    }

    pub async fn delete_task(&self, session_id: &str, id: u64) -> Result<bool, Box<dyn Error>> {
        let task_opt = robot_kanban::Q::tasks()
            .with_id_is(id)
            .with_tenant_matching(Q::tenants().with_name_is(format!("Session {}", session_id)))
            .comment(&format!("Load task {} for deletion", id))
            .purpose(&format!("Load task {} for deletion", id))
            .execute_for_one(&self.ctx)
            .await?;

        if let Some(mut task) = task_opt {
            let task_name = task.name().to_string();
            let comment = format!("Delete task '{}'", task_name);
            task.mark_as_delete();
            teaql_core::Entity::set_comment(&mut task, comment.to_string());
            task.save(&self.ctx)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn move_task(
        &self,
        session_id: &str,
        id: u64,
        target_status: &str,
    ) -> Result<MoveResult, Box<dyn Error>> {
        let trimmed_status = target_status.trim();

        let task_opt = robot_kanban::Q::tasks()
            .with_id_is(id)
            .with_tenant_matching(Q::tenants().with_name_is(format!("Session {}", session_id)))
            .comment(&format!("Load task {} for status transition", id))
            .purpose(&format!("Load task {} for status transition", id))
            .execute_for_one(&self.ctx)
            .await?;

        if let Some(mut task) = task_opt {
            let cmd_obj = TransitionCommand {
                target_status: trimmed_status.to_owned(),
            };
            let transition_result = task.transition_status(&cmd_obj);

            match transition_result {
                Ok(Some(new_status)) => {
                    let old_status_id = task.status_id();
                    match new_status {
                        1001 => {
                            task.update_status_to_planned();
                        }
                        1002 => {
                            task.update_status_to_ready();
                        }
                        1003 => {
                            task.update_status_to_executing();
                        }
                        1004 => {
                            task.update_status_to_verified();
                        }
                        _ => {}
                    }
                    let status_name = self
                        .status_cache
                        .get(&new_status)
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_owned());
                    let old_status_name = self
                        .status_cache
                        .get(&old_status_id)
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_owned());

                    let task_name = task.name().to_string();
                    let detail = format!(
                        "Status changed from {} to {}.",
                        old_status_name, status_name
                    );
                    let log = task.generate_execution_log("STATUS_CHANGED", &detail, &self.ctx);
                    let comment = format!(
                        "DOMAIN: Move '{}' {} => {}",
                        task_name, old_status_name, status_name
                    );

                    task.task_execution_log_list_mut().push(log);
                    teaql_core::Entity::set_comment(&mut task, comment.to_string());
                    task.save(&self.ctx)
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

                    Ok(MoveResult::Moved { status_name })
                }
                Ok(None) => Ok(MoveResult::AlreadyFinal),
                Err(e) => Ok(MoveResult::Error {
                    err_msg: format!("Transition error: {}", e),
                }),
            }
        } else {
            Ok(MoveResult::NotFound)
        }
    }

    pub fn check_sql_logs(&self) -> Vec<String> {
        self.check_sql_logs_metadata()
            .into_iter()
            .map(|(text, _)| text)
            .collect()
    }

    pub fn check_sql_logs_metadata(&self) -> Vec<(String, Option<f64>)> {
        let mut new_logs = Vec::new();
        if let Some(buf) = self.ctx.get_resource::<UnifiedLogBuffer>() {
            if let Ok(mut last_log) = self.last_log_index.lock() {
                if let Ok(entries) = buf.entries.lock() {
                    if entries.len() > *last_log {
                        for entry in &entries[*last_log..] {
                            match &entry.payload {
                                LogPayload::Sql(sql_entry) => {
                                    let local_time: chrono::DateTime<chrono::Local> =
                                        entry.timestamp.into();
                                    let ts = local_time.format("%H:%M:%S%.3f");
                                    let uid = entry
                                        .user_identifier
                                        .as_deref()
                                        .unwrap_or("")
                                        .split('@')
                                        .next()
                                        .unwrap_or("");
                                    let trace = if entry.trace_chain.is_empty() {
                                        "".to_owned()
                                    } else {
                                        format!(
                                            " - [{}]",
                                            entry
                                                .trace_chain
                                                .iter()
                                                .map(|n| n.comment.clone())
                                                .collect::<Vec<_>>()
                                                .join(" -> ")
                                        )
                                    };
                                    let elapsed_us = (sql_entry.elapsed.as_secs_f64() * 1_000_000.0)
                                        .round()
                                        as u64;
                                    let line1 = format!(
                                        "[{}]-[{}]-[{:>5}µs]-[DEBUG]-SqlLogEntry{} - [{}]",
                                        ts, uid, elapsed_us, trace, sql_entry.result_summary
                                    );
                                    let line2 = format!(
                                        "          {}",
                                        sql_entry.pretty_sql.replace("\n", " ")
                                    );
                                    let lat_ms = sql_entry.elapsed.as_secs_f64() * 1000.0;
                                    new_logs.push((line1, Some(lat_ms)));
                                    new_logs.push((line2, None));
                                }
                                LogPayload::Info(info) => {
                                    new_logs.push((info.message.clone(), None));
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

    pub async fn get_admin_tenants(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let tenants = Q::tenants()
            .select_task_list_with(
                Q::tasks()
                    .order_by_id_desc()
                    .limit(3)
            )
            .count_tasks_as("task_count")
            .comment("Get all tenants with stats")
            .purpose("Admin View")
            .execute_for_records(&self.ctx)
            .await?;

        let mut result = Vec::new();
        for t in tenants {
            let task_count = match t.get("task_count") {
                Some(teaql_core::Value::I64(v)) => *v,
                Some(teaql_core::Value::U64(v)) => *v as i64,
                _ => 0,
            };
            
            let mut recent_tasks = Vec::new();
            if let Some(teaql_core::Value::List(tasks)) = t.get("task_list") {
                for task_val in tasks {
                    if let teaql_core::Value::Object(task) = task_val {
                        let id = match task.get("id") {
                            Some(teaql_core::Value::U64(v)) => Some(*v),
                            Some(teaql_core::Value::I64(v)) => Some(*v as u64),
                            _ => None,
                        };
                        let name = match task.get("name") {
                            Some(teaql_core::Value::Text(v)) => Some(v.clone()),
                            _ => None,
                        };
                        let status = match task.get("status") {
                            Some(teaql_core::Value::Text(v)) => Some(v.clone()),
                            _ => None,
                        };

                        recent_tasks.push(serde_json::json!({
                            "id": id,
                            "name": name,
                            "status": status,
                        }));
                    }
                }
            }

            let tenant_id = match t.get("id") {
                Some(teaql_core::Value::U64(v)) => Some(*v),
                Some(teaql_core::Value::I64(v)) => Some(*v as u64),
                _ => None,
            };
            let tenant_name = match t.get("name") {
                Some(teaql_core::Value::Text(v)) => Some(v.clone()),
                _ => None,
            };

            result.push(serde_json::json!({
                "id": tenant_id,
                "name": tenant_name,
                "task_count": task_count,
                "recent_tasks": recent_tasks,
            }));
        }

        Ok(result)
    }
}

