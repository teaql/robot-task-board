import re

with open('src/bin/audit_example.rs', 'r') as f:
    content = f.read()

impl_code = """
impl robot_kanban::request_support::TeaqlRepositoryProvider for teaql_runtime::UserContext {
    type PlatformRepository<'a> = teaql_runtime::ResolvedRepository<'a, teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor> where Self: 'a;
    fn platform_repository(&self) -> Result<Self::PlatformRepository<'_>, teaql_runtime::ContextError> {
        self.resolve_repository::<teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor>("Platform")
    }
    type TaskStatusRepository<'a> = teaql_runtime::ResolvedRepository<'a, teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor> where Self: 'a;
    fn task_status_repository(&self) -> Result<Self::TaskStatusRepository<'_>, teaql_runtime::ContextError> {
        self.resolve_repository::<teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor>("TaskStatus")
    }
    type TaskRepository<'a> = teaql_runtime::ResolvedRepository<'a, teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor> where Self: 'a;
    fn task_repository(&self) -> Result<Self::TaskRepository<'_>, teaql_runtime::ContextError> {
        self.resolve_repository::<teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor>("Task")
    }
    type TaskExecutionLogRepository<'a> = teaql_runtime::ResolvedRepository<'a, teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor> where Self: 'a;
    fn task_execution_log_repository(&self) -> Result<Self::TaskExecutionLogRepository<'_>, teaql_runtime::ContextError> {
        self.resolve_repository::<teaql_provider_rusqlite::RusqliteDialect, teaql_provider_rusqlite::RusqliteMutationExecutor>("TaskExecutionLog")
    }
}
"""

content = content.replace("fn main()", impl_code + "\nfn main()")
content = content.replace("let service_runtime_executor = robot_kanban::ServiceRuntimeExecutor::new(executor.clone());", "")
content = content.replace("ctx.insert_resource(service_runtime_executor);", "")

with open('src/bin/audit_example.rs', 'w') as f:
    f.write(content)
