use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Task", table = "task_data", data_service = "rusqlite")]
pub struct Task {
#[teaql(id)]
    id: u64,

// @source model.xml:41
    name: String,
#[teaql(version)]
    version: i64,
// @source model.xml:41
#[teaql(column = "status")]
    status_id: u64,

// @source model.xml:41
#[teaql(column = "platform")]
    platform_id: u64,
// @source model.xml:41
#[teaql(relation(target = "TaskStatus", local_key = "status_id", foreign_key = "id"))]
    status: Option<crate::TaskStatus>,

// @source model.xml:41
#[teaql(relation(target = "Platform", local_key = "platform_id", foreign_key = "id"))]
    platform: Option<crate::Platform>,
#[teaql(relation(target = "TaskExecutionLog", local_key = "id", foreign_key = "task_id", many))]
    task_execution_log_list: SmartList<crate::TaskExecutionLog>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Task {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            version: 0_i64,
            status_id: 0_u64,
            platform_id: 0_u64,
            status: None,
            platform: None,
            task_execution_log_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Task", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.status {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.platform {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.task_execution_log_list {
            entity.attach_root_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
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

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
                }}

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

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn status_id(&self) -> u64 {
        self.changed_status_id().and_then(|value| value.try_u64()).unwrap_or(self.status_id)
    }

    pub(crate) fn update_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status_id = value.try_u64().unwrap_or(self.status_id.clone());
        self.root.set(self.entity_key(), "status_id", value);
        self
    }

    pub fn changed_status_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status_id")
    }

    pub fn eval_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status_id())
                }}

    pub fn platform_id(&self) -> u64 {
        self.changed_platform_id().and_then(|value| value.try_u64()).unwrap_or(self.platform_id)
    }

    pub fn update_platform_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.platform_id = value.try_u64().unwrap_or(self.platform_id.clone());
        self.root.set(self.entity_key(), "platform_id", value);
        self
    }

    pub fn changed_platform_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "platform_id")
    }

    pub fn eval_platform_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("platform_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_id".to_string(), attempted_path: "platform_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.platform_id())
                }}
    pub fn update_status_to_planned(&mut self) -> &mut Self {
        self.update_status_id(1001_u64)
    }

    pub fn status_is_planned(&self) -> bool {
        self.status_id() == 1001_u64
    }
    pub fn update_status_to_ready(&mut self) -> &mut Self {
        self.update_status_id(1002_u64)
    }

    pub fn status_is_ready(&self) -> bool {
        self.status_id() == 1002_u64
    }
    pub fn update_status_to_executing(&mut self) -> &mut Self {
        self.update_status_id(1003_u64)
    }

    pub fn status_is_executing(&self) -> bool {
        self.status_id() == 1003_u64
    }
    pub fn update_status_to_verified(&mut self) -> &mut Self {
        self.update_status_id(1004_u64)
    }

    pub fn status_is_verified(&self) -> bool {
        self.status_id() == 1004_u64
    }
    pub fn status(&self) -> Option<&crate::TaskStatus> {
        self.status.as_ref()
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<&crate::TaskStatus> {
        if !self.is_loaded("status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
        } else {
            match &self.status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn platform(&self) -> Option<&crate::Platform> {
        self.platform.as_ref()
    }

    pub fn eval_platform(&self) -> teaql_core::eval::EvalResult<&crate::Platform> {
        if !self.is_loaded("platform") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform".to_string(), attempted_path: "platform".to_string() }
        } else {
            match &self.platform {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn task_execution_log_list(&self) -> &SmartList<crate::TaskExecutionLog> {
        &self.task_execution_log_list
    }

    pub fn task_execution_log_list_mut(&mut self) -> &mut SmartList<crate::TaskExecutionLog> {
        &mut self.task_execution_log_list
    }

    pub fn eval_task_execution_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TaskExecutionLog>> {
        if !self.is_loaded("task_execution_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "task_execution_log_list".to_string(), attempted_path: "task_execution_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.task_execution_log_list)
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub async fn save<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::TaskRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .task_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}