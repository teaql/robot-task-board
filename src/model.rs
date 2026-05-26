use teaql_core::{Entity, EntityDescriptor, EntityError, Record, TeaqlEntity};
use teaql_provider_rusqlite::{MutationExecutorError, RusqliteMutationExecutor};
use teaql_runtime::{GraphTransactionBoundary, QueryExecutor};
use teaql_sql::CompiledQuery;

// Import generated entities
use robot_kanban::Task;

#[derive(Clone)]
pub struct LoggingExecutor {
    pub inner: RusqliteMutationExecutor,
}

impl QueryExecutor for LoggingExecutor {
    type Error = MutationExecutorError;

    fn fetch_all(
        &self,
        query: &CompiledQuery,
    ) -> Result<Vec<Record>, Self::Error> {
        QueryExecutor::fetch_all(&self.inner, query)
    }

    fn execute(&self, query: &CompiledQuery) -> Result<u64, Self::Error> {
        QueryExecutor::execute(&self.inner, query)
    }

    fn begin_transaction(&self) -> Result<GraphTransactionBoundary, Self::Error> {
        QueryExecutor::begin_transaction(&self.inner)
    }

    fn commit_transaction(&self) -> Result<(), Self::Error> {
        QueryExecutor::commit_transaction(&self.inner)
    }

    fn rollback_transaction(&self) -> Result<(), Self::Error> {
        QueryExecutor::rollback_transaction(&self.inner)
    }
}

pub struct TransitionCommand {
    pub target_status: String,
}

#[derive(Debug, Clone)]
pub struct DomainTask {
    pub task: Task,
}

impl TeaqlEntity for DomainTask {
    fn entity_descriptor() -> EntityDescriptor {
        Task::entity_descriptor()
    }
}

impl Entity for DomainTask {
    fn from_record(record: Record) -> Result<Self, EntityError> {
        let task = Task::from_record(record)?;
        Ok(Self { task })
    }

    fn into_record(self) -> Record {
        self.task.into_record()
    }
}

impl DomainTask {
    /// Domain behavior method showing DDD Aggregate Root logic.
    /// Transitions task status based on a TransitionCommand object.
    /// If target status is empty, it automatically moves to the next phase.
    pub fn transition_status(&mut self, cmd: &TransitionCommand) -> Result<Option<u64>, String> {
        let current_status = self.task.status_id();
        let target = cmd.target_status.trim().to_lowercase();

        let next_status_id = if target.is_empty() {
            // Planned -> Process -> Done
            if current_status == 1 {
                Some(2_u64)
            } else if current_status == 2 {
                Some(3_u64)
            } else {
                None
            }
        } else {
            match target.as_str() {
                "planned" => Some(1_u64),
                "process" => Some(2_u64),
                "done" => Some(3_u64),
                _ => return Err(format!("Invalid status '{}'. Use planned, process, done, or empty to move next.", cmd.target_status)),
            }
        };

        Ok(next_status_id)
    }
}
