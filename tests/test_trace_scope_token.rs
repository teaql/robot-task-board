use std::sync::Arc;

use teaql_core::{Record, TraceNode, Value};
use teaql_runtime::{
    GraphMutationKind, GraphMutationPlan, TraceScopeToken,
};

// ---------------------------------------------------------------------------
// 1. 单节点：一个 Token 没有父节点，recover 出来只有自己
// ---------------------------------------------------------------------------
#[test]
fn test_single_token_recover() {
    let token = TraceScopeToken {
        parent: None,
        track: TraceNode {
            entity_type: "Order".to_owned(),
            entity_id: Some(100),
            comment: "Create order".to_owned(),
        },
        node_index: 0,
    };

    let chain = token.recover_trace_chain();
    assert_eq!(chain.len(), 1);
    assert_eq!(chain[0].entity_type, "Order");
    assert_eq!(chain[0].entity_id, Some(100));
    assert_eq!(chain[0].comment, "Create order");
}

// ---------------------------------------------------------------------------
// 2. 父子链：Order -> OrderItem，recover 出来顺序是 [Order, OrderItem]
// ---------------------------------------------------------------------------
#[test]
fn test_parent_child_token_recover() {
    let parent_token = Arc::new(TraceScopeToken {
        parent: None,
        track: TraceNode {
            entity_type: "Order".to_owned(),
            entity_id: Some(100),
            comment: "Save order".to_owned(),
        },
        node_index: 0,
    });

    let child_token = TraceScopeToken {
        parent: Some(parent_token.clone()),
        track: TraceNode {
            entity_type: "OrderItem".to_owned(),
            entity_id: Some(201),
            comment: "Cascade save item".to_owned(),
        },
        node_index: 1,
    };

    let chain = child_token.recover_trace_chain();
    assert_eq!(chain.len(), 2);
    // 顺序必须是从根到叶：Order -> OrderItem
    assert_eq!(chain[0].entity_type, "Order");
    assert_eq!(chain[0].comment, "Save order");
    assert_eq!(chain[1].entity_type, "OrderItem");
    assert_eq!(chain[1].comment, "Cascade save item");
}

// ---------------------------------------------------------------------------
// 3. 三层深度链：Order -> OrderItem -> ItemDetail
// ---------------------------------------------------------------------------
#[test]
fn test_deep_chain_recover() {
    let root = Arc::new(TraceScopeToken {
        parent: None,
        track: TraceNode {
            entity_type: "Order".to_owned(),
            entity_id: Some(1),
            comment: "Root save".to_owned(),
        },
        node_index: 0,
    });

    let mid = Arc::new(TraceScopeToken {
        parent: Some(root.clone()),
        track: TraceNode {
            entity_type: "OrderItem".to_owned(),
            entity_id: Some(10),
            comment: "Mid cascade".to_owned(),
        },
        node_index: 1,
    });

    let leaf = TraceScopeToken {
        parent: Some(mid.clone()),
        track: TraceNode {
            entity_type: "ItemDetail".to_owned(),
            entity_id: Some(100),
            comment: "Leaf cascade".to_owned(),
        },
        node_index: 2,
    };

    let chain = leaf.recover_trace_chain();
    assert_eq!(chain.len(), 3);
    assert_eq!(chain[0].entity_type, "Order");
    assert_eq!(chain[1].entity_type, "OrderItem");
    assert_eq!(chain[2].entity_type, "ItemDetail");
}

// ---------------------------------------------------------------------------
// 4. 空 comment 的节点会被跳过（与 ScopedCommentNode 行为一致）
// ---------------------------------------------------------------------------
#[test]
fn test_empty_comment_skipped() {
    let root = Arc::new(TraceScopeToken {
        parent: None,
        track: TraceNode {
            entity_type: "Order".to_owned(),
            entity_id: Some(1),
            comment: "Root".to_owned(),
        },
        node_index: 0,
    });

    // 中间节点没有 comment
    let mid = Arc::new(TraceScopeToken {
        parent: Some(root.clone()),
        track: TraceNode {
            entity_type: "OrderItem".to_owned(),
            entity_id: Some(10),
            comment: "".to_owned(), // 空 comment
        },
        node_index: 1,
    });

    let leaf = TraceScopeToken {
        parent: Some(mid.clone()),
        track: TraceNode {
            entity_type: "ItemDetail".to_owned(),
            entity_id: Some(100),
            comment: "Leaf".to_owned(),
        },
        node_index: 2,
    };

    let chain = leaf.recover_trace_chain();
    // 中间那个空 comment 被跳过了
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].entity_type, "Order");
    assert_eq!(chain[1].entity_type, "ItemDetail");
}

// ---------------------------------------------------------------------------
// 5. 兄弟节点共享同一个父 Token（Arc 共享所有权）
// ---------------------------------------------------------------------------
#[test]
fn test_sibling_tokens_share_parent() {
    let parent = Arc::new(TraceScopeToken {
        parent: None,
        track: TraceNode {
            entity_type: "Order".to_owned(),
            entity_id: Some(1),
            comment: "Save order".to_owned(),
        },
        node_index: 0,
    });

    let sibling_a = TraceScopeToken {
        parent: Some(parent.clone()),
        track: TraceNode {
            entity_type: "OrderItem".to_owned(),
            entity_id: Some(10),
            comment: "Item A".to_owned(),
        },
        node_index: 1,
    };

    let sibling_b = TraceScopeToken {
        parent: Some(parent.clone()),
        track: TraceNode {
            entity_type: "OrderItem".to_owned(),
            entity_id: Some(20),
            comment: "Item B".to_owned(),
        },
        node_index: 2,
    };

    let chain_a = sibling_a.recover_trace_chain();
    let chain_b = sibling_b.recover_trace_chain();

    // 两条链路的根都是同一个 Order
    assert_eq!(chain_a[0].comment, "Save order");
    assert_eq!(chain_b[0].comment, "Save order");

    // 但叶子不同
    assert_eq!(chain_a[1].comment, "Item A");
    assert_eq!(chain_b[1].comment, "Item B");

    // 验证 Arc 确实是共享的（引用计数 = parent变量 + sibling_a + sibling_b = 3）
    assert_eq!(Arc::strong_count(&parent), 3);
}

// ---------------------------------------------------------------------------
// 6. 与 GraphMutationPlan 集成：push 后 item 上的 scope_token 能正确恢复
// ---------------------------------------------------------------------------
#[test]
fn test_plan_item_carries_token() {
    let mut plan = GraphMutationPlan::default();

    let token = Arc::new(TraceScopeToken {
        parent: None,
        track: TraceNode {
            entity_type: "Task".to_owned(),
            entity_id: Some(42),
            comment: "Move task to ready".to_owned(),
        },
        node_index: 0,
    });

    let mut values = Record::new();
    values.insert("id".to_owned(), Value::I64(42));
    values.insert("status".to_owned(), Value::I64(2));

    plan.push(
        "Task",
        GraphMutationKind::Update,
        values,
        vec!["status".to_owned()],
        Some(token.clone()),
        None,
    );

    assert_eq!(plan.items.len(), 1);
    assert_eq!(plan.items[0].item_index, 0);

    // 从 PlanItem 身上恢复 trace chain
    let recovered = plan.items[0]
        .scope_token
        .as_ref()
        .expect("should have scope_token")
        .recover_trace_chain();

    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].comment, "Move task to ready");
}

// ---------------------------------------------------------------------------
// 7. 没有 scope_token 的 PlanItem（比如 Reference 节点），不会 panic
// ---------------------------------------------------------------------------
#[test]
fn test_plan_item_without_token() {
    let mut plan = GraphMutationPlan::default();

    plan.push(
        "Task",
        GraphMutationKind::Reference,
        Record::new(),
        Vec::new(),
        None, // 没有 token
        None,
    );

    assert_eq!(plan.items.len(), 1);
    assert_eq!(plan.items[0].item_index, 0);
    assert!(plan.items[0].scope_token.is_none());
}

// ---------------------------------------------------------------------------
// 8. item_index 自增验证
// ---------------------------------------------------------------------------
#[test]
fn test_item_index_monotonic() {
    let mut plan = GraphMutationPlan::default();

    for i in 0..5 {
        plan.push(
            "Task",
            GraphMutationKind::Create,
            Record::new(),
            Vec::new(),
            None,
            None,
        );
    }

    for (i, item) in plan.items.iter().enumerate() {
        assert_eq!(item.item_index, i as u64);
    }
    assert_eq!(plan.next_item_index, 5);
}
