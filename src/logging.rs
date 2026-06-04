#![allow(clippy::disallowed_types, clippy::disallowed_methods)]
// Infrastructure layer — allowed to use chrono::Utc, std::fs, etc.

use teaql_runtime::{
    UserContext, EntityEvent,
    EntityEventKind, EntityEventSink, RuntimeError, UnifiedLogEntry, UnifiedLogBuffer, LogPayload,
};
use teaql_core::Value;
use teaql_tool_core::{AuditConfig, AuditLevel};

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

/// Resolve a status ID value to a human-readable code.
fn resolve_status_name(raw: &str) -> String {
    match raw {
        "1001" => "PLANNED".to_owned(),
        "1002" => "READY".to_owned(),
        "1003" => "EXECUTING".to_owned(),
        "1004" => "VERIFIED".to_owned(),
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

/// Retrieve the AuditConfig from UserContext, or fall back to production defaults.
fn get_audit_config(ctx: &UserContext) -> AuditConfig {
    ctx.get_resource::<AuditConfig>()
        .cloned()
        .unwrap_or_else(AuditConfig::production)
}

/// Write a formatted message to the UnifiedLogBuffer for TUI display.
/// Infrastructure-only helper — not exposed to the application layer.
fn write_to_buffer(ctx: &UserContext, message: &str) {
    let user = short_user(ctx);
    let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f").to_string();
    let log_line = format!("[{}]-[{}]-[INFO]-{}", timestamp, user, message);

    if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
        if let Ok(mut entries) = buf.entries.lock() {
            entries.push(UnifiedLogEntry {
                timestamp: std::time::SystemTime::now(),
                user_identifier: Some(user),
                trace_chain: Vec::new(),
                payload: LogPayload::Info(teaql_runtime::InfoLogEntry {
                    message: log_line,
                }),
            });
        }
    }
}

/// Emit a UI feedback message to the TUI display buffer.
///
/// This is NOT an audit/logging API — it is a UI event emitter for
/// messages like "System initialized", "Unknown command", etc.
/// It writes to the UnifiedLogBuffer only (no file I/O).
pub fn emit_ui_message(ctx: &UserContext, message: &str) {
    write_to_buffer(ctx, message);
}

/// Write a line to a file, appending. Infrastructure-only.
fn append_to_file(path: &str, line: &str) {
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        use std::io::Write;
        let _ = writeln!(file, "{}", line);
    }
}

pub struct AppAuditSink;

impl EntityEventSink for AppAuditSink {
    fn on_event(&self, ctx: &UserContext, event: &EntityEvent) -> Result<(), RuntimeError> {
        // Bootstrap events are always written to TUI buffer (not controlled by AuditConfig)
        // because they are part of the startup UX, not business audit.
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

        // ── AuditConfig-controlled entity event logging ──────────────────
        let config = get_audit_config(ctx);

        // Entity mutations map to the conceptual "Kv" module (data store operations).
        // If the audit level is Silent, skip all output.
        let level = config.level_for(teaql_tool_core::Module::Kv);
        if level == AuditLevel::Silent {
            return Ok(());
        }

        let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f").to_string();
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

        // ── Summary level: compact one-liner ─────────────────────────────
        let summary_line = format!(
            "[{}]-[{}]-[AUDIT]-Entity [{}] {}.{}",
            timestamp, user, entity_identity, action_name, comment_part
        );

        if level == AuditLevel::Summary {
            // Summary: write to TUI buffer only (no file, no field details)
            if let Some(buf) = ctx.get_resource::<UnifiedLogBuffer>() {
                if let Ok(mut entries) = buf.entries.lock() {
                    entries.push(UnifiedLogEntry {
                        timestamp: std::time::SystemTime::now(),
                        user_identifier: Some(user.clone()),
                        trace_chain: event.trace_chain.clone(),
                        payload: LogPayload::Info(teaql_runtime::InfoLogEntry {
                            message: summary_line,
                        }),
                    });
                }
            }
            return Ok(());
        }

        // ── Full level and above: include field changes ──────────────────
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

        let audit_line = format!("{}{}", summary_line, fields_part);

        // Write to app.log
        append_to_file("app.log", &audit_line);

        // Write to TUI buffer
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

        // Business log for TaskExecutionLog creation
        let is_business_log = event.entity == "TaskExecutionLog" && action_name == "CREATED";
        if is_business_log {
            let mut detail = String::new();
            for change in &event.changes {
                if change.field == "detail" {
                    detail = format_val_helper(&change.new_value);
                    if detail.starts_with('\'') && detail.ends_with('\'') {
                        detail = detail[1..detail.len()-1].to_owned();
                    }
                }
            }
            let business_line = format!(
                "[{}]-[{}]-[INFO]-Business Log: {}{}",
                timestamp, user, detail, comment_part
            );
            append_to_file("app.log", &business_line);

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

        // Write detailed audit.log (Full format with per-field breakdown)
        let timestamp_with_date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let audit_header = format!(
            "[{}] - [{}] - [AUDIT] Entity [{}] {}.{}",
            timestamp_with_date, user, entity_identity, action_name, comment_part
        );
        append_to_file("audit.log", &audit_header);
        for change in &event.changes {
            let old_str = format_field_val(&change.field, &change.old_value);
            let new_str = format_field_val(&change.field, &change.new_value);
            if old_str != new_str {
                let detail = format!(
                    "[{}] - [{}] - [AUDIT]   -> Field [{}]: {} ➔ {}",
                    timestamp_with_date, user, display_field_name(&change.field), old_str, new_str
                );
                append_to_file("audit.log", &detail);
            }
        }
        append_to_file("audit.log", &format!(
            "[{}] - [{}] - [AUDIT] ------------------------------------------------------------",
            timestamp_with_date, user
        ));

        Ok(())
    }
}
