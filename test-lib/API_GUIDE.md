# API Guide — `robot-kanban-service`

> Domain-specific query, mutation, and expression APIs for this project.
> For framework-level runtime APIs (UserContext, SmartList, etc.), see `TOOL_API.md`.

---

## AI Agent Notice: Runtime Logging & Trace

TeaQL Runtime provides rich, automated trace logs for all SQL executions and state mutations. AI Agents are **STRONGLY ENCOURAGED** to evaluate and observe the system through these logs instead of inspecting the raw database directly.

You can configure the logging output via environment variables when running the target binary or tests:
- `TEAQL_LOG_ENDPOINT`: Set to a file path (e.g., `eval.log`) to redirect logs to a specific file. If unset, it prints to standard output.
- `TEAQL_LOG_FORMAT`: Set to `json` (or `debug`) for structured machine-readable log entries, or `human` (default) for beautifully formatted trace chains.

Example for automated evaluation:
\`\`\`bash
TEAQL_LOG_ENDPOINT=eval.log TEAQL_LOG_FORMAT=json cargo run --bin my_app
\`\`\`

---

## Part 1 — API Rules

### 1. Query Chain

Every query follows the same shape:

```rust
use robot_kanban_service::{Q, E};

let results = Q::<entity_plural>()            // 1. entry point
    .with_name_contains("search")              // 2. filters
    .order_by_create_time_desc()               // 3. ordering
    .page(1, 20)                               // 4. pagination
    .comment("List active items")              // 5. intent comment
    .purpose("Load dashboard data")            // 6. purpose (unlocks execute)
    .execute_for_list(&ctx).await?;            // 7. execute
```

**Execution methods** (available on `PurposedQuery`):

| Method | Returns |
|--------|---------|
| `.execute_for_list(&ctx).await?` | `SmartList<Entity>` — paginated list |
| `.execute_for_first(&ctx).await?` | `Option<Entity>` — first match |
| `.execute_for_one(&ctx).await?` | `Option<Entity>` — single match |
| `.execute_for_count(&ctx).await?` | `u64` — total count |
| `.execute_for_exists(&ctx).await?` | `bool` — existence check |
| `.execute_for_page(&ctx, offset, limit).await?` | `SmartList<Entity>` with `total_count` |
| `.execute_for_records(&ctx).await?` | `SmartList<Record>` — raw records |

**Pagination helpers:**

| Method | Meaning |
|--------|---------|
| `.page(page_number, page_size)` | 1-based page number |
| `.page_offset(offset, limit)` | 0-based offset |
| `.top(n)` | Limit to first N results |
| `.unlimited()` | Remove default 200-row limit |

### 2. Filter Operators

Filters use a **human/thing** naming convention derived from each entity.

- **Things** — filter prefix is `with_`: `with_name_is("X")`, `with_name_contains("X")`
- **Humans** — filter prefix is `whose_`: `whose_name_is("X")`, `whose_name_containing("X")`

The suffix also changes for string operations:

| Entity type | Prefix | Verb suffix | Example |
|-------------|--------|-------------|---------|
| Thing | `with_` | `_contains` / `_starts_with` | `with_title_contains("rust")` |
| Human | `whose_` | `_containing` / `_starting_with` | `whose_name_containing("alice")` |

**Available filter methods per field:**

| Method pattern | SQL equivalent |
|----------------|----------------|
| `<prefix>_<field>_is(value)` | `field = value` |
| `<prefix>_<field>_is_not(value)` | `field != value` |
| `<prefix>_<field>_greater_than(value)` | `field > value` |
| `<prefix>_<field>_less_than(value)` | `field < value` |
| `<prefix>_<field>_between(lo, hi)` | `field BETWEEN lo AND hi` |
| `<prefix>_<field>_between_range(DateRange)` | Time range filter |
| `<prefix>_<field>_in([...])` | `field IN (...)` |
| `<prefix>_<field>_not_in([...])` | `field NOT IN (...)` |
| `<prefix>_<field>_contain<suffix>(s)` | `field LIKE '%s%'` |
| `<prefix>_<field>_start<suffix>_with(s)` | `field LIKE 's%'` |
| `<prefix>_<field>_end<suffix>_with(s)` | `field LIKE '%s'` |
| `<prefix>_<field>_is_unknown()` | `field IS NULL` |
| `<prefix>_<field>_is_known()` | `field IS NOT NULL` |
| `<prefix>_<field>_before(value)` | `field < value` (temporal alias) |
| `<prefix>_<field>_after(value)` | `field > value` (temporal alias) |

**Ordering methods:**

| Method | SQL |
|--------|-----|
| `.order_by_<field>_asc()` | `ORDER BY field ASC` |
| `.order_by_<field>_desc()` | `ORDER BY field DESC` |

### 3. Entity Field Methods

For each scalar field `p` on an entity, the generated struct provides:

```rust
entity.p()                  // read current value
entity.update_p(value)      // stage an update (returns &mut Self)
entity.changed_p()          // check if p was changed (returns Option<Value>)
```

For object-relation fields:

```rust
entity.relation()           // Option<&RelatedEntity>
```

For reverse-relation (child) collections:

```rust
entity.children_list()      // &Vec<ChildEntity> or &SmartList<ChildEntity>
entity.children_list_mut()  // &mut Vec<ChildEntity>
```

### 4. Relation Methods on Queries

**Load a relation** (eager-load the related entity):

```rust
Q::<entity_plural>()
    .select_<relation>()                           // default sub-select
    .select_<relation>_with(Q::<related>()...)    // custom sub-query
```

**Filter by relation** (EXISTS / IN subquery):

```rust
Q::<entity_plural>()
    .with_<relation>_matching(Q::<related>()...)      // keep matching
    .without_<relation>_matching(Q::<related>()...)   // exclude matching
    .have_<relation_plural>()                           // has any children
    .have_no_<relation_plural>()                        // has no children
```

### 5. Constant Status Shortcuts

When a field references a **constant entity** (e.g. `Status`, `Type`), the code generator creates shortcut methods:

```rust
// Query filters (on the request)
Q::<entities>()
    .<prefix>_is_<code>()           // e.g. .with_status_is_active()
    .<prefix>_is_not_<code>()       // e.g. .with_status_is_not_active()

// Entity mutations (on the struct)
entity.update_<relation>_to_<code>()   // e.g. entity.update_status_to_active()
entity.<relation>_is_<code>()          // e.g. entity.status_is_active() -> bool
```

### 6. Mutation Patterns

**Create:**

```rust
let mut entity = Q::<entities>().purpose("Create example entity").new_entity(&ctx);
entity.update_name("Example");
entity.update_status_to_active();
let saved = entity.audit_as("Create new item").save(&ctx).await?;
```

**Update:**

```rust
let mut entity = Q::<entities>()
    .filter_by_id(id)
    .comment("Load for update")
    .purpose("Update item name")
    .execute_for_one(&ctx).await?
    .expect("entity not found");

entity.update_name("New Name");
entity.audit_as("Rename item").save(&ctx).await?;
```

**Delete:**

```rust
entity.mark_as_delete();
entity.audit_as("Remove obsolete item").save(&ctx).await?;
```

**Graph save** — a single `save()` persists the entity and all attached child entities:

```rust
let mut parent = Q::<parents>().purpose("Create parent entity").new_entity(&ctx);
parent.update_name("Parent");

let mut child = Q::<children>().purpose("Create child entity").new_entity(&ctx);
child.update_title("Child");
parent.children_list_mut().push(child);

parent.audit_as("Create parent with child").save(&ctx).await?;
```

### 7. Expression Facade (`E`)

`E` provides a safe, chainable way to extract values from loaded entities:

```rust
let entity = Q::<entities>()
    .filter_by_id(id)
    .select_<relation>()
    .comment("Load with relation")
    .purpose("Extract relation field")
    .execute_for_one(&ctx).await?
    .expect("not found");

let value = E::<entity_module>(entity)
    .get_<field>()
    .eval();
```

### 8. Aggregation

```rust
let records = Q::<entities>()
    .group_by_<field>()
    .aggregate_count("count")
    .aggregate_sum("<field>", "total")
    .comment("Aggregate report")
    .purpose("Dashboard stats")
    .execute_for_records(&ctx).await?;
```

You can also group by a field and attach subqueries for advanced aggregation:

```rust
let records = Q::<entities>()
    .group_by_<field>_with(Q::<entities>().aggregate_count("count"))
    .execute_for_records(&ctx).await?;
```

For multifaceted metrics where you need multiple distinct groupings or conditions in a single query:

```rust
let records = Q::<entities>()
    .facet_by_<field>_as("stats", Q::<entities>().aggregate_count("total"))
    .execute_for_records(&ctx).await?;

// Extract the faceted results
let stats = records.facet("stats");
```

---

## Part 2 — Domain Entity Graph

### `Platform`

| Attribute | Value |
|-----------|-------|
| Module | `platform` |
| Query entry | `Q::platforms()` |
| Minimal query | `Q::platforms_minimal()` |
| With-children query | `Q::platforms_with_children()` |
| Expression | `E::platform(value)` |
| Graph save | `platform.audit_as("comment").save(&ctx).await` |
| New entity | `Q::platforms().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `founded`: `chrono::DateTime<chrono::Utc>` — read: `.founded()`, update: `.update_founded(value)`, changed: `.changed_founded()`
- `user_email`: `String` — read: `.user_email()`, update: `.update_user_email(value)`, changed: `.changed_user_email()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Children (one-to-many):**

- `task_status_list` → `TaskStatus` — load: `.select_task_status_list()`, filter: `.with_task_status_list_matching(Q::...)`
- `task_list` → `Task` — load: `.select_task_list()`, filter: `.with_task_list_matching(Q::...)`



### `TaskStatus`

| Attribute | Value |
|-----------|-------|
| Module | `task_status` |
| Query entry | `Q::task_status()` |
| Minimal query | `Q::task_status_minimal()` |
| With-children query | `Q::task_status_with_children()` |
| Expression | `E::task_status(value)` |
| Graph save | `task_status.audit_as("comment").save(&ctx).await` |
| New entity | `Q::task_status().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `code`: `String` — read: `.code()`, update: `.update_code(value)`, changed: `.changed_code()`
- `color`: `String` — read: `.color()`, update: `.update_color(value)`, changed: `.changed_color()`
- `display_order`: `rust_decimal::Decimal` — read: `.display_order()`, update: `.update_display_order(value)`, changed: `.changed_display_order()`
- `progress`: `rust_decimal::Decimal` — read: `.progress()`, update: `.update_progress(value)`, changed: `.changed_progress()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `platform` → `Platform` — load: `.select_platform()`, filter: `.with_platform_matching(Q::...)`

**Children (one-to-many):**

- `task_list` → `Task` — load: `.select_task_list()`, filter: `.with_task_list_matching(Q::...)`



### `Task`

| Attribute | Value |
|-----------|-------|
| Module | `task` |
| Query entry | `Q::tasks()` |
| Minimal query | `Q::tasks_minimal()` |
| With-children query | `Q::tasks_with_children()` |
| Expression | `E::task(value)` |
| Graph save | `task.audit_as("comment").save(&ctx).await` |
| New entity | `Q::tasks().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `status` → `TaskStatus` — load: `.select_status()`, filter: `.with_status_matching(Q::...)`
- `platform` → `Platform` — load: `.select_platform()`, filter: `.with_platform_matching(Q::...)`

**Children (one-to-many):**

- `task_execution_log_list` → `TaskExecutionLog` — load: `.select_task_execution_log_list()`, filter: `.with_task_execution_log_list_matching(Q::...)`


**Constant values for `status` (`TaskStatus`):**

- **Planned** — filter: `._is_planned()` / entity: `.update__to_planned()`, check: `._is_planned()`
- **Ready** — filter: `._is_ready()` / entity: `.update__to_ready()`, check: `._is_ready()`
- **Executing** — filter: `._is_executing()` / entity: `.update__to_executing()`, check: `._is_executing()`
- **Verified** — filter: `._is_verified()` / entity: `.update__to_verified()`, check: `._is_verified()`

### `TaskExecutionLog`

| Attribute | Value |
|-----------|-------|
| Module | `task_execution_log` |
| Query entry | `Q::task_execution_logs()` |
| Minimal query | `Q::task_execution_logs_minimal()` |
| With-children query | `Q::task_execution_logs_with_children()` |
| Expression | `E::task_execution_log(value)` |
| Graph save | `task_execution_log.audit_as("comment").save(&ctx).await` |
| New entity | `Q::task_execution_logs().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `action`: `String` — read: `.action()`, update: `.update_action(value)`, changed: `.changed_action()`
- `detail`: `String` — read: `.detail()`, update: `.update_detail(value)`, changed: `.changed_detail()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `task` → `Task` — load: `.select_task()`, filter: `.with_task_matching(Q::...)`

