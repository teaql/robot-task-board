use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        Self { inner, purpose: purpose.into() }
    }
}

pub struct Q;

impl Q {
    pub fn platforms() -> PlatformRequest {
        PlatformRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_minimal() -> PlatformRequest {
        PlatformRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_with_children() -> PlatformRequest {
        PlatformRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn task_status() -> TaskStatusRequest {
        TaskStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn task_status_minimal() -> TaskStatusRequest {
        TaskStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn task_status_with_children() -> TaskStatusRequest {
        TaskStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tasks() -> TaskRequest {
        TaskRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tasks_minimal() -> TaskRequest {
        TaskRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tasks_with_children() -> TaskRequest {
        TaskRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn task_execution_logs() -> TaskExecutionLogRequest {
        TaskExecutionLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn task_execution_logs_minimal() -> TaskExecutionLogRequest {
        TaskExecutionLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn task_execution_logs_with_children() -> TaskExecutionLogRequest {
        TaskExecutionLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}