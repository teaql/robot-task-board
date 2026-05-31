# TeaQL Coding Agent Instructions

This project uses the TeaQL-generated Rust crate `robot-kanban-service`.

## Core Rule

Always prefer TeaQL-generated APIs over handwritten data-access code.

Business code must stay on the supported surface: `Q` for reads, generated
entity APIs and `entity.save(&ctx).await` for state changes, `E` for safe
expression access, and project-specific extensions or wrappers around
`UserContext` for request context behavior. Accessing lower-level TeaQL runtime
interfaces directly is not allowed from business code.

Do not hand-write SQL, repository orchestration, relation loading, DTO mapping,
or graph persistence unless the user explicitly asks for a low-level escape
hatch.

## Generated Crate

Import the generated domain API from `robot-kanban-service`:

```rust
use robot_kanban_service::{E, Q};
```

The generated crate provides:

- entity structs for the domain objects
- query facade `Q`
- safe expression facade `E`
- generated request builders
- relation loading helpers
- graph save helpers, such as `entity.save(&ctx).await`
- runtime registration helpers:
  - `module()`
  - `module_with_behaviors()`
  - `repository_registry()`
  - `behavior_registry()`

## MANDATORY AUDIT RULE (Zero-cost Intent Logging)

Whenever you query or persist data, you MUST chain a comment explaining your business intent. This allows the system to build an automatic audit trail.
- For queries: chain `.comment("...")` before execution.
- For updates/saves: chain `.set_comment("...")` before saving.

## CRUD & Query Patterns

### 1. Querying (Read)
Use `Q` for reads. Always include a `.comment()` to explain the business context.

```rust
let rows = Q::platforms()
    .comment("Fetch platforms for processing")
    .select_self()
    .page(1, 20)
    .execute_for_list(&ctx)
    .await?;
```

Avoid direct `sqlx::query(...)` unless raw SQL is explicitly requested. Do NOT call generated repositories directly.

### 2. Creating (Create)
Use `Q::platforms().new_entity(&ctx)` to create a new entity with the correct root context, then use graph save:

```rust
let mut entity = Q::platforms().new_entity(&ctx);
// entity.update_name("example");
entity.set_comment("Created new Platform for user request")
      .save(&ctx).await?;
```

### 3. Updating (Update)
Fetch the graph node, use generated typed setters to modify fields, and append intent before saving:

```rust
if let Some(mut entity) = Q::platforms().with_id_is(id).execute_for_one(&ctx).await? {
    // entity.update_status(new_status)
    entity.set_comment("Updating status due to state transition")
          .save(&ctx).await?;
}
```

### 4. Audited Soft-Delete (Delete)
Do NOT call `repo.delete`. Use the elegant `mark_as_delete` method chained with `set_comment`:

```rust
if let Some(mut entity) = Q::platforms().with_id_is(id).execute_for_one(&ctx).await? {
    entity.mark_as_delete()
          .set_comment("Soft deleted Platform as requested")
          .save(&ctx).await?;
}
```

## Advanced TeaQL Paradigms

### Dynamic JSON Filtering
When building multi-condition UI filters, do NOT write complex `if-else` query builders. Use dynamic JSON filtering:


```rust
let items = Q::<platforms>()
    .comment("Search with dynamic UI filters")
    .filter_with_json(filter_json_value)
    .execute_for_list(&ctx).await?;
```

### Faceted Aggregation
For dashboard metrics and grouping, use generated facet methods to let the database handle aggregation:

```rust
let aggregations = Q::<platforms>()
    .comment("Aggregate data for dashboard")
    // .facet_by_status_as("status_stats")
    .execute_for_list(&ctx).await?;
```

### Partial Projections & DTOs
When only a few fields are needed, avoid loading the full entity graph. Project specific columns into a custom Rust struct (`return_type::<T>()`):

```rust
// let stats = Q::<platforms>()
//     .comment("Fetch lightweight specific fields")
//     .select_status()
//     .count_id_as("count")
//     .group_by_status()
//     .return_type::<StatusStatsDTO>()
//     .execute_for_list(&ctx).await?;
```

### Domain Behavior Injection
NEVER manually edit generated Entity files. Inject business logic by defining Rust Extension Traits in your application logic (`service.rs`):

```rust
pub trait <Platform>Ext {
    fn custom_business_logic(&mut self);
}

impl <Platform>Ext for <Platform> {
    fn custom_business_logic(&mut self) {
        // ...
    }
}
```

## Relation Loading

Use generated relation helpers. Avoid manual N+1 query loops.

```rust
Q::<entity_plural>()
    .select_<relation>_with(Q::<target_plural>().select_self())
```

## Standard Filtering

Use generated readable filters when available. Use direct `teaql_core::Expr` only when no generated helper exists.

```rust
Q::<entity_plural>()
    .which_<fields>_is(...)
    .with_<relation>_matching(Q::<target_plural>().select_self())
```

## Low-Level Warnings

Do not manually coordinate multiple repository insert/update calls unless the task explicitly requires low-level control.
Do not call `runtime_new(...)`, `entity_root()`, repositories, or runtime internals to create entities.

## Safe Value Access

Use `E` for long-chain value access:

```rust
let value = E::<entity>(entity)
    .get_<relation>()
    .get_<field>()
    .eval();
```

Do not write nested `unwrap()` chains for optional relations.

## Runtime Setup

Register generated metadata and repositories through the generated crate:

```rust
let ctx = teaql_runtime::UserContext::new()
    .with_module(robot_kanban_service::module())
    .with_repository_registry(robot_kanban_service::repository_registry())
    .with_repository_behavior_registry(robot_kanban_service::behavior_registry());
```

Use `module_with_behaviors()` when behavior hooks should be active.

## SQL Debugging

If query behavior is unclear, enable TeaQL SQL logs through `UserContext`:

```rust
ctx.enable_all_sql_log();
let logs = ctx.sql_logs();
```

Use the debug SQL to explain behavior or diagnose performance.

## Schema Changes

If a requested change requires new entities, fields, relations, constants, or
modules, update the KSML model first and regenerate the crate.

Do not manually edit generated entity/request/expression files. Treat generated
files as disposable.


## Domain Context

Read these files before making domain-specific changes:

- `docs/teaql-domain-map.md`
- `docs/teaql-query-examples.md`
- `docs/teaql-save-graph.md`
- `docs/teaql-sql-log.md`