# Show HN: A Task Board That Explains What Your Business Code Is Doing

![TeaQL task board demo](./assets/teaql-task-board.gif)

Most business applications hide what happens after a user action.

You click a button, something changes, and the actual execution path disappears behind layers of services, ORMs, caches, and logs.

This small terminal task board explores a different idea.

When you move a task from **Planned** to **Process**, the application shows the complete execution path in real time:

```text
Command
  ↓
Domain Transition
  ↓
Generated SQL
  ↓
Audit Diff
  ↓
Domain Event
  ↓
UI Projection
```

For example, moving a task immediately reveals the generated SQL:

```sql
UPDATE task_data
SET status = 1002, version = 2
WHERE id = 1 AND version = 1
```

At the same time, the audit system records the field-level change:

```text
status:
Planned -> Process

version:
1 -> 2
```

And the board statistics are refreshed automatically.

The goal is not to build another Kanban board.

The board is intentionally simple so the runtime behavior is easy to follow. The underlying framework, **TeaQL**, is designed for larger business systems where understanding state transitions, generated SQL, audit trails, and query execution paths becomes increasingly important.

## Query Trace Chains

TeaQL queries carry semantic intent, not just SQL.

```text
Get active tasks
Get active tasks->status_stats
Get active tasks->status_stats->Count status
```

This makes it easier to understand how one business request expands into multiple queries, relation loads, and aggregations.

## Why Embedded Matters

This is not just a desktop demo.

The same application can be cross-compiled into a standalone `armv7` binary and run directly on router-class devices with no external runtime dependencies.

I am particularly interested in bringing business workflows closer to physical infrastructure: industrial gateways, factory equipment, logistics systems, gas filling stations, and other edge devices.

For TeaQL, embedded deployment is not an afterthought. The long-term idea is that domain models, workflow execution, SQL persistence, audit trails, and business logic should be able to run close to the equipment itself, not only inside a cloud backend.

## What Might Be Interesting

- Query tracing with semantic comments
- Field-level audit diffs
- Optimistic concurrency control
- Faceted aggregations
- Strongly typed domain APIs
- SQLite-based local persistence
- Single-binary deployment
- Embedded-friendly runtime

## More Details

I wrote a deeper technical walkthrough covering the architecture, generated APIs, query tracing, audit subsystem, faceted queries, and deployment model:

https://teaql.io/blog/robot-task-board-showcase

GitHub: https://github.com/teaql

Website: https://teaql.io/

I'd love feedback from people building ERP, CRM, workflow, industrial, edge, or embedded business applications.

Does seeing every SQL statement, audit diff, trace chain, and domain transition help you understand and debug business software more effectively?
