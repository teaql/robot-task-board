use robot_kanban::{Q, AuditedSave};
use teaql_core::Entity;
use teaql_provider_rusqlite::{RusqliteMutationExecutor, RusqliteIdSpaceGenerator, RusqliteProviderExt, ensure_rusqlite_schema_for};

#[test]
fn test_graph_diff_short_circuit_on_unmodified_relations() {
    // Open a fresh in-memory SQLite database for testing
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let executor = RusqliteMutationExecutor::new(conn);
    
    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
    ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(executor.clone()));
    ctx.use_rusqlite_provider(executor.clone());
    
    let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(executor.clone());
    ctx.insert_resource(service_runtime_executor);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Ensure schema and seed data
        ensure_rusqlite_schema_for(&ctx).unwrap();

        // 1. Query the seeded platform
        let platform_in_db = Q::platforms()
            .purpose("Get platform")
            .execute_for_list(&ctx)
            .await
            .unwrap()
            .into_iter()
            .next()
            .expect("Expected a seeded platform");

        // 2. Create a Task belonging to the Platform
        let mut task = Q::tasks().purpose("Test").new_entity(&ctx);
        task.update_name("Original Task Name".to_owned());
        task.update_platform_id(platform_in_db.id().clone());
        let _ = task.audit_as("Create task").save(&ctx).await.unwrap();

        // Query the inserted task to get its generated ID
        let task_in_db = Q::tasks().purpose("Get task").execute_for_list(&ctx).await.unwrap().into_iter().next().unwrap();

        // 3. Query the Task WITH its Platform preloaded
        let loaded_task = Q::tasks()
            .with_id_is(task_in_db.id().clone())
            .select_platform() // preload relation
            .purpose("test graph diff engine")
            .execute_for_one(&ctx)
            .await
            .unwrap()
            .unwrap();

        // Ensure the platform was actually loaded
        assert!(loaded_task.platform().is_some());
        
        // 4. Modify the Task (but NOT the Platform)
        let mut task_to_update = loaded_task;
        
        println!("Before update: Task ID = {:?}, Version = {}", task_to_update.id(), task_to_update.version());
        
        task_to_update.update_name("Updated Task Name".to_owned());
        
        println!("After update: Task ID = {:?}, Version = {}", task_to_update.id(), task_to_update.version());
        println!("Dirty fields = {:?}", task_to_update.dirty_fields());
        
        // 5. Save the Task
        // Before teaql-rs v3.2.3, this would trigger an OptimisticLockConflict 
        // because the unmodified Platform would be redundantly UPDATE'd, yielding 0 affected rows.
        let saved_graph_node = task_to_update.audit_as("Update task name").save(&ctx).await.unwrap();

        // If we reach here, the graph diff engine successfully short-circuited the empty Platform update!
        assert_eq!(
            saved_graph_node.values.get("name").unwrap(),
            &teaql_core::Value::Text("Updated Task Name".to_owned())
        );
        
        println!("Graph Diff Engine short-circuit test passed successfully! No OptimisticLockConflict was thrown.");
    });
}
