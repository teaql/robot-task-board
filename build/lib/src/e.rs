use teaql_core::SafeExpression;

pub struct E;

impl E {
    pub fn object(value: crate::Object) -> crate::ObjectExpression<crate::Object> {
        crate::ObjectExpression::new(SafeExpression::value(value))
    }

    pub fn attribute(value: crate::Attribute) -> crate::AttributeExpression<crate::Attribute> {
        crate::AttributeExpression::new(SafeExpression::value(value))
    }
}