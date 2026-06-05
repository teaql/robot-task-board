# TeaQL API Guide — robot-kanban-service

> This file is auto-generated from the domain model.
> AI agents: read this file FIRST. Do NOT grep source code to discover APIs.

## Query & Mutation Rules

### Query Chain (MANDATORY order)

```
Q::<entities>()                         // entry point
    .with_<property>_<operator>(value)  // zero or more filters
    .select_<relation>()               // zero or more relation loads
    .order_by_<property>_asc()         // optional ordering
    .comment("what this query loads")   // REQUIRED
    .purpose("why this data is needed") // REQUIRED
    .execute_for_list(&ctx).await?      // terminal
    // or: .execute_for_one(&ctx).await?
    // or: .execute_for_page(&ctx, page, size).await?
```

### Filter Operators (by property type)

| Type | Operators on `.with_<P>_*()` |
|------|------------------------------|
| **String** | `is`, `is_not`, `in`, `not_in`, `containing`, `not_containing`, `starting_with`, `ending_with`, `greater_than`, `less_than`, `between`, `is_unknown` |
| **Number** | `is`, `is_not`, `in`, `not_in`, `greater_than`, `greater_than_or_equal_to`, `less_than`, `less_than_or_equal_to`, `between` |
| **DateTime** | same as Number + `before`, `after` |
| **id** | `is`, `is_not`, `in`, `not_in` |

### Status Enum Shortcuts

When an entity has a status relation with predefined constants (e.g. TaskStatus),
the generator creates direct shortcuts:

```
.with_status_is_<CODE>()       // e.g. .with_status_is_executing()
.with_status_is_not_<CODE>()   // e.g. .with_status_is_not_planned()
```

### Relation Filters

```
.with_<relation>_matching(Q::<related_entities>().<filters>)
.select_<relation>()               // eager load parent/child
.select_<relation>_with(request)   // eager load with sub-filters
```

### Ordering

```
.order_by_<property>_asc()
.order_by_<property>_desc()
```

### New Entity

```
let mut entity = Q::<entities>().new_entity(&ctx);
entity.update_<property>(value);
entity.audit_as("reason").save(&ctx).await?;
```

### Update Entity

```
let mut task = Q::tasks()
    .with_id_is(42)
    .comment("Load task to update status")
    .purpose("Transition task to executing")
    .execute_for_one(&ctx).await?.expect("not found");

task.update_status_to_executing();  // status enum shortcut
task.audit_as("Robot arm picked up part, starting execution")
    .save(&ctx).await?;
```

### Soft Delete

```
task.mark_as_deleted();
task.audit_as("Remove cancelled task").save(&ctx).await?;
```

---

## Domain Graph

### Entity: Platform
- Query: `Q::platforms()`

| Property | Type |
|----------|------|
| id | u64 |
| create_time | DateTime |

Relations: (root entity, no parent)

---

### Entity: TaskStatus (constant enum)
- Query: `Q::task_status()`

| Property | Type |
|----------|------|
| id | u64 |
| name | String |
| code | String (identifier) |
| color | String |
| display_order | Number |
| progress | Number |

| Code | Name | id |
|------|------|----|
| PLANNED | Planned | 1001 |
| READY | Ready | 1002 |
| EXECUTING | Executing | 1003 |
| VERIFIED | Verified | 1004 |

---

### Entity: Task
- Query: `Q::tasks()`

| Property | Type |
|----------|------|
| id | u64 |
| name | String[1,200] |
| status | →TaskStatus |
| platform | →Platform |
| version | i64 |

Status shortcuts: `update_status_to_planned()`, `update_status_to_ready()`, `update_status_to_executing()`, `update_status_to_verified()`

Status filters: `with_status_is_planned()`, `with_status_is_ready()`, `with_status_is_executing()`, `with_status_is_verified()`

Children: task_execution_log_list (→TaskExecutionLog[])

---

### Entity: TaskExecutionLog
- Query: `Q::task_execution_logs()`

| Property | Type |
|----------|------|
| id | u64 |
| task | →Task |
| action | String |
| detail | String |
| version | i64 |

---

## Relation Map

```
Platform ←──platform── Task ──status──→ TaskStatus (const)
                         │
                         └──(parent)── TaskExecutionLog
```
