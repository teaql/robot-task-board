use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "TaskExecutionLog", table = "task_execution_log_data")]
pub struct TaskExecutionLog {
#[teaql(id)]
    id: u64,

    action: String,

    detail: String,
#[teaql(version)]
    version: i64,
#[teaql(column = "task")]
    task_id: u64,
#[teaql(relation(target = "Task", local_key = "task_id", foreign_key = "id"))]
    task: Option<crate::Task>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
}

impl TaskExecutionLog {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            action: String::new(),
            detail: String::new(),
            version: 0_i64,
            task_id: 0_u64,
            task: None,
            dynamic: BTreeMap::new(),
            root,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TaskExecutionLog", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.task {
            entity.attach_root_recursive(root.clone());
        }
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn action(&self) -> String {
        self.changed_action().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.action.clone())
    }

    pub fn update_action(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.action = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.action.clone());
        self.root.set(self.entity_key(), "action", value);
        self
    }

    pub fn changed_action(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "action")
    }

    pub fn detail(&self) -> String {
        self.changed_detail().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.detail.clone())
    }

    pub fn update_detail(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.detail = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.detail.clone());
        self.root.set(self.entity_key(), "detail", value);
        self
    }

    pub fn changed_detail(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "detail")
    }

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }
    pub fn task_id(&self) -> u64 {
        self.changed_task_id().and_then(|value| value.try_u64()).unwrap_or(self.task_id)
    }

    pub fn update_task_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.task_id = value.try_u64().unwrap_or(self.task_id.clone());
        self.root.set(self.entity_key(), "task_id", value);
        self
    }

    pub fn changed_task_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "task_id")
    }
    pub fn task(&self) -> Option<&crate::Task> {
        self.task.as_ref()
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }
}

impl crate::AuditedSave for teaql_core::Audited<TaskExecutionLog> {
    async fn save(self, ctx: &teaql_runtime::UserContext) -> Result<teaql_runtime::GraphNode, crate::RuntimeRepositoryError> {
        let entity = self.into_entity();
        let repository = ctx
            .resolve_repository::<crate::runtime::DataServiceExecutor>("TaskExecutionLog")
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, entity).await
    }
}