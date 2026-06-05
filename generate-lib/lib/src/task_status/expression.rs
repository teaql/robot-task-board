#[derive(Clone)]
pub struct TaskStatusExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TaskStatus>,
}

impl<'a> TaskStatusExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TaskStatus>) -> Self {
        Self { result }
    }

    fn resolve(&self) -> Option<&'a crate::TaskStatus> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { missing_path } => {
                panic!("Logic Bug! You forgot to query the '{}' relation!", missing_path);
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TaskStatus> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TaskStatus {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next)
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next)
    }

    pub fn get_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("code", |entity| entity.eval_code());
        crate::ValueExpression::new(next)
    }

    pub fn get_color(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("color", |entity| entity.eval_color());
        crate::ValueExpression::new(next)
    }

    pub fn get_display_order(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("display_order", |entity| entity.eval_display_order());
        crate::ValueExpression::new(next)
    }

    pub fn get_progress(self) -> crate::ValueExpression<'a, rust_decimal::Decimal> {
        let next = self.result.and_then("progress", |entity| entity.eval_progress());
        crate::ValueExpression::new(next)
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next)
    }
    pub fn get_task_list(self) -> crate::TaskListExpression<'a> {
        let next = self.result.and_then("task_list", |entity| entity.eval_task_list());
        crate::TaskListExpression::new(next)
    }
}

#[derive(Clone)]
pub struct TaskStatusListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaskStatus>>,
}

impl<'a> TaskStatusListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaskStatus>>) -> Self {
        Self { result }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TaskStatus>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { missing_path } => {
                panic!("Logic Bug! You forgot to query the '{}' relation!", missing_path);
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TaskStatus>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TaskStatus> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next)
    }

    pub fn first(&self) -> crate::TaskStatusExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaskStatusExpression::new(next)
    }

    pub fn get(&self, index: usize) -> crate::TaskStatusExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaskStatusExpression::new(next)
    }
}