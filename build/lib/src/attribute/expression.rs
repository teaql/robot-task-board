use teaql_core::{SafeExpression, SmartList};

#[derive(Clone)]
pub struct AttributeExpression<R> {
    expression: SafeExpression<R, crate::Attribute>,
}

impl<R> AttributeExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, crate::Attribute>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<crate::Attribute> {
        self.expression.eval()
    }

    pub fn get_id(self) -> SafeExpression<R, u64> {
        self.expression.apply(|value| value.id())
    }

    pub fn get_name(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.name())
    }

    pub fn get_type_field(self) -> SafeExpression<R, String> {
        self.expression.apply(|value| value.type_field())
    }

    pub fn get_max(self) -> SafeExpression<R, i32> {
        self.expression.apply(|value| value.max())
    }

    pub fn get_version(self) -> SafeExpression<R, i64> {
        self.expression.apply(|value| value.version())
    }
}

#[derive(Clone)]
pub struct AttributeListExpression<R> {
    expression: SafeExpression<R, SmartList<crate::Attribute>>,
}

impl<R> AttributeListExpression<R>
where
    R: Send + Sync + 'static,
{
    pub fn new(expression: SafeExpression<R, SmartList<crate::Attribute>>) -> Self {
        Self { expression }
    }

    pub fn eval(&self) -> Option<SmartList<crate::Attribute>> {
        self.expression.eval()
    }

    pub fn size(self) -> SafeExpression<R, usize> {
        self.expression.size()
    }

    pub fn first(self) -> AttributeExpression<R> {
        AttributeExpression::new(self.expression.first())
    }

    pub fn get(self, index: usize) -> AttributeExpression<R> {
        AttributeExpression::new(self.expression.get(index))
    }
}