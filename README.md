# Robot Task Board TUI 🚀

A state-of-the-art, premium terminal user interface (TUI) Robot Task Kanban Board designed to demonstrate the advanced capabilities and robust engineering of the **TeaQL (TK) query framework**. 

Written in Rust using **Ratatui + Crossterm**, this application connects to a synchronous SQLite database via the `teaql-provider-rusqlite` adapter and is fully cross-compiled as a standalone statically-linked binary for `armv7` router environments (zero external system dependencies).

---

## 🌟 Key Features

### 1. Single-Query Facet Aggregation (`facet_by`)
In conventional ORMs, loading a Kanban board with active tasks while simultaneously displaying aggregate statistics for different task statuses requires executing multiple separate queries (or parsing a heavy dataset in memory). 

This application fetches both the active tasks and their respective status count facets in a **single, highly-optimized TeaQL query**:
```rust
// Fetch all active tasks and aggregate status facets in a single database round-trip
let select = Q::tasks()
    .find_with_json(self.build_search_json())
    .facet_by_status_as("status_stats", Q::task_status().count_tasks().comment("Count status"))
    .comment(self.build_search_comment());

let all_tasks = select.execute_for_list(&self.ctx).await?;
```
TeaQL's aggregate engine handles the sub-selection natively and delivers populated relation counts directly within the returned `SmartList` container.

### 2. DDD Clean Architecture & Return Type Conversions (`return_type`)
To prevent the anti-pattern of anemic domain models, this project separates database transfer records from rich domain behaviors:
* **Rich Domain Aggregate Root**: Encapsulates status flow logic inside a custom `DomainTask` wrapper wrapping the generated entity `Task`.
* **Behavior Command Pattern**: Integrates a `TransitionCommand` object to handle task status transitions. The rule for automatically moving a task to its next chronological status (e.g., Planned → Process → Done) when no target status is provided is fully encapsulated within the aggregate root.
* **Return Type Binding**: Uses TeaQL's `.return_type::<DomainTask>()` selection API to let the repository natively fetch and map custom domain behaviors directly at database extraction.

### 3. Native Tracing & Comment Stack Propagation (TeaQL v0.9.2)
Leveraging the industry-leading query tracing and diagnostic capabilities in **TeaQL v0.9.2**:
* **Native PID/TID Resolution**: By utilizing `UserContext::new()` natively, the context auto-resolves the OS user, Process ID (PID), and Thread ID (TID) to establish a unique diagnostic trace header: `philip@pid-{pid}.tid-{tid}`.
* **Dynamic Comment Chain Propagation (`->`)**: Parent query comments are dynamically combined and propagated down to nested facets and relation aggregate sub-queries, creating a structured intent chain (e.g. `-[Get all tasks->Count status->count_tasks]-`).
* **SQL Comment Stripping**: Query intent comments are isolated into dedicated metadata fields (`CompiledQuery.comment`), eliminating raw duplication inside SQL queries and keeping query text completely clean.

The TUI SQL log panel natively intercepts and renders these rich, fully-traceable log lines in real-time:
```text
2026-05-26 12:06:00.225-[philip@pid-2007633.tid-1]--DEBUG - SqlLogEntry - [Get all tasks] - [5*Task] SELECT id, name, version, status_id, platform_id FROM task_data WHERE (version > 0) (took 0.184ms)
2026-05-26 12:06:00.226-[philip@pid-2007633.tid-1]--DEBUG - SqlLogEntry - [Get all tasks->Count status] - [3*TaskStatus] SELECT id, name, code, version, platform_id FROM task_status_data WHERE (version > 0) (took 0.138ms)
```

### 4. Interactive Blinking Input Cursor & Premium Aesthetics
* **Dynamic Blinking Cursor**: Crossterm's `Show` and `EnableBlinking` parameters are activated at startup. The rendering loop tracks active cursor characters dynamically (`chunks[3].x + 3 + input.chars().count()`) to present an elegant input cursor.
* **Subtle Indentation Padding**: Left margins on the Command Input box and the bottom Help menu are padded by 2 spaces to separate UI borders from text, providing a highly premium terminal experience.
* **Real-time SQL Audits**: The upper 50% of the screen features a dedicated action logging area, allowing developers to audit live SQL execution times, comment stacks, and query results.

### 5. Zero-Dependency Standalone Static Binary
Using a cross-compilation pipeline powered by `cargo-zigbuild`, the application compiles statically with a musl-based toolchain, linking both the SQLite engine and the TUI backend into a single, standalone binary for `armv7` router environments.

---

## 📐 Design Considerations

### 1. Separation of Concerns
Generated structures act strictly as data transfer records. Rich business validation and state rules are isolated within the `DomainTask` behavior aggregate root.

### 2. Unified Wildcard Searching
Instead of branching the query generation between searching and full loading, the search query leverages TeaQL's native capability to accept an empty JSON object `{}` inside `find_with_json` (which acts as a wildcard, bypassing filters natively):
```rust
fn build_search_json(&self) -> String {
    if let Some(ref term) = self.search_term {
        format!(r#"{{"name": "{}"}}"#, term)
    } else {
        r#"{}"#.to_owned()
    }
}
```
This isolates the JSON tree assembly from the database reload logic, resulting in highly clean and readable code.

### 3. Dynamic Comment Resolution
The programmer's intent comment is dynamically resolved based on the active search state to supply rich trace capabilities within diagnostic channels:
```rust
fn build_search_comment(&self) -> &'static str {
    if self.search_term.is_some() {
        "Search by JSON"
    } else {
        "Get all tasks"
    }
}
```

---

## 🛠 Command Line Guidelines

| Command | Shortcut | Description | Example |
| :--- | :--- | :--- | :--- |
| **`add <name>`** | - | Creates a new task in Planned status | `add calibrate sensor` |
| **`move <id> [status]`** | **`mv`** | Transitions task status (planned/process/done, default is next chronological state) | `move 3` or `mv 3 process` |
| **`search <keyword>`** | **`s`** | Filters Kanban board dynamically using JSON-EXPR (empty keyword clears search) | `search calibrate` or `s` |
| **`delete <id>`** | **`del`** | Permanently deletes a task from the database | `delete 3` |
| **`exit`** | **`q`** | Quits the Kanban board application | `exit` |

---

## 🚀 Compilation & Deployment

### Local Native Build
```bash
# Verify compilation
cargo check

# Run local development TUI
cargo run

# Compile local optimized Release binary
cargo build --release
```

### ARMv7 Cross-Compilation
```bash
# Statically cross-compile release binary for armv7 routers
cargo zigbuild --release --target armv7-unknown-linux-musleabihf
```
The compiled standalone release binary is located at `target/armv7-unknown-linux-musleabihf/release/robot-task-board`. Upload it directly to your router to run instantly with zero dependencies!
