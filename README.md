# Show HN: TeaQL Task Board – See What Your Business Code Actually Does

![TeaQL task board demo](./assets/teaql-task-board.gif)

TeaQL is a Rust framework for building business applications with full visibility into domain transitions, generated SQL, and audit trails.

This demo is a small Kanban board built with Ratatui + SQLite. Every task operation exposes the complete execution path:

```text
Command
  ↓
Domain Transition
  ↓
SQL
  ↓
Audit Diff
  ↓
Event Log
  ↓
UI Projection
```

Unlike traditional ORMs, TeaQL makes query tracing and auditability first-class features.

```rust
Q::tasks()
    .comment("Get active tasks")
    .facet_by_status_as(...)
```

Produces traceable runtime logs like:

```text
Get active tasks
Get active tasks->status_stats
```

The demo also cross-compiles into a standalone armv7 binary and can run directly on router-class devices with no external runtime dependencies.

GitHub: https://github.com/teaql

Website: https://teaql.io/

Detailed technical write-up: [[Blog Link](https://teaql.io/blog/robot-task-board-showcase/)]

I'd love feedback from people building ERP, CRM, workflow, industrial, or embedded business systems.
