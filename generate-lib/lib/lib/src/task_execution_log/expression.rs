use teaql_core::{SafeExpression, SmartList};

#[derive(Clone)]
pub struct TaskExecutionLogExpression<R> {
    expression: SafeExpression<R, crate::TaskExecutionLog>,
}

impl<R> TaskExecutionLogExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, crate::TaskExecutionLog>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<crate::TaskExecutionLog> {
        self.expression.eval()
    }

    pub fn get_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.id())
    }

    pub fn get_action(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.action())
    }

    pub fn get_detail(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.detail())
    }

    pub fn get_version(self) -> SafeExpression<R, i64> {
        self.expression.apply(|value| value.version())
    }
    pub fn get_task_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.task_id())
    }
    pub fn get_task(self) -> crate::TaskExpression<R> {
        crate::TaskExpression::new(
            self.expression.apply_optional(|value| value.task().cloned())
        )
    }
}

#[derive(Clone)]
pub struct TaskExecutionLogListExpression<R> {
    expression: SafeExpression<R, SmartList<crate::TaskExecutionLog>>,
}

impl<R> TaskExecutionLogListExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, SmartList<crate::TaskExecutionLog>>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<SmartList<crate::TaskExecutionLog>> {
        self.expression.eval()
    }

    pub fn size(self) -> SafeExpression<R, usize> {
        self.expression.size()
    }

    pub fn first(self) -> TaskExecutionLogExpression<R> {
        TaskExecutionLogExpression::new(self.expression.first())
    }

    pub fn get(self, index: usize) -> TaskExecutionLogExpression<R> {
        TaskExecutionLogExpression::new(self.expression.get(index))
    }
}