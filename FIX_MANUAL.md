# 修复手册 (Fix Manual)

本手册记录了 `teaql-rs` 框架及 `robot-task-board` 项目中的已知问题及其修复方案，主要涵盖乐观锁冲突（OptimisticLockConflict）和图查分引擎更新带来的问题。

## 1. 乐观锁冲突 (OptimisticLockConflict) 与无效的依赖更新

**问题描述**: 
在执行复杂的图保存操作时，图查分引擎 (Graph Diff Engine) 有时会针对同一个实体实例生成多个重复的 `UpdateCommand`。由于底层执行器会在每次更新后使得版本号加一，重复的更新如果预期版本号不同，就会触发 `OptimisticLockConflict`。
同时，当更新被子实体或外键关系触发时，即使子实体并没有任何变动，框架也会生成空字段的更新，造成不必要的数据库写操作及版本号跳跃。

**修复方案**:
- **引入已访问节点去重 (Visited Nodes Deduplication)**: 在 `teaql-runtime/src/graph.rs` 的 `GraphMutationPlan` 结构体中新增了 `visited_nodes: HashSet<teaql_core::Value>` 集合。在 `collect_graph_plan` 阶段，通过该集合来过滤掉针对同一主键实体的重复遍历，防止向计划中追加多个同一记录的 `UpdateCommand`。
- **过滤空的更新操作 (Empty Update Pruning)**: 修改 `teaql-runtime/src/repository/graph.rs` 中的 `execute_graph_plan` 方法，拦截那些 `update_fields` 字段为空的 `Update` 节点（代表没有任何字段发生变更），直接跳过它们的执行。这样不仅防止了版本号乱跳，而且减少了对数据库的压力。

---

## 2. SQL 日志截断与 Batch 批量执行问题

**问题描述**:
在引入图查分引擎的上述去重修复后，`teaql-runtime` 中执行插入或更新时合并成了 `BatchInsertCommand` 和 `BatchUpdateCommand`，进而通过 `MutationRequest::Batch` 下发到 SQL 驱动（如 `teaql-provider-rusqlite`）。这导致 SQL 驱动只在 Batch 结束后产生一条元数据日志（`ExecutionMetadata`），致使各个被打包的 SQL 语句日志（`SqlLogEntry`）丢失。在诸如 `tests_ui::test_mission_simulation` 等严格核对 SQL 输出的 UI 测试中，会报文件不匹配（`actual-log.txt` 丢失了具体的 `UPDATE` 语句）。

**修复方案**:
- **拆分执行**：在 `teaql-runtime/src/repository/base.rs` 的 `batch_update` 和 `batch_insert` 中，放弃将操作装箱入 `MutationRequest::Batch`。而是通过遍历每个记录生成独立的 `MutationRequest::Update` 和 `MutationRequest::Insert` 逐一发送给执行器，并对每次调用的结果使用 `self.metadata.record_metadata_log(&res.metadata)` 记录，从而恢复了完整的 `[DEBUG]-SqlLogEntry` 输出。
- **按 `update_fields` 严格过滤字段**：修复了原本 `batch_update` 不检查 `command.update_fields` 盲目更新整个实体的错误。现在在构造单个 `UpdateCommand` 的 `values` 时，仅放入 `command.update_fields` 内存在的键值对，保证生成的 `UPDATE` SQL 只包含实际修改了的字段（如 `UPDATE task_data SET status = 1002, version = 2 WHERE ...`），不再冗余地更新 `name` 等未修改的字段。

## 3. 文档不一致问题
- 修复了 `API_GUIDE.md`（包括生成器模板）中 `execute_for_exists` 被错误地放置于 `PurposedQuery` （需要 `.purpose()` 前置调用）下的问题。实际上，该方法直接挂载在未经 Purpose 包装的基础 `Request` 查询结构体上。
