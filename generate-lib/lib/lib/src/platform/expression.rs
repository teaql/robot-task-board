use teaql_core::{SafeExpression, SmartList};

#[derive(Clone)]
pub struct PlatformExpression<R> {
    expression: SafeExpression<R, crate::Platform>,
}

impl<R> PlatformExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, crate::Platform>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<crate::Platform> {
        self.expression.eval()
    }

    pub fn get_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.id())
    }

    pub fn get_name(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.name())
    }

    pub fn get_founded(self) -> SafeExpression<R, chrono::DateTime<chrono::Utc>> {
        self.expression.apply(|value| value.founded())
    }

    pub fn get_version(self) -> SafeExpression<R, i64> {
        self.expression.apply(|value| value.version())
    }
    pub fn get_task_list(self) -> crate::TaskListExpression<R> {
        crate::TaskListExpression::new(
            self.expression.apply(|value| value.task_list().clone())
        )
    }
}

#[derive(Clone)]
pub struct PlatformListExpression<R> {
    expression: SafeExpression<R, SmartList<crate::Platform>>,
}

impl<R> PlatformListExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, SmartList<crate::Platform>>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<SmartList<crate::Platform>> {
        self.expression.eval()
    }

    pub fn size(self) -> SafeExpression<R, usize> {
        self.expression.size()
    }

    pub fn first(self) -> PlatformExpression<R> {
        PlatformExpression::new(self.expression.first())
    }

    pub fn get(self, index: usize) -> PlatformExpression<R> {
        PlatformExpression::new(self.expression.get(index))
    }
}