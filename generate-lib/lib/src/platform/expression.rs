#[derive(Clone)]
pub struct PlatformExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Platform>,
}

impl<'a> PlatformExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Platform>) -> Self {
        Self { result }
    }

    fn resolve(&self) -> Option<&'a crate::Platform> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { missing_path } => {
                panic!("Logic Bug! You forgot to query the '{}' relation!", missing_path);
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Platform> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Platform {
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

    pub fn get_founded(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("founded", |entity| entity.eval_founded());
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
pub struct PlatformListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Platform>>,
}

impl<'a> PlatformListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Platform>>) -> Self {
        Self { result }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Platform>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { missing_path } => {
                panic!("Logic Bug! You forgot to query the '{}' relation!", missing_path);
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Platform>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Platform> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next)
    }

    pub fn first(&self) -> crate::PlatformExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PlatformExpression::new(next)
    }

    pub fn get(&self, index: usize) -> crate::PlatformExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PlatformExpression::new(next)
    }
}