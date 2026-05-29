# Show HN: TeaQL Showcase – See What Your Business Code Actually Does

> **TeaQL Website**: [https://teaql.io/](https://teaql.io/)

Instead of hiding database behavior behind an opaque ORM, our new demo application shows the full execution path of a domain action: `Command` → `Domain transition` → `SQL` → `Audit diff` → `Event log` → `UI projection`.

![TeaQL task board demo](./assets/teaql-task-board.gif)

To make the idea concrete, we built a terminal-based Kanban board using Ratatui + SQLite. When you move a task from *Planned* to *Process*, TeaQL shows the generated SQL, optimistic concurrency update, audit trail, lifecycle event, and refreshed status facets — all in real time.

The app also cross-compiles as a standalone statically linked binary for `armv7` router environments, with no external runtime dependencies. 

✨ **Powered by native `rusqlite`**: The TeaQL code generator natively supports `rusqlite`, producing 100% Rust-native SQLite execution code that compiles directly into your binary with zero external driver overhead.

## 🔬 Why TeaQL? (10 Applied Scenarios)

This application exercises **10 distinct TeaQL capabilities** across its CRUD and query workflows:

1. **Schema Bootstrap**: Automatically creates tables and seeds initial data (`ensure_rusqlite_schema_for`).
2. **JSON-Based Dynamic Filtering**: Dynamically constructs WHERE clauses from JSON at runtime (`filter_with_json`).
3. **Faceted Aggregation**: Computes aggregate counts grouped by a relation within a single round-trip (`facet_by_status_as`).
4. **Entity Factory**: Creates pre-wired entities ready for population (`new_entity`).
5. **ID Space Generation**: Generates globally unique, monotonically increasing IDs via a dedicated SQLite sequence table.
6. **Domain Behavior Injection**: Attaches rich domain logic to generated entities using Extension Traits.
7. **Partial Projections & Aggregations**: Deserializes results into custom DTOs for performance (`return_type::<T>()`).
8. **Audited Soft-Delete**: Enforces optimistic concurrency and propagates deletion context for audit logging.
9. **Comment Chain Propagation**: Attaches human-readable intent annotations (`.comment()`) that propagate through nested queries for perfect SQL auditing.
10. **Entity Audit Subsystem**: Hooks into the persistence lifecycle to track Entity Events and compute field-level diffs (`EntityEventSink`).

## 📐 Architecture

The application uses a clean 3-layer architecture:
- **UI (`ui.rs`)**: Ratatui layout, syntax highlighting.
- **Application (`main.rs`)**: App state, command parsing, event loop.
- **Domain (`service.rs`)**: TeaQL queries, DDD aggregate root (`DomainTask`), domain logic.

The domain model is generated from `models/model.xml`, keeping the application logic strictly typed and highly predictable. This approach naturally tames AI coding assistants by forcing them to use safe, semantic, generated APIs (e.g., `task.update_status_to_planned()`) instead of hallucinatable raw queries.

## 🚀 Try It Out

Check out the code and run it yourself! The repository includes everything you need to test the workflow:

```bash
git clone https://github.com/teaql/robot-task-board.git
cd robot-task-board
cargo run
```

*For more details on the architecture and generated SQL, view the full blog post at [teaql.io](https://teaql.io/).*
