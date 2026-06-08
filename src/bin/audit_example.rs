use std::error::Error;
use chrono::Local;

use robot_kanban::{AuditedSave, Q, Task};
use teaql_core::{Entity, Value, TeaqlEntity};
use teaql_runtime::{
    SafeAuditEvent, RawAuditEventKind, SafeAuditEventSink, UserContext, RuntimeError,
};
use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteIdSpaceGenerator, RusqliteMutationExecutor,
    RusqliteProviderExt,
};

/// Format a TeaQL Value into a clear English string representation
fn format_teaql_value(val: &Option<teaql_core::Value>) -> String {
    match val {
        Some(teaql_core::Value::Null) | None => "NULL".to_owned(),
        Some(teaql_core::Value::Text(s)) => format!("'{}'", s),
        Some(teaql_core::Value::I64(n)) => n.to_string(),
        Some(teaql_core::Value::U64(n)) => n.to_string(),
        Some(teaql_core::Value::Bool(b)) => b.to_string(),
        Some(teaql_core::Value::Timestamp(t)) => t.format("%Y-%m-%d %H:%M:%S").to_string(),
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

impl SafeAuditEventSink for AuditLogSink {
    fn on_safe_event(&self, ctx: &UserContext, event: &SafeAuditEvent) -> Result<(), RuntimeError> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let user = ctx.user_identifier().unwrap_or("anonymous").to_string();

        let action_name = match event.kind {
            RawAuditEventKind::Created => "CREATED",
            RawAuditEventKind::Updated => "UPDATED",
            RawAuditEventKind::Deleted => "DELETED",
            RawAuditEventKind::Recovered => "RECOVERED",
            RawAuditEventKind::SchemaCreated | RawAuditEventKind::SchemaVerified | RawAuditEventKind::FieldAdded | RawAuditEventKind::DataSeeded => return Ok(()),
        };

        // Extract ID value from event record to represent entity as Type:ID
        // In SafeAuditEvent, we only have changed fields, so id might not be there.
        // We'll just use entity name for now.
        let entity_identity = format!("{}", event.entity);

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

        for field in &event.fields {
            let new_str = field.value.clone().unwrap_or_else(|| "NULL".to_owned());
            
            let mut meta_tags = vec![];
            if field.masked { meta_tags.push("MASKED"); }
            if field.truncated { meta_tags.push("TRUNCATED"); }
            let meta_str = if meta_tags.is_empty() { String::new() } else { format!(" [{}]", meta_tags.join(",")) };

            let detail = format!(
                "[{}] - [{}] - [AUDIT]   -> Field [{}]: {}{}",
                timestamp, user, field.name, new_str, meta_str
            );
            lines.push(detail);
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

    ctx.set_custom_event_sink(AuditLogSink);

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

    let mut task = Q::tasks().purpose("Create audit task").new_entity(&ctx);
    task.update_id(next_id)
        .update_name("Analyze Network Traffic Logs".to_owned())
        .update_version(1_i64)
        .update_status_to_planned()
        .update_platform_id(1_u64);

    // Saving a clone will trigger the CREATED event automatically, allowing us to reuse the local variable
    {
        let cloned = task.clone();
        cloned.audit_as("Create task 'Analyze Network Traffic Logs'").save(&ctx).await?;
    }

    // 7. Action 2: Move the Task (Update status)
    println!("--- Action 2: Moving the Task to 'Process' Status ---");
    task.update_status_to_ready();
    {
        let cloned = task.clone();
        cloned.audit_as("Move task 'Analyze Network Traffic Logs' to Process").save(&ctx).await?;
    }

    // 8. Action 3: Deleting the Task
    println!("--- Action 3: Deleting the Task ---");
    
    // Standard DDD/TeaQL practice: Load the latest entity state to get the correct database version
    let found_tasks = Q::tasks()
        .with_id_is(next_id)
        .purpose("Load task by id")
        .execute_for_list(&ctx)
        .await?;

    if let Some(mut latest_task) = found_tasks.into_iter().next() {
        // Deleting via entity API emits the DELETED event
        latest_task.mark_as_delete();
        latest_task.audit_as("Soft delete the test task").save(&ctx).await?;
    } else {
        println!("[ERROR] Task with ID {} could not be found for deletion.", next_id);
    }

    println!("============================================================");
    println!("Audit Log Demonstration Completed. Logs saved to 'audit_example.log'.");
    println!("============================================================");

    Ok(())
}
