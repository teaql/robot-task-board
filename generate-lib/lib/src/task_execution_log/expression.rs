#[derive(Clone)]
pub struct TaskExecutionLogExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TaskExecutionLog>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaskExecutionLogExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TaskExecutionLog>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TaskExecutionLog> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TaskExecutionLog> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TaskExecutionLog {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_action(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("action", |entity| entity.eval_action());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_detail(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("detail", |entity| entity.eval_detail());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_task_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("task_id", |entity| entity.eval_task_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_task(self) -> crate::TaskExpression<'a> {
        let next = self.result.and_then("task", |entity| entity.eval_task());
        crate::TaskExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TaskExecutionLogListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaskExecutionLog>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaskExecutionLogListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaskExecutionLog>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TaskExecutionLog>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TaskExecutionLog>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TaskExecutionLog> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TaskExecutionLogExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaskExecutionLogExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TaskExecutionLogExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaskExecutionLogExpression::new(next, self.root_desc.clone())
    }
}