use std::error::Error;

mod app;
mod commands;
mod logging;
mod models;
pub mod service;
mod startup;
mod tui;
mod ui;
mod utils;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;

    startup::draw_welcome(&mut terminal)?;
    startup::wait_for_key()?;

    let service = startup::bootstrap(&mut terminal, "robot_kanban.db").await?;

    tui::show_cursor(&mut terminal)?;

    let mut app = App::new(service);
    app.reload_data().await?;
    app.service.log_info("=================================================================================================");

    app.run(&mut terminal).await?;

    tui::restore(&mut terminal)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::service::TaskService;
    use crate::models::MoveResult;

    #[tokio::test]
    async fn test_comment_propagation() -> Result<(), Box<dyn std::error::Error>> {
        // 1. Delete old test files if present
        let _ = std::fs::remove_file("test_propagation.db");

        // 2. Initialize service directly
        let db = TaskService::new("test_propagation.db").await?;

        // 3. Reload data with None (triggering Get active tasks)
        let _ = db.reload_data(&None).await?;

        // 4. Retrieve logged entries returned by check_sql_logs()
        let formatted_logs = db.check_sql_logs();
        
        println!("=== Captured Formatted Logs ===");
        let mut found_facet_status_query = false;
        let mut found_facet_task_query = false;
        let mut current_trace_contains_target = false;
        for log in &formatted_logs {
            println!("{}", log);
            if log.contains("Get active tasks->status_stats->Count status") || log.contains("Get active tasks -> status_stats -> Count status") {
                current_trace_contains_target = true;
            }
            if current_trace_contains_target {
                if log.contains("task_status_data") {
                    found_facet_status_query = true;
                }
                if log.contains("task_data") && log.contains("COUNT(*)") {
                    found_facet_task_query = true;
                }
            }
        }

        // Cleanup test db and log
        let _ = std::fs::remove_file("test_propagation.db");

        assert!(
            found_facet_status_query,
            "Comment propagation chain [Get active tasks->status_stats->Count status] not found in task_status_data subquery log!"
        );
        assert!(
            found_facet_task_query,
            "Comment propagation chain [Get active tasks->status_stats->Count status] not found in task_data relation aggregate subquery log!"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_add_and_delete_task() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_add_delete.db";
        let _ = std::fs::remove_file(db_file);

        // 1. Initialize Service and add a task
        let db = TaskService::new(db_file).await?;
        let task_id = db.add_task("Verify Task Flow").await?;
        assert!(task_id > 0, "Task ID should be greater than 0");

        // 2. Reload data and verify task presence
        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 1, "Should have exactly 1 planned task");
        assert_eq!(reloaded.planned_tasks[0].name, "Verify Task Flow");

        // 3. Delete task and verify it's gone
        let deleted = db.delete_task(task_id).await?;
        assert!(deleted, "Task deletion should be successful");

        let reloaded_after = db.reload_data(&None).await?;
        assert_eq!(reloaded_after.planned_tasks.len(), 0, "Planned task list should be empty after deletion");

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }

    #[tokio::test]
    async fn test_move_task_ddd() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_move_task.db";
        let _ = std::fs::remove_file(db_file);

        // 1. Initialize and add a task
        let db = TaskService::new(db_file).await?;
        let task_id = db.add_task("DDD Aggregates Transition").await?;

        // Verify initial status (Planned = 1)
        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 1);
        assert_eq!(reloaded.ready_tasks.len(), 0);

        // 2. Move next (empty command moves Planned -> Ready)
        let res = db.move_task(task_id, "").await?;
        match res {
            MoveResult::Moved { status_name, .. } => {
                assert_eq!(status_name, "READY");
            }
            _ => panic!("Expected task to be moved"),
        }

        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 0);
        assert_eq!(reloaded.ready_tasks.len(), 1);

        // 3. Move directly to Verified
        let res2 = db.move_task(task_id, "verified").await?;
        match res2 {
            MoveResult::Moved { status_name, .. } => {
                assert_eq!(status_name, "VERIFIED");
            }
            _ => panic!("Expected task to be moved to Verified"),
        }

        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.ready_tasks.len(), 0);
        assert_eq!(reloaded.verified_tasks.len(), 1);

        // 4. Test invalid status move
        let res3 = db.move_task(task_id, "invalid_status").await?;
        match res3 {
            MoveResult::Error { err_msg, .. } => {
                assert!(err_msg.contains("Invalid status"));
            }
            _ => panic!("Expected move to fail with invalid status"),
        }

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }

    #[tokio::test]
    async fn test_task_execution_log_lineage() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_execution_log_lineage.db";
        let _ = std::fs::remove_file(db_file);

        let db = TaskService::new(db_file).await?;
        let task_id = db.add_task("Lineage Test Task").await?;

        // Move task to Ready
        let _ = db.move_task(task_id, "ready").await?;

        // Delete task (which explicitly cascade soft-deletes the related logs)
        let _ = db.delete_task(task_id).await?;

        // Retrieve SQL logs from the context to check for the lineage comment
        let mut sql_logs_with_trace = Vec::new();
        if let Some(buf) = db.context().get_resource::<teaql_runtime::UnifiedLogBuffer>() {
            if let Ok(entries) = buf.entries.lock() {
                for entry in entries.iter() {
                    if let teaql_runtime::LogPayload::Sql(sql_entry) = &entry.payload {
                        sql_logs_with_trace.push((entry.trace_chain.clone(), sql_entry.clone()));
                    }
                }
            }
        }

        println!("=== SQL Logs for Lineage Test ===");
        let mut found_created_log_lineage = false;
        let mut found_status_changed_log_lineage = false;

        for (trace, sql_entry) in sql_logs_with_trace {
            if !trace.is_empty() {
                let comments: Vec<String> = trace.iter().map(|t| t.comment.clone()).collect();
                let full_trace = comments.join(" -> ");
                println!("SQL with Trace [{}]: {}", full_trace, sql_entry.debug_sql);
                
                if full_trace.contains("Create task") {
                    found_created_log_lineage = true;
                }
                if full_trace.contains(" => ") {
                    found_status_changed_log_lineage = true;
                }
            }
        }

        let _ = std::fs::remove_file(db_file);

        assert!(
            found_created_log_lineage,
            "Hierarchical lineage comment not propagated to SQL insert log for task creation execution log!"
        );
        assert!(
            found_status_changed_log_lineage,
            "Hierarchical lineage comment not propagated to SQL insert log for status transition execution log!"
        );
        // Verify task is actually deleted (functional check instead of SQL log check)
        let reloaded = db.reload_data(&None).await?;
        assert_eq!(
            reloaded.planned_tasks.len() + reloaded.ready_tasks.len() + reloaded.executing_tasks.len() + reloaded.verified_tasks.len(),
            0,
            "Task should be soft-deleted and not visible after reload"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_unified_log_order() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_unified_log_order.db";
        let _ = std::fs::remove_file(db_file);

        let db = TaskService::new(db_file).await?;

        // 1. Initial reload (simulates app startup)
        let _ = db.reload_data(&None).await?;
        let startup_logs = db.check_sql_logs();
        println!("=== STARTUP RELOAD ===");
        for (i, log) in startup_logs.iter().enumerate() {
            println!("{:02}: {}", i, log);
        }

        // 2. Add a task + reload
        let _ = db.add_task("My New Task").await?;
        let _ = db.reload_data(&None).await?;
        let add_logs = db.check_sql_logs();
        println!("\n=== ADD TASK + RELOAD ===");
        for (i, log) in add_logs.iter().enumerate() {
            println!("{:02}: {}", i, log);
        }

        // 3. Move task + reload
        let _ = db.move_task(1, "Ready").await?;
        let _ = db.reload_data(&None).await?;
        let move_logs = db.check_sql_logs();
        println!("\n=== MOVE TASK + RELOAD ===");
        for (i, log) in move_logs.iter().enumerate() {
            println!("{:02}: {}", i, log);
        }

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }

    #[tokio::test]
    async fn test_bulk_task_operations() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_bulk_operations.db";
        let _ = std::fs::remove_file(db_file);

        let db = TaskService::new(db_file).await?;

        // Bulk insert 10 tasks
        for i in 1..=10 {
            db.add_task(&format!("Task number {}", i)).await?;
        }

        let reloaded = db.reload_data(&None).await?;
        assert_eq!(reloaded.planned_tasks.len(), 10, "Should have 10 planned tasks");

        // Move the first 5 to 'process' (which is 'Ready' status)
        for i in 1..=5 {
            db.move_task(i as u64, "ready").await?;
        }

        let reloaded_after_move = db.reload_data(&None).await?;
        assert_eq!(reloaded_after_move.planned_tasks.len(), 5, "Should have 5 planned tasks left");
        assert_eq!(reloaded_after_move.ready_tasks.len(), 5, "Should have 5 ready tasks");

        // Delete the remaining 5 planned tasks
        for i in 6..=10 {
            db.delete_task(i as u64).await?;
        }

        let final_state = db.reload_data(&None).await?;
        assert_eq!(final_state.planned_tasks.len(), 0, "No planned tasks should remain");
        assert_eq!(final_state.ready_tasks.len(), 5, "Ready tasks should still be 5");

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }

    #[tokio::test]
    async fn test_graph_mixed_mutations() -> Result<(), Box<dyn std::error::Error>> {
        let db_file = "test_mixed_mutations.db";
        let _ = std::fs::remove_file(db_file);

        let db = TaskService::new(db_file).await?;
        
        // 1. First, we create a task (Add)
        let task_id = db.add_task("Test Mixed Mutations").await?;

        // Let's get the Task repository to do advanced graph manipulation directly
        let ctx = db.context();
        let repo = ctx.resolve_repository::<robot_kanban::ServiceRuntimeExecutor>("Task")?;

        // 2. Fetch the task record
        let task_record = repo.fetch_graph_current_row("Task", "id", &teaql_core::Value::U64(task_id), Vec::new()).await?
            .expect("Task should exist");

        // 3. We modify the task in a GraphNode:
        let mut task_node = teaql_runtime::GraphNode::new("Task");
        task_node.original_values = Some(task_record.clone());
        task_node.values = task_record;
        
        // a) Update: Change task name
        task_node.values.insert("name".to_owned(), teaql_core::Value::Text("Updated Task Name".to_owned()));
        
        // b) Delete: Remove the first creation log
        // We don't have the ID easily, so we just remove a hypothetical log id=999
        let mut removed_log = teaql_runtime::GraphNode::new("TaskExecutionLog");
        removed_log.values.insert("id".to_owned(), teaql_core::Value::U64(999));
        removed_log.original_values = Some(removed_log.values.clone());
        removed_log.operation = teaql_runtime::GraphOperation::Remove;
            
        // c) Create: Add a new dummy log
        let mut new_log = teaql_runtime::GraphNode::new("TaskExecutionLog");
        new_log.values.insert("task_id".to_owned(), teaql_core::Value::U64(task_id));
        new_log.values.insert("action".to_owned(), teaql_core::Value::Text("TEST_ACTION".to_owned()));
        new_log.values.insert("detail".to_owned(), teaql_core::Value::Text("A newly added log".to_owned()));
        new_log.operation = teaql_runtime::GraphOperation::Create;
        
        task_node.relations.insert("task_execution_log_list".to_owned(), vec![removed_log, new_log]);

        // 4. Generate the mutation plan and execute it
        // The plan will naturally sequence: Update (Task), Create (TaskExecutionLog), Delete (TaskExecutionLog)
        let plan = repo.plan_graph(task_node).await?;
        
        // Verify that the plan contains Create, Update, Delete
        let mut has_create = false;
        let mut has_update = false;
        let mut has_delete = false;
        for batch in &plan.batches {
            match batch.kind {
                teaql_runtime::GraphMutationKind::Create => has_create = true,
                teaql_runtime::GraphMutationKind::Update => has_update = true,
                teaql_runtime::GraphMutationKind::Delete => has_delete = true,
                _ => {}
            }
        }
        
        assert!(has_create, "Plan should contain a Create operation");
        assert!(has_update, "Plan should contain an Update operation");
        assert!(has_delete, "Plan should contain a Delete operation");

        // Execute the single plan
        repo.execute_graph_plan(plan).await?;

        // 5. Verify the data changes
        let updated_task = repo.fetch_graph_current_row("Task", "id", &teaql_core::Value::U64(task_id), Vec::new()).await?
            .expect("Task should exist");
        
        assert_eq!(
            updated_task.get("name"),
            Some(&teaql_core::Value::Text("Updated Task Name".to_owned()))
        );

        let _ = std::fs::remove_file(db_file);
        Ok(())
    }
}
