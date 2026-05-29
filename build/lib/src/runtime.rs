use crate::*;

use teaql_provider_sqlx_postgres::PostgresProviderExt as _;

pub type DataServiceDialect = teaql_provider_sqlx_postgres::PostgresDialect;
pub type DataServiceMutationExecutor = teaql_provider_sqlx_postgres::PgMutationExecutor;
pub type DataServiceMutationError = teaql_provider_sqlx_postgres::MutationExecutorError;
pub type DataServiceIdGenerator = teaql_provider_sqlx_postgres::PgIdSpaceGenerator;
pub type DataServicePool = sqlx::PgPool;
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
    Sqlx(sqlx::Error),
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::MissingEnv { name, source } => {
                write!(f, "missing environment variable {name}: {source}")
            }
            ServiceRuntimeError::Sqlx(err) => write!(f, "sqlx error: {err}"),
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::MissingEnv { source, .. } => Some(source),
            ServiceRuntimeError::Sqlx(err) => Some(err),
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<sqlx::Error> for ServiceRuntimeError {
    fn from(err: sqlx::Error) -> Self {
        ServiceRuntimeError::Sqlx(err)
    }
}

impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
    fn from(err: teaql_runtime::RuntimeError) -> Self {
        ServiceRuntimeError::Runtime(err)
    }
}

#[derive(Clone)]
pub struct ServiceRuntimeExecutor {
    inner: DataServiceMutationExecutor,
}

impl ServiceRuntimeExecutor {
    pub fn new(inner: DataServiceMutationExecutor) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &DataServiceMutationExecutor {
        &self.inner
    }
}

impl teaql_runtime::QueryExecutor for ServiceRuntimeExecutor {
    type Error = DataServiceMutationError;

    fn fetch_all(
        &self,
        query: &teaql_sql::CompiledQuery,
    ) -> Result<Vec<teaql_core::Record>, Self::Error> {
        let inner = self.inner.clone();
        let query = query.clone();
        block_on_data_service(async move { inner.fetch_all(&query).await })
    }

    fn execute(&self, query: &teaql_sql::CompiledQuery) -> Result<u64, Self::Error> {
        let inner = self.inner.clone();
        let query = query.clone();
        block_on_data_service(async move { inner.execute(&query).await })
    }

}

fn block_on_data_service<F, T>(future: F) -> T
where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    if tokio::runtime::Handle::try_current().is_ok() {
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("data service runtime")
                .block_on(future)
        })
        .join()
        .expect("data service runtime thread")
    } else {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("data service runtime")
            .block_on(future)
    }
}

pub async fn service_runtime_from_env() -> Result<ServiceRuntime, ServiceRuntimeError> {
    service_runtime(ServiceRuntimeConfig::from_env()?).await
}

pub async fn service_runtime(config: ServiceRuntimeConfig) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let pool = connect_data_service_pool(&config).await?;
    service_runtime_from_pool(pool).await
}

pub async fn service_runtime_from_pool(pool: DataServicePool) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let id_generator = DataServiceIdGenerator::new(pool.clone());
    let mutation_executor = DataServiceMutationExecutor::new(pool);
    let runtime_executor = ServiceRuntimeExecutor::new(mutation_executor.clone());
    let mut context = module_with_behaviors_and_checkers().into_context();
    context.set_internal_id_generator(id_generator);
    context.use_postgres_provider(mutation_executor);
    context.insert_resource(runtime_executor);
    context.ensure_schema().await?;
    Ok(context)
}



fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
    std::env::var(name).map_err(|source| ServiceRuntimeError::MissingEnv { name, source })
}

async fn connect_data_service_pool(config: &ServiceRuntimeConfig) -> Result<DataServicePool, ServiceRuntimeError> {
    use std::str::FromStr as _;

    let options = sqlx::postgres::PgConnectOptions::from_str(&config.database_url)?
        .username(&config.database_user)
        .password(&config.database_password);
    Ok(DataServicePool::connect_with(options).await?)
}
pub fn repository_registry() -> teaql_runtime::InMemoryRepositoryRegistry {
    teaql_runtime::InMemoryRepositoryRegistry::new()
        .with_entity("Object")
        .with_entity("Attribute")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryRepositoryBehaviorRegistry {
    teaql_runtime::InMemoryRepositoryBehaviorRegistry::new()
        .with_behavior("Object", ObjectBehavior::default())
        .with_behavior("Attribute", AttributeBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Object, _>::new(ObjectChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Attribute, _>::new(AttributeChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Object>()
        .entity::<Attribute>()
        .initial_graph(teaql_runtime::GraphNode::new("Object")
            .value("id", 1_u64)
            .value("name", "task")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Attribute")
            .value("id", 1_u64)
            .value("name", "title")
            .value("type", "string")
            .value("max", 255_i32)
            .value("version", 1_i64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Object>()
        .checker(teaql_runtime::TypedEntityChecker::<Object, _>::new(ObjectChecker::default()))
        .entity::<Attribute>()
        .checker(teaql_runtime::TypedEntityChecker::<Attribute, _>::new(AttributeChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Object")
            .value("id", 1_u64)
            .value("name", "task")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Attribute")
            .value("id", 1_u64)
            .value("name", "title")
            .value("type", "string")
            .value("max", 255_i32)
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Object, _>(ObjectBehavior::default())
        .entity_with_behavior::<Attribute, _>(AttributeBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Object")
            .value("id", 1_u64)
            .value("name", "task")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Attribute")
            .value("id", 1_u64)
            .value("name", "title")
            .value("type", "string")
            .value("max", 255_i32)
            .value("version", 1_i64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Object, _>(ObjectBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Object, _>::new(ObjectChecker::default()))
        .entity_with_behavior::<Attribute, _>(AttributeBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Attribute, _>::new(AttributeChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Object")
            .value("id", 1_u64)
            .value("name", "task")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("Attribute")
            .value("id", 1_u64)
            .value("name", "title")
            .value("type", "string")
            .value("max", 255_i32)
            .value("version", 1_i64))
}