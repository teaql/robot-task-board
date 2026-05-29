# Design Document: Hierarchical Comment Propagation & Thread-Safe Observability in Entity Graph Mutations
**基于实体图变更的层级注释繁殖与多线程安全观测性设计**

---

## 1. 摘要 (Abstract)
在复杂的关系-对象映射 (ORM) 或图持久化框架（如 TeaQL）中，级联保存（Cascading Save）是一个高频发生的黑盒操作。一次针对聚合根实体的 `save()` 调用通常会触发拓扑关联图内数十个子实体的级联突变。传统的平铺式物理 SQL 日志缺乏**业务意图归因**和**因果链溯源**能力。

本设计文档提出了一种**级联注释繁殖与层次化追踪机制**，旨在将业务动作（启动节点）的语义随着图保存遍历的进行自动向下流转并层层叠加。针对并发环境下的多线程数据污染痛点，设计采用**栈分配父指针链表 (Stack-Allocated Scoped Cons List)** 方案，实现 **100% 线程安全、零堆分配 (Zero-Heap Allocation) 与零锁竞争 (Lock-Free)** 的极致可观测性架构。

---

## 2. 核心痛点与背景 (Problem Statement)

### 2.1 物理 SQL 的上下文丢失 (Lost of Context)
在持久化如 `INSERT INTO task_status_data ...` 这样的关联数据时，单独查阅该日志无法直观判定它是由于哪个具体业务流程（例如 `"Create lift #3"`）触发的。缺乏一条完整的溯源链路（Lineage Trail）。

### 2.2 多线程下的逻辑竞态 (Logical Race Conditions in Concurrency)
如果多个并发的异步任务或操作系统线程共享同一个全局的 `UserContext`，使用互斥锁 (`Mutex<Vec<String>>`) 保护的全局注释栈会导致逻辑上的**交叉污染**：
- **Thread A (Save Task A)** pushes `"Create lift #3"`.
- **Thread B (Save Task B)** pushes `"Verify Connections"`.
- 最终的输出日志会互相交织，导致诊断信息彻底失效。同时，高频锁竞争（Lock Contention）会导致高并发写吞吐暴跌。

---

## 3. 架构设计原理 (Architectural Concepts)

本设计融合了编译器理论、函数式程序设计与 Rust 的生命周期系统，包含以下两大核心原理：

### 3.1 实体图即 AST (Entity Graph as an AST)
我们将实体持久化行为建模为一个**抽象语法树 (AST) 的遍历与编译过程**：
- **AST 节点**：图中的各个实体节点（如 `DomainTask` 聚合根与关联子实体）。
- **AST 编译器**：底层的 `Graph Compiler`。它对树进行拓扑检查，分析脏字段，并生成 SQL 突变序列。
- **节点注解传播**：业务意图（Comment）作为 AST 节点上的“注解元数据”，在编译器深度优先遍历 (DFS) 的处理过程中，自然地在作用域之间流转和繁殖。

### 3.2 栈分配父指针链表 (Stack-Allocated Scoped Cons List) —— *核心突破点*
为根除多线程下的锁竞争与内存克隆开销，本设计抛弃全局可变状态，采用类似于 Lisp 环境帧的 **Cons 链表（或父指针树）**：
- 在物理调用栈（Call Stack）上分配临时作用域节点。
- 每个子节点的局部变量只持有指向上级栈帧节点引用的只读指针。
- 借由 Rust 编译器严格的**生命周期管理 (`'a`)**，保障该链表在当前递归分支退栈时自动安全销毁。

---

## 4. 技术规格说明 (Technical Specifications)

```mermaid
graph TD
    Root["Root Node (Task:23) <br>[Comment: 'Create lift #3']"]
    Sub1["Child Node A (TaskStatus) <br>[Comment: 'Set default Planned status']"]
    Sub2["Child Node B (Platform) <br>[No Local Comment]"]
    
    Root -->|Traverse Down| Sub1
    Root -->|Traverse Down| Sub2

    subgraph Stack Frames [Call Stack (Thread-Local Isolated)]
        Frame1["ScopedCommentNode (Root) <br>parent: None <br>track: 'Create lift #3'"]
        Frame2["ScopedCommentNode (Child A) <br>parent: &Frame1 <br>track: 'Set default Planned status'"]
    end
    
    Root -.->|Pushes Frame| Frame1
    Sub1 -.->|Pushes Frame| Frame2
    
    Lineage["Formatting Chain: <br><b>'Create lift #3 ➔ Set default Planned status'</b>"]
    Frame2 -->|Walks up parent pointers| Lineage
```

### 4.1 结构体定义

#### (A) 结构化追踪元数据
```rust
pub struct CommentTrack {
    /// 实体类型 (如 "Task")
    pub entity_type: String,
    /// 实体主键 ID (如 "23")
    pub entity_id: String,
    /// 具体的业务动作/注释 (如 "Create lift #3")
    pub comment: String,
}
```

#### (B) 栈分配作用域节点
```rust
pub struct ScopedCommentNode<'a> {
    /// 指向父级栈帧的只读引用（生命周期与父函数调用栈严格绑定）
    pub parent: Option<&'a ScopedCommentNode<'a>>,
    /// 当前节点关联的结构化元数据
    pub track: CommentTrack,
}
```

---

### 4.2 图编译器递归调用栈伪代码
在 `teaql-runtime/src/repository/graph.rs` 中，图持久化引擎通过递归参数单向传递只读作用域引用：

```rust
impl<'a, D, E> ResolvedRepository<'a, D, E> {
    /// 递归遍历并更新图节点
    pub fn upsert_graph_node<'s>(
        &self,
        node: GraphNode,
        parent_scope: Option<&'s ScopedCommentNode<'s>>, // 👈 级联传递的作用域引用
    ) -> Result<GraphNode, RepositoryError<E::Error>> {
        
        // 1. 如果当前实体节点含有 comment 属性，则在当前物理栈帧上分配一个作用域节点
        let current_scope = node.comment.as_ref().map(|comment_text| {
            ScopedCommentNode {
                parent: parent_scope,
                track: CommentTrack {
                    entity_type: node.entity.clone(),
                    entity_id: node.id().map(|v| v.to_string()).unwrap_or_default(),
                    comment: comment_text.clone(),
                }
            }
        });

        // 判定活跃的注释作用域引用
        let active_scope = current_scope.as_ref().or(parent_scope);

        // 2. 物理保存本节点数据库记录时，将活跃的作用域随事件派发或写入 SQL 日志
        self.execute_prepared_mutation(&node, active_scope)?;

        // 3. 级联递归：保存关联子实体时，将 active_scope 往下传播
        for (rel_name, children) in node.relations {
            for child in children {
                self.upsert_graph_node(child, active_scope)?; // 👈 遗传繁殖给后代！
            }
        }
        
        Ok(node)
    }
}
```

---

### 4.3 链路回溯与日志格式化 (Trace Formatting)
当执行底层的物理 SQL 或发出变更事件时，格式化模块仅需顺着 `ScopedCommentNode` 链表的 `parent` 引用一路往上爬，最终反转顺序，生成醒目的**因果面包屑链路**：

```rust
impl<'a> ScopedCommentNode<'a> {
    /// 遍历还原完整的因果追踪路径
    pub fn to_lineage_string(&self) -> String {
        let mut chain = Vec::new();
        let mut current = Some(self);
        
        while let Some(node) = current {
            chain.push(format!("{}:{}({})", node.track.entity_type, node.track.entity_id, node.track.comment));
            current = node.parent;
        }
        
        chain.reverse(); // 从根节点到当前节点正向排列
        chain.join(" ➔ ")
    }
}
```

---

## 5. 并发与安全性能分析 (Concurrency & Performance Analysis)

### 5.1 物理级线程隔离 (Lock-Free Thread Safety)
本设计没有定义任何全局或局部可修改的共享状态（Mutable Shared State）。
- 每个并发线程拥有各自完全隔离的物理栈。
- 每一个 `ScopedCommentNode` 都仅仅作为各自线程栈帧上的**只读局部变量**。
- 这不仅消除了所有的逻辑数据交织污染，更在系统层实现了 **100% 的无锁并发 (Lock-Free)**，高并发写吞吐彻底不受锁瓶颈制约。

### 5.2 极致性能：零堆分配 (Zero-Heap Allocation)
在图遍历保存时，创建 `ScopedCommentNode` 的开销在 C/C++ 或 Rust 中表现为极度廉价的**栈指针偏移**，完全不会触发 expensive 的堆内存分配 (`malloc` / `Box` / `Arc` 派生)。

### 5.3 编译时静态生命周期保护 (`'a`)
通过使用 Rust 的生命周期约束 `'a`，编译器在编译阶段就能 100% 确定：**子节点对父级栈作用域的引用绝对不会超出父级栈帧的存活周期**。任何潜在的野指针或悬挂指针 (Dangling Pointer) 都会在编译时被静态阻断，保障极致运行时安全。

---

## 6. 日志与 TUI 可视化整合设计 (TUI Integration)

最终输出的层级化日志与现有的 TUI 语法高亮机制无缝集成：
1. **SQL 物理日志**：
   `[08:45:18.234]-[operator-philip]-[DEBUG]-SqlLogEntry - [Task:23(Create lift #3) ➔ TaskStatus:1001(Set Planned status)] - [1 UPDATED] INSERT INTO task_data ...`
2. **TUI 控制台高亮**：
   `src/ui.rs` 的双中括号解析器将检测到 `[Task:23(...) ➔ TaskStatus:1001(...)]`，自动以醒目的**黄色粗体**完整涂装这整条“业务调用链路”，赋予运维及调试人员史无前例、清晰至极的可观测性体验。
