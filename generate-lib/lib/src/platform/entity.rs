use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Platform", table = "platform_data", data_service = "rusqlite")]
pub struct Platform {
#[teaql(id)]
    id: u64,

// @source models/main.xml:9
    name: String,

// @source models/main.xml:9
    founded: chrono::DateTime<chrono::Utc>,

// @source models/main.xml:9
    user_email: String,
#[teaql(version)]
    version: i64,
#[teaql(relation(target = "TaskStatus", local_key = "id", foreign_key = "platform_id", many))]
    task_status_list: SmartList<crate::TaskStatus>,
#[teaql(relation(target = "Task", local_key = "id", foreign_key = "platform_id", many))]
    task_list: SmartList<crate::Task>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Platform {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            founded: chrono::Utc::now(),
            user_email: String::new(),
            version: 0_i64,
            task_status_list: Default::default(),
            task_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Platform", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        for entity in &mut self.task_status_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.task_list {
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

    pub fn founded(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_founded().and_then(|value| value.try_timestamp()).unwrap_or(self.founded)
    }

    pub fn update_founded(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.founded = value.try_timestamp().unwrap_or(self.founded.clone());
        self.root.set(self.entity_key(), "founded", value);
        self
    }

    pub fn changed_founded(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "founded")
    }

    pub fn eval_founded(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("founded") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "founded".to_string(), attempted_path: "founded".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.founded())
                }}

    pub fn user_email(&self) -> String {
        self.changed_user_email().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.user_email.clone())
    }

    pub fn update_user_email(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.user_email = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.user_email.clone());
        self.root.set(self.entity_key(), "user_email", value);
        self
    }

    pub fn changed_user_email(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "user_email")
    }

    pub fn eval_user_email(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("user_email") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_email".to_string(), attempted_path: "user_email".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.user_email())
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
    pub fn task_status_list(&self) -> &SmartList<crate::TaskStatus> {
        &self.task_status_list
    }

    pub fn task_status_list_mut(&mut self) -> &mut SmartList<crate::TaskStatus> {
        &mut self.task_status_list
    }

    pub fn eval_task_status_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TaskStatus>> {
        if !self.is_loaded("task_status_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "task_status_list".to_string(), attempted_path: "task_status_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.task_status_list)
        }
    }

    pub fn task_list(&self) -> &SmartList<crate::Task> {
        &self.task_list
    }

    pub fn task_list_mut(&mut self) -> &mut SmartList<crate::Task> {
        &mut self.task_list
    }

    pub fn eval_task_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Task>> {
        if !self.is_loaded("task_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "task_list".to_string(), attempted_path: "task_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.task_list)
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

    pub(crate) async fn save<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::PlatformRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .platform_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

