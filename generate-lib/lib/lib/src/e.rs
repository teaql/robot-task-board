use teaql_core::SafeExpression;

pub struct E;

impl E {
    pub fn platform(value: crate::Platform) -> crate::PlatformExpression<crate::Platform> {
        crate::PlatformExpression::new(SafeExpression::value(value))
    }

    pub fn task_status(value: crate::TaskStatus) -> crate::TaskStatusExpression<crate::TaskStatus> {
        crate::TaskStatusExpression::new(SafeExpression::value(value))
    }

    pub fn task(value: crate::Task) -> crate::TaskExpression<crate::Task> {
        crate::TaskExpression::new(SafeExpression::value(value))
    }

    pub fn task_execution_log(value: crate::TaskExecutionLog) -> crate::TaskExecutionLogExpression<crate::TaskExecutionLog> {
        crate::TaskExecutionLogExpression::new(SafeExpression::value(value))
    }
}