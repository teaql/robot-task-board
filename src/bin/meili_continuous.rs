use robot_kanban::{ServiceRuntimeExecutor, AuditedSave};
use teaql_provider_meilisearch::MeilisearchProvider;
use teaql_runtime::UserContext;
use teaql_provider_rusqlite::{RusqliteMutationExecutor, RusqliteIdSpaceGenerator, ensure_rusqlite_schema_for, RusqliteProviderExt};
use std::error::Error;
use rand::prelude::IndexedRandom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Initializing continuous CLI simulation...");
    
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

    let actions = ["start", "warning", "error", "info", "move"];
    let events = [
        "Robot started moving towards sector 7G",
        "Encountered an obstacle at sector 7G, calculating alternate route",
        "Task failed due to low battery",
        "Arrived at destination sector 9A",
        "Detected human presence, slowing down",
        "Picked up object from sector 2B",
        "Lost connection to control server temporarily",
    ];

    let mut iteration = 0;
    println!("Starting continuous data generation loop. Press Ctrl+C to stop.");

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));

    loop {
        interval.tick().await;
        iteration += 1;

        let mut rng = rand::rng();
        let action = actions.choose(&mut rng).unwrap();
        let detail = events.choose(&mut rng).unwrap();

        // 1. Write log to Meilisearch
        let mut log = robot_kanban::Q::task_execution_logs().purpose("simulate").new_entity(&ctx);
        log.update_action(action.to_string())
            .update_detail(detail.to_string())
            .update_version(0);
            
        log.save(&ctx).await?;
        println!("[{}] Generated log -> Action: {:<8} | Detail: {}", iteration, action, detail);

        // 2. Periodically query from Meilisearch
        if iteration % 5 == 0 {
            println!("\n--- [Search Triggered] Checking index for 'obstacle' ---");
            // Give Meilisearch a tiny bit of time to index the most recent document
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;

            let results = robot_kanban::Q::task_execution_logs()
                .search_with_text("obstacle")
                .purpose("search")
                .execute_for_list(&ctx)
                .await?
                .data;

            println!("Found {} results:", results.len());
            for r in results.iter().take(3) {
                println!("  - [{}]: {}", r.action(), r.detail());
            }
            if results.len() > 3 {
                println!("  ... and {} more", results.len() - 3);
            }
            println!("------------------------------------------------------\n");
        }
    }
}
