# TeaQL Java Coding Agent Instructions

This project uses the TeaQL-generated Java domain package `com.doublechaintech.robotkanbanservice`.

## Core Rule

Always prefer TeaQL-generated Java APIs over handwritten SQL, repository
orchestration, DTO mapping, and relation loading.

Business code must stay on the supported surface: `Q` for reads, generated
entity APIs and `save(userContext)` for state changes, `E` for safe expression
access, and project-specific extensions of `UserContext` for request context
behavior. Accessing lower-level TeaQL runtime interfaces directly is not
allowed from business code.

Do not edit generated entity, request, expression, repository, or metadata files
directly. Change the KSML model and regenerate.

## Application Layer Safety Guardrails

### 1. Use the Entity API Exclusively
- Always perform state transitions or updates via generated `updateXxx(...)` methods and call `save(userContext)`.
- Never use `setXxx(...)` — setters are deprecated even when visible.
- Never use repository-level methods directly. They execute full graph replacements and may implicitly delete child records.
- Data MUST NOT be deleted implicitly. Use the generated soft-delete API. Do not write SQL delete/update statements.

### 2. BAN ON RAW IO AND UTILITY LIBRARIES
- Inside the application layer, do not use raw IO or utility libraries directly (e.g. `java.nio.file.Files`, `java.net.HttpURLConnection`, Apache HttpClient, OkHttp).
- All side effects (network, file, time) and stateful computations SHOULD go through `UserContext` extensions or dedicated Util classes that are centrally auditable.
- Do not use `System.currentTimeMillis()`, `LocalDateTime.now()`, or `Instant.now()` directly. Use the context-aware time provider to ensure correct timezone handling for global deployments.

### 3. MANDATORY TRIPLE-INTENT (comment + purpose + auditAs)

Every query MUST chain both `.comment("description")` and `.purpose("business intent")` before `.executeForList(ctx)` / `.executeForOne(ctx)`:

```java
Q.orders().filterByStatus("PENDING")
    .comment("Load pending orders for dashboard")
    .purpose("Display order count widget")
    .executeForList(ctx);
```

Every entity mutation MUST call `.auditAs("action description")` before `.save(ctx)`:

```java
order.updateStatus("SHIPPED")
    .auditAs("Mark order as shipped after warehouse confirmation")
    .save(ctx);
```

When the environment variable `TEAQL_ENFORCE_INTENT` is set to `strict`, missing `.comment()`, `.purpose()`, or `.auditAs()` will throw a `RepositoryException`. Set to `warn` for logging-only mode.

## Main Imports

```java
import com.doublechaintech.robotkanbanservice.E;
import com.doublechaintech.robotkanbanservice.Q;
```

The generated Java stack provides:

- entity classes for domain objects
- `Q` query facade and typed request builders
- `E` expression facade for safe long-chain value access
- relation selection helpers
- entity `save(userContext)` helpers for graph-style persistence
- generated metadata, repositories, validators, and constants

## Query Style

Use `Q` for reads:

```java
var rows = Q.<entityPlural>()
    .selectSelf()
    .page(1, 20)
    .executeForList(userContext);
```

Use generated `select()`, `filterBy(...)`, and
`select(...)` helpers before reaching for raw SQL.

AI agents must not instantiate request builders directly. Do not write
`new XxxRequest(...)` in generated-library or workspace code; start from
`Q.xxx()` and chain the generated request helpers from there.

Do not call generated repositories, repository registries, metadata registries,
SQL executors, transaction internals, or other TeaQL runtime internals directly.
If `Q`, generated entity APIs, `E`, and `UserContext` extensions are not enough,
report the missing generated API instead of bypassing it.

## Relation Loading

Load relations through generated request helpers:

```java
Q.<entityPlural>()
    .select<Relation>(Q.<targetPlural>().selectSelf());
```

Avoid manual loops that run one query per row.

## Saving Data

Use generated entity save methods when persisting business objects:

```java
entity.save(userContext);
```

Treat save as graph persistence: one call may coordinate multiple repositories
when the entity graph contains related objects.

## Safe Value Access

Use `E` for long-chain object access:

```java
var value = E.<entity>(entity)
    .get<Relation>()
    .get<Field>()
    .eval();
```

Do not write nested null checks or unchecked casts when a generated expression
chain is available.

## Internationalization (i18n) Guidance

For multilingual support and translation of domain terms (e.g. validation error messages) when using non-English locales (such as Chinese, Spanish, etc.), TeaQL provides a clean, offline i18n translation dictionary workflow:

1. **Locate the Sample File**: Find the auto-generated `teaql-i18n.sample.json` file in the project root. It contains all unique domain entity and property vocabulary keys.
2. **Translate via AI**: Paste the contents of `teaql-i18n.sample.json` into your AI tool, or instruct this AI agent to translate it. Use the following prompt:
   > **Prompt**: "Translate this JSON file containing TeaQL domain vocabulary keys into 15 languages: Arabic (ar), Simplified Chinese (zh_CN), Traditional Chinese (zh_TW), Spanish (es), French (fr), German (de), Japanese (ja), Korean (ko), Portuguese (pt), Thai (th), Ukrainian (uk), Filipino (fil), Indonesian (id), English (en) under each key. Return only the valid JSON result."
3. **Save the JSON**: Save the translated JSON result as `teaql-i18n.json` externally or on the classpath.
4. **Configure JVM Parameter**: Set the JVM property `-Dteaql.i18n.path=/path/to/teaql-i18n.json` pointing to your translated file.
5. **Runtime Validation Rule**: If a non-English translator is instantiated at runtime and `-Dteaql.i18n.path` is unconfigured, invalid, or the file is empty, the starter throws a strict `IllegalStateException` to prevent unlocalized output.

## Domain Context

Read these files before making domain-specific changes:

- `docs/teaql-java-domain-map.md`
- `docs/teaql-java-request-examples.md`
- `docs/teaql-java-save-expression-guide.md`