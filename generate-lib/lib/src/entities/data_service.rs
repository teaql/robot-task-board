use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataService {
    pub id: u64,
    pub name: Option<String>,
    pub version: i64,
    pub comment: String,
    pub deleted: bool,
}

impl DataService {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            version: 0,
            comment: String::new(),
            deleted: false,
        }
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn update_id(&mut self, id: impl Into<u64>) -> &mut Self {
        self.id = id.into();
        self
    }

    pub fn version(&self) -> i64 {
        self.version
    }

    pub fn update_version(&mut self, version: i64) -> &mut Self {
        self.version = version;
        self
    }

    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    pub fn mark_as_delete(&mut self) {
        self.deleted = true;
    }
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
    pub fn update_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.name = Some(value.into());
        self
    }

    pub fn save(mut self, ctx: &teaql_runtime::UserContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, std::io::Error>> + Send + '_>> {
        Box::pin(async move {
            let repo = ctx.resolve_repository::<crate::ServiceRuntimeExecutor>("_data_service")
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            let mut node = teaql_runtime::GraphNode::new("_data_service");
            if self.deleted {
                node.operation = teaql_runtime::GraphOperation::Remove;
            } else if self.id == 0 {
                node.operation = teaql_runtime::GraphOperation::Create;
            } else {
                node.operation = teaql_runtime::GraphOperation::Upsert;
            }
            if !self.comment.is_empty() {
                node.comment = Some(self.comment.clone());
            }
            
            let values = teaql_core::Entity::into_record(self);
            node.values = values;
            repo.save_graph(node).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })
    }
}

impl teaql_core::TeaqlEntity for DataService {
    fn entity_descriptor() -> teaql_core::EntityDescriptor {
        teaql_core::EntityDescriptor { 
            name: "_data_service".to_string(),
            table_name: "_data_service_data".to_string(),
            properties: vec![
                teaql_core::PropertyDescriptor {
                    name: "id".to_string(),
                    column_name: "id".to_string(),
                    data_type: teaql_core::DataType::U64,
                    nullable: false,
                    is_id: true,
                    is_version: false,
                },
                teaql_core::PropertyDescriptor {
                    name: "version".to_string(),
                    column_name: "version".to_string(),
                    data_type: teaql_core::DataType::I64,
                    nullable: false,
                    is_id: false,
                    is_version: true,
                },
                teaql_core::PropertyDescriptor {
                    name: "name".to_string(),
                    column_name: "name".to_string(),
                    data_type: match "Option<String>" {
                        "String" | "Option<String>" => teaql_core::DataType::Text,
                        "u64" | "Option<u64>" => teaql_core::DataType::U64,
                        "i64" | "Option<i64>" => teaql_core::DataType::I64,
                        "i32" | "Option<i32>" => teaql_core::DataType::I64,
                        "bool" | "Option<bool>" => teaql_core::DataType::Bool,
                        "chrono::NaiveDate" | "Option<chrono::NaiveDate>" => teaql_core::DataType::Date,
                        "chrono::DateTime<chrono::Utc>" | "Option<chrono::DateTime<chrono::Utc>>" => teaql_core::DataType::Timestamp,
                        _ => teaql_core::DataType::Text,
                    },
                    nullable: true,
                    is_id: false,
                    is_version: false,
                },
            ],
            relations: vec![
            ],
        }
    }
}

impl teaql_core::Entity for DataService {
    fn from_record(mut record: std::collections::BTreeMap<String, teaql_core::Value>) -> Result<Self, teaql_core::EntityError> {
        let mut entity = Self::new();
        if let Some(val) = record.remove("id") {
            if let teaql_core::Value::U64(v) = val { entity.id = v; }
            else if let teaql_core::Value::I64(v) = val { entity.id = v as u64; }
        }
        if let Some(val) = record.remove("version") {
            if let teaql_core::Value::I64(v) = val { entity.version = v; }
        }
        if let Some(val) = record.remove("name") {
            if let teaql_core::Value::Text(v) = val { entity.name = Some(v); }
        }
        Ok(entity)
    }

    fn into_record(self) -> std::collections::BTreeMap<String, teaql_core::Value> {
        let mut record = std::collections::BTreeMap::new();
        record.insert("id".to_string(), teaql_core::Value::U64(self.id));
        record.insert("version".to_string(), teaql_core::Value::I64(self.version));
        if let Some(v) = self.name { record.insert("name".to_string(), teaql_core::Value::Text(v)); }
        record
    }
}
