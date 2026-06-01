# TeaQL Java CRUD Guide

Generated for `com.doublechaintech.robotkanbanservice`. Use this guide when adding controllers, services, jobs, or integration code in this workspace.

## Setup

```java
import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.RobotKanbanServiceUserContext;
import io.teaql.data.web.WebResponse;
```

Most workspace code receives a TeaQL context from Spring:

```java
public WebResponse handle(@TQLContext UserContext userContext) {
    // use Q, WebResponse, and entity.save(userContext)
}
```

## Non-Negotiable Database Rule

Never use SQL to operate on the database from workspace business code. This includes select, insert, update, delete, schema changes, JDBC templates, native queries, direct repositories, and ad hoc SQL helpers.

If the generated TeaQL API does not provide a path for the requested change, stop and report the missing API. Do not implement a SQL workaround.

## Mandatory Update Method Rule

Use generated `updateXxx(...)` methods for all entity field changes. Do not use `setXxx(...)` in new code. Setters are deprecated even when the IDE or generated class makes them visible.

```java
entity.updateName("new value");
```

Do not write:

```java
entity.setName("new value");
```

## Read One Entity

Prefer typed generated query entry points. Controller methods that return one object should wrap the result in `WebResponse.of(...)`:

```java
public WebResponse getOne(@TQLContext UserContext userContext, Long id) {
    try {
        var entity = Q.platforms()
            .filterById(id)
            .selectSelf()
            .executeForOne(userContext);
        return WebResponse.of(entity);
    } catch (Exception e) {
        return WebResponse.fail(e.getMessage());
    }
}
```

## Read A Page

Controller methods that return multiple objects should wrap the list in `WebResponse.of(...)`:

```java
public WebResponse list(@TQLContext UserContext userContext) {
    try {
        var list = Q.platforms()
            .selectSelf()
            .page(1, 20)
            .executeForList(userContext);
        return WebResponse.of(list);
    } catch (Exception e) {
        return WebResponse.fail(e.getMessage());
    }
}
```

## Create

```java
var entity = new Platform();
// Fill fields with generated updateXxx(...) methods, not setXxx(...).
entity.save(userContext);
```

## Update

```java
var entity = Q.platforms()
    .filterById(id)
    .selectSelf()
    .executeForOne(userContext);

// Use updateXxx(...) methods for state changes.
// entity.updateName("new value");
entity.save(userContext);
```

## Load Relations

Use generated selectors. Do not write a loop that queries children one row at a time. The examples below focus on the entities with the highest reverse relation counts because they are stronger aggregate-root candidates.

```java
var list = Q.platforms()
    .selectTaskListWith(Q.tasks().selectSelf())
    .executeForList(userContext);
```

```java
var list = Q.tasks()
    .selectStatusWith(Q.taskStatuses().selectSelf())
    .executeForList(userContext);
```
```java
var list = Q.tasks()
    .selectPlatformWith(Q.platforms().selectSelf())
    .executeForList(userContext);
```

```java
var list = Q.tasks()
    .selectTaskExecutionLogListWith(Q.taskExecutionLogs().selectSelf())
    .executeForList(userContext);
```


```java
var list = Q.taskStatuses()
    .selectTaskListWith(Q.tasks().selectSelf())
    .executeForList(userContext);
```


## Delete

Use the generated TeaQL soft-delete API available on the entity/request class in this domain. If unsure, inspect the generated request class for the target entity and use the soft-delete operation it exposes.

Do not hard-delete rows. Do not write SQL `DELETE` or `UPDATE` statements. If the soft-delete API is not visible, stop and report that blocker instead of changing data through SQL.

## Common Mistakes

- Do not instantiate repositories directly in workspace business code.
- Do not use SQL for any database operation.
- Do not use deprecated `setXxx(...)` methods for updates. Use `updateXxx(...)`.
- Do not return raw entities or lists from controller query methods. Return `WebResponse.of(entity)` or `WebResponse.of(list)`.
- Do not forget `.executeForOne(userContext)` or `.executeForList(userContext)`.
- Do not assume relations are loaded unless the query selected them.
- Do not bypass `UserContext`; context carries logging, tenant, security, and repository resolution behavior.

## Entity Cheat Sheet

These entities are selected by reverse relation count, not by model declaration order.

- `Platform`: reverse relations `1`, query `Q.platforms()`, save `new Platform().save(userContext)`, request `com.doublechaintech.robotkanbanservice.platform.PlatformRequest`
- `Task`: reverse relations `1`, query `Q.tasks()`, save `new Task().save(userContext)`, request `com.doublechaintech.robotkanbanservice.task.TaskRequest`
- `TaskStatus`: reverse relations `1`, query `Q.taskStatuses()`, save `new TaskStatus().save(userContext)`, request `com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest`