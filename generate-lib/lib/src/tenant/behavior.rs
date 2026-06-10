use teaql_runtime::RepositoryBehavior;

#[derive(Clone, Debug, Default)]
pub struct TenantBehavior;

impl RepositoryBehavior for TenantBehavior {}