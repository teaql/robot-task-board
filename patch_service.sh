sed -i 's/use teaql_provider_sqlite::{/use teaql_provider_postgres::{/' src/service.rs
sed -i 's/ensure_sqlite_schema_for/ensure_postgres_schema_for/' src/service.rs
sed -i 's/SqliteIdSpaceGenerator/PgIdSpaceGenerator/' src/service.rs
sed -i 's/SqliteMutationExecutor/PgMutationExecutor/' src/service.rs
sed -i 's/SqliteProviderExt/PostgresProviderExt/' src/service.rs
sed -i 's/ctx.use_sqlite_provider/ctx.use_postgres_provider/' src/service.rs
