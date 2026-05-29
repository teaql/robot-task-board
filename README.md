# Show HN: TeaQL Showcase – See What Your Business Code Actually Does

> **TeaQL Website**: [https://teaql.io/](https://teaql.io/)

TeaQL is a Rust query framework and domain modeling tool for making business logic observable.

This demo is a tiny terminal Kanban board, but it exercises the same runtime features TeaQL was built for in much larger business domains:

`Command` → `Domain transition` → `SQL` → `Audit diff` → `Event log` → `UI projection`

![TeaQL task board demo](./assets/teaql-task-board.gif)

Instead of hiding database behavior behind an opaque ORM, this showcase lets you follow the full execution path of a domain action: from command input, to domain transition, to generated SQL, to audit diff, to UI refresh.

The task board intentionally uses a tiny domain model so the runtime behavior is easy to follow. In real systems, TeaQL is meant for much larger business domains, where domain transitions, generated SQL, audit trails, query paths, and lifecycle events quickly become difficult to reason about without framework-level visibility.

To make the idea concrete, we built a terminal-based Kanban board using Ratatui + SQLite. When you move a task from *Planned* to *Process*, TeaQL shows the generated SQL, optimistic concurrency update, audit trail, lifecycle event, and refreshed status facets in real time.

This is not just a task board. It is a small showcase of how TeaQL bridges Domain-Driven Design, raw SQL transparency, query introspection, faceted aggregation, lifecycle auditing, and embedded-friendly Rust deployment.

The app can also cross-compile as a standalone statically linked binary for `armv7` router environments, with no external runtime dependencies.

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
│   └── model.xml        # TeaQL declarative domain model definition
├── generate-lib/
│   └── lib/             # Auto-generated TeaQL domain library (robot-kanban)
├── Cargo.toml
└── README.md
```

---

## 🔬 Why TeaQL? (9 Applied Scenarios)

This application exercises **9 distinct TeaQL capabilities** across its CRUD and query workflows. Each scenario below maps a TeaQL API to its concrete usage in this app and the exact SQL it produces.

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

**Applied in:** `/search` or `/s` command — filters the Kanban board in real-time.

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

![TeaQL facet aggregation demo](./assets/teaql-facet-aggregation.gif)

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

**Applied in:** bare input `<name>` or `/add` command — creates a new task in Planned status.

---

### Scenario 5: ID Space Generation (`RusqliteIdSpaceGenerator`)

**What it does:** Generates globally unique, monotonically increasing IDs per entity type using a dedicated SQLite sequence table — no auto-increment column needed.

```rust
// service.rs — add_task()
let id_gen = RusqliteIdSpaceGenerator::from_executor(self.inner_executor.clone());
let next_id = id_gen.next_id("Task")?;
```

**Applied in:** bare input `<name>` or `/add` command — each new task receives a unique ID from the `Task` ID space.

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

**Applied in:** `/mv` and `/del` commands — the fetched `DomainTask` carries `transition_status()` and `delete()` domain methods that raw `Task` entities don't have.

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

**Applied in:** `/del` command.

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

### Scenario 9: Entity Audit Subsystem (`EntityEventSink`)

**What it does:** TeaQL automatically hooks into the persistence lifecycle to track fine-grained Entity Events (Create, Update, Delete, Recover) and computes precise field-level diffs (`old_value` ➔ `new_value`).

```rust
// service.rs — Implement the sink to intercept framework audit events
pub struct AppAuditSink;

impl EntityEventSink for AppAuditSink {
    fn on_event(&self, ctx: &UserContext, event: &EntityEvent) -> Result<(), RuntimeError> {
        let user = short_user(ctx);
        // ... format changes and output to TUI Log Area and app.log
        for change in &event.changes {
            let detail_line = format!(
                "[{}]-[{}]-[AUDIT]-  -> Field [{}]: {} ➔ {}",
                timestamp, user, change.field, change.old_value, change.new_value
            );
        }
        Ok(())
    }
}

// Attach it during runtime initialization
ctx.set_event_sink(AppAuditSink);
```

**Resulting log output:**

```text
[12:04:23.529]-[philip]-[AUDIT]-Entity [Task(1)] was UPDATED. [Task(1): Move task 'My New Task' to Process]
[12:04:23.529]-[philip]-[AUDIT]-  -> Field [status]: Planned ➔ Process
[12:04:23.529]-[philip]-[AUDIT]-  -> Field [version]: 1 ➔ 2
```

![TeaQL audit trail demo](./assets/teaql-audit-trail.gif)

**Next Steps / Coming Soon:** 
In the next phase, we will introduce the **`audit ignore`** feature. By adding an attribute in the `model.xml`, developers will be able to explicitly exclude sensitive data (like passwords, PII, or internal tokens) from being captured or diffed by the audit subsystem.

---

### Scenario Summary

| # | TeaQL API | App Feature | Command |
|:---|:---|:---|:---|
| 1 | `ensure_rusqlite_schema_for` | Auto-create tables & seed data | Startup |
| 2 | `filter_with_json` | Dynamic search / wildcard load | `/s` |
| 3 | `facet_by_status_as` | Status count aggregation | Board reload |
| 4 | `Q::tasks().new_entity()` | Create task with defaults | `<name>` |
| 5 | `RusqliteIdSpaceGenerator` | Unique ID generation | `<name>` |
| 6 | `.return_type::<DomainTask>()` | Custom domain type mapping | `/mv`, `/del` |
| 7 | `DeleteCommand.expected_version()` | Optimistic concurrency delete | `/del` |
| 8 | `.comment()` | Query intent tracing | All queries |
| 9 | `EntityEventSink` | Field-level lifecycle diffs & Audit | All mutations |

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

### 🤖 AI-Friendly Domain Modeling via Service-Generated APIs

A hidden paradigm shift in this architecture is how naturally it guides AI coding assistants. 

The workflow forms a highly predictable closed loop:
1. **AI Generation:** An AI easily drafts the declarative domain model (`model.xml`) from raw business requirements. To automate this process entirely, we built the [teaql-agent-kit](https://github.com/teaql/teaql-agent-kit).
2. **Translation Service:** A dedicated background service takes this model and translates it into a dense, strictly-typed Rust API layer.
3. **High-Obedience Implementation:** When the AI assistant helps you write the actual application logic (like in `service.rs`), it is fed these compiler-enforced APIs as context. 

Because the AI is bounded by strict generated types rather than scattered database migrations, it has far less room to hallucinate raw SQL strings or guess table schemas. The result is a higher-obedience workflow where AI-generated application logic is more likely to compile, align with the domain model, and stay understandable to humans.

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
- **TeaQL Runtime Packages** — the following crates are expected to be available (e.g., via relative path or git submodule):
  - `teaql-core`, `teaql-runtime`, `teaql-macros`, `teaql-sql`, `teaql-provider-rusqlite`
  
> **A Note on Open Source:** The TeaQL *runtime* (which executes the queries, handles concurrency, audits, and powers the TUI) is open source. The *generator* that compiles `model.xml` into the Rust code found in `generate-lib/lib/` is currently closed-source while we refine it. However, we have **checked in the generated code** so you can compile, run, and modify this demo app immediately using only the open-source runtime components!
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

---

## 💬 What We'd Love Feedback On

We're building TeaQL because we believe developers shouldn't have to choose between clean Domain-Driven Design and raw SQL performance/visibility.

If you try out this Kanban board or look at the code:
- **Does the query tracing (`.comment()`) actually help you understand what the app is doing?**
- **How do you feel about defining your domain in `model.xml` vs writing Rust structs directly?** (Even though the generator is currently closed-source, we'd love your thoughts on the DX of declarative modeling).
- **Upcoming Feature:** We are working on an `audit ignore` attribute to exclude PII/sensitive data from the `EntityEventSink`. How do you currently handle this in your stack?

Drop a comment on HN, open an issue, or reach out! 

*(P.S. TeaQL was originally born out of our need to manage complex workflows and data at scale. Check out the framework behind this at [teaql.io](https://teaql.io/) — if you're building physical infra or complex business logic, come say hi!)*
