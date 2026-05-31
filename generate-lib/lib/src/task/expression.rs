use teaql_core::{SafeExpression, SmartList};

#[derive(Clone)]
pub struct TaskExpression<R> {
    expression: SafeExpression<R, crate::Task>,
}

impl<R> TaskExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, crate::Task>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<crate::Task> {
        self.expression.eval()
    }

    pub fn get_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.id())
    }

    pub fn get_name(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.name())
    }

    pub fn get_version(self) -> SafeExpression<R, i64> {
        self.expression.apply(|value| value.version())
    }
    pub fn get_status_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.status_id())
    }

    pub fn get_platform_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.platform_id())
    }
    pub fn get_status(self) -> crate::TaskStatusExpression<R> {
        crate::TaskStatusExpression::new(
            self.expression.apply_optional(|value| value.status().cloned())
        )
    }

    pub fn get_platform(self) -> crate::PlatformExpression<R> {
        crate::PlatformExpression::new(
            self.expression.apply_optional(|value| value.platform().cloned())
        )
    }
    pub fn status_is_planned(self) -> SafeExpression<R, bool> {
        self.expression.apply(|value| value.status_is_planned())
    }

    pub fn status_is_ready(self) -> SafeExpression<R, bool> {
        self.expression.apply(|value| value.status_is_ready())
    }

    pub fn status_is_executing(self) -> SafeExpression<R, bool> {
        self.expression.apply(|value| value.status_is_executing())
    }

    pub fn status_is_verified(self) -> SafeExpression<R, bool> {
        self.expression.apply(|value| value.status_is_verified())
    }
    pub fn get_task_execution_log_list(self) -> crate::TaskExecutionLogListExpression<R> {
        crate::TaskExecutionLogListExpression::new(
            self.expression.apply(|value| value.task_execution_log_list().clone())
        )
    }
}

#[derive(Clone)]
pub struct TaskListExpression<R> {
    expression: SafeExpression<R, SmartList<crate::Task>>,
}

impl<R> TaskListExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, SmartList<crate::Task>>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<SmartList<crate::Task>> {
        self.expression.eval()
    }

    pub fn size(self) -> SafeExpression<R, usize> {
        self.expression.size()
    }

    pub fn first(self) -> TaskExpression<R> {
        TaskExpression::new(self.expression.first())
    }

    pub fn get(self, index: usize) -> TaskExpression<R> {
        TaskExpression::new(self.expression.get(index))
    }
}