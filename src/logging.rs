use teaql_provider_rusqlite::{RusqliteMutationExecutor, MutationExecutorError};
use teaql_runtime::{
    UserContext, QueryExecutor, GraphTransactionBoundary, EntityEvent,
    EntityEventKind, EntityEventSink, RuntimeError, UnifiedLogEntry, UnifiedLogBuffer, LogPayload,
};
use teaql_core::Value;
use teaql_sql::CompiledQuery;

/// Extract just the OS username from the full user identifier (e.g. "philip@pid-123.tid-1" → "philip")
pub fn short_user(ctx: &UserContext) -> String {
    let full = ctx.user_identifier().unwrap_or("unknown");
    full.split('@').next().unwrap_or(full).to_owned()
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

/// Resolve a status ID value to a human-readable name.
fn resolve_status_name(raw: &str) -> String {
    match raw {
        "1001" => "Planned".to_owned(),
        "1002" => "Ready".to_owned(),
        "1003" => "Executing".to_owned(),
        "1004" => "Verified".to_owned(),
        other => other.to_owned(),
    }
}

/// Format a field change value, resolving known ID fields to names.
fn format_field_val(field: &str, val: &Option<Value>) -> String {
    let raw = format_val_helper(val);
    if field == "status_id" || field == "status" {
        resolve_status_name(&raw)
    } else {
        raw
    }
}

/// Map internal field names to user-friendly display names.
fn display_field_name(field: &str) -> &str {
    match field {
        "status_id" => "status",
        other => other,
    }
}

/// Check if a log message is a bootstrap event (schema or seed).
pub fn is_bootstrap_message(msg: &str) -> bool {
    msg.starts_with("Create ")
        || msg.starts_with("Verified ")
        || msg.starts_with("Seed ")
        || msg.starts_with("  + field ")
        || msg.ends_with(" entities discovered")
}

pub fn log_info(ctx: &UserContext, message: &str) {
    let timestamp = chrono::Local::now().format("%H:%M:%S%.3f").to_string();
    let user = short_user(ctx);
    let log_line = format!("[{}]-[{}]-[INFO]-{}", timestamp, user, message);
    
    // Write to TUI buffer
    if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
        if let Ok(mut entries) = buf.entries.lock() {
            entries.push(teaql_runtime::UnifiedLogEntry {
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

pub struct AppAuditSink;

impl EntityEventSink for AppAuditSink {
    fn on_event(&self, ctx: &UserContext, event: &EntityEvent) -> Result<(), RuntimeError> {
        // Bootstrap events are handled separately —
        // they are written to the UnifiedLogBuffer for the startup screen to observe.
        match event.kind {
            EntityEventKind::SchemaCreated
            | EntityEventKind::SchemaVerified
            | EntityEventKind::FieldAdded
            | EntityEventKind::DataSeeded => {
                let table_name = event.values.get("table_name")
                    .map(|v| match v { Value::Text(s) => s.as_str(), _ => "unknown" })
                    .unwrap_or("unknown");
                let field_count = event.values.get("field_count")
                    .map(|v| match v { Value::I64(n) => *n as usize, _ => 0 })
                    .unwrap_or(0);

                let message = match event.kind {
                    EntityEventKind::SchemaCreated => {
                        format!("Create {} ({} fields)", table_name, field_count)
                    }
                    EntityEventKind::SchemaVerified => {
                        format!("Verified {} ({} fields)", table_name, field_count)
                    }
                    EntityEventKind::FieldAdded => {
                        let field_name = event.values.get("field_name")
                            .map(|v| match v { Value::Text(s) => s.as_str(), _ => "?" })
                            .unwrap_or("?");
                        format!("  + field {} on {}", field_name, table_name)
                    }
                    EntityEventKind::DataSeeded => {
                        let inserted = event.values.get("inserted")
                            .map(|v| match v { Value::I64(n) => *n as usize, _ => 0 })
                            .unwrap_or(0);
                        let updated = event.values.get("updated")
                            .map(|v| match v { Value::I64(n) => *n as usize, _ => 0 })
                            .unwrap_or(0);
                        if updated > 0 && inserted > 0 {
                            format!("Seed {} ({} inserted, {} updated)", table_name, inserted, updated)
                        } else if updated > 0 {
                            let word = if updated == 1 { "record" } else { "records" };
                            format!("Seed {} ({} {})", table_name, updated, word)
                        } else {
                            let word = if inserted == 1 { "record" } else { "records" };
                            format!("Seed {} ({} {} inserted)", table_name, inserted, word)
                        }
                    }
                    _ => unreachable!(),
                };

                if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
                    if let Ok(mut entries) = buf.entries.lock() {
                        entries.push(UnifiedLogEntry {
                            timestamp: std::time::SystemTime::now(),
                            user_identifier: None,
                            trace_chain: Vec::new(),
                            payload: LogPayload::Info(teaql_runtime::InfoLogEntry { message }),
                        });
                    }
                }
                return Ok(());
            }
            _ => {}
        }

        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f").to_string();
        let user = short_user(ctx);

        let action_name = match event.kind {
            EntityEventKind::Created => "CREATED",
            EntityEventKind::Updated => "UPDATED",
            EntityEventKind::Deleted => "DELETED",
            EntityEventKind::Recovered => "RECOVERED",
            _ => unreachable!(),
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
            let old_str = format_field_val(&change.field, &change.old_value);
            let new_str = format_field_val(&change.field, &change.new_value);
            if old_str != new_str {
                field_changes.push(format!("{}: [{} ➔ {}]", display_field_name(&change.field), old_str, new_str));
            }
        }
        let fields_part = if field_changes.is_empty() {
            String::new()
        } else {
            format!(" {{{}}}", field_changes.join(",  "))
        };

        let audit_line = format!(
            "[{}]-[{}]-[AUDIT]-Entity [{}] was {}.{}{}",
            timestamp, user, entity_identity, action_name, comment_part, fields_part
        );

        // Write audit log to app.log
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("app.log")
        {
            use std::io::Write;
            let _ = writeln!(file, "{}", audit_line);
        }

        // Write audit log to TUI buffer
        if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
            if let Ok(mut entries) = buf.entries.lock() {
                entries.push(UnifiedLogEntry {
                    timestamp: std::time::SystemTime::now(),
                    user_identifier: Some(user.clone()),
                    trace_chain: event.trace_chain.clone(),
                    payload: LogPayload::Info(teaql_runtime::InfoLogEntry {
                        message: audit_line,
                    }),
                });
            }
        }

        // If it's a business log, ALSO emit a Business Log line
        let is_business_log = event.entity == "TaskExecutionLog" && action_name == "CREATED";
        if is_business_log {
            let mut detail = String::new();
            for change in &event.changes {
                if change.field == "detail" {
                    detail = format_val_helper(&change.new_value);
                    // Remove quotes if any
                    if detail.starts_with('\'') && detail.ends_with('\'') {
                        detail = detail[1..detail.len()-1].to_owned();
                    }
                }
            }
            let business_line = format!(
                "[{}]-[{}]-[INFO]-Business Log: {}{}",
                timestamp, user, detail, comment_part
            );

            // Write business log to app.log
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("app.log")
            {
                use std::io::Write;
                let _ = writeln!(file, "{}", business_line);
            }

            // Write business log to TUI buffer
            if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
                if let Ok(mut entries) = buf.entries.lock() {
                    entries.push(UnifiedLogEntry {
                        timestamp: std::time::SystemTime::now(),
                        user_identifier: Some(user.clone()),
                        trace_chain: event.trace_chain.clone(),
                        payload: LogPayload::Info(teaql_runtime::InfoLogEntry {
                            message: business_line,
                        }),
                    });
                }
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
            let old_str = format_field_val(&change.field, &change.old_value);
            let new_str = format_field_val(&change.field, &change.new_value);
            if old_str != new_str {
                let detail = format!(
                    "[{}] - [{}] - [AUDIT]   -> Field [{}]: {} ➔ {}",
                    timestamp_with_date, user, display_field_name(&change.field), old_str, new_str
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
    ) -> Result<Vec<teaql_core::Record>, Self::Error> {
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
