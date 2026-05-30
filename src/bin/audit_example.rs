use std::error::Error;
use chrono::Local;

use robot_kanban::{Q, TeaqlRuntime, Task};
use teaql_core::{Value, TeaqlEntity};
use teaql_runtime::{
    EntityEvent, EntityEventKind, EntityEventSink, UserContext, RuntimeError,
};
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteIdSpaceGenerator, RusqliteMutationExecutor,
    RusqliteProviderExt,
};

/// Format a TeaQL Value into a clear English string representation
fn format_teaql_value(val: &Option<Value>) -> String {
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

/// Format an entity's primary key ID for display
fn format_entity_id(val: &Value) -> String {
    match val {
        Value::Text(s) => s.clone(),
        Value::I64(n) => n.to_string(),
        Value::U64(n) => n.to_string(),
        Value::Null => "NULL".to_owned(),
        other => format!("{:?}", other),
    }
}


/// Custom EntityEventSink that captures object modifications in real-time.
/// It prints logs in English with the current local timestamp and the custom user identifier.
pub struct AuditLogSink;

impl EntityEventSink for AuditLogSink {
    fn on_event(&self, ctx: &UserContext, event: &EntityEvent) -> Result<(), RuntimeError> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let user = ctx.user_identifier().unwrap_or("anonymous").to_string();

        let action_name = match event.kind {
            EntityEventKind::Created => "CREATED",
            EntityEventKind::Updated => "UPDATED",
            EntityEventKind::Deleted => "DELETED",
            EntityEventKind::Recovered => "RECOVERED",
            EntityEventKind::SchemaCreated | EntityEventKind::DataSeeded => return Ok(()),
        };

        // Extract ID value from event record to represent entity as Type:ID
        let entity_id_str = match event.values.get("id") {
            Some(id_val) => format_entity_id(id_val),
            None => "UNKNOWN".to_owned(),
        };
        let entity_identity = format!("{}:{}", event.entity, entity_id_str);

        let comment_part = if event.trace_chain.is_empty() {
            "".to_owned()
        } else {
            let trace = event.trace_chain.iter().map(|n| n.comment.clone()).collect::<Vec<_>>().join(" -> ");
            format!(" [{}]", trace)
        };

        let header = format!(
            "[{}] - [{}] - [AUDIT] Entity [{}] was {}.{}",
            timestamp, user, entity_identity, action_name, comment_part
        );

        let mut lines = vec![header];
        let divider = format!("[{}] - [{}] - [AUDIT] ------------------------------------------------------------", timestamp, user);

        for change in &event.changes {
            let old_str = format_teaql_value(&change.old_value);
            let new_str = format_teaql_value(&change.new_value);
            
            if old_str != new_str {
                let detail = format!(
                    "[{}] - [{}] - [AUDIT]   -> Field [{}]: {} ➔ {}",
                    timestamp, user, change.field, old_str, new_str
                );
                lines.push(detail);
            }
        }

        // Output to console and log file
        for line in &lines {
            // Print to console
            println!("{}", line);

            // Write to local audit file
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("audit_example.log")
            {
                use std::io::Write;
                let _ = writeln!(file, "{}", line);
            }
        }

        // Print a dividing line after each event batch
        println!("{}", divider);
        println!();
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("audit_example.log")
        {
            use std::io::Write;
            let _ = writeln!(file, "{}", divider);
            let _ = writeln!(file, ""); // blank line separator
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("============================================================");
    println!("TeaQL Custom UserContext & Audit Tracker Demo");
    println!("============================================================\n");

    // 1. Initialize TeaQL Domain Context
    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();

    // 2. Customize the User Identifier in the UserContext
    // We explicitly identify the operator for the current context.
    let custom_user = "operator-philip@node-01.router-env";
    ctx = ctx.with_user_identifier(custom_user);
    println!("[SYSTEM] Context initialized with Custom User Identifier: '{}'\n", custom_user);

    // 3. Register our custom AuditLogSink
    ctx.set_event_sink(AuditLogSink);

    // 4. Configure SQLite Database Connection & Executor
    let db_path = "robot_kanban.db";
    let conn = rusqlite::Connection::open(db_path)?;
    let executor = RusqliteMutationExecutor::new(conn);
    ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(executor.clone()));
    ctx.use_rusqlite_provider(executor.clone());
    
    let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(executor.clone());
    ctx.insert_resource(service_runtime_executor);

    // 5. Ensure Schema is bootstrapped
    ensure_rusqlite_schema_for(&ctx)?;

    // 6. Action 1: Create a new Task
    println!("--- Action 1: Creating a Task ---");
    let next_id = ctx.generate_id(&Task::entity_descriptor().name)?.expect("ID generator configured");

    let mut task = Q::tasks().new_entity(&ctx);
    task.update_id(next_id)
        .update_name("Analyze Network Traffic Logs".to_owned())
        .update_version(1_i64)
        .update_status_to_planned()
        .update_platform_id(1_u64);

    // Saving a clone will trigger the CREATED event automatically, allowing us to reuse the local variable
    {
        let repo = ctx.task_repository()?;
        teaql_runtime::ResolvedRepository::save_entity_with_comment(&repo, task.clone(), teaql_runtime::EntityStatus::New, "Create task 'Analyze Network Traffic Logs'".to_owned())?;
    }

    // 7. Action 2: Move the Task (Update status)
    println!("--- Action 2: Moving the Task to 'Process' Status ---");
    task.update_status_to_process();
    {
        let repo = ctx.task_repository()?;
        teaql_runtime::ResolvedRepository::save_entity_with_comment(&repo, task.clone(), teaql_runtime::EntityStatus::Updated, "Move task 'Analyze Network Traffic Logs' to Process".to_owned())?;
    }

    // 8. Action 3: Deleting the Task
    println!("--- Action 3: Deleting the Task ---");
    
    // Standard DDD/TeaQL practice: Load the latest entity state to get the correct database version
    let found_tasks = Q::tasks()
        .with_id_is(next_id)
        .execute_for_list(&ctx)
        .await?;

    if let Some(latest_task) = found_tasks.into_iter().next() {
        let repo = ctx.task_repository()?;
        // Deleting via repository emits the DELETED event
        repo.save_entity_with_comment(latest_task.clone(), teaql_runtime::EntityStatus::UpdatedDeleted, "Soft delete the test task".to_string())?;
    } else {
        println!("[ERROR] Task with ID {} could not be found for deletion.", next_id);
    }

    println!("============================================================");
    println!("Audit Log Demonstration Completed. Logs saved to 'audit_example.log'.");
    println!("============================================================");

    Ok(())
}
