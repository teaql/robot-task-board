#[derive(Clone)]
pub struct TaskExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Task>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaskExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Task>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Task> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Task> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Task {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("status_id", |entity| entity.eval_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tenant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("tenant_id", |entity| entity.eval_tenant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_status(self) -> crate::TaskStatusExpression<'a> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::TaskStatusExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tenant(self) -> crate::TenantExpression<'a> {
        let next = self.result.and_then("tenant", |entity| entity.eval_tenant());
        crate::TenantExpression::new(next, self.root_desc.clone())
    }
    pub fn status_is_planned(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_planned())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_ready(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_ready())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_executing(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_executing())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_verified(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_verified())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_task_execution_log_list(self) -> crate::TaskExecutionLogListExpression<'a> {
        let next = self.result.and_then("task_execution_log_list", |entity| entity.eval_task_execution_log_list());
        crate::TaskExecutionLogListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TaskListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Task>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaskListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Task>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Task>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Task>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Task> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TaskExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaskExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TaskExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaskExpression::new(next, self.root_desc.clone())
    }
}