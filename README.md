# TeaQL Robot Task Board

[HERO SCREENSHOT HERE]

*A tiny self-bootstrapping business application built with TeaQL Runtime.*

```bash
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

[BOOTSTRAP SCREENSHOT HERE]

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

[TASK BOARD SCREENSHOT HERE]

The task board is intentionally simple.

Its purpose is not to demonstrate task management.

Its purpose is to demonstrate how TeaQL Runtime powers a real business application through generated domain models, business traces, audit trails, and query execution.

---

## Business Trace & SQL Introspection

[BUSINESS TRACE SCREENSHOT HERE]

Unlike traditional ORMs, TeaQL exposes how business operations are translated into runtime behavior.

Example:

```text
Get active tasks
 └── status_stats
      └── Count status

Generated SQL:
SELECT ...
```

The goal is to make business behavior observable rather than hidden behind framework internals.

---

# What Is TeaQL?

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

# Why This Matters

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

# Design Philosophy

TeaQL focuses on making business software:

* Observable
* Traceable
* Domain-driven
* Resource-efficient
* AI-friendly
* Portable

The goal is to let developers understand what their business code actually does.

---

Powered by TeaQL (@teaqlio)
