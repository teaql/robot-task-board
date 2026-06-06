use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
// @source model.xml:10
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
// @source model.xml:10
    pub id: u64,
// @source model.xml:10
    pub name: Option<String>,
// @source model.xml:10
    pub code: Option<String>,
// @source model.xml:10
    pub color: Option<String>,
// @source model.xml:10
    pub display_order: Option<i32>,
// @source model.xml:10
    pub progress: Option<i32>,
// @source model.xml:30
    pub tasks: Vec<crate::entities::task::Task>,
    pub version: i64,
    pub comment: String,
    pub deleted: bool,
    #[serde(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TaskStatus {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            code: None,
            color: None,
            display_order: None,
            progress: None,
            tasks: Vec::new(),
            version: 0,
            comment: String::new(),
            deleted: false,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
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

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }
    pub fn tasks_mut(&mut self) -> &mut Vec<crate::entities::task::Task> {
        &mut self.tasks
    }
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
    pub fn update_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.name = Some(value.into());
        self
    }
    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.__load_state.is_loaded("name") {
            teaql_core::eval::EvalResult::NotLoaded { missing_path: "name".to_string() }
        } else {
            match &self.name {
                Some(v) => teaql_core::eval::EvalResult::Value(v.clone()),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn code(&self) -> String {
        self.code.clone().unwrap_or_default()
    }
    pub fn update_code(&mut self, value: impl Into<String>) -> &mut Self {
        self.code = Some(value.into());
        self
    }
    pub fn eval_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.__load_state.is_loaded("code") {
            teaql_core::eval::EvalResult::NotLoaded { missing_path: "code".to_string() }
        } else {
            match &self.code {
                Some(v) => teaql_core::eval::EvalResult::Value(v.clone()),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn color(&self) -> String {
        self.color.clone().unwrap_or_default()
    }
    pub fn update_color(&mut self, value: impl Into<String>) -> &mut Self {
        self.color = Some(value.into());
        self
    }
    pub fn eval_color(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.__load_state.is_loaded("color") {
            teaql_core::eval::EvalResult::NotLoaded { missing_path: "color".to_string() }
        } else {
            match &self.color {
                Some(v) => teaql_core::eval::EvalResult::Value(v.clone()),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn display_order(&self) -> i32 {
        self.display_order.clone().unwrap_or_default()
    }
    pub fn update_display_order(&mut self, value: impl Into<i32>) -> &mut Self {
        self.display_order = Some(value.into());
        self
    }
    pub fn eval_display_order(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.__load_state.is_loaded("display_order") {
            teaql_core::eval::EvalResult::NotLoaded { missing_path: "display_order".to_string() }
        } else {
            match &self.display_order {
                Some(v) => teaql_core::eval::EvalResult::Value(v.clone()),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn progress(&self) -> i32 {
        self.progress.clone().unwrap_or_default()
    }
    pub fn update_progress(&mut self, value: impl Into<i32>) -> &mut Self {
        self.progress = Some(value.into());
        self
    }
    pub fn eval_progress(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.__load_state.is_loaded("progress") {
            teaql_core::eval::EvalResult::NotLoaded { missing_path: "progress".to_string() }
        } else {
            match &self.progress {
                Some(v) => teaql_core::eval::EvalResult::Value(v.clone()),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn eval_tasks(&self) -> teaql_core::eval::EvalResult<&[crate::entities::task::Task]> {
        if !self.__load_state.is_loaded("task_list") {
            teaql_core::eval::EvalResult::NotLoaded { missing_path: "task_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.tasks)
        }
    }

    pub fn audit_as(self, comment: impl Into<String>) -> teaql_core::entity::Audited<Self> {
        teaql_core::entity::Audited::new(self, comment)
    }
    
    pub(crate) fn _save(mut self, ctx: &teaql_runtime::UserContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, std::io::Error>> + Send + '_>> {
        Box::pin(async move {
            let repo = ctx.resolve_repository::<crate::ServiceRuntimeExecutor>("task_status")
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            let mut node = teaql_runtime::GraphNode::new("task_status");
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
            let mut items = Vec::new();
            let mut list = std::mem::take(&mut self.tasks);
            for item in list {
                let mut log_node = teaql_runtime::GraphNode::new("task");
                if item.deleted {
                    log_node.operation = teaql_runtime::GraphOperation::Remove;
                } else if item.id == 0 {
                    log_node.operation = teaql_runtime::GraphOperation::Create;
                } else {
                    log_node.operation = teaql_runtime::GraphOperation::Upsert;
                }
                log_node.values = teaql_core::Entity::into_record(item);
                items.push(log_node);
            }
            if !items.is_empty() {
                node.relations.insert("task_list".to_string(), items);
            }
            
            let values = teaql_core::Entity::into_record(self);
            node.values = values;
            repo.save_graph(node).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })
    }
}

impl teaql_core::TeaqlEntity for TaskStatus {
    fn entity_descriptor() -> teaql_core::EntityDescriptor {
        teaql_core::EntityDescriptor { 
            name: "task_status".to_string(),
            table_name: "task_status_data".to_string(),
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
                teaql_core::PropertyDescriptor {
                    name: "code".to_string(),
                    column_name: "code".to_string(),
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
                teaql_core::PropertyDescriptor {
                    name: "color".to_string(),
                    column_name: "color".to_string(),
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
                teaql_core::PropertyDescriptor {
                    name: "display_order".to_string(),
                    column_name: "display_order".to_string(),
                    data_type: match "Option<i32>" {
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
                teaql_core::PropertyDescriptor {
                    name: "progress".to_string(),
                    column_name: "progress".to_string(),
                    data_type: match "Option<i32>" {
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
                teaql_core::RelationDescriptor {
                    name: "task_list".to_string(),
                    target_entity: "task".to_string(),
                    local_key: "id".to_string(),
                    foreign_key: "status_id".to_string(),
                    many: true,
                    attach: false,
                    delete_missing: false,
                },
            ],
        }
    }
}

impl teaql_core::Entity for TaskStatus {
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
        if let Some(val) = record.remove("code") {
            if let teaql_core::Value::Text(v) = val { entity.code = Some(v); }
        }
        if let Some(val) = record.remove("color") {
            if let teaql_core::Value::Text(v) = val { entity.color = Some(v); }
        }
        if let Some(val) = record.remove("display_order") {
        }
        if let Some(val) = record.remove("progress") {
        }
        Ok(entity)
    }

    fn into_record(self) -> std::collections::BTreeMap<String, teaql_core::Value> {
        let mut record = std::collections::BTreeMap::new();
        record.insert("id".to_string(), teaql_core::Value::U64(self.id));
        record.insert("version".to_string(), teaql_core::Value::I64(self.version));
        if let Some(v) = self.name { record.insert("name".to_string(), teaql_core::Value::Text(v)); }
        if let Some(v) = self.code { record.insert("code".to_string(), teaql_core::Value::Text(v)); }
        if let Some(v) = self.color { record.insert("color".to_string(), teaql_core::Value::Text(v)); }
        record
    }
}
