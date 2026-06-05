// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn platform<'a>(value: &'a crate::Platform) -> crate::PlatformExpression<'a> {
        crate::PlatformExpression::new(teaql_core::eval::EvalResult::Value(value))
    }

    pub fn task_status<'a>(value: &'a crate::TaskStatus) -> crate::TaskStatusExpression<'a> {
        crate::TaskStatusExpression::new(teaql_core::eval::EvalResult::Value(value))
    }

    pub fn task<'a>(value: &'a crate::Task) -> crate::TaskExpression<'a> {
        crate::TaskExpression::new(teaql_core::eval::EvalResult::Value(value))
    }

    pub fn task_execution_log<'a>(value: &'a crate::TaskExecutionLog) -> crate::TaskExecutionLogExpression<'a> {
        crate::TaskExecutionLogExpression::new(teaql_core::eval::EvalResult::Value(value))
    }
}

#[derive(Clone)]
pub struct ValueExpression<'a, T> {
    result: teaql_core::eval::EvalResult<T>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: Clone> ValueExpression<'a, T> {
    pub fn new(result: teaql_core::eval::EvalResult<T>) -> Self {
        Self { result, _phantom: std::marker::PhantomData }
    }

    fn resolve(self) -> Option<T> {
        match self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { missing_path } => {
                panic!("Logic Bug! You forgot to query the '{}' field or relation!", missing_path);
            }
        }
    }

    pub fn eval(self) -> Option<T> {
        self.resolve()
    }

    pub fn unwrap(self) -> T {
        self.resolve().expect("Value was legitimately null in database!")
    }

    pub fn or_else(self, default_value: T) -> T {
        self.eval().unwrap_or(default_value)
    }

    pub fn or_else_with(self, default_fn: impl FnOnce() -> T) -> T {
        self.eval().unwrap_or_else(default_fn)
    }

    pub fn or_default(self) -> T where T: Default {
        self.eval().unwrap_or_default()
    }
}

