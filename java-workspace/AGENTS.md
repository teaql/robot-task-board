# TeaQL Workspace Agent Guide

This workspace is the Spring Boot runtime for the TeaQL domain package `com.doublechaintech.robotkanbanservice`.

## Read First

- Absolute rule: never use SQL to operate on the database from workspace business code.
- If a change cannot be implemented with generated TeaQL APIs, stop and report the missing API. Do not work around it with SQL.
- Use generated TeaQL APIs before writing repository code, DTO mapping, or manual relation loaders.
- Before writing code against the generated TeaQL library, read the library guide at `../generate-lib/AGENTS.md`. If the library is supplied from a package repository instead of a local generated directory, locate the unpacked dependency sources or published source artifact and read that library's `AGENTS.md` before using its APIs.
- Do not use other database access technologies from workspace business code. This includes JPA/Hibernate, Spring Data repositories, MyBatis, JDBC templates, jOOQ, QueryDSL, native queries, raw SQL clients, and hand-written DAO layers.
- Do not add dependencies or helper wrappers that bypass TeaQL for persistence, querying, transactions, relation loading, or DTO mapping.
- Use `Q` for reads and generated request builders.
- AI agents must not instantiate request builders directly. Do not write `new XxxRequest(...)`; start from `Q.xxx()` and chain generated request helpers from there.
- Business code may use only the supported TeaQL surface: `Q`, generated entity APIs, `save(userContext)`, `E`, and workspace-specific extensions of `UserContext`.
- Do not access lower-level TeaQL internals directly: repositories, repository registries, metadata registries, SQL executors, transaction internals, or framework persistence hooks.
- Single-object queries use `.executeForOne(userContext)`.
- Return query results from controllers with `WebResponse.of(entity)` or `WebResponse.of(list)`, and return `WebResponse.fail(e.getMessage())` on exceptions.
- Use entity `save(userContext)` for creates and updates.
- Mandatory update rule: use generated `updateXxx(...)` methods to change entity fields. Do not use `setXxx(...)`; setters are deprecated even when visible.
- Use the generated soft-delete API for deletes. Do not hard-delete rows and do not write SQL delete/update statements.
- Use generated relation selectors such as `selectXxx()` and `selectXxxWith(...)` instead of N+1 loops.
- Keep request-handling code in this workspace. Do not edit generated library classes copied from the domain package.
- Do not edit generated library files. If generated APIs are missing or awkward, update the KSML model or report the missing TeaQL API and regenerate.
- If the API shape is unclear, read `docs/teaql-java-crud-guide.md` before searching external code.

## Business Code Boundaries

TeaQL business code should express intent through layers:

```text
Controller -> Service -> Helper -> <Subject><DomainEntity> -> save/update
```

- Controller owns HTTP/API only: accept request payloads, create command/event/input objects, call services, and return `WebResponse`. Do not put business logic, transactions, Q queries, setters, or `updateXxx(...)` calls in controllers.
- Service owns use-case orchestration and transaction boundaries. Add `@Transactional` to methods that modify data. Services may load entities through helpers, call domain business methods, and persist changes, but must not return `WebResponse` or implement business transitions as field assignments.
- Helper owns reusable Q request semantics and business-oriented loading only. Use names like `loadOrderForShipping(ctx, id)`, apply query-side preconditions, and use `returnType(<Subject><DomainEntity>.class)` when behavior-specific domain objects are needed. Helpers must not mutate state, start transactions, return `WebResponse`, or execute business actions.
- `<Subject><DomainEntity>` classes carry business behavior for a generated entity in a specific role, for example `ShippingOrder`, `CancelableOrder`, or `ApprovingInvoice`. Business methods accept command/event/input objects, validate invariants, and use chainable `updateXxx(...)` internally.
- Util classes isolate low-level technical concerns such as time, string, JSON, crypto, encoding, files, IDs, or external libraries. Utils must not know business concepts, use Q APIs, access the database, return `WebResponse`, or mutate domain entities.

Hard rule: generated setters are not business APIs. New business code must not call `setXxx(...)` from controllers, services, helpers, utils, or domain behavior methods.

## Main Imports

```java
import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.RobotKanbanServiceUserContext;
import io.teaql.data.web.WebResponse;
```

Use `CustomUserContext` for workspace-specific request context behavior.
Extend `CustomUserContext` or the generated `RobotKanbanServiceUserContext` type for context-specific behavior instead of reaching into runtime internals.

## Query Pattern

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

## Save Pattern

```java
var entity = new Platform();
// entity.updateName("example");
entity.save(userContext);
```

Use `updateXxx(...)` for every field change. Never use `setXxx(...)` in new workspace code.

## Internationalization (i18n) Guidance

For multilingual support and translation of domain terms (e.g. validation error messages) when using non-English locales (such as Chinese, Spanish, etc.), TeaQL provides a clean, offline i18n translation dictionary workflow:

1. **Locate the Sample File**: Find the auto-generated `teaql-i18n.sample.json` file in the workspace root. It contains all unique domain entity and property vocabulary keys.
2. **Translate via AI**: Paste the contents of `teaql-i18n.sample.json` into your AI tool, or instruct this AI agent to translate it. Use the following prompt:
   > **Prompt**: "Translate this JSON file containing TeaQL domain vocabulary keys into 15 languages: Arabic (ar), Simplified Chinese (zh_CN), Traditional Chinese (zh_TW), Spanish (es), French (fr), German (de), Japanese (ja), Korean (ko), Portuguese (pt), Thai (th), Ukrainian (uk), Filipino (fil), Indonesian (id), English (en) under each key. Return only the valid JSON result."
3. **Save the JSON**: Save the translated JSON result as `teaql-i18n.json` externally or on the classpath.
4. **Configure JVM Parameter**: Set the JVM property `-Dteaql.i18n.path=/path/to/teaql-i18n.json` pointing to your translated file.
5. **Runtime Validation Rule**: If a non-English translator is instantiated at runtime and `-Dteaql.i18n.path` is unconfigured, invalid, or the file is empty, the starter throws a strict `IllegalStateException` to prevent unlocalized output.

## Core Business Entry Points

These examples use entities with the highest reverse relation counts. A high reverse relation count means many other objects point back to this entity, making it a stronger aggregate-root candidate.

- `Platform`: `Q.platforms()`, reverse relations `1`, class `com.doublechaintech.robotkanbanservice.platform.Platform`
- `Task`: `Q.tasks()`, reverse relations `1`, class `com.doublechaintech.robotkanbanservice.task.Task`
- `TaskStatus`: `Q.taskStatuses()`, reverse relations `1`, class `com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus`

## All Domain Entry Points

- `Platform`: `Q.platforms()`, reverse relations `1`, class `com.doublechaintech.robotkanbanservice.platform.Platform`
- `TaskStatus`: `Q.taskStatuses()`, reverse relations `1`, class `com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus`
- `Task`: `Q.tasks()`, reverse relations `1`, class `com.doublechaintech.robotkanbanservice.task.Task`
- `TaskExecutionLog`: `Q.taskExecutionLogs()`, reverse relations `0`, class `com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog`