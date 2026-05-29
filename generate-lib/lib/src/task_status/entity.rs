use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "TaskStatus", table = "task_status_data")]
pub struct TaskStatus {
#[teaql(id)]
    id: u64,

    name: String,

    code: String,

    color: String,

    display_order: rust_decimal::Decimal,

    progress: rust_decimal::Decimal,
#[teaql(version)]
    version: i64,
#[teaql(relation(target = "Task", local_key = "id", foreign_key = "status_id", many))]
    task_list: SmartList<crate::Task>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
}

impl TaskStatus {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            code: String::new(),
            color: String::new(),
            display_order: rust_decimal::Decimal::ZERO,
            progress: rust_decimal::Decimal::ZERO,
            version: 0_i64,
            task_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TaskStatus", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        for entity in &mut self.task_list {
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

    pub fn code(&self) -> String {
        self.changed_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.code.clone())
    }

    pub fn update_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.code = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.code.clone());
        self.root.set(self.entity_key(), "code", value);
        self
    }

    pub fn changed_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "code")
    }

    pub fn color(&self) -> String {
        self.changed_color().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.color.clone())
    }

    pub fn update_color(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.color = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.color.clone());
        self.root.set(self.entity_key(), "color", value);
        self
    }

    pub fn changed_color(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "color")
    }

    pub fn display_order(&self) -> rust_decimal::Decimal {
        self.changed_display_order().and_then(|value| value.try_decimal()).unwrap_or(self.display_order)
    }

    pub fn update_display_order(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.display_order = value.try_decimal().unwrap_or(self.display_order.clone());
        self.root.set(self.entity_key(), "display_order", value);
        self
    }

    pub fn changed_display_order(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "display_order")
    }

    pub fn progress(&self) -> rust_decimal::Decimal {
        self.changed_progress().and_then(|value| value.try_decimal()).unwrap_or(self.progress)
    }

    pub fn update_progress(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.progress = value.try_decimal().unwrap_or(self.progress.clone());
        self.root.set(self.entity_key(), "progress", value);
        self
    }

    pub fn changed_progress(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "progress")
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
    pub fn task_list(&self) -> &SmartList<crate::Task> {
        &self.task_list
    }

    pub async fn save<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::TaskStatusRepository<'a>>>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        let repository = ctx
            .task_status_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self)
    }
}