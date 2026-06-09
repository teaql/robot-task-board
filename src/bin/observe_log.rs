use teaql_provider_rusqlite::{
    ensure_rusqlite_schema_for, RusqliteIdSpaceGenerator,
    RusqliteMutationExecutor, RusqliteProviderExt,
};
use robot_kanban::{AuditedSave, Q};
use teaql_core::Entity;
use rusqlite::Connection;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Without Application Customization Log Observation ===");

    // Use generated module which pre-registers repositories and domain logic
    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
    ctx.set_user_identifier("system_tester");
    
    // Setup Rusqlite directly
    let conn = Connection::open("observe_test.db")?;
    let executor = RusqliteMutationExecutor::new(conn);
    
    ctx.use_rusqlite_provider(executor.clone());
    ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(executor.clone()));
    
    let service_executor = robot_kanban::ServiceRuntimeExecutor::new(executor);
    ctx.insert_resource(service_executor);

    // Auto schema migration
    ensure_rusqlite_schema_for(&ctx)?;

    println!("\n[Action] Creating a new Task entity...");
    let mut task = Q::tasks()
        .comment("Controller: Receive CreateTaskRequest")
        .comment("Service: Validate inputs")
        .comment("Repository: Prepare entity for insertion")
        .purpose("DB: Persist Task")
        .new_entity(&ctx);
        
    task.update_id(999)
        .update_name("Test Terminal Logging")
        .update_status_to_planned()
        .update_platform_id(1)
        .update_version(0);

    // Save will trigger the audit/trace hooks in the framework
    task.audit_as("Audit: Track entity creation").save(&ctx).await?;

    println!("\n[Action] Querying tasks...");
    let exists = Q::tasks()
        .comment("CronJob: Scan for planned tasks")
        .comment("DB: Check task existence")
        .execute_for_exists(&ctx).await?;
    println!("Task exists: {}", exists);

    println!("\n[Action] Demonstrating complex trace chain...");
    let mut fake_event = teaql_runtime::RawAuditEvent::created("ComplexProcess", std::collections::BTreeMap::new());
    fake_event.trace_chain = vec![
        teaql_core::TraceNode { entity_type: "".into(), entity_id: None, comment: "User clicks checkout".into() },
        teaql_core::TraceNode { entity_type: "".into(), entity_id: None, comment: "Order Service validates stock".into() },
        teaql_core::TraceNode { entity_type: "".into(), entity_id: None, comment: "DB persists order".into() },
    ];
    println!("\n[Action] Faceting tasks by status...");
    let facets = Q::tasks()
        .facet_by_status_as("status_stats", Q::task_status().comment("Count tasks in this status").count_tasks())
        .purpose("Facet: Group by status")
        .execute_for_records(&ctx).await?;
    println!("Status facets: {:?}", facets);

    Ok(())
}
