use std::error::Error;

use crate::app::App;
use crate::models::MoveResult;

/// Parse and execute a user command. Called when the user presses Enter.
pub async fn execute(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.log_scroll_offset = 0;
    let trimmed = app.input.trim().to_owned();
    if trimmed.is_empty() {
        return Ok(());
    }

    // Slash-prefixed commands; bare input defaults to add task
    if trimmed.starts_with('/') {
        let without_slash = &trimmed[1..];
        let parts: Vec<&str> = without_slash.splitn(2, ' ').collect();
        let cmd = parts[0].to_lowercase();
        let args = if parts.len() > 1 { parts[1].trim() } else { "" };

        match cmd.as_str() {
            "exit" | "quit" | "q" => {
                app.should_quit = true;
                app.service.log_info("Exiting application...");
            }
            "search" | "s" => {
                if args.is_empty() {
                    app.search_term = None;
                    app.service.log_info("Cleared active search query.");
                } else {
                    app.search_term = Some(args.to_owned());
                    app.service.log_info(&format!("Searching for tasks by keyword: '{}'", args));
                }
                app.reload_data().await?;
            }
            "add" => {
                if args.is_empty() {
                    app.service.log_info("Error: Task name cannot be empty. Usage: /add <task name>");
                } else {
                    let _next_id = app.service.add_task(args).await?;
                    app.reload_data().await?;
                }
            }
            "delete" | "del" => {
                if args.is_empty() {
                    app.service.log_info("Error: Missing task ID. Usage: /del <id>");
                } else if let Ok(id) = args.parse::<u64>() {
                    if app.service.delete_task(id).await? {
                        app.reload_data().await?;
                    }
                } else {
                    app.service.log_info(&format!("Error: Invalid task ID '{}'", args));
                }
            }
            "move" | "mv" => {
                if args.is_empty() {
                    app.service.log_info("Error: Missing arguments. Usage: /mv <id> [planned|ready|executing|verified|next]");
                    app.input.clear();
                    return Ok(());
                }

                let move_parts: Vec<&str> = args.split_whitespace().collect();

                if let Ok(id) = move_parts[0].parse::<u64>() {
                    let target_status = if move_parts.len() > 1 {
                        move_parts[1].to_lowercase()
                    } else {
                        "".to_owned()
                    };

                    let res = app.service.move_task(id, &target_status).await?;
                    match res {
                        MoveResult::Moved { .. } => {
                            app.reload_data().await?;
                        }
                        _ => {}
                    }
                } else {
                    app.service.log_info(&format!("Error: Invalid task ID '{}'", move_parts[0]));
                }
            }
            _ => {
                app.service.log_info(&format!("Unknown command: '/{}'. Type a task name directly or use /mv, /del, /s, /q", cmd));
            }
        }
    } else {
        // Default: bare input = add task
        let _next_id = app.service.add_task(&trimmed).await?;
        app.reload_data().await?;
    }

    app.input.clear();
    app.check_sql_logs();
    app.add_log("--------------------------------------------------------------------------------");
    Ok(())
}
