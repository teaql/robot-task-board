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

## Query Style

Use `Q` for reads.

Preferred:

```rust
let rows = Q::<entity_plural>()
    .select_self()
    .page(1, 20)
    .execute_for_list(&ctx)
    .await?;
```

Avoid direct `sqlx::query(...)` unless the user explicitly asks for raw SQL.

Do not call generated repositories, repository registries, metadata registries,
SQL executors, transaction internals, or other TeaQL runtime internals directly.
If `Q`, generated entity APIs, `E`, and `UserContext` extensions are not enough,
report the missing generated API instead of bypassing it.

## Relation Loading

Use generated relation helpers.

Preferred:

```rust
Q::<entity_plural>()
    .select_<relation>_with(Q::<target_plural>().select_self())
```

Avoid manual N+1 query loops.

## Filtering

Use generated readable filters when available:

```rust
Q::<entity_plural>()
    .which_<fields>_is(...)
    .with_<relation>_matching(Q::<target_plural>().select_self())
```

Use direct `teaql_core::Expr` only when no generated helper exists.

## Saving Data

Use `Q::platforms().new_entity(&ctx)` to create a new
entity with the correct TeaQL root context, then use graph save for
business-object persistence:

```rust
let mut entity = Q::platforms().new_entity(&ctx);
// entity.update_name("example");
entity.save(&ctx).await?;
```

Do not manually coordinate multiple repository insert/update calls unless the
task explicitly requires low-level control.
Do not call `runtime_new(...)`, `entity_root()`, repositories, or runtime
internals to create entities.

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

## Internationalization (i18n) Guidance

For multilingual support and translation of domain terms (e.g. validation error messages) when using non-English locales (such as Chinese, Spanish, etc.), TeaQL provides a clean, offline i18n translation dictionary workflow:

1. **Locate the Sample File**: Find the auto-generated `teaql-i18n.sample.json` file in the project root. It contains all unique domain entity and property vocabulary keys.
2. **Translate via AI**: Paste the contents of `teaql-i18n.sample.json` into your AI tool, or instruct this AI agent to translate it. Use the following prompt:
   > **Prompt**: "Translate this JSON file containing TeaQL domain vocabulary keys into 15 languages: Arabic (ar), Simplified Chinese (zh_CN), Traditional Chinese (zh_TW), Spanish (es), French (fr), German (de), Japanese (ja), Korean (ko), Portuguese (pt), Thai (th), Ukrainian (uk), Filipino (fil), Indonesian (id), English (en) under each key. Return only the valid JSON result."
3. **Save the JSON**: Save the translated JSON result as `teaql-i18n.json` externally or on the classpath.
4. **Configure JVM Parameter**: Set the JVM property `-Dteaql.i18n.path=/path/to/teaql-i18n.json` pointing to your translated file.
5. **Runtime Validation Rule**: If a non-English translator is instantiated at runtime and `-Dteaql.i18n.path` is unconfigured, invalid, or the file is empty, the starter throws a strict `IllegalStateException` to prevent unlocalized output.

## Domain Context

Read these files before making domain-specific changes:

- `docs/teaql-domain-map.md`
- `docs/teaql-query-examples.md`
- `docs/teaql-save-graph.md`
- `docs/teaql-sql-log.md`