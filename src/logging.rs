use teaql_runtime::{
    UserContext, SafeAuditEvent,
    RawAuditEventKind, SafeAuditEventSink, RuntimeError, UnifiedLogEntry, UnifiedLogBuffer, LogPayload,
};
use teaql_core::Value;

/// Extract just the OS username from the full user identifier (e.g. "philip@pid-123.tid-1" → "philip")
pub fn short_user(ctx: &UserContext) -> String {
    let full = ctx.user_identifier().unwrap_or("unknown");
    full.split('@').next().unwrap_or(full).to_owned()
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

pub struct AppAuditSink;

impl SafeAuditEventSink for AppAuditSink {
    fn on_safe_event(&self, ctx: &UserContext, event: &SafeAuditEvent) -> Result<(), RuntimeError> {
        // Helper to extract a string value from SafeAuditField
        let get_field = |name: &str| -> String {
            event.fields.iter()
                .find(|f| f.name == name)
                .and_then(|f| f.value.clone())
                .unwrap_or_else(|| "unknown".to_owned())
        };

        // Bootstrap events are handled separately
        match event.kind {
            RawAuditEventKind::SchemaCreated
            | RawAuditEventKind::SchemaVerified
            | RawAuditEventKind::FieldAdded
            | RawAuditEventKind::DataSeeded => {
                let table_name = get_field("table_name").trim_matches('\'').to_string();
                let field_count: usize = get_field("field_count").parse().unwrap_or(0);

                let message = match event.kind {
                    RawAuditEventKind::SchemaCreated => {
                        format!("Create {} ({} fields)", table_name, field_count)
                    }
                    RawAuditEventKind::SchemaVerified => {
                        format!("Verified {} ({} fields)", table_name, field_count)
                    }
                    RawAuditEventKind::FieldAdded => {
                        let field_name = get_field("field_name").trim_matches('\'').to_string();
                        format!("  + field {} on {}", field_name, table_name)
                    }
                    RawAuditEventKind::DataSeeded => {
                        let inserted: usize = get_field("inserted").parse().unwrap_or(0);
                        let updated: usize = get_field("updated").parse().unwrap_or(0);
                        format!("Seed {}: {} inserted, {} updated", table_name, inserted, updated)
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
            RawAuditEventKind::Created => "CREATED",
            RawAuditEventKind::Updated => "UPDATED",
            RawAuditEventKind::Deleted => "DELETED",
            RawAuditEventKind::Recovered => "RECOVERED",
            _ => unreachable!(),
        };

        let entity_identity = format!("{}", event.entity);

        let comment_part = if event.trace_chain.is_empty() {
            "".to_owned()
        } else {
            let trace = event.trace_chain.iter().map(|n| n.comment.clone()).collect::<Vec<_>>().join(" -> ");
            format!(" [{}]", trace)
        };

        // Build compact single-line audit for TUI and app.log
        let mut field_changes = Vec::new();
        for field in &event.fields {
            let mut val_str = field.value.clone().unwrap_or_else(|| "NULL".to_owned());
            if field.masked { val_str.push_str(" [MASKED]"); }
            if field.truncated { val_str.push_str(" [TRUNCATED]"); }
            field_changes.push(format!("{}: {}", display_field_name(&field.name), val_str));
        }
        let fields_part = if field_changes.is_empty() {
            String::new()
        } else {
            format!(" {{{}}}", field_changes.join(",  "))
        };

        let audit_line = format!(
            "[{}]-[{}]-[AUDIT]-Entity [{}] {}.{}{}",
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
            for field in &event.fields {
                if field.name == "detail" {
                    detail = field.value.clone().unwrap_or_default();
                    // Remove quotes if any
                    if detail.starts_with('\'') && detail.ends_with('\'') {
                        detail = detail[1..detail.len()-1].to_owned();
                    }
                    if field.masked { detail.push_str(" [MASKED]"); }
                    if field.truncated { detail.push_str(" [TRUNCATED]"); }
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
            "[{}] - [{}] - [AUDIT] Entity [{}] {}.{}",
            timestamp_with_date, user, entity_identity, action_name, comment_part
        );
        let mut audit_lines = vec![audit_header];
        for field in &event.fields {
            let mut val_str = field.value.clone().unwrap_or_else(|| "NULL".to_owned());
            if field.masked { val_str.push_str(" [MASKED]"); }
            if field.truncated { val_str.push_str(" [TRUNCATED]"); }

            let detail = format!(
                "[{}] - [{}] - [AUDIT]   -> Field [{}]: {}",
                timestamp_with_date, user, display_field_name(&field.name), val_str
            );
            audit_lines.push(detail);
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

