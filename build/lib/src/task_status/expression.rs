use teaql_core::{SafeExpression, SmartList};

#[derive(Clone)]
pub struct TaskStatusExpression<R> {
    expression: SafeExpression<R, crate::TaskStatus>,
}

impl<R> TaskStatusExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, crate::TaskStatus>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<crate::TaskStatus> {
        self.expression.eval()
    }

    pub fn get_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.id())
    }

    pub fn get_name(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.name())
    }

    pub fn get_code(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.code())
    }

    pub fn get_color(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.color())
    }

    pub fn get_display_order(self) -> SafeExpression<R, rust_decimal::Decimal> {
        self.expression.apply(|value| value.display_order())
    }

    pub fn get_progress(self) -> SafeExpression<R, rust_decimal::Decimal> {
        self.expression.apply(|value| value.progress())
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
pub struct TaskStatusListExpression<R> {
    expression: SafeExpression<R, SmartList<crate::TaskStatus>>,
}

impl<R> TaskStatusListExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, SmartList<crate::TaskStatus>>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<SmartList<crate::TaskStatus>> {
        self.expression.eval()
    }

    pub fn size(self) -> SafeExpression<R, usize> {
        self.expression.size()
    }

    pub fn first(self) -> TaskStatusExpression<R> {
        TaskStatusExpression::new(self.expression.first())
    }

    pub fn get(self, index: usize) -> TaskStatusExpression<R> {
        TaskStatusExpression::new(self.expression.get(index))
    }
}