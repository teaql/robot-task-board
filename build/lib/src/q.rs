use teaql_core::Expr;

use crate::*;

pub struct Q;

impl Q {
    pub fn objects() -> ObjectRequest {
        ObjectRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn objects_minimal() -> ObjectRequest {
        ObjectRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn objects_with_children() -> ObjectRequest {
        ObjectRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn attributes() -> AttributeRequest {
        AttributeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attributes_minimal() -> AttributeRequest {
        AttributeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attributes_with_children() -> AttributeRequest {
        AttributeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}