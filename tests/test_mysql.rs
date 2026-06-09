use robot_kanban::{Q, AuditedSave};
use teaql_core::Entity;
use teaql_provider_mysql::{MysqlMutationExecutor, MysqlIdSpaceGenerator, MysqlProviderExt, ensure_mysql_schema_for};

#[tokio::test]
async fn test_mysql_basic() {
    let pool = mysql_async::Pool::new("mysql://root:0254891276@localhost:3306/testdb");
    let inner_executor = MysqlMutationExecutor::new(pool.clone());

    let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();

    ctx.use_mysql_provider(inner_executor.clone());
    ctx.set_internal_id_generator(MysqlIdSpaceGenerator::new(pool.clone()));

    let service_executor = robot_kanban::ServiceRuntimeExecutor::new(inner_executor.clone());
    ctx.insert_resource(service_executor);

    ensure_mysql_schema_for(&ctx).await.unwrap();

    let task = Q::tasks().purpose("Test").new_entity(&ctx);
    println!("Successfully connected and initialized schema on Postgres!");
}
