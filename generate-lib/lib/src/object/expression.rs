use teaql_core::{SafeExpression, SmartList};

#[derive(Clone)]
pub struct ObjectExpression<R> {
    expression: SafeExpression<R, crate::Object>,
}

impl<R> ObjectExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, crate::Object>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<crate::Object> {
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
}

#[derive(Clone)]
pub struct ObjectListExpression<R> {
    expression: SafeExpression<R, SmartList<crate::Object>>,
}

impl<R> ObjectListExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, SmartList<crate::Object>>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<SmartList<crate::Object>> {
        self.expression.eval()
    }

    pub fn size(self) -> SafeExpression<R, usize> {
        self.expression.size()
    }

    pub fn first(self) -> ObjectExpression<R> {
        ObjectExpression::new(self.expression.first())
    }

    pub fn get(self, index: usize) -> ObjectExpression<R> {
        ObjectExpression::new(self.expression.get(index))
    }
}