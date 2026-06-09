use robot_kanban::{Q, AuditedSave};
use teaql_core::Entity;
use teaql_provider_postgres::{PgMutationExecutor, PgIdSpaceGenerator, PostgresProviderExt, ensure_postgres_schema_for};

#[tokio::test]
async fn test_pg_basic() {
    let mut cfg = deadpool_postgres::Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("postgres".to_string());
    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), tokio_postgres::NoTls).unwrap();
    let executor = PgMutationExecutor::new(pool);
    
    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
    ctx.set_internal_id_generator(PgIdSpaceGenerator::from_executor(executor.clone()));
    ctx.use_postgres_provider(executor.clone());
    
    let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(executor.clone());
    ctx.insert_resource(service_runtime_executor);

    ensure_postgres_schema_for(&ctx).await.unwrap();

    let task = Q::tasks().purpose("Test").new_entity(&ctx);
    println!("Successfully connected and initialized schema on Postgres!");
}
