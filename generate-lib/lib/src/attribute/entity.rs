use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Attribute", table = "attribute_data")]
pub struct Attribute {
#[teaql(id)]
    id: u64,

    name: String,
#[teaql(column = "type")]
    type_field: String,

    max: i32,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
}

impl Attribute {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            type_field: String::new(),
            max: 0_i32,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Attribute", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
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

    pub fn type_field(&self) -> String {
        self.changed_type_field().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.type_field.clone())
    }

    pub fn update_type_field(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.type_field = value.try_text().map(|value| value.to_owned()).unwrap_or_else(|| self.type_field.clone());
        self.root.set(self.entity_key(), "type", value);
        self
    }

    pub fn changed_type_field(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "type")
    }

    pub fn max(&self) -> i32 {
        self.changed_max().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.max)
    }

    pub fn update_max(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.max = value.try_i64().map(|value| value as i32).unwrap_or(self.max.clone());
        self.root.set(self.entity_key(), "max", value);
        self
    }

    pub fn changed_max(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "max")
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

    pub async fn save<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::AttributeRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .attribute_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self)
    }
}