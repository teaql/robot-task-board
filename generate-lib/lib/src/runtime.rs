use crate::*;
use teaql_core::TeaqlEntity;

use teaql_provider_sqlite::SqliteProviderExt as _;

pub type DataServiceDialect = teaql_provider_sqlite::SqliteDialect;
pub type DataServiceMutationExecutor = teaql_provider_sqlite::SqliteMutationExecutor;
pub type DataServiceMutationError = teaql_provider_sqlite::MutationExecutorError;
pub type DataServiceIdGenerator = teaql_provider_sqlite::SqliteIdSpaceGenerator;
pub type DataServicePool = std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>;
pub type DataServiceExecutor = ServiceRuntimeExecutor;
pub type ServiceRuntime = teaql_runtime::UserContext;

pub const DATABASE_URL_ENV: &str = "ROBOT_KANBAN_SERVICE_DATABASE_URL";
pub const DATABASE_USER_ENV: &str = "ROBOT_KANBAN_SERVICE_DATABASE_USER";
pub const DATABASE_PASSWORD_ENV: &str = "ROBOT_KANBAN_SERVICE_DATABASE_PASSWORD";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRuntimeConfig {
    pub database_url: String,
    pub database_user: String,
    pub database_password: String,
}

impl ServiceRuntimeConfig {
    pub fn from_env() -> Result<Self, ServiceRuntimeError> {
        Ok(Self {
            database_url: env_value(DATABASE_URL_ENV)?,
            database_user: env_value(DATABASE_USER_ENV)?,
            database_password: env_value(DATABASE_PASSWORD_ENV)?,
        })
    }
}

#[derive(Debug)]
pub enum ServiceRuntimeError {
    MissingEnv {
        name: &'static str,
        source: std::env::VarError,
    },
    ConnectionError(String),
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::MissingEnv { name, source } => {
                write!(f, "missing environment variable {name}: {source}")
            }
            ServiceRuntimeError::ConnectionError(err) => write!(f, "connection error: {err}"),
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::MissingEnv { source, .. } => Some(source),
            ServiceRuntimeError::ConnectionError(_) => None,
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
    fn from(err: teaql_runtime::RuntimeError) -> Self {
        ServiceRuntimeError::Runtime(err)
    }
}

#[derive(Clone)]
pub struct LocalSchemaProvider;

impl teaql_data_service::SchemaProvider for LocalSchemaProvider {
    fn get_entity(&self, name: &str) -> Option<std::sync::Arc<teaql_core::EntityDescriptor>> {
        match name {
            "Platform" => Some(std::sync::Arc::new(crate::Platform::entity_descriptor())),
            "TaskStatus" => Some(std::sync::Arc::new(crate::TaskStatus::entity_descriptor())),
            "Tenant" => Some(std::sync::Arc::new(crate::Tenant::entity_descriptor())),
            "Task" => Some(std::sync::Arc::new(crate::Task::entity_descriptor())),
            "TaskExecutionLog" => Some(std::sync::Arc::new(crate::TaskExecutionLog::entity_descriptor())),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ServiceRuntimeExecutor {
    inner: teaql_sql::SqlDataServiceExecutor<
        DataServiceDialect,
        DataServiceMutationExecutor,
        LocalSchemaProvider
    >,
    meilisearch: Option<teaql_provider_meilisearch::MeilisearchProvider>,
}

impl ServiceRuntimeExecutor {
    pub fn new(inner: DataServiceMutationExecutor) -> Self {
        Self {
            inner: teaql_sql::SqlDataServiceExecutor::new(
                DataServiceDialect::default(),
                inner,
                LocalSchemaProvider
            ),
            meilisearch: None,
        }
    }

    pub fn with_meilisearch(mut self, meilisearch: teaql_provider_meilisearch::MeilisearchProvider) -> Self {
        self.meilisearch = Some(meilisearch);
        self
    }
}

impl teaql_data_service::DataServiceExecutor for ServiceRuntimeExecutor {
    type Error = teaql_sql::SqlExecutorError<DataServiceMutationError>;
    fn capabilities(&self) -> teaql_data_service::DataServiceCapabilities {
        teaql_data_service::DataServiceExecutor::capabilities(&self.inner)
    }
}

impl teaql_data_service::QueryExecutor for ServiceRuntimeExecutor {
    async fn query(&self, request: teaql_data_service::QueryRequest) -> Result<teaql_data_service::QueryResult, Self::Error> {
        use teaql_data_service::SchemaProvider;
        if let Some(desc) = self.inner.schema_provider.get_entity(&request.query.entity) {
            if desc.data_service.as_deref() == Some("meilisearch") {
                if let Some(meili) = &self.meilisearch {
                    return teaql_data_service::QueryExecutor::query(meili, request).await.map_err(|e| teaql_sql::SqlExecutorError::Compile(teaql_sql::SqlCompileError::UnknownEntity(e.to_string())));
                }
            }
        }
        teaql_data_service::QueryExecutor::query(&self.inner, request).await
    }
}

impl teaql_data_service::MutationExecutor for ServiceRuntimeExecutor {
    async fn mutate(&self, request: teaql_data_service::MutationRequest) -> Result<teaql_data_service::MutationResult, Self::Error> {
        use teaql_data_service::SchemaProvider;
        let entity_name = match &request {
            teaql_data_service::MutationRequest::Insert(cmd) => Some(&cmd.entity),
            teaql_data_service::MutationRequest::Update(cmd) => Some(&cmd.entity),
            teaql_data_service::MutationRequest::Delete(cmd) => Some(&cmd.entity),
            teaql_data_service::MutationRequest::Recover(cmd) => Some(&cmd.entity),
            teaql_data_service::MutationRequest::Batch(_) => None,
        };
        if let Some(entity_name) = entity_name {
            if let Some(desc) = self.inner.schema_provider.get_entity(entity_name) {
                if desc.data_service.as_deref() == Some("meilisearch") {
                    if let Some(meili) = &self.meilisearch {
                        // Always fallback to Meilisearch if it's explicitly set for the entity.
                        return teaql_data_service::MutationExecutor::mutate(meili, request).await.map_err(|e| teaql_sql::SqlExecutorError::Compile(teaql_sql::SqlCompileError::UnknownEntity(e.to_string())));
                    }
                }
            }
        }
        teaql_data_service::MutationExecutor::mutate(&self.inner, request).await
    }
}

impl teaql_data_service::TransactionExecutor for ServiceRuntimeExecutor {
    type Tx<'a> = teaql_sql::SqlDataServiceTransaction<'a, DataServiceDialect, <DataServiceMutationExecutor as teaql_sql::SqlTransactionTransport>::Tx<'a>, LocalSchemaProvider> where Self: 'a;

    async fn begin(&self) -> Result<Self::Tx<'_ >, Self::Error> {
        teaql_data_service::TransactionExecutor::begin(&self.inner).await
    }
}

/*
pub async fn service_runtime_from_env() -> Result<ServiceRuntime, ServiceRuntimeError> {
    service_runtime(ServiceRuntimeConfig::from_env()?).await
}
*/

/*
pub async fn service_runtime(config: ServiceRuntimeConfig) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let pool = connect_data_service_pool(&config).await?;
    service_runtime_from_pool(pool).await
}

pub async fn service_runtime_from_pool(pool: DataServicePool) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let id_generator = DataServiceIdGenerator::new(pool.clone());
    let mutation_executor = DataServiceMutationExecutor::new(pool);let mut context = module_with_behaviors_and_checkers().into_context();
    context.set_internal_id_generator(id_generator);
    context.use_sqlite_provider(mutation_executor.clone());
    context.insert_resource(ServiceRuntimeExecutor::new(mutation_executor));

    // 自动加载 Zero-Code 审计配置与 Schema 模式
    let env_config = teaql_tool_core::audit_config_from_env(&[
        "platform_data", "task_status_data", "tenant_data", "task_data", "task_execution_log_data"
    ]);
    let schema_mode = env_config.schema_mode;
    context.insert_resource(env_config.config.clone());
    context.insert_resource(env_config);

    match schema_mode {
        teaql_tool_core::SchemaMode::Execute => {
            context.ensure_schema().await?;
        }
        teaql_tool_core::SchemaMode::DryRun => {
            // DryRun: 目前等效于验证
            context.ensure_schema().await?;
        }
        teaql_tool_core::SchemaMode::Verify => {
            context.ensure_schema().await?;
        }
    }

    Ok(context)
}
*/



fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
    std::env::var(name).map_err(|source| ServiceRuntimeError::MissingEnv { name, source })
}

async fn connect_data_service_pool(config: &ServiceRuntimeConfig) -> Result<DataServicePool, ServiceRuntimeError> {
    let conn = rusqlite::Connection::open(&config.database_url).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?;
    Ok(std::sync::Arc::new(std::sync::Mutex::new(conn)))
}
pub fn repository_registry() -> teaql_runtime::InMemoryRepositoryRegistry {
    teaql_runtime::InMemoryRepositoryRegistry::new()
        .with_entity("Platform")
        .with_entity("TaskStatus")
        .with_entity("Tenant")
        .with_entity("Task")
        .with_entity("TaskExecutionLog")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryRepositoryBehaviorRegistry {
    teaql_runtime::InMemoryRepositoryBehaviorRegistry::new()
        .with_behavior("Platform", PlatformBehavior::default())
        .with_behavior("TaskStatus", TaskStatusBehavior::default())
        .with_behavior("Tenant", TenantBehavior::default())
        .with_behavior("Task", TaskBehavior::default())
        .with_behavior("TaskExecutionLog", TaskExecutionLogBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TaskStatus, _>::new(TaskStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Tenant, _>::new(TenantChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Task, _>::new(TaskChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog, _>::new(TaskExecutionLogChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Platform>()
        .entity::<TaskStatus>()
        .entity::<Tenant>()
        .entity::<Task>()
        .entity::<TaskExecutionLog>()
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("user_email", "string()")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Ready")
            .value("code", "READY")
            .value("color", "#3B82F6")
            .value("display_order", "20")
            .value("progress", "25")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Executing")
            .value("code", "EXECUTING")
            .value("color", "#F59E0B")
            .value("display_order", "30")
            .value("progress", "50")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1004_u64)
            .value("name", "Verified")
            .value("code", "VERIFIED")
            .value("color", "#16A34A")
            .value("display_order", "40")
            .value("progress", "100")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Platform>()
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity::<TaskStatus>()
        .checker(teaql_runtime::TypedEntityChecker::<TaskStatus, _>::new(TaskStatusChecker::default()))
        .entity::<Tenant>()
        .checker(teaql_runtime::TypedEntityChecker::<Tenant, _>::new(TenantChecker::default()))
        .entity::<Task>()
        .checker(teaql_runtime::TypedEntityChecker::<Task, _>::new(TaskChecker::default()))
        .entity::<TaskExecutionLog>()
        .checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog, _>::new(TaskExecutionLogChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("user_email", "string()")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Ready")
            .value("code", "READY")
            .value("color", "#3B82F6")
            .value("display_order", "20")
            .value("progress", "25")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Executing")
            .value("code", "EXECUTING")
            .value("color", "#F59E0B")
            .value("display_order", "30")
            .value("progress", "50")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1004_u64)
            .value("name", "Verified")
            .value("code", "VERIFIED")
            .value("color", "#16A34A")
            .value("display_order", "40")
            .value("progress", "100")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .entity_with_behavior::<TaskStatus, _>(TaskStatusBehavior::default())
        .entity_with_behavior::<Tenant, _>(TenantBehavior::default())
        .entity_with_behavior::<Task, _>(TaskBehavior::default())
        .entity_with_behavior::<TaskExecutionLog, _>(TaskExecutionLogBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("user_email", "string()")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Ready")
            .value("code", "READY")
            .value("color", "#3B82F6")
            .value("display_order", "20")
            .value("progress", "25")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Executing")
            .value("code", "EXECUTING")
            .value("color", "#F59E0B")
            .value("display_order", "30")
            .value("progress", "50")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1004_u64)
            .value("name", "Verified")
            .value("code", "VERIFIED")
            .value("color", "#16A34A")
            .value("display_order", "40")
            .value("progress", "100")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity_with_behavior::<TaskStatus, _>(TaskStatusBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TaskStatus, _>::new(TaskStatusChecker::default()))
        .entity_with_behavior::<Tenant, _>(TenantBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Tenant, _>::new(TenantChecker::default()))
        .entity_with_behavior::<Task, _>(TaskBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Task, _>::new(TaskChecker::default()))
        .entity_with_behavior::<TaskExecutionLog, _>(TaskExecutionLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog, _>::new(TaskExecutionLogChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("user_email", "string()")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Ready")
            .value("code", "READY")
            .value("color", "#3B82F6")
            .value("display_order", "20")
            .value("progress", "25")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Executing")
            .value("code", "EXECUTING")
            .value("color", "#F59E0B")
            .value("display_order", "30")
            .value("progress", "50")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1004_u64)
            .value("name", "Verified")
            .value("code", "VERIFIED")
            .value("color", "#16A34A")
            .value("display_order", "40")
            .value("progress", "100")
            .value("version", 1_i64)
            .value("platform_id", 1_u64))
}