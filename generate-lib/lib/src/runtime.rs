use crate::*;

use teaql_provider_rusqlite::RusqliteProviderExt as _;

pub type DataServiceDialect = teaql_provider_rusqlite::RusqliteDialect;
pub type DataServiceMutationExecutor = teaql_provider_rusqlite::RusqliteMutationExecutor;
pub type DataServiceMutationError = teaql_provider_rusqlite::MutationExecutorError;
pub type DataServiceIdGenerator = teaql_provider_rusqlite::RusqliteIdSpaceGenerator;
pub type DataServicePool = rusqlite::Connection;
pub type DataServiceExecutor = ServiceRuntimeExecutor;
pub type ServiceRuntime = teaql_runtime::UserContext;

pub const DATABASE_URL_ENV: &str = "ROBOT_KANBAN_SERVICE_DATABASE_URL";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRuntimeConfig {
    pub database_url: String,
}

impl ServiceRuntimeConfig {
    pub fn from_env() -> Result<Self, ServiceRuntimeError> {
        Ok(Self {
            database_url: env_value(DATABASE_URL_ENV)?,
        })
    }
}

#[derive(Debug)]
pub enum ServiceRuntimeError {
    MissingEnv {
        name: &'static str,
        source: std::env::VarError,
    },
    Rusqlite(rusqlite::Error),
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::MissingEnv { name, source } => {
                write!(f, "missing environment variable {name}: {source}")
            }
            ServiceRuntimeError::Rusqlite(err) => write!(f, "rusqlite error: {err}"),
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::MissingEnv { source, .. } => Some(source),
            ServiceRuntimeError::Rusqlite(err) => Some(err),
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<rusqlite::Error> for ServiceRuntimeError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceRuntimeError::Rusqlite(err)
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
        inner.fetch_all(&query)
    }

    fn execute(&self, query: &teaql_sql::CompiledQuery) -> Result<u64, Self::Error> {
        let inner = self.inner.clone();
        let query = query.clone();
        inner.execute(&query)
    }

    fn begin_transaction(&self) -> Result<teaql_runtime::GraphTransactionBoundary, Self::Error> {
        let inner = self.inner.clone();
        teaql_runtime::QueryExecutor::begin_transaction(&self.inner)?;
        Ok(teaql_runtime::GraphTransactionBoundary::Started)
    }

    fn commit_transaction(&self) -> Result<(), Self::Error> {
        let inner = self.inner.clone();
        teaql_runtime::QueryExecutor::commit_transaction(&self.inner)
    }

    fn rollback_transaction(&self) -> Result<(), Self::Error> {
        let inner = self.inner.clone();
        teaql_runtime::QueryExecutor::rollback_transaction(&self.inner)
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
    let mutation_executor = DataServiceMutationExecutor::new(pool);
    let id_generator = DataServiceIdGenerator::from_executor(mutation_executor.clone());let runtime_executor = ServiceRuntimeExecutor::new(mutation_executor.clone());
    let mut context = module_with_behaviors_and_checkers().into_context();
    context.set_internal_id_generator(id_generator);
    context.use_rusqlite_provider(mutation_executor);
    context.insert_resource(runtime_executor);
    context.ensure_schema().await?;
    Ok(context)
}



fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
    std::env::var(name).map_err(|source| ServiceRuntimeError::MissingEnv { name, source })
}

async fn connect_data_service_pool(config: &ServiceRuntimeConfig) -> Result<DataServicePool, ServiceRuntimeError> {
    use std::path::Path;

    let url = &config.database_url;
    let sanitized_url = if url.starts_with("sqlite:") {
        let raw_path = url.strip_prefix("sqlite:").unwrap();
        let (is_absolute, file_path_str) = if raw_path.starts_with("///") {
            (true, &raw_path[2..])
        } else if raw_path.starts_with("//") {
            (false, &raw_path[2..])
        } else if raw_path.starts_with("/") {
            (true, raw_path)
        } else {
            (false, raw_path)
        };
        let pure_file_path = file_path_str.split('?').next().unwrap_or(file_path_str);
        let path = Path::new(pure_file_path);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|err| teaql_runtime::RuntimeError::Graph(err.to_string()))?;
            }
        }
        if is_absolute {
            format!("sqlite://{}?mode=rwc", pure_file_path)
        } else {
            format!("sqlite:{}?mode=rwc", pure_file_path)
        }
    } else {
        url.clone()
    };
    let path_str = config.database_url.strip_prefix("sqlite://").or_else(|| config.database_url.strip_prefix("sqlite:")).unwrap_or(&config.database_url);
    Ok(DataServicePool::open(path_str)?)}

pub fn repository_registry() -> teaql_runtime::InMemoryRepositoryRegistry {
    teaql_runtime::InMemoryRepositoryRegistry::new()
        .with_entity("Platform")
        .with_entity("TaskStatus")
        .with_entity("Task")
        .with_entity("TaskExecutionLog")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryRepositoryBehaviorRegistry {
    teaql_runtime::InMemoryRepositoryBehaviorRegistry::new()
        .with_behavior("Platform", PlatformBehavior::default())
        .with_behavior("TaskStatus", TaskStatusBehavior::default())
        .with_behavior("Task", TaskBehavior::default())
        .with_behavior("TaskExecutionLog", TaskExecutionLogBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TaskStatus, _>::new(TaskStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Task, _>::new(TaskChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog, _>::new(TaskExecutionLogChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Platform>()
        .entity::<TaskStatus>()
        .entity::<Task>()
        .entity::<TaskExecutionLog>()
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Process")
            .value("code", "PROCESS")
            .value("color", "#F59E0B")
            .value("display_order", "20")
            .value("progress", "50")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Done")
            .value("code", "DONE")
            .value("color", "#16A34A")
            .value("display_order", "30")
            .value("progress", "100")
            .value("version", 1_i64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<Platform>()
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity::<TaskStatus>()
        .checker(teaql_runtime::TypedEntityChecker::<TaskStatus, _>::new(TaskStatusChecker::default()))
        .entity::<Task>()
        .checker(teaql_runtime::TypedEntityChecker::<Task, _>::new(TaskChecker::default()))
        .entity::<TaskExecutionLog>()
        .checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog, _>::new(TaskExecutionLogChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Process")
            .value("code", "PROCESS")
            .value("color", "#F59E0B")
            .value("display_order", "20")
            .value("progress", "50")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Done")
            .value("code", "DONE")
            .value("color", "#16A34A")
            .value("display_order", "30")
            .value("progress", "100")
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .entity_with_behavior::<TaskStatus, _>(TaskStatusBehavior::default())
        .entity_with_behavior::<Task, _>(TaskBehavior::default())
        .entity_with_behavior::<TaskExecutionLog, _>(TaskExecutionLogBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Process")
            .value("code", "PROCESS")
            .value("color", "#F59E0B")
            .value("display_order", "20")
            .value("progress", "50")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Done")
            .value("code", "DONE")
            .value("color", "#16A34A")
            .value("display_order", "30")
            .value("progress", "100")
            .value("version", 1_i64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<Platform, _>(PlatformBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Platform, _>::new(PlatformChecker::default()))
        .entity_with_behavior::<TaskStatus, _>(TaskStatusBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TaskStatus, _>::new(TaskStatusChecker::default()))
        .entity_with_behavior::<Task, _>(TaskBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Task, _>::new(TaskChecker::default()))
        .entity_with_behavior::<TaskExecutionLog, _>(TaskExecutionLogBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog, _>::new(TaskExecutionLogChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Platform")
            .value("id", 1_u64)
            .value("name", "Robot System")
            .value("founded", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1001_u64)
            .value("name", "Planned")
            .value("code", "PLANNED")
            .value("color", "#94A3B8")
            .value("display_order", "10")
            .value("progress", "0")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1002_u64)
            .value("name", "Process")
            .value("code", "PROCESS")
            .value("color", "#F59E0B")
            .value("display_order", "20")
            .value("progress", "50")
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("TaskStatus")
            .value("id", 1003_u64)
            .value("name", "Done")
            .value("code", "DONE")
            .value("color", "#16A34A")
            .value("display_order", "30")
            .value("progress", "100")
            .value("version", 1_i64))
}