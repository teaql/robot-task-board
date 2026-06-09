use robot_kanban::Q;
use teaql_provider_rusqlite::{RusqliteMutationExecutor, RusqliteIdSpaceGenerator, RusqliteProviderExt};

#[test]
fn test_execute_for_list_load_state() {
    // Open SQLite database
    let conn = rusqlite::Connection::open("robot_kanban.db").unwrap();
    let executor = RusqliteMutationExecutor::new(conn);
    
    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
    ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(executor.clone()));
    ctx.use_rusqlite_provider(executor.clone());
    
    let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(executor.clone());
    ctx.insert_resource(service_runtime_executor);

    let tasks_future = Q::tasks().purpose("test").execute_for_list(&ctx);
    let tasks = tokio::runtime::Runtime::new().unwrap().block_on(tasks_future).unwrap().data;
    
    println!("Tasks returned: {}", tasks.len());
    for task in &tasks {
        println!("Task id: {}, name: {}", task.id(), task.name());
        println!("Task load state: {:?}", task.__load_state);
        // Let's assert that important fields are loaded.
        assert!(task.is_loaded("id"), "id field should be loaded");
        assert!(task.is_loaded("name"), "name field should be loaded");
    }
}
