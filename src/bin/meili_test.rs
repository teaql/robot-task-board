use robot_kanban::{ServiceRuntimeExecutor, AuditedSave};
use teaql_provider_meilisearch::MeilisearchProvider;
use teaql_core::Entity;
use teaql_provider_rusqlite::{RusqliteMutationExecutor, RusqliteIdSpaceGenerator, ensure_rusqlite_schema_for, RusqliteProviderExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Initializing databases...");
    
    // 1. Create SQLite provider as fallback base runtime
    let conn = rusqlite::Connection::open_in_memory()?;
    let inner = RusqliteMutationExecutor::new(conn);
    
    // 2. Create Meilisearch provider
    let meili = MeilisearchProvider::new("http://localhost:7700", Some("teaql_test_key".to_string()));
    
    // 3. Assemble Runtime
    let runtime = ServiceRuntimeExecutor::new(inner.clone())
        .with_meilisearch(meili);
        
    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
    ctx.use_rusqlite_provider(inner.clone());
    ctx.set_internal_id_generator(RusqliteIdSpaceGenerator::from_executor(inner));
    ctx.insert_resource(runtime);

    // ensure schema
    ensure_rusqlite_schema_for(&ctx)?;

    println!("Creating TaskExecutionLog data...");
    
    // Create 3 Action logs
    let mut log1 = robot_kanban::Q::task_execution_logs().purpose("create").new_entity(&ctx);
    log1.update_action("start".to_string())
        .update_detail("Robot started moving towards sector 7G".to_string())
        .update_version(0);
    log1.audit_as("create log").save(&ctx).await?;

    let mut log2 = robot_kanban::Q::task_execution_logs().purpose("create").new_entity(&ctx);
    log2.update_action("warning".to_string())
        .update_detail("Encountered an obstacle at sector 7G, calculating alternate route".to_string())
        .update_version(0);
    log2.audit_as("create log").save(&ctx).await?;

    let mut log3 = robot_kanban::Q::task_execution_logs().purpose("create").new_entity(&ctx);
    log3.update_action("error".to_string())
        .update_detail("Task failed due to low battery".to_string())
        .update_version(0);
    log3.audit_as("create log").save(&ctx).await?;

    println!("Waiting for Meilisearch indexing (1000ms)...");
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    
    println!("Executing Full-Text Search for 'obstacle'...");
    
    // Call the generated search method!
    let results = robot_kanban::Q::task_execution_logs()
        .search_with_text("obstacle")
        .purpose("search")
        .execute_for_list(&ctx)
        .await?
        .data;
        
    println!("--- Search Results ---");
    for log in results {
        println!("Action: {}, Detail: {}", log.action(), log.detail());
    }
    
    Ok(())
}
