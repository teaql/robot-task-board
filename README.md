# TeaQL Robot Task Board

![Start Screen](https://raw.githubusercontent.com/teaql/robot-task-board/main/assets/001-start-screen.png)

*A tiny self-bootstrapping business application built with TeaQL Runtime.*

```bash
docker rmi -f teaql/robot-task-board:latest
docker run --rm -it teaql/robot-task-board:latest
```

### Highlights

* ~4.7 MB Docker image
* No distro layer
* Self-bootstrap SQLite DB
* Business Trace & SQL Introspection
* Domain-model-driven runtime
* Runs comfortably within a few megabytes of memory

---

## Demo Video

Watch TeaQL Runtime bootstrap itself, verify the domain schema, initialize reference data, and start the application.

[VIDEO LINK HERE]

---

## Runtime Bootstrap



![BOOTSTRAP](https://raw.githubusercontent.com/teaql/robot-task-board/main/assets/002-bootstrap.png)

TeaQL Runtime starts from an empty environment and automatically:

1. Opens the database
2. Discovers domain entities
3. Verifies storage structures
4. Verifies reference data
5. Starts the runtime

Example startup trace:

```text
Open SQLite database

4 entities discovered

Verified platform_data (4 fields)
Verified task_data (5 fields)
Verified task_execution_log_data (5 fields)
Verified task_status_data (7 fields)

Seed platform_data (1 record)
Seed task_status_data (4 records)

TeaQL Runtime ready

4 entities, 4 tables verified, 2 seeds
24348μs total
```

---

## Robot Task Board

![Task Board](https://raw.githubusercontent.com/teaql/robot-task-board/main/assets/003-task-board.png)
The task board is intentionally simple.

Its purpose is not to demonstrate task management.

Its purpose is to demonstrate how TeaQL Runtime powers a real business application through generated domain models, business traces, audit trails, and query execution.

---

## Business Trace & SQL Introspection

![Task Board](https://raw.githubusercontent.com/teaql/robot-task-board/main/assets/004-trace-chain.png)

Unlike traditional ORMs, TeaQL exposes how business operations are translated into runtime behavior.

Example: comment propagation

```text
Get active tasks
 └── status_stats
      └── Count status
         └── count_tasks

```

```sql

[06:36:50.540]-[root]-[INFO]-Execute TeaQL - Q::tasks().comment("Get active tasks").facet_by_status_as("status_stats", Q::task_status().comment("Count status").count_tasks())          │
│[06:36:50.540]-[root]-[  443µs]-[DEBUG]-SqlLogEntry - [Get active tasks] - [16*Task]                                                                                                    │
│          SELECT id, name, version, status AS status_id, platform AS platform_id FROM task_data WHERE (version > 0)                                                                     │
│[06:36:50.541]-[root]-[  256µs]-[DEBUG]-SqlLogEntry - [Get active tasks -> status_stats -> Count status] - [5*TaskStatus]                                                               │
│          SELECT id, name, code, color, display_order, progress, version FROM task_status_data WHERE (version > 0)                                                                      │
│[06:36:50.541]-[root]-[  303µs]-[DEBUG]-SqlLogEntry - [Get active tasks -> status_stats -> Count status -> count_tasks] - [4*Task]                                                      │
│          SELECT status, COUNT(*) AS count_tasks FROM task_data WHERE ((version > 0)   AND (version > 0)   AND (status IN (1, 1001, 1002, 1003, 1004))) GROUP BY status                 │
│[06:36:50.542]-[root]-[INFO]-✔ Get active tasks                                                                                                                                         │
│--------------------------------------------------------------------------------

```

TeaQL propagates comments through nested queries, facets, and aggregates, so every generated SQL statement can still be traced back to the business intent that produced it.

---

## What Is TeaQL?

TeaQL applications are composed of three layers.

```text
┌─────────────────────────────────────┐
│ Application Layer                   │
│                                     │
│ Robot Task Board                    │
│ ERP                                 │
│ API Services                        │
│ CLI Applications                    │
│ Industrial Systems                  │
│                                     │
│ Controlled by application teams     │
└─────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│ Generated Domain Model Layer        │
│                                     │
│ Entities                            │
│ Requests                            │
│ State Changes                       │
│ Query APIs                          │
│ Business Behaviors                  │
│                                     │
│ Generated from domain definitions   │
└─────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│ TeaQL Runtime Layer                 │
│                                     │
│ Query Execution                     │
│ SQL Generation                      │
│ Transactions                        │
│ Audit Trails                        │
│ Business Trace                      │
│ Facets                              │
│ Bootstrap                           │
│ Schema Verification                 │
│ Runtime Infrastructure              │
│                                     │
└─────────────────────────────────────┘
```

TeaQL Runtime is not the application itself.

TeaQL Runtime is the infrastructure layer that supports generated domain model code and allows business applications to run.

---

## Why This Matters

Traditional business software often requires heavyweight infrastructure.

TeaQL Runtime is designed to support business applications across a very wide range of environments.

### Edge & Embedded

* Routers
* Industrial gateways
* ARM devices
* Embedded Linux systems
* Edge computing platforms

### Standard Business Systems

* Internal enterprise applications
* ERP systems
* Workflow systems
* Operational platforms

### Mission-Critical Systems

* Financial systems
* Trading infrastructure
* Audit-heavy applications
* Low-latency business services

The same domain model can scale from embedded devices to mission-critical business systems.

---

## Design Philosophy

TeaQL focuses on making business software:

* Observable
* Traceable
* Domain-driven
* Resource-efficient
* AI-friendly
* Portable

The goal is to let developers understand what their business code actually does.

[Make a deeper dive](<https://teaql.io/blog/robot-task-board-showcase/>)

---

Powered by TeaQL (@teaqlio)
