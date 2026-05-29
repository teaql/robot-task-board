# Robot Task Board TUI 🚀

A terminal-based Kanban board for robot task management, built to showcase the **TeaQL query framework** in a real-world Rust application.

Written in Rust using **Ratatui + Crossterm**, backed by SQLite via `teaql-provider-rusqlite`. Cross-compiles as a standalone statically-linked binary for `armv7` router environments — zero external runtime dependencies.

---

## 📁 Project Structure

```
robot-task-board/
├── src/
│   ├── main.rs          # App state, command parsing, event loop
│   ├── service.rs       # TaskService, DomainTask aggregate root, TeaQL queries
│   ├── ui.rs            # Ratatui layout, syntax-highlighted log rendering
│   └── utils.rs         # System info (CPU/memory) from /proc
├── models/
│   └── model.xml        # TeaQL domain model definition
├── generate-lib/
│   └── lib/             # Auto-generated TeaQL domain library (robot-kanban)
├── Cargo.toml
└── README.md
```

---

## 🔬 TeaQL Applied Scenarios

This application exercises **8 distinct TeaQL capabilities** across its CRUD and query workflows. Each scenario maps a TeaQL API to its concrete usage and the SQL it produces.

---

### Scenario 1: Schema Bootstrap (`ensure_rusqlite_schema_for`)

**What it does:** Automatically creates or migrates all database tables and seeds initial reference data (status values, platform) from the domain model — zero manual SQL.

```rust
// service.rs — One-line schema setup
let mut ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
ctx.use_rusqlite_provider(inner_executor.clone());
ensure_rusqlite_schema_for(&ctx)?;
```

**Applied in:** `TaskService::new()` — on first run, creates `task_data`, `task_status_data`, and `platform_data` tables with seed data; on subsequent runs, applies any schema changes from the model.

---

### Scenario 2: JSON-Based Dynamic Filtering (`filter_with_json`)

**What it does:** Accepts a JSON object to dynamically construct WHERE clauses at runtime. An empty `{}` acts as a wildcard (no filter), enabling a single code path for both search and full-load.

```rust
// service.rs — Unified search/load query
let search_json = if let Some(ref term) = search_term {
    let escaped_name = serde_json::Value::String(term.clone());
    format!(r#"{{"name": {}}}"#, escaped_name)   // → {"name": "calibrate"}
} else {
    r#"{}"#.to_owned()                            // → {} (wildcard)
};

let select = Q::tasks()
    .filter_with_json(&search_json);
```

| Input | JSON | Generated SQL |
|:---|:---|:---|
| No search | `{}` | `SELECT ... FROM task_data WHERE (version > 0)` |
| `calibrate` | `{"name": "calibrate"}` | `SELECT ... FROM task_data WHERE (version > 0) AND (name LIKE '%calibrate%')` |

**Applied in:** `search` / `s` command — filters the Kanban board in real-time.

---

### Scenario 3: Faceted Aggregation (`facet_by_status_as`)

**What it does:** Attaches a sub-query that computes aggregate counts grouped by a relation (status), all within a single database round-trip alongside the main entity query.

```rust
// service.rs — Single query fetches tasks + status counts
let select = Q::tasks()
    .comment(search_comment)
    .filter_with_json(&search_json)
    .facet_by_status_as("status_stats",
        Q::task_status().comment("Count status").count_tasks()
    );

let all_tasks = select.execute_for_list(&self.ctx).await?;

// Access facet results from the same SmartList
if let Some(facet_list) = all_tasks.facet("status_stats") {
    for record in facet_list.iter() {
        let status_id = record.get("id");
        let count = record.get("count_tasks");
    }
}
```

**Generated SQL (3 queries in one round-trip):**

```sql
-- 1. Main entity query
SELECT id, name, version, status AS status_id, platform AS platform_id
  FROM task_data WHERE (version > 0)
-- 2. Facet: load status reference data
SELECT id, name, code, version, platform AS platform_id
  FROM task_status_data WHERE (version > 0)
-- 3. Facet: aggregate task counts per status
SELECT status, COUNT(*) AS count_tasks
  FROM task_data WHERE (version > 0) AND (status IN (1, 2, 3)) GROUP BY status
```

**Applied in:** Board reload — the Planned/Process/Done count badges and task lists are all populated from this single query.

---

### Scenario 4: Entity Factory (`Q::tasks().new_entity()`)

**What it does:** Creates a new entity instance pre-wired with the runtime context, ready for field population and persistence.

```rust
// service.rs — DomainTask::create()
let mut task = Q::tasks().new_entity(ctx);
task.update_id(next_id)
    .update_name(cmd.name.clone())
    .update_version(1_i64)
    .update_status_id(1_u64)
    .update_platform_id(1_u64);
```

**Generated SQL:**

```sql
INSERT INTO task_data (id, name, version, status, platform)
  VALUES (1, 'calibrate sensor', 1, 1, 1)
```

**Applied in:** `add` command — creates a new task in Planned status.

---

### Scenario 5: ID Space Generation (`RusqliteIdSpaceGenerator`)

**What it does:** Generates globally unique, monotonically increasing IDs per entity type using a dedicated SQLite sequence table — no auto-increment column needed.

```rust
// service.rs — add_task()
let id_gen = RusqliteIdSpaceGenerator::from_executor(self.inner_executor.clone());
let next_id = id_gen.next_id("Task")?;
```

**Applied in:** `add` command — each new task receives a unique ID from the `Task` ID space.

---

### Scenario 6: Custom Return Type Mapping (`return_type::<DomainTask>()`)

**What it does:** Tells TeaQL to deserialize query results into a custom domain type instead of the default generated entity. The custom type must implement the `Entity` and `TeaqlEntity` traits.

```rust
// service.rs — DomainTask wraps the generated Task with business logic
pub struct DomainTask {
    pub task: Task,
}

impl Entity for DomainTask {
    fn from_record(record: Record) -> Result<Self, EntityError> {
        let task = Task::from_record(record)?;
        Ok(Self { task })
    }
}

// Fetch as DomainTask instead of raw Task
let select = Q::tasks()
    .comment("Get task for DDD")
    .with_id_is(id)
    .return_type::<DomainTask>();

let found_tasks = select.execute_for_list(&self.ctx).await?;
// found_tasks contains DomainTask instances with business methods attached
```

**Applied in:** `move` / `mv` and `delete` / `del` commands — the fetched `DomainTask` carries `transition_status()` and `delete()` domain methods that raw `Task` entities don't have.

---

### Scenario 7: Optimistic Concurrency (`DeleteCommand` with `expected_version`)

**What it does:** Deletes an entity only if its current version matches the expected version, preventing concurrent modification conflicts.

```rust
// service.rs — delete_task()
let repo = self.ctx.task_repository()?;
repo.delete(
    &DeleteCommand::new("Task", id)
        .expected_version(domain_task.task.version())
)?;
```

**Generated SQL:**

```sql
UPDATE task_data SET version = -2 WHERE id = 1 AND version = 1
```

TeaQL uses a soft-delete pattern — `version` is set to a negative value rather than removing the row, preserving audit history.

**Applied in:** `delete` / `del` command.

---

### Scenario 8: Comment Chain Propagation (`.comment()`)

**What it does:** Attaches human-readable intent annotations to queries. When queries have nested sub-queries (e.g., facets), comments propagate down the chain with `->` separators, creating a full trace of query intent.

```rust
// service.rs — Comments propagate through facet sub-queries
let select = Q::tasks()
    .comment("Get active tasks")                       // Parent comment
    .filter_with_json(&search_json)
    .facet_by_status_as("status_stats",
        Q::task_status().comment("Count status")       // Child comment
            .count_tasks()
    );
```

**Resulting log trace chain:**

```text
[Get active tasks]                                → main task query
[Get active tasks->status_stats->Count status]    → facet status lookup
[Get active tasks->status_stats->Count status]    → facet aggregate count
```

The TUI renders these traces in real-time with syntax-highlighted colors — timestamp, user context (`[philip]`), comment chains, result summaries, and elapsed times are each distinctly colored:

```text
[12:06:00.225]-[philip]-[0.184ms]-[DEBUG]-SqlLogEntry - [Get active tasks] - [5*Task] SELECT ... 
[12:06:00.226]-[philip]-[0.138ms]-[DEBUG]-SqlLogEntry - [Get active tasks->status_stats->Count status] - [3*TaskStatus] SELECT ... 
```

**Applied in:** Every query in the application — enables real-time SQL auditing from the TUI log panel.

---

### Scenario Summary

| # | TeaQL API | App Feature | Command |
|:---|:---|:---|:---|
| 1 | `ensure_rusqlite_schema_for` | Auto-create tables & seed data | Startup |
| 2 | `filter_with_json` | Dynamic search / wildcard load | `search` |
| 3 | `facet_by_status_as` | Status count aggregation | Board reload |
| 4 | `Q::tasks().new_entity()` | Create task with defaults | `add` |
| 5 | `RusqliteIdSpaceGenerator` | Unique ID generation | `add` |
| 6 | `.return_type::<DomainTask>()` | Custom domain type mapping | `move`, `delete` |
| 7 | `DeleteCommand.expected_version()` | Optimistic concurrency delete | `delete` |
| 8 | `.comment()` | Query intent tracing | All queries |

---

## 📐 Architecture

### 3-Layer Separation

| Layer | File | Responsibility |
|:---|:---|:---|
| **UI** | `ui.rs` | Ratatui layout, log syntax highlighting, cursor management |
| **Application** | `main.rs` | App state, command parsing, event loop |
| **Service/Domain** | `service.rs` | TeaQL queries, DDD aggregate root, domain logic |

`main.rs` has no direct dependency on TeaQL types — it only interacts with `TaskService`, `TaskModel`, and `MoveResult`.

### DDD Aggregate Root

Generated `Task` entities act strictly as data transfer records. Rich business logic is encapsulated within the `DomainTask` aggregate root:

- **`DomainTask::create()`** — factory method with validation
- **`DomainTask::transition_status()`** — automatic next-status resolution (Planned → Process → Done)
- **`DomainTask::delete()`** — deletion validation hook

### Domain Model

Defined in `models/model.xml`, the TeaQL domain model declares two entities with a status relation:

```xml
<task_status
    name="Planned|Process|Done"
    code="PLANNED|PROCESS|DONE"
    _features="status"
    _identified_by="code" />

<task
    name="Task Name|[1,200]"
    status="task_status()"
    _features="custom" />
```

---

## 🛠 Commands

Commands use a slash (`/`) prefix. **Any bare text (without a slash) is treated as a quick-add for a new task.**

| Command | Shortcut | Description | Example |
|:---|:---|:---|:---|
| `<name>` / `/add <name>` | — | Create a new task in Planned status | `calibrate sensor` or `/add calibrate` |
| `/move <id> [status]` | `/mv` | Transition task status (planned/process/done; default: next) | `/move 3` or `/mv 3 done` |
| `/search <keyword>` | `/s` | Filter tasks by keyword (empty to clear) | `/search calibrate` or `/s` |
| `/delete <id>` | `/del` | Permanently delete a task | `/delete 3` |
| `/exit` / `/quit` | `/q` | Quit the application | `/exit` |
| — | `ESC` | Immediate exit | — |
| — | `Up/Dn` | Scroll Action Logs viewport | — |

---

## ⚙️ Prerequisites

- **Rust toolchain** (1.70+)
- **TeaQL workspace packages** — the following crates are expected at `/home/philip/teaql-home/teaql-rs/`:
  - `teaql-core`, `teaql-runtime`, `teaql-macros`, `teaql-sql`, `teaql-provider-rusqlite`
- **For cross-compilation**: `cargo-zigbuild` and the `armv7-unknown-linux-musleabihf` target

> **Note:** `Cargo.toml` uses absolute paths to the TeaQL workspace. Adjust these paths if building on a different machine.

---

## 🚀 Build & Run

### Local Development

```bash
# Check compilation
cargo check

# Run the TUI
cargo run

# Run the TUI in compact mode (hides the SQL log area)
cargo run -- -c

# Build optimized release binary
cargo build --release
```

### ARMv7 Cross-Compilation

```bash
# Static cross-compile for armv7 routers
cargo zigbuild --release --target armv7-unknown-linux-musleabihf
```

The output binary is at `target/armv7-unknown-linux-musleabihf/release/robot-task-board` — upload directly to a router and run with zero dependencies.

### Running Tests

```bash
cargo test
```

Tests cover:
- **Comment propagation** — verifies TeaQL comment chains propagate through facet sub-queries
- **CRUD lifecycle** — add → reload → verify → delete → verify
- **DDD transitions** — Planned → Process → Done with automatic and explicit status moves
