#![feature(prelude_import)]
//! Generated TeaQL domain crate for `robot-kanban-service`.
//!
//! AI coding agents must read this crate's `AGENTS.md` before using generated
//! APIs. If this crate was downloaded from a Cargo registry, locate the
//! unpacked crate source or vendor the dependency, then read `AGENTS.md` from
//! the crate root before writing code against it.
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;

pub mod e {

    use teaql_core::SafeExpression;
    pub struct E;
    impl E {
        pub fn platform(value: crate::Platform)
            -> crate::PlatformExpression<crate::Platform> {
            crate::PlatformExpression::new(SafeExpression::value(value))
        }
        pub fn task_status(value: crate::TaskStatus)
            -> crate::TaskStatusExpression<crate::TaskStatus> {
            crate::TaskStatusExpression::new(SafeExpression::value(value))
        }
        pub fn task(value: crate::Task)
            -> crate::TaskExpression<crate::Task> {
            crate::TaskExpression::new(SafeExpression::value(value))
        }
        pub fn task_execution_log(value: crate::TaskExecutionLog)
            -> crate::TaskExecutionLogExpression<crate::TaskExecutionLog> {
            crate::TaskExecutionLogExpression::new(SafeExpression::value(value))
        }
    }
}
pub mod q {
    use teaql_core::Expr;
    use crate::*;
    pub struct Q;
    impl Q {
        pub fn platforms() -> PlatformRequest {
            PlatformRequest::new().select_self().and_filter(Expr::gt("version",
                    0_i64))
        }
        pub fn platforms_minimal() -> PlatformRequest {
            PlatformRequest::new().and_filter(Expr::gt("version", 0_i64))
        }
        pub fn platforms_with_children() -> PlatformRequest {
            PlatformRequest::new().unlimited().select_self_fields().enhance_children_if_needed()
        }
        pub fn task_status() -> TaskStatusRequest {
            TaskStatusRequest::new().select_self().and_filter(Expr::gt("version",
                    0_i64))
        }
        pub fn task_status_minimal() -> TaskStatusRequest {
            TaskStatusRequest::new().and_filter(Expr::gt("version", 0_i64))
        }
        pub fn task_status_with_children() -> TaskStatusRequest {
            TaskStatusRequest::new().unlimited().select_self_fields().enhance_children_if_needed()
        }
        pub fn tasks() -> TaskRequest {
            TaskRequest::new().select_self().and_filter(Expr::gt("version",
                    0_i64))
        }
        pub fn tasks_minimal() -> TaskRequest {
            TaskRequest::new().and_filter(Expr::gt("version", 0_i64))
        }
        pub fn tasks_with_children() -> TaskRequest {
            TaskRequest::new().unlimited().select_self_fields().enhance_children_if_needed()
        }
        pub fn task_execution_logs() -> TaskExecutionLogRequest {
            TaskExecutionLogRequest::new().select_self().and_filter(Expr::gt("version",
                    0_i64))
        }
        pub fn task_execution_logs_minimal() -> TaskExecutionLogRequest {
            TaskExecutionLogRequest::new().and_filter(Expr::gt("version",
                    0_i64))
        }
        pub fn task_execution_logs_with_children()
            -> TaskExecutionLogRequest {
            TaskExecutionLogRequest::new().unlimited().select_self_fields().enhance_children_if_needed()
        }
    }
}
pub mod request_support {
    #![allow(unused_imports)]
    use std::{collections::BTreeMap, future::Future, marker::PhantomData};
    use serde_json::Value as JsonValue;
    use teaql_core::{
        BinaryOp, Expr, Record, RelationAggregate as RuntimeRelationAggregate,
        SelectQuery, SmartList,
    };
    use teaql_runtime::{
        ContextError, GraphNode, QueryExecutor, RepositoryError, RuntimeError,
        UserContext, QueryCommentGuard,
    };
    pub(crate) const COUNT_ALIAS: &str = "count";
    pub(crate) const TYPE_FIELD: &str = "internal_type";
    pub(crate) const TYPE_GROUP_FIELD: &str = "type_group";
    pub enum FieldOperator {
        Equal,
        NotEqual,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
        Between,
        In,
        NotIn,
        Contain,
        NotContain,
        BeginWith,
        NotBeginWith,
        EndWith,
        NotEndWith,
        SoundsLike,
        IsNull,
        IsNotNull,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for FieldOperator { }
    #[automatically_derived]
    impl ::core::clone::Clone for FieldOperator {
        #[inline]
        fn clone(&self) -> FieldOperator { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FieldOperator { }
    #[automatically_derived]
    impl ::core::fmt::Debug for FieldOperator {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    FieldOperator::Equal => "Equal",
                    FieldOperator::NotEqual => "NotEqual",
                    FieldOperator::GreaterThan => "GreaterThan",
                    FieldOperator::GreaterThanOrEqual => "GreaterThanOrEqual",
                    FieldOperator::LessThan => "LessThan",
                    FieldOperator::LessThanOrEqual => "LessThanOrEqual",
                    FieldOperator::Between => "Between",
                    FieldOperator::In => "In",
                    FieldOperator::NotIn => "NotIn",
                    FieldOperator::Contain => "Contain",
                    FieldOperator::NotContain => "NotContain",
                    FieldOperator::BeginWith => "BeginWith",
                    FieldOperator::NotBeginWith => "NotBeginWith",
                    FieldOperator::EndWith => "EndWith",
                    FieldOperator::NotEndWith => "NotEndWith",
                    FieldOperator::SoundsLike => "SoundsLike",
                    FieldOperator::IsNull => "IsNull",
                    FieldOperator::IsNotNull => "IsNotNull",
                })
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FieldOperator { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FieldOperator {
        #[inline]
        fn eq(&self, other: &FieldOperator) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for FieldOperator {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {}
    }
    pub struct DateRange<T> {
        pub start: T,
        pub end: T,
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for DateRange<T> {
        #[inline]
        fn clone(&self) -> DateRange<T> {
            DateRange {
                start: ::core::clone::Clone::clone(&self.start),
                end: ::core::clone::Clone::clone(&self.end),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for DateRange<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(f, "DateRange",
                "start", &self.start, "end", &&self.end)
        }
    }
    #[automatically_derived]
    impl<T> ::core::marker::StructuralPartialEq for DateRange<T> { }
    #[automatically_derived]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for DateRange<T> {
        #[inline]
        fn eq(&self, other: &DateRange<T>) -> bool {
            self.start == other.start && self.end == other.end
        }
    }
    impl<T> DateRange<T> {
        pub fn new(start: T, end: T) -> Self { Self { start, end } }
    }
    pub trait EntityReference {
        fn entity_id_value(self)
        -> teaql_core::Value;
    }
    pub trait TeaqlRecordRepository {
        type Error: std::error::Error + Send + Sync + 'static;
        fn fetch_all(&self, query: &SelectQuery)
        -> Result<Vec<Record>, RepositoryError<Self::Error>>;
        fn fetch_smart_list(&self, query: &SelectQuery)
        -> Result<SmartList<Record>, RepositoryError<Self::Error>>;
        fn fetch_smart_list_with_relation_aggregates(&self,
        query: &SelectQuery, relation_aggregates: &[RuntimeRelationAggregate])
        -> Result<SmartList<Record>, RepositoryError<Self::Error>>;
    }
    pub trait TeaqlEntityRepository: TeaqlRecordRepository {
        fn fetch_enhanced_entities<T>(&self, query: &SelectQuery)
        -> Result<SmartList<T>, RepositoryError<Self::Error>>
        where
        T: teaql_core::Entity;
        fn fetch_enhanced_entities_with_relation_aggregates<T>(&self,
        query: &SelectQuery, relation_aggregates: &[RuntimeRelationAggregate])
        -> Result<SmartList<T>, RepositoryError<Self::Error>>
        where
        T: teaql_core::Entity;
        fn save_entity_graph<T>(&self, entity: T)
        -> Result<GraphNode, RepositoryError<Self::Error>>
        where
        T: teaql_core::Entity;
        fn save_entity<T>(&self, entity: T,
        status: teaql_runtime::EntityStatus)
        -> Result<GraphNode, RepositoryError<Self::Error>>
        where
        T: teaql_core::Entity;
        fn save_entity_with_comment<T>(&self, entity: T,
        status: teaql_runtime::EntityStatus, comment: String)
        -> Result<GraphNode, RepositoryError<Self::Error>>
        where
        T: teaql_core::Entity;
    }
    impl<'a, D, E> TeaqlRecordRepository for
        teaql_runtime::ResolvedRepository<'a, D, E> where
        D: teaql_sql::SqlDialect, E: QueryExecutor {
        type Error = E::Error;
        fn fetch_all(&self, query: &SelectQuery)
            -> Result<Vec<Record>, RepositoryError<Self::Error>> {
            teaql_runtime::ResolvedRepository::fetch_all(self, query)
        }
        fn fetch_smart_list(&self, query: &SelectQuery)
            -> Result<SmartList<Record>, RepositoryError<Self::Error>> {
            teaql_runtime::ResolvedRepository::fetch_smart_list(self, query)
        }
        fn fetch_smart_list_with_relation_aggregates(&self,
            query: &SelectQuery,
            relation_aggregates: &[RuntimeRelationAggregate])
            -> Result<SmartList<Record>, RepositoryError<Self::Error>> {
            teaql_runtime::ResolvedRepository::fetch_smart_list_with_relation_aggregates(self,
                query, relation_aggregates)
        }
    }
    impl<'a, D, E> TeaqlEntityRepository for
        teaql_runtime::ResolvedRepository<'a, D, E> where
        D: teaql_sql::SqlDialect, E: QueryExecutor {
        fn fetch_enhanced_entities<T>(&self, query: &SelectQuery)
            -> Result<SmartList<T>, RepositoryError<Self::Error>> where
            T: teaql_core::Entity {
            teaql_runtime::ResolvedRepository::fetch_enhanced_entities(self,
                query)
        }
        fn fetch_enhanced_entities_with_relation_aggregates<T>(&self,
            query: &SelectQuery,
            relation_aggregates: &[RuntimeRelationAggregate])
            -> Result<SmartList<T>, RepositoryError<Self::Error>> where
            T: teaql_core::Entity {
            teaql_runtime::ResolvedRepository::fetch_enhanced_entities_with_relation_aggregates(self,
                query, relation_aggregates)
        }
        fn save_entity_graph<T>(&self, entity: T)
            -> Result<GraphNode, RepositoryError<Self::Error>> where
            T: teaql_core::Entity {
            teaql_runtime::ResolvedRepository::save_entity_graph(self, entity)
        }
        fn save_entity<T>(&self, entity: T,
            status: teaql_runtime::EntityStatus)
            -> Result<GraphNode, RepositoryError<Self::Error>> where
            T: teaql_core::Entity {
            teaql_runtime::ResolvedRepository::save_entity(self, entity,
                status)
        }
        fn save_entity_with_comment<T>(&self, entity: T,
            status: teaql_runtime::EntityStatus, comment: String)
            -> Result<GraphNode, RepositoryError<Self::Error>> where
            T: teaql_core::Entity {
            teaql_runtime::ResolvedRepository::save_entity_with_comment(self,
                entity, status, comment)
        }
    }
    pub type TeaqlRepositoryError<R> =
        RepositoryError<<R as TeaqlRecordRepository>::Error>;
    pub trait TeaqlRuntime {
        type PlatformRepository<'a>: TeaqlEntityRepository + 'a where
            Self: 'a;
        fn platform_repository(&self)
        -> Result<Self::PlatformRepository<'_>, ContextError>;
        type TaskStatusRepository<'a>: TeaqlEntityRepository + 'a where
            Self: 'a;
        fn task_status_repository(&self)
        -> Result<Self::TaskStatusRepository<'_>, ContextError>;
        type TaskRepository<'a>: TeaqlEntityRepository + 'a where Self: 'a;
        fn task_repository(&self)
        -> Result<Self::TaskRepository<'_>, ContextError>;
        type TaskExecutionLogRepository<'a>: TeaqlEntityRepository + 'a where
            Self: 'a;
        fn task_execution_log_repository(&self)
        -> Result<Self::TaskExecutionLogRepository<'_>, ContextError>;
        fn user_context(&self)
        -> &UserContext;
        fn fetch_facet_smart_list(&self, entity: &str, query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate])
        -> Result<SmartList<Record>, RuntimeError>;
    }
    #[allow(async_fn_in_trait)]
    pub trait TeaqlUserContextExt {
        async fn commit_data(&self)
        ->
            Result<(),
            RepositoryError<crate::runtime::DataServiceMutationError>>;
        async fn transaction_data<F, Fut>(&self, f: F)
        ->
            Result<(),
            RepositoryError<crate::runtime::DataServiceMutationError>>
        where
        F: FnOnce()
        -> Fut,
        Fut: Future<Output =
        Result<(),
        RepositoryError<crate::runtime::DataServiceMutationError>>>;
    }
    impl TeaqlUserContextExt for teaql_runtime::UserContext {
        async fn commit_data(&self)
            ->
                Result<(),
                RepositoryError<crate::runtime::DataServiceMutationError>> {
            self.commit_changes::<crate::runtime::DataServiceDialect,
                crate::runtime::DataServiceExecutor>()
        }
        async fn transaction_data<F, Fut>(&self, f: F)
            ->
                Result<(),
                RepositoryError<crate::runtime::DataServiceMutationError>>
            where F: FnOnce() -> Fut,
            Fut: Future<Output =
            Result<(),
            RepositoryError<crate::runtime::DataServiceMutationError>>> {
            let executor =
                self.require_resource::<crate::runtime::DataServiceExecutor>().map_err(|err|
                            {
                                RepositoryError::Runtime(RuntimeError::Graph(::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("cannot start transaction without executor: {0}",
                                                        err))
                                            })))
                            })?;
            let root = self.entity_root();
            executor.begin_transaction().map_err(RepositoryError::Executor)?;
            root.push_change_set();
            let result = f().await;
            match result {
                Ok(()) => {
                    root.pop_change_set();
                    executor.commit_transaction().map_err(RepositoryError::Executor)?;
                    Ok(())
                }
                Err(err) => {
                    root.pop_change_set();
                    executor.rollback_transaction().map_err(RepositoryError::Executor)?;
                    Err(err)
                }
            }
        }
    }
    impl TeaqlRuntime for teaql_runtime::UserContext {
        type PlatformRepository<'a> =
            teaql_runtime::ResolvedRepository<'a,
            crate::runtime::DataServiceDialect,
            crate::runtime::DataServiceExecutor> where Self: 'a;
        fn platform_repository(&self)
            -> Result<Self::PlatformRepository<'_>, ContextError> {
            self.resolve_repository::<crate::runtime::DataServiceDialect,
                crate::runtime::DataServiceExecutor>("Platform")
        }
        type TaskStatusRepository<'a> =
            teaql_runtime::ResolvedRepository<'a,
            crate::runtime::DataServiceDialect,
            crate::runtime::DataServiceExecutor> where Self: 'a;
        fn task_status_repository(&self)
            -> Result<Self::TaskStatusRepository<'_>, ContextError> {
            self.resolve_repository::<crate::runtime::DataServiceDialect,
                crate::runtime::DataServiceExecutor>("TaskStatus")
        }
        type TaskRepository<'a> =
            teaql_runtime::ResolvedRepository<'a,
            crate::runtime::DataServiceDialect,
            crate::runtime::DataServiceExecutor> where Self: 'a;
        fn task_repository(&self)
            -> Result<Self::TaskRepository<'_>, ContextError> {
            self.resolve_repository::<crate::runtime::DataServiceDialect,
                crate::runtime::DataServiceExecutor>("Task")
        }
        type TaskExecutionLogRepository<'a> =
            teaql_runtime::ResolvedRepository<'a,
            crate::runtime::DataServiceDialect,
            crate::runtime::DataServiceExecutor> where Self: 'a;
        fn task_execution_log_repository(&self)
            -> Result<Self::TaskExecutionLogRepository<'_>, ContextError> {
            self.resolve_repository::<crate::runtime::DataServiceDialect,
                crate::runtime::DataServiceExecutor>("TaskExecutionLog")
        }
        fn user_context(&self) -> &UserContext { self }
        fn fetch_facet_smart_list(&self, entity: &str, query: &SelectQuery,
            relation_aggregates: &[RuntimeRelationAggregate])
            -> Result<SmartList<Record>, RuntimeError> {
            self.resolve_repository::<crate::runtime::DataServiceDialect,
                                crate::runtime::DataServiceExecutor>(entity.to_owned()).map_err(|err|
                                RuntimeError::Graph(err.to_string()))?.fetch_smart_list_with_relation_aggregates(query,
                    relation_aggregates).map_err(|err|
                    RuntimeError::Graph(err.to_string()))
        }
    }
    pub struct QuerySelection {
        pub query: SelectQuery,
        pub relation_selections: Vec<RelationSelection>,
        pub relation_filters: Vec<RelationFilter>,
        pub child_enhancements: Vec<QuerySelection>,
        pub query_options: QueryOptions,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for QuerySelection {
        #[inline]
        fn clone(&self) -> QuerySelection {
            QuerySelection {
                query: ::core::clone::Clone::clone(&self.query),
                relation_selections: ::core::clone::Clone::clone(&self.relation_selections),
                relation_filters: ::core::clone::Clone::clone(&self.relation_filters),
                child_enhancements: ::core::clone::Clone::clone(&self.child_enhancements),
                query_options: ::core::clone::Clone::clone(&self.query_options),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QuerySelection {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(f,
                "QuerySelection", "query", &self.query, "relation_selections",
                &self.relation_selections, "relation_filters",
                &self.relation_filters, "child_enhancements",
                &self.child_enhancements, "query_options",
                &&self.query_options)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QuerySelection { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for QuerySelection {
        #[inline]
        fn eq(&self, other: &QuerySelection) -> bool {
            self.query == other.query &&
                            self.relation_selections == other.relation_selections &&
                        self.relation_filters == other.relation_filters &&
                    self.child_enhancements == other.child_enhancements &&
                self.query_options == other.query_options
        }
    }
    impl QuerySelection {
        pub fn new(query: impl Into<SelectQuery>) -> Self {
            Self {
                query: query.into(),
                relation_selections: Vec::new(),
                relation_filters: Vec::new(),
                child_enhancements: Vec::new(),
                query_options: QueryOptions::default(),
            }
        }
        pub fn into_query(self) -> SelectQuery {
            let query =
                apply_relation_selections(self.query,
                    self.relation_selections);
            apply_runtime_metadata(query, &self.query_options,
                &self.child_enhancements)
        }
    }
    impl From<SelectQuery> for QuerySelection {
        fn from(query: SelectQuery) -> Self { QuerySelection::new(query) }
    }
    pub struct RelationSelection {
        pub name: String,
        pub query: SelectQuery,
        pub relation_selections: Vec<RelationSelection>,
        pub relation_filters: Vec<RelationFilter>,
        pub child_enhancements: Vec<QuerySelection>,
        pub query_options: QueryOptions,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RelationSelection {
        #[inline]
        fn clone(&self) -> RelationSelection {
            RelationSelection {
                name: ::core::clone::Clone::clone(&self.name),
                query: ::core::clone::Clone::clone(&self.query),
                relation_selections: ::core::clone::Clone::clone(&self.relation_selections),
                relation_filters: ::core::clone::Clone::clone(&self.relation_filters),
                child_enhancements: ::core::clone::Clone::clone(&self.child_enhancements),
                query_options: ::core::clone::Clone::clone(&self.query_options),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RelationSelection {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ =
                &["name", "query", "relation_selections", "relation_filters",
                            "child_enhancements", "query_options"];
            let values: &[&dyn ::core::fmt::Debug] =
                &[&self.name, &self.query, &self.relation_selections,
                            &self.relation_filters, &self.child_enhancements,
                            &&self.query_options];
            ::core::fmt::Formatter::debug_struct_fields_finish(f,
                "RelationSelection", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RelationSelection { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RelationSelection {
        #[inline]
        fn eq(&self, other: &RelationSelection) -> bool {
            self.name == other.name && self.query == other.query &&
                            self.relation_selections == other.relation_selections &&
                        self.relation_filters == other.relation_filters &&
                    self.child_enhancements == other.child_enhancements &&
                self.query_options == other.query_options
        }
    }
    impl RelationSelection {
        pub fn new(name: impl Into<String>,
            selection: impl Into<QuerySelection>) -> Self {
            let selection = selection.into();
            Self {
                name: name.into(),
                query: selection.query,
                relation_selections: selection.relation_selections,
                relation_filters: selection.relation_filters,
                child_enhancements: selection.child_enhancements,
                query_options: selection.query_options,
            }
        }
        pub fn into_query(self) -> SelectQuery {
            let query =
                apply_relation_selections(self.query,
                    self.relation_selections);
            apply_runtime_metadata(query, &self.query_options,
                &self.child_enhancements)
        }
    }
    pub struct RelationFilter {
        pub name: String,
        pub query: SelectQuery,
        pub relation_selections: Vec<RelationSelection>,
        pub relation_filters: Vec<RelationFilter>,
        pub child_enhancements: Vec<QuerySelection>,
        pub query_options: QueryOptions,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RelationFilter {
        #[inline]
        fn clone(&self) -> RelationFilter {
            RelationFilter {
                name: ::core::clone::Clone::clone(&self.name),
                query: ::core::clone::Clone::clone(&self.query),
                relation_selections: ::core::clone::Clone::clone(&self.relation_selections),
                relation_filters: ::core::clone::Clone::clone(&self.relation_filters),
                child_enhancements: ::core::clone::Clone::clone(&self.child_enhancements),
                query_options: ::core::clone::Clone::clone(&self.query_options),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RelationFilter {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ =
                &["name", "query", "relation_selections", "relation_filters",
                            "child_enhancements", "query_options"];
            let values: &[&dyn ::core::fmt::Debug] =
                &[&self.name, &self.query, &self.relation_selections,
                            &self.relation_filters, &self.child_enhancements,
                            &&self.query_options];
            ::core::fmt::Formatter::debug_struct_fields_finish(f,
                "RelationFilter", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RelationFilter { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RelationFilter {
        #[inline]
        fn eq(&self, other: &RelationFilter) -> bool {
            self.name == other.name && self.query == other.query &&
                            self.relation_selections == other.relation_selections &&
                        self.relation_filters == other.relation_filters &&
                    self.child_enhancements == other.child_enhancements &&
                self.query_options == other.query_options
        }
    }
    impl RelationFilter {
        pub fn new(name: impl Into<String>,
            selection: impl Into<QuerySelection>) -> Self {
            let selection = selection.into();
            Self {
                name: name.into(),
                query: selection.query,
                relation_selections: selection.relation_selections,
                relation_filters: selection.relation_filters,
                child_enhancements: selection.child_enhancements,
                query_options: selection.query_options,
            }
        }
    }
    pub struct QueryOptions {
        pub comment: Option<String>,
        pub raw_sql: Option<String>,
        pub raw_sql_search_criteria: Vec<String>,
        pub dynamic_properties: Vec<RawDynamicProperty>,
        pub raw_projections: Vec<RawProjection>,
        pub relation_aggregates: Vec<RelationAggregate>,
        pub object_group_bys: Vec<ObjectGroupBy>,
        pub facets: Vec<FacetRequest>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for QueryOptions {
        #[inline]
        fn clone(&self) -> QueryOptions {
            QueryOptions {
                comment: ::core::clone::Clone::clone(&self.comment),
                raw_sql: ::core::clone::Clone::clone(&self.raw_sql),
                raw_sql_search_criteria: ::core::clone::Clone::clone(&self.raw_sql_search_criteria),
                dynamic_properties: ::core::clone::Clone::clone(&self.dynamic_properties),
                raw_projections: ::core::clone::Clone::clone(&self.raw_projections),
                relation_aggregates: ::core::clone::Clone::clone(&self.relation_aggregates),
                object_group_bys: ::core::clone::Clone::clone(&self.object_group_bys),
                facets: ::core::clone::Clone::clone(&self.facets),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QueryOptions {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ =
                &["comment", "raw_sql", "raw_sql_search_criteria",
                            "dynamic_properties", "raw_projections",
                            "relation_aggregates", "object_group_bys", "facets"];
            let values: &[&dyn ::core::fmt::Debug] =
                &[&self.comment, &self.raw_sql, &self.raw_sql_search_criteria,
                            &self.dynamic_properties, &self.raw_projections,
                            &self.relation_aggregates, &self.object_group_bys,
                            &&self.facets];
            ::core::fmt::Formatter::debug_struct_fields_finish(f,
                "QueryOptions", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for QueryOptions {
        #[inline]
        fn default() -> QueryOptions {
            QueryOptions {
                comment: ::core::default::Default::default(),
                raw_sql: ::core::default::Default::default(),
                raw_sql_search_criteria: ::core::default::Default::default(),
                dynamic_properties: ::core::default::Default::default(),
                raw_projections: ::core::default::Default::default(),
                relation_aggregates: ::core::default::Default::default(),
                object_group_bys: ::core::default::Default::default(),
                facets: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QueryOptions { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for QueryOptions {
        #[inline]
        fn eq(&self, other: &QueryOptions) -> bool {
            self.comment == other.comment && self.raw_sql == other.raw_sql &&
                                    self.raw_sql_search_criteria ==
                                        other.raw_sql_search_criteria &&
                                self.dynamic_properties == other.dynamic_properties &&
                            self.raw_projections == other.raw_projections &&
                        self.relation_aggregates == other.relation_aggregates &&
                    self.object_group_bys == other.object_group_bys &&
                self.facets == other.facets
        }
    }
    pub struct UnsafeRawSqlSegment {
        sql: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnsafeRawSqlSegment {
        #[inline]
        fn clone(&self) -> UnsafeRawSqlSegment {
            UnsafeRawSqlSegment {
                sql: ::core::clone::Clone::clone(&self.sql),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnsafeRawSqlSegment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f,
                "UnsafeRawSqlSegment", "sql", &&self.sql)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnsafeRawSqlSegment { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnsafeRawSqlSegment {
        #[inline]
        fn eq(&self, other: &UnsafeRawSqlSegment) -> bool {
            self.sql == other.sql
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for UnsafeRawSqlSegment {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    impl UnsafeRawSqlSegment {
        pub fn trusted(sql: impl Into<String>) -> Self {
            Self { sql: sql.into() }
        }
        pub fn into_sql(self) -> String { self.sql }
    }
    pub struct RawDynamicProperty {
        pub property_name: String,
        pub raw_sql_segment: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RawDynamicProperty {
        #[inline]
        fn clone(&self) -> RawDynamicProperty {
            RawDynamicProperty {
                property_name: ::core::clone::Clone::clone(&self.property_name),
                raw_sql_segment: ::core::clone::Clone::clone(&self.raw_sql_segment),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RawDynamicProperty {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(f,
                "RawDynamicProperty", "property_name", &self.property_name,
                "raw_sql_segment", &&self.raw_sql_segment)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RawDynamicProperty { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RawDynamicProperty {
        #[inline]
        fn eq(&self, other: &RawDynamicProperty) -> bool {
            self.property_name == other.property_name &&
                self.raw_sql_segment == other.raw_sql_segment
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for RawDynamicProperty {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    impl RawDynamicProperty {
        pub fn new(property_name: impl Into<String>,
            raw_sql_segment: UnsafeRawSqlSegment) -> Self {
            Self {
                property_name: property_name.into(),
                raw_sql_segment: raw_sql_segment.into_sql(),
            }
        }
    }
    pub struct RawProjection {
        pub property_name: String,
        pub raw_sql_segment: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RawProjection {
        #[inline]
        fn clone(&self) -> RawProjection {
            RawProjection {
                property_name: ::core::clone::Clone::clone(&self.property_name),
                raw_sql_segment: ::core::clone::Clone::clone(&self.raw_sql_segment),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RawProjection {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(f,
                "RawProjection", "property_name", &self.property_name,
                "raw_sql_segment", &&self.raw_sql_segment)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RawProjection { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RawProjection {
        #[inline]
        fn eq(&self, other: &RawProjection) -> bool {
            self.property_name == other.property_name &&
                self.raw_sql_segment == other.raw_sql_segment
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for RawProjection {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    impl RawProjection {
        pub fn new(property_name: impl Into<String>,
            raw_sql_segment: UnsafeRawSqlSegment) -> Self {
            Self {
                property_name: property_name.into(),
                raw_sql_segment: raw_sql_segment.into_sql(),
            }
        }
    }
    pub struct RelationAggregate {
        pub relation_name: String,
        pub alias: String,
        pub query: QuerySelection,
        pub single_result: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RelationAggregate {
        #[inline]
        fn clone(&self) -> RelationAggregate {
            RelationAggregate {
                relation_name: ::core::clone::Clone::clone(&self.relation_name),
                alias: ::core::clone::Clone::clone(&self.alias),
                query: ::core::clone::Clone::clone(&self.query),
                single_result: ::core::clone::Clone::clone(&self.single_result),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RelationAggregate {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(f,
                "RelationAggregate", "relation_name", &self.relation_name,
                "alias", &self.alias, "query", &self.query, "single_result",
                &&self.single_result)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RelationAggregate { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RelationAggregate {
        #[inline]
        fn eq(&self, other: &RelationAggregate) -> bool {
            self.single_result == other.single_result &&
                        self.relation_name == other.relation_name &&
                    self.alias == other.alias && self.query == other.query
        }
    }
    impl RelationAggregate {
        pub fn new(relation_name: impl Into<String>, alias: impl Into<String>,
            query: impl Into<QuerySelection>, single_result: bool) -> Self {
            Self {
                relation_name: relation_name.into(),
                alias: alias.into(),
                query: query.into(),
                single_result,
            }
        }
    }
    pub struct FacetRequest {
        pub facet_name: String,
        pub relation_name: String,
        pub query: QuerySelection,
        pub include_all_facets: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FacetRequest {
        #[inline]
        fn clone(&self) -> FacetRequest {
            FacetRequest {
                facet_name: ::core::clone::Clone::clone(&self.facet_name),
                relation_name: ::core::clone::Clone::clone(&self.relation_name),
                query: ::core::clone::Clone::clone(&self.query),
                include_all_facets: ::core::clone::Clone::clone(&self.include_all_facets),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FacetRequest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(f,
                "FacetRequest", "facet_name", &self.facet_name,
                "relation_name", &self.relation_name, "query", &self.query,
                "include_all_facets", &&self.include_all_facets)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FacetRequest { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FacetRequest {
        #[inline]
        fn eq(&self, other: &FacetRequest) -> bool {
            self.include_all_facets == other.include_all_facets &&
                        self.facet_name == other.facet_name &&
                    self.relation_name == other.relation_name &&
                self.query == other.query
        }
    }
    impl FacetRequest {
        pub fn new(facet_name: impl Into<String>,
            relation_name: impl Into<String>,
            query: impl Into<QuerySelection>, include_all_facets: bool)
            -> Self {
            Self {
                facet_name: facet_name.into(),
                relation_name: relation_name.into(),
                query: query.into(),
                include_all_facets,
            }
        }
    }
    pub struct ObjectGroupBy {
        pub property_name: String,
        pub storage_field: String,
        pub query: QuerySelection,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObjectGroupBy {
        #[inline]
        fn clone(&self) -> ObjectGroupBy {
            ObjectGroupBy {
                property_name: ::core::clone::Clone::clone(&self.property_name),
                storage_field: ::core::clone::Clone::clone(&self.storage_field),
                query: ::core::clone::Clone::clone(&self.query),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObjectGroupBy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(f,
                "ObjectGroupBy", "property_name", &self.property_name,
                "storage_field", &self.storage_field, "query", &&self.query)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObjectGroupBy { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObjectGroupBy {
        #[inline]
        fn eq(&self, other: &ObjectGroupBy) -> bool {
            self.property_name == other.property_name &&
                    self.storage_field == other.storage_field &&
                self.query == other.query
        }
    }
    impl ObjectGroupBy {
        pub fn new(property_name: impl Into<String>,
            storage_field: impl Into<String>,
            query: impl Into<QuerySelection>) -> Self {
            Self {
                property_name: property_name.into(),
                storage_field: storage_field.into(),
                query: query.into(),
            }
        }
    }
    pub(crate) fn apply_relation_selections(mut query: SelectQuery,
        relation_selections: Vec<RelationSelection>) -> SelectQuery {
        for selection in relation_selections {
            query =
                query.relation_query(selection.name.clone(),
                    selection.into_query());
        }
        query
    }
    pub(crate) fn runtime_relation_aggregates(options: &QueryOptions)
        -> Vec<RuntimeRelationAggregate> {
        options.relation_aggregates.iter().map(|aggregate|
                    {
                        RuntimeRelationAggregate::new(aggregate.relation_name.clone(),
                            aggregate.alias.clone(),
                            aggregate.query.clone().into_query(),
                            aggregate.single_result)
                    }).collect()
    }
    pub(crate) fn execute_facets<C>(ctx: &C, outer_query: &SelectQuery,
        options: &QueryOptions)
        -> Result<BTreeMap<String, SmartList<Record>>, RuntimeError> where
        C: TeaqlRuntime + ?Sized {
        let _guard =
            QueryCommentGuard::new(ctx.user_context(),
                options.comment.clone().or(outer_query.comment.clone()));
        let mut facets = BTreeMap::new();
        for facet in &options.facets {
            let _facet_guard =
                QueryCommentGuard::new(ctx.user_context(),
                    Some(facet.facet_name.clone()));
            let mut selection = facet.query.clone();
            merge_outer_filter_into_facet_aggregates(&mut selection,
                outer_query);
            if !facet.include_all_facets {
                selection =
                    restrict_facet_to_outer_query(ctx, selection, outer_query,
                            &facet.relation_name)?;
            }
            let relation_aggregates =
                runtime_relation_aggregates(&selection.query_options);
            let query =
                apply_runtime_metadata(selection.query,
                    &selection.query_options, &selection.child_enhancements);
            let facet_rows =
                ctx.fetch_facet_smart_list(&query.entity, &query,
                        &relation_aggregates)?;
            facets.insert(facet.facet_name.clone(), facet_rows);
        }
        Ok(facets)
    }
    pub(crate) fn merge_outer_filter_into_facet_aggregates(selection:
            &mut QuerySelection, outer_query: &SelectQuery) {
        let Some(filter) = outer_query.filter.clone() else { return; };
        for aggregate in &mut selection.query_options.relation_aggregates {
            if aggregate.query.query.entity == outer_query.entity {
                aggregate.query.query =
                    aggregate.query.query.clone().and_filter(filter.clone());
            }
        }
    }
    pub(crate) fn restrict_facet_to_outer_query<C>(ctx: &C,
        mut selection: QuerySelection, outer_query: &SelectQuery,
        relation_name: &str) -> Result<QuerySelection, RuntimeError> where
        C: TeaqlRuntime + ?Sized {
        let descriptor =
            ctx.user_context().entity(&outer_query.entity).cloned().ok_or_else(||
                        RuntimeError::Graph(::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("missing entity: {0}",
                                            outer_query.entity))
                                })))?;
        let relation =
            descriptor.relation_by_name(relation_name).cloned().ok_or_else(||
                        RuntimeError::MissingRelation {
                            entity: outer_query.entity.clone(),
                            relation: relation_name.to_owned(),
                        })?;
        let mut subquery = outer_query.clone();
        subquery.projection.clear();
        subquery.expr_projection.clear();
        subquery.order_by.clear();
        subquery.slice = None;
        subquery.aggregates.clear();
        subquery.group_by.clear();
        subquery.relations.clear();
        selection.query =
            selection.query.and_filter(Expr::in_subquery(relation.foreign_key,
                    descriptor, subquery, relation.local_key));
        Ok(selection)
    }
    pub(crate) fn attach_facets<T>(rows: &mut SmartList<T>,
        facets: BTreeMap<String, SmartList<Record>>) {
        for (name, facet) in facets { rows.add_facet(name, facet); }
    }
    pub(crate) fn apply_runtime_metadata(mut query: SelectQuery,
        options: &QueryOptions, child_enhancements: &[QuerySelection])
        -> SelectQuery {
        query.comment = options.comment.clone();
        query.raw_sql = options.raw_sql.clone();
        query.raw_sql_search_criteria =
            options.raw_sql_search_criteria.clone();
        query.dynamic_properties =
            options.dynamic_properties.iter().map(|projection|
                        {
                            teaql_core::RawSqlProjection::new(projection.property_name.clone(),
                                projection.raw_sql_segment.clone())
                        }).collect();
        query.raw_projections =
            options.raw_projections.iter().map(|projection|
                        {
                            teaql_core::RawSqlProjection::new(projection.property_name.clone(),
                                projection.raw_sql_segment.clone())
                        }).collect();
        query.object_group_bys =
            options.object_group_bys.iter().map(|group_by|
                        {
                            teaql_core::ObjectGroupBy::new(group_by.property_name.clone(),
                                group_by.storage_field.clone(),
                                group_by.query.clone().into_query())
                        }).collect();
        query.child_enhancements =
            child_enhancements.iter().cloned().map(QuerySelection::into_query).collect();
        query
    }
    pub(crate) fn field_operator_expr(field: &str, operator: FieldOperator,
        values: Vec<teaql_core::Value>) -> Expr {
        match operator {
            FieldOperator::Equal =>
                Expr::eq(field, required_value(operator, &values, 0)),
            FieldOperator::NotEqual =>
                Expr::ne(field, required_value(operator, &values, 0)),
            FieldOperator::GreaterThan =>
                Expr::gt(field, required_value(operator, &values, 0)),
            FieldOperator::GreaterThanOrEqual =>
                Expr::gte(field, required_value(operator, &values, 0)),
            FieldOperator::LessThan =>
                Expr::lt(field, required_value(operator, &values, 0)),
            FieldOperator::LessThanOrEqual =>
                Expr::lte(field, required_value(operator, &values, 0)),
            FieldOperator::Between =>
                Expr::between(field, required_value(operator, &values, 0),
                    required_value(operator, &values, 1)),
            FieldOperator::In => Expr::in_list(field, values),
            FieldOperator::NotIn => Expr::not_in_list(field, values),
            FieldOperator::Contain =>
                Expr::contain(field, required_text(operator, &values, 0)),
            FieldOperator::NotContain =>
                Expr::not_contain(field, required_text(operator, &values, 0)),
            FieldOperator::BeginWith =>
                Expr::begin_with(field, required_text(operator, &values, 0)),
            FieldOperator::NotBeginWith =>
                Expr::not_begin_with(field,
                    required_text(operator, &values, 0)),
            FieldOperator::EndWith =>
                Expr::end_with(field, required_text(operator, &values, 0)),
            FieldOperator::NotEndWith =>
                Expr::not_end_with(field,
                    required_text(operator, &values, 0)),
            FieldOperator::SoundsLike =>
                Expr::sound_like(field, required_value(operator, &values, 0)),
            FieldOperator::IsNull => Expr::is_null(field),
            FieldOperator::IsNotNull => Expr::is_not_null(field),
        }
    }
    pub(crate) fn field_operator_column_expr(field: &str,
        operator: FieldOperator, other_field: &str) -> Expr {
        let binary_op =
            match operator {
                FieldOperator::Equal => BinaryOp::Eq,
                FieldOperator::NotEqual => BinaryOp::Ne,
                FieldOperator::GreaterThan => BinaryOp::Gt,
                FieldOperator::GreaterThanOrEqual => BinaryOp::Gte,
                FieldOperator::LessThan => BinaryOp::Lt,
                FieldOperator::LessThanOrEqual => BinaryOp::Lte,
                FieldOperator::Contain => BinaryOp::Like,
                FieldOperator::NotContain => BinaryOp::NotLike,
                FieldOperator::BeginWith => BinaryOp::Like,
                FieldOperator::NotBeginWith => BinaryOp::NotLike,
                FieldOperator::EndWith => BinaryOp::Like,
                FieldOperator::NotEndWith => BinaryOp::NotLike,
                unsupported => {
                    ::core::panicking::panic_fmt(format_args!("{0:?} is not supported for property-to-property filters",
                            unsupported));
                }
            };
        Expr::compare_columns(field, binary_op, other_field)
    }
    pub(crate) fn dynamic_json_value_to_teaql_value(value: &JsonValue)
        -> teaql_core::Value {
        match value {
            JsonValue::Null => teaql_core::Value::Null,
            JsonValue::Bool(value) => teaql_core::Value::Bool(*value),
            JsonValue::Number(value) => {
                if let Some(value) = value.as_i64() {
                    teaql_core::Value::I64(value)
                } else if let Some(value) = value.as_u64() {
                    teaql_core::Value::U64(value)
                } else if let Some(value) = value.as_f64() {
                    teaql_core::Value::F64(value)
                } else { teaql_core::Value::Null }
            }
            JsonValue::String(value) =>
                teaql_core::Value::Text(value.trim().to_owned()),
            JsonValue::Array(values) =>
                teaql_core::Value::List(values.iter().map(dynamic_json_value_to_teaql_value).collect()),
            JsonValue::Object(object) =>
                object.get("id").map(dynamic_json_value_to_teaql_value).unwrap_or(teaql_core::Value::Null),
        }
    }
    pub(crate) fn dynamic_json_values(value: &JsonValue)
        -> Vec<teaql_core::Value> {
        match value {
            JsonValue::Array(values) =>
                values.iter().map(dynamic_json_value_to_teaql_value).collect(),
            value =>
                ::alloc::boxed::box_assume_init_into_vec_unsafe(::alloc::intrinsics::write_box_via_move(::alloc::boxed::Box::new_uninit(),
                        [dynamic_json_value_to_teaql_value(value)])),
        }
    }
    pub(crate) fn dynamic_json_operator(value: &JsonValue) -> FieldOperator {
        match value {
            JsonValue::String(value) if
                value.eq_ignore_ascii_case("__is_null__") =>
                FieldOperator::IsNull,
            JsonValue::String(value) if
                value.eq_ignore_ascii_case("__is_not_null__") => {
                FieldOperator::IsNotNull
            }
            JsonValue::String(_) => FieldOperator::Contain,
            JsonValue::Number(_) | JsonValue::Bool(_) => FieldOperator::Equal,
            JsonValue::Array(values) if
                values.first().map(JsonValue::is_string).unwrap_or(false) => {
                FieldOperator::In
            }
            JsonValue::Array(values) if
                values.first().map(JsonValue::is_object).unwrap_or(false) => {
                FieldOperator::In
            }
            JsonValue::Array(values) if values.len() == 2 =>
                FieldOperator::Between,
            _ => FieldOperator::Equal,
        }
    }
    pub(crate) fn dynamic_json_filter_expr(field: &str, value: &JsonValue)
        -> Expr {
        let operator = dynamic_json_operator(value);
        field_operator_expr(field, operator, dynamic_json_values(value))
    }
    pub(crate) fn dynamic_json_u64_field(object:
            &serde_json::Map<String, JsonValue>, field: &str) -> Option<u64> {
        object.get(field).and_then(|value|
                {
                    value.as_u64().or_else(||
                            value.as_i64().and_then(|value| u64::try_from(value).ok()))
                })
    }
    pub(crate) fn remove_default_live_filter(filter: Option<Expr>)
        -> Option<Expr> {
        let default_filter = Expr::gt("version", 0_i64);
        remove_filter_expr(filter?, &default_filter)
    }
    pub(crate) fn remove_filter_expr(filter: Expr, target: &Expr)
        -> Option<Expr> {
        if &filter == target { return None; }
        match filter {
            Expr::And(parts) => {
                let mut retained =
                    parts.into_iter().filter_map(|part|
                                remove_filter_expr(part, target)).collect::<Vec<_>>();
                match retained.len() {
                    0 => None,
                    1 => retained.pop(),
                    _ => Some(Expr::And(retained)),
                }
            }
            other => Some(other),
        }
    }
    pub(crate) fn required_value(operator: FieldOperator,
        values: &[teaql_core::Value], index: usize) -> teaql_core::Value {
        values.get(index).cloned().unwrap_or_else(||
                {
                    {
                        ::core::panicking::panic_fmt(format_args!("{0:?} requires value at index {1}",
                                operator, index));
                    }
                })
    }
    pub(crate) fn required_text(operator: FieldOperator,
        values: &[teaql_core::Value], index: usize) -> String {
        match required_value(operator, values, index) {
            teaql_core::Value::Text(value) => value,
            value => {
                ::core::panicking::panic_fmt(format_args!("{0:?} requires text value, got {1:?}",
                        operator, value));
            }
        }
    }
    impl EntityReference for teaql_core::Value {
        fn entity_id_value(self) -> teaql_core::Value { self }
    }
    impl EntityReference for u64 {
        fn entity_id_value(self) -> teaql_core::Value {
            teaql_core::Value::U64(self)
        }
    }
}
pub mod runtime {
    use crate::*;
    use teaql_provider_rusqlite::RusqliteProviderExt as _;
    pub type DataServiceDialect = teaql_provider_rusqlite::RusqliteDialect;
    pub type DataServiceMutationExecutor =
        teaql_provider_rusqlite::RusqliteMutationExecutor;
    pub type DataServiceMutationError =
        teaql_provider_rusqlite::MutationExecutorError;
    pub type DataServiceIdGenerator =
        teaql_provider_rusqlite::RusqliteIdSpaceGenerator;
    pub type DataServicePool = rusqlite::Connection;
    pub type DataServiceExecutor = ServiceRuntimeExecutor;
    pub type ServiceRuntime = teaql_runtime::UserContext;
    pub const DATABASE_URL_ENV: &str = "ROBOT_KANBAN_SERVICE_DATABASE_URL";
    pub const DATABASE_USER_ENV: &str = "ROBOT_KANBAN_SERVICE_DATABASE_USER";
    pub const DATABASE_PASSWORD_ENV: &str =
        "ROBOT_KANBAN_SERVICE_DATABASE_PASSWORD";
    pub struct ServiceRuntimeConfig {
        pub database_url: String,
        pub database_user: String,
        pub database_password: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceRuntimeConfig {
        #[inline]
        fn clone(&self) -> ServiceRuntimeConfig {
            ServiceRuntimeConfig {
                database_url: ::core::clone::Clone::clone(&self.database_url),
                database_user: ::core::clone::Clone::clone(&self.database_user),
                database_password: ::core::clone::Clone::clone(&self.database_password),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceRuntimeConfig {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(f,
                "ServiceRuntimeConfig", "database_url", &self.database_url,
                "database_user", &self.database_user, "database_password",
                &&self.database_password)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ServiceRuntimeConfig { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ServiceRuntimeConfig {
        #[inline]
        fn eq(&self, other: &ServiceRuntimeConfig) -> bool {
            self.database_url == other.database_url &&
                    self.database_user == other.database_user &&
                self.database_password == other.database_password
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ServiceRuntimeConfig {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    impl ServiceRuntimeConfig {
        pub fn from_env() -> Result<Self, ServiceRuntimeError> {
            Ok(Self {
                    database_url: env_value(DATABASE_URL_ENV)?,
                    database_user: env_value(DATABASE_USER_ENV)?,
                    database_password: env_value(DATABASE_PASSWORD_ENV)?,
                })
        }
    }
    pub enum ServiceRuntimeError {
        MissingEnv {
            name: &'static str,
            source: std::env::VarError,
        },
        Runtime(teaql_runtime::RuntimeError),
        Rusqlite(rusqlite::Error),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceRuntimeError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ServiceRuntimeError::MissingEnv {
                    name: __self_0, source: __self_1 } =>
                    ::core::fmt::Formatter::debug_struct_field2_finish(f,
                        "MissingEnv", "name", __self_0, "source", &__self_1),
                ServiceRuntimeError::Runtime(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                        "Runtime", &__self_0),
                ServiceRuntimeError::Rusqlite(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f,
                        "Rusqlite", &__self_0),
            }
        }
    }
    impl std::fmt::Display for ServiceRuntimeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ServiceRuntimeError::MissingEnv { name, source } => {
                    f.write_fmt(format_args!("missing environment variable {0}: {1}",
                            name, source))
                }
                ServiceRuntimeError::Runtime(err) =>
                    f.write_fmt(format_args!("runtime error: {0}", err)),
                ServiceRuntimeError::Rusqlite(err) =>
                    f.write_fmt(format_args!("rusqlite error: {0}", err)),
            }
        }
    }
    impl std::error::Error for ServiceRuntimeError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                ServiceRuntimeError::MissingEnv { source, .. } =>
                    Some(source),
                ServiceRuntimeError::Runtime(err) => Some(err),
                ServiceRuntimeError::Rusqlite(err) => Some(err),
            }
        }
    }
    impl From<rusqlite::Error> for ServiceRuntimeError {
        fn from(err: rusqlite::Error) -> Self {
            ServiceRuntimeError::Rusqlite(err)
        }
    }
    impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
        fn from(err: teaql_runtime::RuntimeError) -> Self {
            ServiceRuntimeError::Runtime(err)
        }
    }
    pub struct ServiceRuntimeExecutor {
        inner: DataServiceMutationExecutor,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceRuntimeExecutor {
        #[inline]
        fn clone(&self) -> ServiceRuntimeExecutor {
            ServiceRuntimeExecutor {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    impl ServiceRuntimeExecutor {
        pub fn new(inner: DataServiceMutationExecutor) -> Self {
            Self { inner }
        }
        pub fn inner(&self) -> &DataServiceMutationExecutor { &self.inner }
    }
    impl teaql_runtime::QueryExecutor for ServiceRuntimeExecutor {
        type Error = DataServiceMutationError;
        fn fetch_all(&self, query: &teaql_sql::CompiledQuery)
            -> Result<Vec<teaql_core::Record>, Self::Error> {
            let inner = self.inner.clone();
            let query = query.clone();
            inner.fetch_all(&query)
        }
        fn execute(&self, query: &teaql_sql::CompiledQuery)
            -> Result<u64, Self::Error> {
            let inner = self.inner.clone();
            let query = query.clone();
            inner.execute(&query)
        }
        fn begin_transaction(&self)
            -> Result<teaql_runtime::GraphTransactionBoundary, Self::Error> {
            teaql_runtime::QueryExecutor::begin_transaction(&self.inner)
        }
        fn commit_transaction(&self) -> Result<(), Self::Error> {
            teaql_runtime::QueryExecutor::commit_transaction(&self.inner)
        }
        fn rollback_transaction(&self) -> Result<(), Self::Error> {
            teaql_runtime::QueryExecutor::rollback_transaction(&self.inner)
        }
    }
    fn block_on_data_service<F, T>(future: F) -> T where
        F: std::future::Future<Output = T> + Send + 'static, T: Send +
        'static {
        if tokio::runtime::Handle::try_current().is_ok() {
            std::thread::spawn(move ||
                            {
                                tokio::runtime::Builder::new_current_thread().enable_all().build().expect("data service runtime").block_on(future)
                            }).join().expect("data service runtime thread")
        } else {
            tokio::runtime::Builder::new_current_thread().enable_all().build().expect("data service runtime").block_on(future)
        }
    }
    pub async fn service_runtime_from_pool(pool: DataServicePool)
        -> Result<ServiceRuntime, ServiceRuntimeError> {
        let mutation_executor = DataServiceMutationExecutor::new(pool);
        let id_generator =
            DataServiceIdGenerator::from_executor(mutation_executor.clone());
        let runtime_executor =
            ServiceRuntimeExecutor::new(mutation_executor.clone());
        let mut context = module_with_behaviors_and_checkers().into_context();
        context.set_internal_id_generator(id_generator);
        context.use_rusqlite_provider(mutation_executor);
        context.insert_resource(runtime_executor);
        context.ensure_schema().await?;
        Ok(context)
    }
    fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
        std::env::var(name).map_err(|source|
                ServiceRuntimeError::MissingEnv { name, source })
    }
    pub fn repository_registry()
        -> teaql_runtime::InMemoryRepositoryRegistry {
        teaql_runtime::InMemoryRepositoryRegistry::new().with_entity("Platform").with_entity("TaskStatus").with_entity("Task").with_entity("TaskExecutionLog")
    }
    pub fn behavior_registry()
        -> teaql_runtime::InMemoryRepositoryBehaviorRegistry {
        teaql_runtime::InMemoryRepositoryBehaviorRegistry::new().with_behavior("Platform",
                        PlatformBehavior::default()).with_behavior("TaskStatus",
                    TaskStatusBehavior::default()).with_behavior("Task",
                TaskBehavior::default()).with_behavior("TaskExecutionLog",
            TaskExecutionLogBehavior::default())
    }
    pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
        teaql_runtime::InMemoryCheckerRegistry::new().with_checker(teaql_runtime::TypedEntityChecker::<Platform,
                                _>::new(PlatformChecker::default())).with_checker(teaql_runtime::TypedEntityChecker::<TaskStatus,
                            _>::new(TaskStatusChecker::default())).with_checker(teaql_runtime::TypedEntityChecker::<Task,
                        _>::new(TaskChecker::default())).with_checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog,
                    _>::new(TaskExecutionLogChecker::default()))
    }
    pub fn module() -> teaql_runtime::RuntimeModule {
        teaql_runtime::RuntimeModule::new().entity::<Platform>().entity::<TaskStatus>().entity::<Task>().entity::<TaskExecutionLog>().initial_graph(teaql_runtime::GraphNode::new("Platform").value("id",
                                            1_u64).value("name",
                                        "Robot System").value("founded",
                                    chrono::Utc::now()).value("version",
                                1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                    1_u64).value("name",
                                                "Planned").value("code",
                                            "PLANNED").value("color",
                                        "#94A3B8").value("display_order",
                                    "10").value("progress",
                                "0").value("version",
                            1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                1001_u64).value("name",
                                            "Planned").value("code",
                                        "PLANNED").value("color",
                                    "#94A3B8").value("display_order",
                                "10").value("progress",
                            "0").value("version",
                        1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                            1002_u64).value("name",
                                        "Process").value("code",
                                    "PROCESS").value("color",
                                "#F59E0B").value("display_order",
                            "20").value("progress",
                        "50").value("version",
                    1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                        1003_u64).value("name",
                                    "Done").value("code",
                                "DONE").value("color",
                            "#16A34A").value("display_order",
                        "30").value("progress", "100").value("version", 1_i64))
    }
    pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
        teaql_runtime::RuntimeModule::new().entity::<Platform>().checker(teaql_runtime::TypedEntityChecker::<Platform,
                                                                _>::new(PlatformChecker::default())).entity::<TaskStatus>().checker(teaql_runtime::TypedEntityChecker::<TaskStatus,
                                                        _>::new(TaskStatusChecker::default())).entity::<Task>().checker(teaql_runtime::TypedEntityChecker::<Task,
                                                _>::new(TaskChecker::default())).entity::<TaskExecutionLog>().checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog,
                                        _>::new(TaskExecutionLogChecker::default())).initial_graph(teaql_runtime::GraphNode::new("Platform").value("id",
                                            1_u64).value("name",
                                        "Robot System").value("founded",
                                    chrono::Utc::now()).value("version",
                                1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                    1_u64).value("name",
                                                "Planned").value("code",
                                            "PLANNED").value("color",
                                        "#94A3B8").value("display_order",
                                    "10").value("progress",
                                "0").value("version",
                            1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                1001_u64).value("name",
                                            "Planned").value("code",
                                        "PLANNED").value("color",
                                    "#94A3B8").value("display_order",
                                "10").value("progress",
                            "0").value("version",
                        1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                            1002_u64).value("name",
                                        "Process").value("code",
                                    "PROCESS").value("color",
                                "#F59E0B").value("display_order",
                            "20").value("progress",
                        "50").value("version",
                    1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                        1003_u64).value("name",
                                    "Done").value("code",
                                "DONE").value("color",
                            "#16A34A").value("display_order",
                        "30").value("progress", "100").value("version", 1_i64))
    }
    pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
        teaql_runtime::RuntimeModule::new().entity_with_behavior::<Platform,
                                            _>(PlatformBehavior::default()).entity_with_behavior::<TaskStatus,
                                        _>(TaskStatusBehavior::default()).entity_with_behavior::<Task,
                                    _>(TaskBehavior::default()).entity_with_behavior::<TaskExecutionLog,
                                _>(TaskExecutionLogBehavior::default()).initial_graph(teaql_runtime::GraphNode::new("Platform").value("id",
                                            1_u64).value("name",
                                        "Robot System").value("founded",
                                    chrono::Utc::now()).value("version",
                                1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                    1_u64).value("name",
                                                "Planned").value("code",
                                            "PLANNED").value("color",
                                        "#94A3B8").value("display_order",
                                    "10").value("progress",
                                "0").value("version",
                            1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                1001_u64).value("name",
                                            "Planned").value("code",
                                        "PLANNED").value("color",
                                    "#94A3B8").value("display_order",
                                "10").value("progress",
                            "0").value("version",
                        1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                            1002_u64).value("name",
                                        "Process").value("code",
                                    "PROCESS").value("color",
                                "#F59E0B").value("display_order",
                            "20").value("progress",
                        "50").value("version",
                    1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                        1003_u64).value("name",
                                    "Done").value("code",
                                "DONE").value("color",
                            "#16A34A").value("display_order",
                        "30").value("progress", "100").value("version", 1_i64))
    }
    pub fn module_with_behaviors_and_checkers()
        -> teaql_runtime::RuntimeModule {
        teaql_runtime::RuntimeModule::new().entity_with_behavior::<Platform,
                                                            _>(PlatformBehavior::default()).checker(teaql_runtime::TypedEntityChecker::<Platform,
                                                                _>::new(PlatformChecker::default())).entity_with_behavior::<TaskStatus,
                                                    _>(TaskStatusBehavior::default()).checker(teaql_runtime::TypedEntityChecker::<TaskStatus,
                                                        _>::new(TaskStatusChecker::default())).entity_with_behavior::<Task,
                                            _>(TaskBehavior::default()).checker(teaql_runtime::TypedEntityChecker::<Task,
                                                _>::new(TaskChecker::default())).entity_with_behavior::<TaskExecutionLog,
                                    _>(TaskExecutionLogBehavior::default()).checker(teaql_runtime::TypedEntityChecker::<TaskExecutionLog,
                                        _>::new(TaskExecutionLogChecker::default())).initial_graph(teaql_runtime::GraphNode::new("Platform").value("id",
                                            1_u64).value("name",
                                        "Robot System").value("founded",
                                    chrono::Utc::now()).value("version",
                                1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                    1_u64).value("name",
                                                "Planned").value("code",
                                            "PLANNED").value("color",
                                        "#94A3B8").value("display_order",
                                    "10").value("progress",
                                "0").value("version",
                            1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                                1001_u64).value("name",
                                            "Planned").value("code",
                                        "PLANNED").value("color",
                                    "#94A3B8").value("display_order",
                                "10").value("progress",
                            "0").value("version",
                        1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                            1002_u64).value("name",
                                        "Process").value("code",
                                    "PROCESS").value("color",
                                "#F59E0B").value("display_order",
                            "20").value("progress",
                        "50").value("version",
                    1_i64)).initial_graph(teaql_runtime::GraphNode::new("TaskStatus").value("id",
                                        1003_u64).value("name",
                                    "Done").value("code",
                                "DONE").value("color",
                            "#16A34A").value("display_order",
                        "30").value("progress", "100").value("version", 1_i64))
    }
}
pub mod sample_data {
    use std::collections::BTreeMap;
    use crate::TeaqlRuntime;
    use crate::Q;
    pub trait IntoU64 {
        fn into_u64(self)
        -> u64;
    }
    impl IntoU64 for u64 {
        fn into_u64(self) -> u64 { self }
    }
    impl IntoU64 for Option<&teaql_core::Value> {
        fn into_u64(self) -> u64 {
            self.and_then(|v| v.try_u64()).unwrap_or_default()
        }
    }
    pub enum SampleDataScale { Tiny, Small, Medium, }
    #[automatically_derived]
    impl ::core::fmt::Debug for SampleDataScale {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    SampleDataScale::Tiny => "Tiny",
                    SampleDataScale::Small => "Small",
                    SampleDataScale::Medium => "Medium",
                })
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SampleDataScale { }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for SampleDataScale { }
    #[automatically_derived]
    impl ::core::clone::Clone for SampleDataScale {
        #[inline]
        fn clone(&self) -> SampleDataScale { *self }
    }
    pub struct SampleDataPlan {
        pub scale: SampleDataScale,
        pub seed: u64,
    }
    impl SampleDataPlan {
        pub fn small() -> Self {
            Self { scale: SampleDataScale::Small, seed: 0 }
        }
    }
    pub struct SampleDataReport {
        pub generated: BTreeMap<&'static str, usize>,
        pub skipped: Vec<SampleDataSkipped>,
    }
    pub struct SampleDataSkipped {
        pub entity: &'static str,
        pub reason: String,
    }
    pub struct SampleDataState {
        pub plan: SampleDataPlan,
        pub references: BTreeMap<&'static str, Vec<u64>>,
        pub generated: BTreeMap<&'static str, usize>,
        pub skipped: Vec<SampleDataSkipped>,
    }
    impl SampleDataState {
        pub fn new(plan: SampleDataPlan) -> Self {
            Self {
                plan,
                references: BTreeMap::new(),
                generated: BTreeMap::new(),
                skipped: Vec::new(),
            }
        }
        pub fn add_reference(&mut self, entity: &'static str, id: u64) {
            self.references.entry(entity).or_default().push(id);
        }
        pub fn ids(&self, entity: &'static str) -> &[u64] {
            self.references.get(entity).map(|v| v.as_slice()).unwrap_or(&[])
        }
        pub fn pick_id(&self, entity: &'static str, salt: usize)
            -> Option<u64> {
            let ids = self.ids(entity);
            if ids.is_empty() { None } else { Some(ids[salt % ids.len()]) }
        }
        pub fn pick_unused_id(&self, entity: &'static str, salt: usize,
            used: &std::collections::HashSet<u64>) -> Option<u64> {
            let ids = self.ids(entity);
            if ids.is_empty() { return None; }
            let best_id = ids[salt % ids.len()];
            if !used.contains(&best_id) { return Some(best_id); }
            for id in ids { if !used.contains(id) { return Some(*id); } }
            Some(best_id)
        }
        pub fn record_generated(&mut self, entity: &'static str) {
            *self.generated.entry(entity).or_default() += 1;
        }
        pub fn record_skipped(&mut self, entity: &'static str,
            reason: String) {
            self.skipped.push(SampleDataSkipped { entity, reason });
        }
        pub fn into_report(self) -> SampleDataReport {
            SampleDataReport {
                generated: self.generated,
                skipped: self.skipped,
            }
        }
    }
    pub async fn generate_sample_data<C>(ctx: &C, plan: SampleDataPlan)
        -> Result<SampleDataReport, String> where C: TeaqlRuntime + ?Sized {
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                    {
                    ::log::__private_api::log({
                            ::log::__private_api::GlobalLogger
                        },
                        format_args!("Starting sample data generation. Scale: {0:?}, Seed: {1}",
                            plan.scale, plan.seed), lvl,
                        &("robot_kanban_service::sample_data",
                                "robot_kanban_service::sample_data",
                                ::log::__private_api::loc()), ());
                }
            }
        };
        let mut state = SampleDataState::new(plan);
        load_root_platforms(ctx, &mut state).await?;
        load_root_task_status(ctx, &mut state).await?;
        use crate::request_support::TeaqlUserContextExt;
        ctx.user_context().transaction_data(||
                            async {
                                let res =
                                    async {
                                            generate_tasks(ctx, &mut state).await?;
                                            generate_task_execution_logs(ctx, &mut state).await?;
                                            Ok::<(), String>(())
                                        }.await;
                                res.map_err(|e|
                                        {
                                            teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(e))
                                        })
                            }).await.map_err(|e| e.to_string())?;
        let report = state.into_report();
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                    {
                    ::log::__private_api::log({
                            ::log::__private_api::GlobalLogger
                        },
                        format_args!("Sample data generation completed successfully. Generated: {0} tables, Skipped: {1} tables.",
                            report.generated.len(), report.skipped.len()), lvl,
                        &("robot_kanban_service::sample_data",
                                "robot_kanban_service::sample_data",
                                ::log::__private_api::loc()), ());
                }
            }
        };
        Ok(report)
    }
    async fn load_root_platforms<C>(ctx: &C, state: &mut SampleDataState)
        -> Result<(), String> where C: TeaqlRuntime + ?Sized {
        let list =
            Q::platforms().execute_for_list(ctx).await.unwrap_or_default();
        for item in list {
            state.add_reference("Platform", item.id().into_u64());
        }
        Ok(())
    }
    async fn load_root_task_status<C>(ctx: &C, state: &mut SampleDataState)
        -> Result<(), String> where C: TeaqlRuntime + ?Sized {
        let list =
            Q::task_status().execute_for_list(ctx).await.unwrap_or_default();
        for item in list {
            state.add_reference("Task Status", item.id().into_u64());
        }
        Ok(())
    }
    async fn generate_tasks<C>(ctx: &C, state: &mut SampleDataState)
        -> Result<(), String> where C: TeaqlRuntime + ?Sized {
        if state.ids("Task Status").is_empty() {
            state.record_skipped("Task",
                "Required dependency Task Status is missing in reference pool".to_string());
            {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                            lvl <= ::log::max_level() {
                        ::log::__private_api::log({
                                ::log::__private_api::GlobalLogger
                            },
                            format_args!("Skipped generating Task: Required dependency Task Status is missing in reference pool."),
                            lvl,
                            &("robot_kanban_service::sample_data",
                                    "robot_kanban_service::sample_data",
                                    ::log::__private_api::loc()), ());
                    }
                }
            };
            return Ok(());
        }
        if state.ids("Platform").is_empty() {
            state.record_skipped("Task",
                "Required dependency Platform is missing in reference pool".to_string());
            {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                            lvl <= ::log::max_level() {
                        ::log::__private_api::log({
                                ::log::__private_api::GlobalLogger
                            },
                            format_args!("Skipped generating Task: Required dependency Platform is missing in reference pool."),
                            lvl,
                            &("robot_kanban_service::sample_data",
                                    "robot_kanban_service::sample_data",
                                    ::log::__private_api::loc()), ());
                    }
                }
            };
            return Ok(());
        }
        let object_fields_count = 0 + 1 + 1;
        let base_fanout = std::cmp::max(1, object_fields_count) * 20;
        let fanout =
            match state.plan.scale {
                SampleDataScale::Tiny => base_fanout,
                SampleDataScale::Small => base_fanout * 5,
                SampleDataScale::Medium => base_fanout * 50,
            };
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                    {
                    ::log::__private_api::log({
                            ::log::__private_api::GlobalLogger
                        },
                        format_args!("Generating sample data for Task (expected: {0})...",
                            fanout), lvl,
                        &("robot_kanban_service::sample_data",
                                "robot_kanban_service::sample_data",
                                ::log::__private_api::loc()), ());
                }
            }
        };
        for i in 0..fanout {
            let mut entity = Q::tasks().new_entity(ctx);
            let mut used_refs = std::collections::HashSet::new();
            if let Some(ref_id) =
                    state.pick_unused_id("Task Status", i as usize, &used_refs)
                {
                entity.update_status_id(ref_id);
                used_refs.insert(ref_id);
            } else {}
            if let Some(ref_id) =
                    state.pick_unused_id("Platform", i as usize, &used_refs) {
                entity.update_platform_id(ref_id);
                used_refs.insert(ref_id);
            } else {}
            entity.update_name(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} {1}", "Task Name",
                                i + 1))
                    }));
            let entity = entity.save(ctx).await.map_err(|e| e.to_string())?;
            state.record_generated("Task");
            if i % 20 == 0 {
                {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                                lvl <= ::log::max_level() {
                            ::log::__private_api::log({
                                    ::log::__private_api::GlobalLogger
                                }, format_args!("Generating Task: {0}/{1}", i, fanout), lvl,
                                &("robot_kanban_service::sample_data",
                                        "robot_kanban_service::sample_data",
                                        ::log::__private_api::loc()), ());
                        }
                    }
                };
            }
            state.add_reference("Task", entity.id().into_u64());
        }
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                    {
                    ::log::__private_api::log({
                            ::log::__private_api::GlobalLogger
                        },
                        format_args!("Successfully generated sample records for Task."),
                        lvl,
                        &("robot_kanban_service::sample_data",
                                "robot_kanban_service::sample_data",
                                ::log::__private_api::loc()), ());
                }
            }
        };
        Ok(())
    }
    async fn generate_task_execution_logs<C>(ctx: &C,
        state: &mut SampleDataState) -> Result<(), String> where
        C: TeaqlRuntime + ?Sized {
        if state.ids("Task").is_empty() {
            state.record_skipped("Task Execution Log",
                "Required dependency Task is missing in reference pool".to_string());
            {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                            lvl <= ::log::max_level() {
                        ::log::__private_api::log({
                                ::log::__private_api::GlobalLogger
                            },
                            format_args!("Skipped generating Task Execution Log: Required dependency Task is missing in reference pool."),
                            lvl,
                            &("robot_kanban_service::sample_data",
                                    "robot_kanban_service::sample_data",
                                    ::log::__private_api::loc()), ());
                    }
                }
            };
            return Ok(());
        }
        let object_fields_count = 0 + 1;
        let base_fanout = std::cmp::max(1, object_fields_count) * 20;
        let fanout =
            match state.plan.scale {
                SampleDataScale::Tiny => base_fanout,
                SampleDataScale::Small => base_fanout * 5,
                SampleDataScale::Medium => base_fanout * 50,
            };
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                    {
                    ::log::__private_api::log({
                            ::log::__private_api::GlobalLogger
                        },
                        format_args!("Generating sample data for Task Execution Log (expected: {0})...",
                            fanout), lvl,
                        &("robot_kanban_service::sample_data",
                                "robot_kanban_service::sample_data",
                                ::log::__private_api::loc()), ());
                }
            }
        };
        for i in 0..fanout {
            let mut entity = Q::task_execution_logs().new_entity(ctx);
            let mut used_refs = std::collections::HashSet::new();
            if let Some(ref_id) =
                    state.pick_unused_id("Task", i as usize, &used_refs) {
                entity.update_task_id(ref_id);
                used_refs.insert(ref_id);
            } else {}
            entity.update_action(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} {1}", "string()",
                                i + 1))
                    }));
            entity.update_detail(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} {1}", "string()",
                                i + 1))
                    }));
            entity.save(ctx).await.map_err(|e| e.to_string())?;
            state.record_generated("Task Execution Log");
            if i % 20 == 0 {
                {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                                lvl <= ::log::max_level() {
                            ::log::__private_api::log({
                                    ::log::__private_api::GlobalLogger
                                },
                                format_args!("Generating Task Execution Log: {0}/{1}", i,
                                    fanout), lvl,
                                &("robot_kanban_service::sample_data",
                                        "robot_kanban_service::sample_data",
                                        ::log::__private_api::loc()), ());
                        }
                    }
                };
            }
        }
        {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                    {
                    ::log::__private_api::log({
                            ::log::__private_api::GlobalLogger
                        },
                        format_args!("Successfully generated sample records for Task Execution Log."),
                        lvl,
                        &("robot_kanban_service::sample_data",
                                "robot_kanban_service::sample_data",
                                ::log::__private_api::loc()), ());
                }
            }
        };
        Ok(())
    }
}
pub mod platform {
    mod behavior {
        use teaql_runtime::RepositoryBehavior;
        pub struct PlatformBehavior;
        #[automatically_derived]
        impl ::core::clone::Clone for PlatformBehavior {
            #[inline]
            fn clone(&self) -> PlatformBehavior { PlatformBehavior }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlatformBehavior {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "PlatformBehavior")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for PlatformBehavior {
            #[inline]
            fn default() -> PlatformBehavior { PlatformBehavior {} }
        }
        impl RepositoryBehavior for PlatformBehavior {}
    }
    mod checker {
        use teaql_runtime::{
            CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker,
            UserContext,
        };
        pub trait PlatformCheckerLogic: Send + Sync {
            fn check_and_fix_platform(&self, _ctx: &UserContext,
                _entity: &mut crate::Platform, _status: CheckObjectStatus,
                _location: &ObjectLocation, _results: &mut CheckResults) {}
            fn required(&self, value: bool, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if !value {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_option<V>(&self, value: Option<&V>, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.is_none() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_text(&self, value: &str, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.trim().is_empty() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn min_string_length(&self, value: &str, field: &str,
                min_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() < min_len {
                    results.push(teaql_runtime::CheckResult::min_str(location.clone().member(field),
                            min_len as u64, value.to_owned()));
                }
            }
            fn max_string_length(&self, value: &str, field: &str,
                max_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() > max_len {
                    results.push(teaql_runtime::CheckResult::max_str(location.clone().member(field),
                            max_len as u64, value.to_owned()));
                }
            }
        }
        pub struct NoopPlatformChecker;
        #[automatically_derived]
        impl ::core::clone::Clone for NoopPlatformChecker {
            #[inline]
            fn clone(&self) -> NoopPlatformChecker { NoopPlatformChecker }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NoopPlatformChecker {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "NoopPlatformChecker")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NoopPlatformChecker {
            #[inline]
            fn default() -> NoopPlatformChecker { NoopPlatformChecker {} }
        }
        impl PlatformCheckerLogic for NoopPlatformChecker {}
        pub struct PlatformChecker<L = NoopPlatformChecker> {
            logic: L,
        }
        #[automatically_derived]
        impl<L: ::core::clone::Clone> ::core::clone::Clone for
            PlatformChecker<L> {
            #[inline]
            fn clone(&self) -> PlatformChecker<L> {
                PlatformChecker {
                    logic: ::core::clone::Clone::clone(&self.logic),
                }
            }
        }
        #[automatically_derived]
        impl<L: ::core::fmt::Debug> ::core::fmt::Debug for PlatformChecker<L>
            {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "PlatformChecker", "logic", &&self.logic)
            }
        }
        impl Default for PlatformChecker<NoopPlatformChecker> {
            fn default() -> Self { Self { logic: NoopPlatformChecker } }
        }
        impl<L> PlatformChecker<L> where L: PlatformCheckerLogic {
            pub fn new(logic: L) -> Self { Self { logic } }
        }
        impl<L> TypedChecker<crate::Platform> for PlatformChecker<L> where
            L: PlatformCheckerLogic {
            fn check_and_fix_typed(&self, ctx: &UserContext,
                entity: &mut crate::Platform, status: CheckObjectStatus,
                location: &ObjectLocation, results: &mut CheckResults) {
                self.logic.check_and_fix_platform(ctx, entity, status,
                    location, results);
            }
        }
    }
    mod entity {
        use std::collections::BTreeMap;
        use teaql_core::SmartList;
        use teaql_macros::TeaqlEntity;
        #[teaql(entity = "Platform", table = "platform_data")]
        pub struct Platform {
            #[teaql(id)]
            id: u64,
            name: String,
            founded: chrono::DateTime<chrono::Utc>,
            #[teaql(version)]
            version: i64,
            #[teaql(relation(target = "Task", local_key = "id", foreign_key =
            "platform_id", many))]
            task_list: SmartList<crate::Task>,
            #[teaql(dynamic)]
            dynamic: BTreeMap<String, teaql_core::Value>,
            #[teaql(skip)]
            root: teaql_runtime::EntityRoot,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Platform {
            #[inline]
            fn clone(&self) -> Platform {
                Platform {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    founded: ::core::clone::Clone::clone(&self.founded),
                    version: ::core::clone::Clone::clone(&self.version),
                    task_list: ::core::clone::Clone::clone(&self.task_list),
                    dynamic: ::core::clone::Clone::clone(&self.dynamic),
                    root: ::core::clone::Clone::clone(&self.root),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Platform {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["id", "name", "founded", "version", "task_list",
                                "dynamic", "root"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.id, &self.name, &self.founded, &self.version,
                                &self.task_list, &self.dynamic, &&self.root];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "Platform", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Platform { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Platform {
            #[inline]
            fn eq(&self, other: &Platform) -> bool {
                self.id == other.id && self.version == other.version &&
                                    self.name == other.name && self.founded == other.founded &&
                            self.task_list == other.task_list &&
                        self.dynamic == other.dynamic && self.root == other.root
            }
        }
        impl ::teaql_core::TeaqlEntity for Platform {
            fn entity_descriptor() -> ::teaql_core::EntityDescriptor {
                let mut descriptor =
                    ::teaql_core::EntityDescriptor::new("Platform").table_name("platform_data");
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("id",
                                        ::teaql_core::DataType::U64).column_name("id").not_null().id());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("name",
                                    ::teaql_core::DataType::Text).column_name("name").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("founded",
                                    ::teaql_core::DataType::Timestamp).column_name("founded").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("version",
                                        ::teaql_core::DataType::I64).column_name("version").not_null().version());
                descriptor =
                    descriptor.relation(::teaql_core::RelationDescriptor::new("task_list",
                                        "Task").local_key("id").foreign_key("platform_id").many());
                descriptor
            }
        }
        impl ::teaql_core::Entity for Platform {
            fn from_record(record: ::teaql_core::Record)
                -> Result<Self, ::teaql_core::EntityError> {
                Ok(Self {
                        id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("Platform",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("Platform",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Platform",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "id", other))
                                                                        }))),
                                                })
                                        })()?,
                        name: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("name") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Platform",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "name", other))
                                                                        }))),
                                                })
                                        })()?,
                        founded: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("founded") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Timestamp(v) => *v,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Platform",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "founded", other))
                                                                        }))),
                                                })
                                        })()?,
                        version: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("version") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::I64(v) => *v,
                                                    ::teaql_core::Value::U64(v) =>
                                                        i64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("Platform",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: u64 out of i64 range",
                                                                                        "version"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Platform",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "version", other))
                                                                        }))),
                                                })
                                        })()?,
                        task_list: match record.get("task_list") {
                            Some(::teaql_core::Value::List(values)) =>
                                ::teaql_core::SmartList::from(values.iter().map(|value|
                                                    match value {
                                                        ::teaql_core::Value::Object(record) => {
                                                            <crate::Task as
                                                                    ::teaql_core::Entity>::from_record(record.clone())
                                                        }
                                                        other =>
                                                            Err(::teaql_core::EntityError::new("Platform",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid relation list item {0}: {1:?}",
                                                                                    "task_list", other))
                                                                        }))),
                                                    }).collect::<Result<Vec<_>, _>>()?),
                            Some(::teaql_core::Value::Null) | None =>
                                ::teaql_core::SmartList::default(),
                            other => {
                                return Err(::teaql_core::EntityError::new("Platform",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("invalid relation field {0}: {1:?}",
                                                            "task_list", other))
                                                })))
                            }
                        },
                        dynamic: {
                            let known_fields =
                                ["id", "name", "founded", "version", "task_list"];
                            record.iter().filter(|(key, _)|
                                            !known_fields.contains(&key.as_str())).map(|(key, value)|
                                        (key.clone(), value.clone())).collect()
                        },
                        root: Default::default(),
                    })
            }
            fn into_record(self) -> ::teaql_core::Record {
                let mut record = ::teaql_core::Record::new();
                record.insert("id".to_owned(), (self.id).into());
                record.insert("name".to_owned(), (self.name).into());
                record.insert("founded".to_owned(), (self.founded).into());
                record.insert("version".to_owned(), (self.version).into());
                record.insert("task_list".to_owned(),
                    ::teaql_core::Value::List((self.task_list).data.into_iter().map(|entity|
                                    ::teaql_core::Value::object(entity.into_record())).collect()));
                for (key, value) in self.dynamic {
                    record.insert(key, value);
                }
                record
            }
            fn on_loaded(&mut self, _context: &dyn std::any::Any) {}
        }
        impl ::teaql_core::IdentifiableEntity for Platform {
            fn id_value(&self) -> ::teaql_core::Value {
                ::teaql_core::Value::U64((*&self.id).into())
            }
        }
        impl ::teaql_core::VersionedEntity for Platform {
            fn version(&self) -> i64 { self.version }
        }
        impl Platform {
            pub fn with_id(id: u64) -> teaql_core::Value {
                teaql_core::Value::U64(id)
            }
            pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot)
                -> Self {
                Self {
                    id: 0_u64,
                    name: String::new(),
                    founded: chrono::Utc::now(),
                    version: 0_i64,
                    task_list: Default::default(),
                    dynamic: BTreeMap::new(),
                    root,
                }
            }
            pub fn entity_key(&self) -> teaql_runtime::EntityKey {
                teaql_runtime::EntityKey::new("Platform", self.id)
            }
            pub fn attach_root_recursive(&mut self,
                root: teaql_runtime::EntityRoot) {
                self.root = root.clone();
                for entity in &mut self.task_list {
                    entity.attach_root_recursive(root.clone());
                }
            }
            pub fn id(&self) -> u64 {
                self.changed_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.id)
            }
            pub fn update_id(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.id = value.try_u64().unwrap_or(self.id.clone());
                self.root.set(self.entity_key(), "id", value);
                self
            }
            pub fn changed_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "id")
            }
            pub fn name(&self) -> String {
                self.changed_name().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.name.clone())
            }
            pub fn update_name(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.name =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.name.clone());
                self.root.set(self.entity_key(), "name", value);
                self
            }
            pub fn changed_name(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "name")
            }
            pub fn founded(&self) -> chrono::DateTime<chrono::Utc> {
                self.changed_founded().and_then(|value|
                            value.try_timestamp()).unwrap_or(self.founded)
            }
            pub fn update_founded(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.founded =
                    value.try_timestamp().unwrap_or(self.founded.clone());
                self.root.set(self.entity_key(), "founded", value);
                self
            }
            pub fn changed_founded(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "founded")
            }
            pub fn version(&self) -> i64 {
                self.changed_version().and_then(|value|
                            value.try_i64()).unwrap_or(self.version)
            }
            pub fn update_version(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.version =
                    value.try_i64().unwrap_or(self.version.clone());
                self.root.set(self.entity_key(), "version", value);
                self
            }
            pub fn changed_version(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "version")
            }
            pub fn task_list(&self) -> &SmartList<crate::Task> {
                &self.task_list
            }
            pub async fn save<'a, C>(self, ctx: &'a C)
                ->
                    Result<teaql_runtime::GraphNode,
                    crate::TeaqlRepositoryError<C::PlatformRepository<'a>>>
                where C: crate::TeaqlRuntime + ?Sized {
                let repository =
                    ctx.platform_repository().map_err(|err|
                                teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
                crate::TeaqlEntityRepository::save_entity_graph(&repository,
                    self)
            }
        }
    }
    mod expression {
        use teaql_core::{SafeExpression, SmartList};
        pub struct PlatformExpression<R> {
            expression: SafeExpression<R, crate::Platform>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            PlatformExpression<R> {
            #[inline]
            fn clone(&self) -> PlatformExpression<R> {
                PlatformExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> PlatformExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression: SafeExpression<R, crate::Platform>)
                -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<crate::Platform> {
                self.expression.eval()
            }
            pub fn get_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.id())
            }
            pub fn get_name(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.name())
            }
            pub fn get_founded(self)
                -> SafeExpression<R, chrono::DateTime<chrono::Utc>> {
                self.expression.apply(|value| value.founded())
            }
            pub fn get_version(self) -> SafeExpression<R, i64> {
                self.expression.apply(|value| value.version())
            }
            pub fn get_task_list(self) -> crate::TaskListExpression<R> {
                crate::TaskListExpression::new(self.expression.apply(|value|
                            value.task_list().clone()))
            }
        }
        pub struct PlatformListExpression<R> {
            expression: SafeExpression<R, SmartList<crate::Platform>>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            PlatformListExpression<R> {
            #[inline]
            fn clone(&self) -> PlatformListExpression<R> {
                PlatformListExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> PlatformListExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression:
                    SafeExpression<R, SmartList<crate::Platform>>) -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<SmartList<crate::Platform>> {
                self.expression.eval()
            }
            pub fn size(self) -> SafeExpression<R, usize> {
                self.expression.size()
            }
            pub fn first(self) -> PlatformExpression<R> {
                PlatformExpression::new(self.expression.first())
            }
            pub fn get(self, index: usize) -> PlatformExpression<R> {
                PlatformExpression::new(self.expression.get(index))
            }
        }
    }
    mod request {
        use std::marker::PhantomData;
        use serde_json::Value as JsonValue;
        use teaql_core::{
            Aggregate, AggregateFunction, EntityDescriptor, Expr, Record,
            SelectQuery, SmartList,
        };
        use teaql_runtime::{RepositoryError, RuntimeError};
        use crate::request_support::*;
        impl EntityReference for crate::Platform {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(&self)
            }
        }
        impl EntityReference for &crate::Platform {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(self)
            }
        }
        pub struct PlatformRequest<R = crate::Platform> {
            query: SelectQuery,
            relation_selections: Vec<RelationSelection>,
            relation_filters: Vec<RelationFilter>,
            child_enhancements: Vec<QuerySelection>,
            query_options: QueryOptions,
            marker: PhantomData<R>,
        }
        #[automatically_derived]
        impl<R: ::core::fmt::Debug> ::core::fmt::Debug for PlatformRequest<R>
            {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["query", "relation_selections", "relation_filters",
                                "child_enhancements", "query_options", "marker"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.query, &self.relation_selections,
                                &self.relation_filters, &self.child_enhancements,
                                &self.query_options, &&self.marker];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "PlatformRequest", names, values)
            }
        }
        impl<R> Clone for PlatformRequest<R> {
            fn clone(&self) -> Self {
                Self {
                    query: self.query.clone(),
                    relation_selections: self.relation_selections.clone(),
                    relation_filters: self.relation_filters.clone(),
                    child_enhancements: self.child_enhancements.clone(),
                    query_options: self.query_options.clone(),
                    marker: PhantomData,
                }
            }
        }
        impl<R> PlatformRequest<R> {
            pub(crate) fn new() -> Self {
                Self {
                    query: SelectQuery::new("Platform"),
                    relation_selections: Vec::new(),
                    relation_filters: Vec::new(),
                    child_enhancements: Vec::new(),
                    query_options: QueryOptions::default(),
                    marker: PhantomData,
                }
            }
            pub fn return_type<T>(self) -> PlatformRequest<T> {
                PlatformRequest {
                    query: self.query,
                    relation_selections: self.relation_selections,
                    relation_filters: self.relation_filters,
                    child_enhancements: self.child_enhancements,
                    query_options: self.query_options,
                    marker: PhantomData,
                }
            }
            pub fn query(&self) -> &SelectQuery { &self.query }
            pub fn relation_selections(&self) -> &[RelationSelection] {
                &self.relation_selections
            }
            pub fn relation_filters(&self) -> &[RelationFilter] {
                &self.relation_filters
            }
            pub fn child_enhancements(&self) -> &[QuerySelection] {
                &self.child_enhancements
            }
            pub fn query_options(&self) -> &QueryOptions {
                &self.query_options
            }
            pub fn into_query(self) -> SelectQuery { self.query }
            pub fn new_entity<C>(&self, ctx: &C) -> crate::Platform where
                C: TeaqlRuntime + ?Sized {
                crate::Platform::runtime_new(ctx.user_context().entity_root())
            }
            pub async fn execute_for_list<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let repository =
                    ctx.platform_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_enhanced_entities_with_relation_aggregates::<R>(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_first<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let rows = self.limit(1).execute_for_list(ctx).await?;
                Ok(rows.into_iter().next())
            }
            pub async fn execute_for_one<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.execute_for_first(ctx).await
            }
            pub async fn execute_by_id<'a,
                C>(self, ctx: &'a C, id: impl Into<teaql_core::Value>)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.and_filter(Expr::eq("id",
                                id)).execute_for_first(ctx).await
            }
            pub async fn execute_for_page<'a,
                C>(self, ctx: &'a C, offset: u64, limit: u64)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let total_count = self.clone().execute_for_count(ctx).await?;
                let mut rows =
                    self.page_offset(offset,
                                    limit).execute_for_list(ctx).await?;
                rows.total_count = Some(total_count);
                Ok(rows)
            }
            pub async fn execute_for_count<'a, C>(self, ctx: &'a C)
                ->
                    Result<u64, TeaqlRepositoryError<C::PlatformRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.platform_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query;
                query.projection.clear();
                query.expr_projection.clear();
                query.order_by.clear();
                query.slice = None;
                query.relations.clear();
                query = query.count(COUNT_ALIAS);
                let rows = repository.fetch_all(&query)?;
                rows.first().and_then(|row|
                                row.get(COUNT_ALIAS)).and_then(teaql_core::Value::try_u64).ok_or_else(||
                        RepositoryError::Runtime(RuntimeError::Graph(::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("count result for Platform is missing or not numeric"))
                                    }))))
            }
            pub async fn execute_for_exists<'a, C>(self, ctx: &'a C)
                ->
                    Result<bool,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.platform_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query.limit(1);
                query.relations.clear();
                let rows = repository.fetch_all(&query)?;
                Ok(!rows.is_empty())
            }
            pub async fn execute_for_records<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<Record>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.platform_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_smart_list_with_relation_aggregates(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_record<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<Record>,
                    TeaqlRepositoryError<C::PlatformRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let records = self.limit(1).execute_for_records(ctx).await?;
                Ok(records.into_iter().next())
            }
            pub fn filter(mut self, filter: Expr) -> Self {
                self.query = self.query.filter(filter);
                self
            }
            pub fn and_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.and_filter(filter);
                self
            }
            pub fn or_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.or_filter(filter);
                self
            }
            pub fn append_search_criteria(self, criteria: Expr) -> Self {
                self.and_filter(criteria)
            }
            pub fn filter_property(mut self, property1: impl AsRef<str>,
                operator: FieldOperator, property2: impl AsRef<str>) -> Self {
                self.query =
                    self.query.and_filter(field_operator_column_expr(property1.as_ref(),
                            operator, property2.as_ref()));
                self
            }
            pub fn with_deleted_rows(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self
            }
            pub fn deleted_rows_only(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self.query =
                    self.query.and_filter(Expr::lte("version", 0_i64));
                self
            }
            pub fn match_types(mut self,
                types: impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list(TYPE_FIELD,
                            types.into_iter().map(Into::into)));
                self
            }
            pub fn with_type_group(mut self) -> Self {
                self.query = self.query.project(TYPE_GROUP_FIELD);
                self
            }
            pub fn matching_any_of(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                let entity =
                    EntityDescriptor::new(selection.query.entity.clone());
                self.query =
                    self.query.and_filter(Expr::in_subquery("id", entity,
                            selection.query.clone(), "id"));
                self
            }
            pub fn match_any_of(self, request: impl Into<QuerySelection>)
                -> Self {
                self.matching_any_of(request)
            }
            pub fn enhance_child(mut self, request: impl Into<QuerySelection>)
                -> Self {
                self.child_enhancements.push(request.into());
                self
            }
            pub fn enhance_children_if_needed(self) -> Self {
                let request = self;
                request
            }
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.query_options.comment = Some(comment.into());
                self
            }
            pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment)
                -> Self {
                self.query_options.raw_sql = Some(raw_sql.into_sql());
                self
            }
            pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql_filter(mut self,
                raw_sql: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
                self
            }
            pub fn filter_with_json(self, json_expr: impl Into<String>)
                -> Self {
                self.merge_dynamic_json_expr(json_expr.into())
            }
            fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
                let json =
                    serde_json::from_str::<JsonValue>(&json_expr).unwrap_or_else(|_|
                            {
                                ::core::panicking::panic_fmt(format_args!("Input JSON format error: {0}",
                                        json_expr));
                            });
                self.merge_dynamic_json(&json)
            }
            fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
                let Some(object) = json.as_object() else { return self; };
                for (field, value) in object {
                    if field.starts_with('_') { continue; }
                    self = self.apply_dynamic_json_filter(field, value);
                }
                self =
                    self.apply_dynamic_json_order_by(object.get("_orderBy"));
                if let Some(offset) = dynamic_json_u64_field(object, "_start")
                    {
                    self = self.skip(offset);
                }
                if let Some(size) = dynamic_json_u64_field(object, "_size") {
                    self = self.limit(size);
                }
                if let Some(page_size) =
                        dynamic_json_u64_field(object, "_pageSize") {
                    self = self.limit(page_size);
                }
                if let Some(page_number) =
                        dynamic_json_u64_field(object, "_page") {
                    if page_number > 0 {
                        let size =
                            dynamic_json_u64_field(object,
                                        "_pageSize").or_else(||
                                        self.query.slice.as_ref().and_then(|slice|
                                                slice.limit)).unwrap_or(10);
                        let offset =
                            page_number.saturating_sub(1).saturating_mul(size);
                        self = self.page_offset(offset, size);
                    }
                }
                self
            }
            pub(crate) fn apply_dynamic_json_filter(self, field: &str,
                value: &JsonValue) -> Self {
                if let Some((head, tail)) = field.split_once('.') {
                    self.apply_dynamic_json_chain_filter(head, tail, value)
                } else if let Some(storage_field) =
                        Self::dynamic_json_self_field(field) {
                    self.and_filter(dynamic_json_filter_expr(storage_field,
                            value))
                } else { self }
            }
            fn apply_dynamic_json_order_by(mut self,
                order_by: Option<&JsonValue>) -> Self {
                match order_by {
                    Some(JsonValue::String(field)) => {
                        if let Some(storage_field) =
                                Self::dynamic_json_self_field(field) {
                            self.query = self.query.order_desc(storage_field);
                        }
                    }
                    Some(JsonValue::Object(order_by)) => {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                    Some(JsonValue::Array(order_bys)) => {
                        for order_by in order_bys {
                            if let Some(order_by) = order_by.as_object() {
                                self = self.apply_dynamic_json_single_order_by(order_by);
                            }
                        }
                    }
                    _ => {}
                }
                self
            }
            fn apply_dynamic_json_single_order_by(mut self,
                order_by: &serde_json::Map<String, JsonValue>) -> Self {
                let Some(field) =
                    order_by.get("field").and_then(JsonValue::as_str) else {
                        return self;
                    };
                let Some(storage_field) =
                    Self::dynamic_json_self_field(field) else { return self; };
                if order_by.get("useAsc").and_then(JsonValue::as_bool).unwrap_or(false)
                    {
                    self.query = self.query.order_asc(storage_field);
                } else { self.query = self.query.order_desc(storage_field); }
                self
            }
            fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
                match field {
                    "id" => Some("id"),
                    "name" => Some("name"),
                    "founded" => Some("founded"),
                    "version" => Some("version"),
                    _ => None,
                }
            }
            fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str,
                value: &JsonValue) -> Self {
                let _ = (tail, value);
                match head {
                    "task_list" => {
                        self.with_task_list_matching(crate::Q::tasks_minimal().apply_dynamic_json_filter(tail,
                                value))
                    }
                    _ => self,
                }
            }
            pub fn create_property_as(self, property_name: impl Into<String>,
                raw_sql_segment: impl Into<String>) -> Self {
                self.unsafe_create_property_as(property_name,
                    UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn unsafe_create_property_as(mut self,
                property_name: impl Into<String>,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.dynamic_properties.push(RawDynamicProperty::new(property_name,
                        raw_sql_segment));
                self
            }
            pub fn limit(mut self, limit: u64) -> Self {
                self.query = self.query.limit(limit);
                self
            }
            pub fn skip(mut self, offset: u64) -> Self {
                self.query = self.query.offset(offset);
                self
            }
            pub fn offset_only(self, offset: u64) -> Self {
                self.skip(offset)
            }
            pub fn offset(self, offset: u64, size: u64) -> Self {
                self.page_offset(offset, size)
            }
            pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
                self.query = self.query.page(offset, limit);
                self
            }
            pub fn top(self, top_n: u64) -> Self { self.limit(top_n) }
            pub fn offset_size(self, offset: u64, size: u64) -> Self {
                self.offset(offset, size)
            }
            pub fn unlimited(mut self) -> Self {
                self.query.slice = None;
                self
            }
            pub fn page_number(self, page_number: u64, page_size: u64)
                -> Self {
                let offset =
                    page_number.saturating_sub(1).saturating_mul(page_size);
                self.page_offset(offset, page_size)
            }
            pub fn page_number_default(self, page_number: u64) -> Self {
                self.page_number(page_number, 10)
            }
            pub fn page(self, page_number: u64, page_size: u64) -> Self {
                self.page_number(page_number, page_size)
            }
            pub fn page_default(self, page_number: u64) -> Self {
                self.page_number_default(page_number)
            }
            pub fn select_self(mut self) -> Self {
                self.query = self.query.project("id");
                self.query = self.query.project("name");
                self.query = self.query.project("founded");
                self.query = self.query.project("version");
                self
            }
            pub fn select_self_fields(self) -> Self { self.select_self() }
            pub fn select_self_without_parent(self) -> Self {
                self.select_self_fields()
            }
            pub fn select_all(self) -> Self { self.select_self() }
            pub fn select_children(self) -> Self {
                let mut request = self.select_all();
                request = request.select_task_list();
                request
            }
            pub fn select_any(self) -> Self { self.select_children() }
            pub fn group_by(mut self, field: impl Into<String>) -> Self {
                self.query = self.query.group_by(field);
                self
            }
            pub fn aggregate_count(mut self, alias: impl Into<String>)
                -> Self {
                self.query = self.query.count(alias);
                self
            }
            pub fn aggregate_count_field(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.count_field(field, alias);
                self
            }
            pub fn aggregate_with_function(mut self, field: impl Into<String>,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.query =
                    self.query.aggregate(Aggregate::new(function, field,
                            alias));
                self
            }
            pub fn aggregate_sum(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.sum(field, alias);
                self
            }
            pub fn aggregate_avg(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.avg(field, alias);
                self
            }
            pub fn aggregate_min(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.min(field, alias);
                self
            }
            pub fn aggregate_max(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.max(field, alias);
                self
            }
            pub fn aggregate_stddev(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev(field, alias);
                self
            }
            pub fn aggregate_stddev_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev_pop(field, alias);
                self
            }
            pub fn aggregate_var_samp(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_samp(field, alias);
                self
            }
            pub fn aggregate_var_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_pop(field, alias);
                self
            }
            pub fn aggregate_bit_and(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_and(field, alias);
                self
            }
            pub fn aggregate_bit_or(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_or(field, alias);
                self
            }
            pub fn aggregate_bit_xor(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_xor(field, alias);
                self
            }
            pub fn enable_aggregation_cache(mut self) -> Self {
                self.query = self.query.enable_aggregation_cache();
                self
            }
            pub fn enable_aggregation_cache_for(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.enable_aggregation_cache_for(cache_expired_millis);
                self
            }
            pub fn propagate_aggregation_cache(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.propagate_aggregation_cache(cache_expired_millis);
                self
            }
            pub fn select_id(mut self) -> Self {
                self.query = self.query.project("id");
                self
            }
            pub fn project_id(self) -> Self { self.select_id() }
            pub fn select_id_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_id_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("id",
                        raw_sql_segment));
                self
            }
            pub fn group_by_id(self) -> Self { self.group_by("id") }
            pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("id");
                request.query =
                    request.query.project_expr(alias, Expr::column("id"));
                request
            }
            pub fn group_by_id_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("id").aggregate_with_function("id", alias,
                    function)
            }
            pub fn count_id(self) -> Self { self.count_id_as("id_count") }
            pub fn count_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("id", alias)
            }
            pub fn sum_id(self) -> Self { self.sum_id_as("sum_id") }
            pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("id", alias)
            }
            pub fn avg_id(self) -> Self { self.avg_id_as("avg_id") }
            pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("id", alias)
            }
            pub fn min_id(self) -> Self { self.min_id_as("min_id") }
            pub fn min_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("id", alias)
            }
            pub fn max_id(self) -> Self { self.max_id_as("max_id") }
            pub fn max_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("id", alias)
            }
            pub fn unselect_id(mut self) -> Self {
                self.query.projection.retain(|field| field != "id");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "id");
                self
            }
            pub fn with_id(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("id", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_id_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("id", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("id", value));
                self
            }
            pub fn with_id_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("id", value));
                self
            }
            pub fn with_id_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_id_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn order_by_id_asc(mut self) -> Self {
                self.query = self.query.order_asc("id");
                self
            }
            pub fn order_by_id_desc(mut self) -> Self {
                self.query = self.query.order_desc("id");
                self
            }
            pub fn order_by_id_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("id");
                self
            }
            pub fn order_by_id_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("id");
                self
            }
            pub fn select_name(mut self) -> Self {
                self.query = self.query.project("name");
                self
            }
            pub fn project_name(self) -> Self { self.select_name() }
            pub fn select_name_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_name_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("name",
                        raw_sql_segment));
                self
            }
            pub fn group_by_name(self) -> Self { self.group_by("name") }
            pub fn group_by_name_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("name");
                request.query =
                    request.query.project_expr(alias, Expr::column("name"));
                request
            }
            pub fn group_by_name_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("name").aggregate_with_function("name", alias,
                    function)
            }
            pub fn count_name(self) -> Self {
                self.count_name_as("name_count")
            }
            pub fn count_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("name", alias)
            }
            pub fn sum_name(self) -> Self { self.sum_name_as("sum_name") }
            pub fn sum_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("name", alias)
            }
            pub fn avg_name(self) -> Self { self.avg_name_as("avg_name") }
            pub fn avg_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("name", alias)
            }
            pub fn min_name(self) -> Self { self.min_name_as("min_name") }
            pub fn min_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("name", alias)
            }
            pub fn max_name(self) -> Self { self.max_name_as("max_name") }
            pub fn max_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("name", alias)
            }
            pub fn unselect_name(mut self) -> Self {
                self.query.projection.retain(|field| field != "name");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "name");
                self
            }
            pub fn with_name(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("name", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_name_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("name", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_name_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("name", value));
                self
            }
            pub fn with_name_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("name", value));
                self
            }
            pub fn with_name_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("name", value));
                self
            }
            pub fn with_name_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gte("name", value));
                self
            }
            pub fn with_name_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("name", value));
                self
            }
            pub fn with_name_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lte("name", value));
                self
            }
            pub fn with_name_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("name", lower, upper));
                self
            }
            pub fn with_name_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("name", range.start,
                            range.end));
                self
            }
            pub fn with_name_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("name",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_name_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("name",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_name_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("name", value));
                self
            }
            pub fn with_name_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("name", value));
                self
            }
            pub fn with_name_starting_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("name", value));
                self
            }
            pub fn with_name_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("name", value));
                self
            }
            pub fn with_name_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("name", value));
                self
            }
            pub fn with_name_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("name", value));
                self
            }
            pub fn with_name_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("name", value));
                self
            }
            pub fn with_name_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("name", value));
                self
            }
            pub fn with_name_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("name", value));
                self
            }
            pub fn with_name_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("name"));
                self
            }
            pub fn with_name_is_known(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_not_null("name"));
                self
            }
            pub fn order_by_name_asc(mut self) -> Self {
                self.query = self.query.order_asc("name");
                self
            }
            pub fn order_by_name_desc(mut self) -> Self {
                self.query = self.query.order_desc("name");
                self
            }
            pub fn order_by_name_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("name");
                self
            }
            pub fn order_by_name_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("name");
                self
            }
            pub fn select_founded(mut self) -> Self {
                self.query = self.query.project("founded");
                self
            }
            pub fn project_founded(self) -> Self { self.select_founded() }
            pub fn select_founded_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_founded_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_founded_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("founded",
                        raw_sql_segment));
                self
            }
            pub fn group_by_founded(self) -> Self { self.group_by("founded") }
            pub fn group_by_founded_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("founded");
                request.query =
                    request.query.project_expr(alias, Expr::column("founded"));
                request
            }
            pub fn group_by_founded_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("founded").aggregate_with_function("founded",
                    alias, function)
            }
            pub fn count_founded(self) -> Self {
                self.count_founded_as("founded_count")
            }
            pub fn count_founded_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("founded", alias)
            }
            pub fn sum_founded(self) -> Self {
                self.sum_founded_as("sum_founded")
            }
            pub fn sum_founded_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("founded", alias)
            }
            pub fn avg_founded(self) -> Self {
                self.avg_founded_as("avg_founded")
            }
            pub fn avg_founded_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("founded", alias)
            }
            pub fn min_founded(self) -> Self {
                self.min_founded_as("min_founded")
            }
            pub fn min_founded_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("founded", alias)
            }
            pub fn max_founded(self) -> Self {
                self.max_founded_as("max_founded")
            }
            pub fn max_founded_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("founded", alias)
            }
            pub fn unselect_founded(mut self) -> Self {
                self.query.projection.retain(|field| field != "founded");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "founded");
                self
            }
            pub fn with_founded(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("founded",
                            operator, values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_founded_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("founded", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_founded_is(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::eq("founded", value));
                self
            }
            pub fn with_founded_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::ne("founded", value));
                self
            }
            pub fn with_founded_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gt("founded", value));
                self
            }
            pub fn with_founded_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gte("founded", value));
                self
            }
            pub fn with_founded_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lt("founded", value));
                self
            }
            pub fn with_founded_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lte("founded", value));
                self
            }
            pub fn with_founded_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("founded", lower,
                            upper));
                self
            }
            pub fn with_founded_between_range<T>(mut self,
                range: DateRange<T>) -> Self where
                T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("founded", range.start,
                            range.end));
                self
            }
            pub fn with_founded_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("founded",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_founded_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("founded",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_founded_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lt("founded", value));
                self
            }
            pub fn with_founded_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gt("founded", value));
                self
            }
            pub fn with_founded_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("founded"));
                self
            }
            pub fn with_founded_is_known(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("founded"));
                self
            }
            pub fn order_by_founded_asc(mut self) -> Self {
                self.query = self.query.order_asc("founded");
                self
            }
            pub fn order_by_founded_desc(mut self) -> Self {
                self.query = self.query.order_desc("founded");
                self
            }
            pub fn order_by_founded_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("founded");
                self
            }
            pub fn order_by_founded_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("founded");
                self
            }
            pub fn select_version(mut self) -> Self {
                self.query = self.query.project("version");
                self
            }
            pub fn project_version(self) -> Self { self.select_version() }
            pub fn select_version_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_version_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_version_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("version",
                        raw_sql_segment));
                self
            }
            pub fn group_by_version(self) -> Self { self.group_by("version") }
            pub fn group_by_version_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("version");
                request.query =
                    request.query.project_expr(alias, Expr::column("version"));
                request
            }
            pub fn group_by_version_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("version").aggregate_with_function("version",
                    alias, function)
            }
            pub fn count_version(self) -> Self {
                self.count_version_as("version_count")
            }
            pub fn count_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("version", alias)
            }
            pub fn sum_version(self) -> Self {
                self.sum_version_as("sum_version")
            }
            pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("version", alias)
            }
            pub fn avg_version(self) -> Self {
                self.avg_version_as("avg_version")
            }
            pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("version", alias)
            }
            pub fn min_version(self) -> Self {
                self.min_version_as("min_version")
            }
            pub fn min_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("version", alias)
            }
            pub fn max_version(self) -> Self {
                self.max_version_as("max_version")
            }
            pub fn max_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("version", alias)
            }
            pub fn unselect_version(mut self) -> Self {
                self.query.projection.retain(|field| field != "version");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "version");
                self
            }
            pub fn order_by_version_asc(mut self) -> Self {
                self.query = self.query.order_asc("version");
                self
            }
            pub fn order_by_version_desc(mut self) -> Self {
                self.query = self.query.order_desc("version");
                self
            }
            pub fn order_by_version_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("version");
                self
            }
            pub fn order_by_version_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("version");
                self
            }
            pub fn name_is_robot_system(self) -> Self {
                self.with_name_is("Robot System")
            }
            pub fn with_name_is_robot_system(self) -> Self {
                self.with_name_is("Robot System")
            }
            pub fn with_name_is_not_robot_system(self) -> Self {
                self.with_name_is_not("Robot System")
            }
            pub fn founded_is_create_time(self) -> Self {
                self.with_founded_is("createTime()")
            }
            pub fn with_founded_is_create_time(self) -> Self {
                self.with_founded_is("createTime()")
            }
            pub fn with_founded_is_not_create_time(self) -> Self {
                self.with_founded_is_not("createTime()")
            }
            pub fn have_tasks(self) -> Self {
                self.with_task_list_matching(SelectQuery::new("Task"))
            }
            pub fn have_no_tasks(self) -> Self {
                self.without_task_list_matching(SelectQuery::new("Task"))
            }
            pub fn with_task_list_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::in_subquery("id",
                            <crate::Task as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "platform_id"));
                self.relation_filters.push(RelationFilter::new("task_list",
                        selection));
                self
            }
            pub fn without_task_list_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::not_in_subquery("id",
                            <crate::Task as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "platform_id"));
                self.relation_filters.push(RelationFilter::new("task_list",
                        selection));
                self
            }
            pub fn select_task_list(mut self) -> Self {
                self.query = self.query.relation("task_list");
                self
            }
            pub fn select_task_list_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.relation_query("task_list",
                        selection.clone().into_query());
                self.relation_selections.push(RelationSelection::new("task_list",
                        selection));
                self
            }
            pub fn count_tasks(self) -> Self {
                self.count_tasks_as("count_tasks")
            }
            pub fn count_tasks_as(self, alias: impl Into<String>) -> Self {
                self.count_tasks_with(alias, crate::Q::tasks().unlimited())
            }
            pub fn count_tasks_with(mut self, alias: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query_options.relation_aggregates.push(RelationAggregate::new("task_list",
                        alias, selection, true));
                self
            }
            pub fn stats_from_tasks(self, request: impl Into<QuerySelection>)
                -> Self {
                self.stats_from_tasks_as("refinements", request)
            }
            pub fn stats_from_tasks_as(mut self, alias: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query_options.relation_aggregates.push(RelationAggregate::new("task_list",
                        alias, selection, false));
                self
            }
            pub fn group_by_tasks_with_details(self,
                request: impl Into<QuerySelection>) -> Self {
                self.stats_from_tasks(request)
            }
        }
        impl<R> Default for PlatformRequest<R> {
            fn default() -> Self { Self::new() }
        }
        impl<R> From<PlatformRequest<R>> for SelectQuery {
            fn from(request: PlatformRequest<R>) -> Self {
                QuerySelection::from(request).into_query()
            }
        }
        impl<R> From<PlatformRequest<R>> for QuerySelection {
            fn from(request: PlatformRequest<R>) -> Self {
                Self {
                    query: request.query,
                    relation_selections: request.relation_selections,
                    relation_filters: request.relation_filters,
                    child_enhancements: request.child_enhancements,
                    query_options: request.query_options,
                }
            }
        }
    }
    pub use behavior::*;
    pub use checker::*;
    pub use entity::Platform;
    pub use expression::*;
    pub use request::*;
}
pub mod task_status {
    mod behavior {
        use teaql_runtime::RepositoryBehavior;
        pub struct TaskStatusBehavior;
        #[automatically_derived]
        impl ::core::clone::Clone for TaskStatusBehavior {
            #[inline]
            fn clone(&self) -> TaskStatusBehavior { TaskStatusBehavior }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TaskStatusBehavior {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "TaskStatusBehavior")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TaskStatusBehavior {
            #[inline]
            fn default() -> TaskStatusBehavior { TaskStatusBehavior {} }
        }
        impl RepositoryBehavior for TaskStatusBehavior {}
    }
    mod checker {
        use teaql_runtime::{
            CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker,
            UserContext,
        };
        pub trait TaskStatusCheckerLogic: Send + Sync {
            fn check_and_fix_task_status(&self, _ctx: &UserContext,
                _entity: &mut crate::TaskStatus, _status: CheckObjectStatus,
                _location: &ObjectLocation, _results: &mut CheckResults) {}
            fn required(&self, value: bool, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if !value {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_option<V>(&self, value: Option<&V>, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.is_none() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_text(&self, value: &str, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.trim().is_empty() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn min_string_length(&self, value: &str, field: &str,
                min_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() < min_len {
                    results.push(teaql_runtime::CheckResult::min_str(location.clone().member(field),
                            min_len as u64, value.to_owned()));
                }
            }
            fn max_string_length(&self, value: &str, field: &str,
                max_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() > max_len {
                    results.push(teaql_runtime::CheckResult::max_str(location.clone().member(field),
                            max_len as u64, value.to_owned()));
                }
            }
        }
        pub struct NoopTaskStatusChecker;
        #[automatically_derived]
        impl ::core::clone::Clone for NoopTaskStatusChecker {
            #[inline]
            fn clone(&self) -> NoopTaskStatusChecker { NoopTaskStatusChecker }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NoopTaskStatusChecker {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "NoopTaskStatusChecker")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NoopTaskStatusChecker {
            #[inline]
            fn default() -> NoopTaskStatusChecker { NoopTaskStatusChecker {} }
        }
        impl TaskStatusCheckerLogic for NoopTaskStatusChecker {}
        pub struct TaskStatusChecker<L = NoopTaskStatusChecker> {
            logic: L,
        }
        #[automatically_derived]
        impl<L: ::core::clone::Clone> ::core::clone::Clone for
            TaskStatusChecker<L> {
            #[inline]
            fn clone(&self) -> TaskStatusChecker<L> {
                TaskStatusChecker {
                    logic: ::core::clone::Clone::clone(&self.logic),
                }
            }
        }
        #[automatically_derived]
        impl<L: ::core::fmt::Debug> ::core::fmt::Debug for
            TaskStatusChecker<L> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "TaskStatusChecker", "logic", &&self.logic)
            }
        }
        impl Default for TaskStatusChecker<NoopTaskStatusChecker> {
            fn default() -> Self { Self { logic: NoopTaskStatusChecker } }
        }
        impl<L> TaskStatusChecker<L> where L: TaskStatusCheckerLogic {
            pub fn new(logic: L) -> Self { Self { logic } }
        }
        impl<L> TypedChecker<crate::TaskStatus> for TaskStatusChecker<L> where
            L: TaskStatusCheckerLogic {
            fn check_and_fix_typed(&self, ctx: &UserContext,
                entity: &mut crate::TaskStatus, status: CheckObjectStatus,
                location: &ObjectLocation, results: &mut CheckResults) {
                self.logic.check_and_fix_task_status(ctx, entity, status,
                    location, results);
            }
        }
    }
    mod entity {
        use std::collections::BTreeMap;
        use teaql_core::SmartList;
        use teaql_macros::TeaqlEntity;
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
            #[teaql(relation(target = "Task", local_key = "id", foreign_key =
            "status_id", many))]
            task_list: SmartList<crate::Task>,
            #[teaql(dynamic)]
            dynamic: BTreeMap<String, teaql_core::Value>,
            #[teaql(skip)]
            root: teaql_runtime::EntityRoot,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TaskStatus {
            #[inline]
            fn clone(&self) -> TaskStatus {
                TaskStatus {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    code: ::core::clone::Clone::clone(&self.code),
                    color: ::core::clone::Clone::clone(&self.color),
                    display_order: ::core::clone::Clone::clone(&self.display_order),
                    progress: ::core::clone::Clone::clone(&self.progress),
                    version: ::core::clone::Clone::clone(&self.version),
                    task_list: ::core::clone::Clone::clone(&self.task_list),
                    dynamic: ::core::clone::Clone::clone(&self.dynamic),
                    root: ::core::clone::Clone::clone(&self.root),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TaskStatus {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["id", "name", "code", "color", "display_order",
                                "progress", "version", "task_list", "dynamic", "root"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.id, &self.name, &self.code, &self.color,
                                &self.display_order, &self.progress, &self.version,
                                &self.task_list, &self.dynamic, &&self.root];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "TaskStatus", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TaskStatus { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TaskStatus {
            #[inline]
            fn eq(&self, other: &TaskStatus) -> bool {
                self.id == other.id && self.version == other.version &&
                                                self.name == other.name && self.code == other.code &&
                                        self.color == other.color &&
                                    self.display_order == other.display_order &&
                                self.progress == other.progress &&
                            self.task_list == other.task_list &&
                        self.dynamic == other.dynamic && self.root == other.root
            }
        }
        impl ::teaql_core::TeaqlEntity for TaskStatus {
            fn entity_descriptor() -> ::teaql_core::EntityDescriptor {
                let mut descriptor =
                    ::teaql_core::EntityDescriptor::new("TaskStatus").table_name("task_status_data");
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("id",
                                        ::teaql_core::DataType::U64).column_name("id").not_null().id());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("name",
                                    ::teaql_core::DataType::Text).column_name("name").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("code",
                                    ::teaql_core::DataType::Text).column_name("code").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("color",
                                    ::teaql_core::DataType::Text).column_name("color").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("display_order",
                                    ::teaql_core::DataType::Decimal).column_name("display_order").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("progress",
                                    ::teaql_core::DataType::Decimal).column_name("progress").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("version",
                                        ::teaql_core::DataType::I64).column_name("version").not_null().version());
                descriptor =
                    descriptor.relation(::teaql_core::RelationDescriptor::new("task_list",
                                        "Task").local_key("id").foreign_key("status_id").many());
                descriptor
            }
        }
        impl ::teaql_core::Entity for TaskStatus {
            fn from_record(record: ::teaql_core::Record)
                -> Result<Self, ::teaql_core::EntityError> {
                Ok(Self {
                        id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("TaskStatus",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("TaskStatus",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "id", other))
                                                                        }))),
                                                })
                                        })()?,
                        name: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("name") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "name", other))
                                                                        }))),
                                                })
                                        })()?,
                        code: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("code") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "code", other))
                                                                        }))),
                                                })
                                        })()?,
                        color: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("color") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "color", other))
                                                                        }))),
                                                })
                                        })()?,
                        display_order: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("display_order") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Decimal(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        ::teaql_core::Decimal::from(*v),
                                                    ::teaql_core::Value::U64(v) =>
                                                        ::teaql_core::Decimal::from(*v),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "display_order", other))
                                                                        }))),
                                                })
                                        })()?,
                        progress: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("progress") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Decimal(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        ::teaql_core::Decimal::from(*v),
                                                    ::teaql_core::Value::U64(v) =>
                                                        ::teaql_core::Decimal::from(*v),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "progress", other))
                                                                        }))),
                                                })
                                        })()?,
                        version: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("version") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::I64(v) => *v,
                                                    ::teaql_core::Value::U64(v) =>
                                                        i64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("TaskStatus",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: u64 out of i64 range",
                                                                                        "version"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "version", other))
                                                                        }))),
                                                })
                                        })()?,
                        task_list: match record.get("task_list") {
                            Some(::teaql_core::Value::List(values)) =>
                                ::teaql_core::SmartList::from(values.iter().map(|value|
                                                    match value {
                                                        ::teaql_core::Value::Object(record) => {
                                                            <crate::Task as
                                                                    ::teaql_core::Entity>::from_record(record.clone())
                                                        }
                                                        other =>
                                                            Err(::teaql_core::EntityError::new("TaskStatus",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid relation list item {0}: {1:?}",
                                                                                    "task_list", other))
                                                                        }))),
                                                    }).collect::<Result<Vec<_>, _>>()?),
                            Some(::teaql_core::Value::Null) | None =>
                                ::teaql_core::SmartList::default(),
                            other => {
                                return Err(::teaql_core::EntityError::new("TaskStatus",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("invalid relation field {0}: {1:?}",
                                                            "task_list", other))
                                                })))
                            }
                        },
                        dynamic: {
                            let known_fields =
                                ["id", "name", "code", "color", "display_order", "progress",
                                        "version", "task_list"];
                            record.iter().filter(|(key, _)|
                                            !known_fields.contains(&key.as_str())).map(|(key, value)|
                                        (key.clone(), value.clone())).collect()
                        },
                        root: Default::default(),
                    })
            }
            fn into_record(self) -> ::teaql_core::Record {
                let mut record = ::teaql_core::Record::new();
                record.insert("id".to_owned(), (self.id).into());
                record.insert("name".to_owned(), (self.name).into());
                record.insert("code".to_owned(), (self.code).into());
                record.insert("color".to_owned(), (self.color).into());
                record.insert("display_order".to_owned(),
                    (self.display_order).into());
                record.insert("progress".to_owned(), (self.progress).into());
                record.insert("version".to_owned(), (self.version).into());
                record.insert("task_list".to_owned(),
                    ::teaql_core::Value::List((self.task_list).data.into_iter().map(|entity|
                                    ::teaql_core::Value::object(entity.into_record())).collect()));
                for (key, value) in self.dynamic {
                    record.insert(key, value);
                }
                record
            }
            fn on_loaded(&mut self, _context: &dyn std::any::Any) {}
        }
        impl ::teaql_core::IdentifiableEntity for TaskStatus {
            fn id_value(&self) -> ::teaql_core::Value {
                ::teaql_core::Value::U64((*&self.id).into())
            }
        }
        impl ::teaql_core::VersionedEntity for TaskStatus {
            fn version(&self) -> i64 { self.version }
        }
        impl TaskStatus {
            pub fn with_id(id: u64) -> teaql_core::Value {
                teaql_core::Value::U64(id)
            }
            pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot)
                -> Self {
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
            pub fn attach_root_recursive(&mut self,
                root: teaql_runtime::EntityRoot) {
                self.root = root.clone();
                for entity in &mut self.task_list {
                    entity.attach_root_recursive(root.clone());
                }
            }
            pub fn id(&self) -> u64 {
                self.changed_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.id)
            }
            pub fn update_id(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.id = value.try_u64().unwrap_or(self.id.clone());
                self.root.set(self.entity_key(), "id", value);
                self
            }
            pub fn changed_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "id")
            }
            pub fn name(&self) -> String {
                self.changed_name().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.name.clone())
            }
            pub fn update_name(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.name =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.name.clone());
                self.root.set(self.entity_key(), "name", value);
                self
            }
            pub fn changed_name(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "name")
            }
            pub fn code(&self) -> String {
                self.changed_code().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.code.clone())
            }
            pub fn update_code(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.code =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.code.clone());
                self.root.set(self.entity_key(), "code", value);
                self
            }
            pub fn changed_code(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "code")
            }
            pub fn color(&self) -> String {
                self.changed_color().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.color.clone())
            }
            pub fn update_color(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.color =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.color.clone());
                self.root.set(self.entity_key(), "color", value);
                self
            }
            pub fn changed_color(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "color")
            }
            pub fn display_order(&self) -> rust_decimal::Decimal {
                self.changed_display_order().and_then(|value|
                            value.try_decimal()).unwrap_or(self.display_order)
            }
            pub fn update_display_order(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.display_order =
                    value.try_decimal().unwrap_or(self.display_order.clone());
                self.root.set(self.entity_key(), "display_order", value);
                self
            }
            pub fn changed_display_order(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "display_order")
            }
            pub fn progress(&self) -> rust_decimal::Decimal {
                self.changed_progress().and_then(|value|
                            value.try_decimal()).unwrap_or(self.progress)
            }
            pub fn update_progress(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.progress =
                    value.try_decimal().unwrap_or(self.progress.clone());
                self.root.set(self.entity_key(), "progress", value);
                self
            }
            pub fn changed_progress(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "progress")
            }
            pub fn version(&self) -> i64 {
                self.changed_version().and_then(|value|
                            value.try_i64()).unwrap_or(self.version)
            }
            pub fn update_version(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.version =
                    value.try_i64().unwrap_or(self.version.clone());
                self.root.set(self.entity_key(), "version", value);
                self
            }
            pub fn changed_version(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "version")
            }
            pub fn task_list(&self) -> &SmartList<crate::Task> {
                &self.task_list
            }
            pub async fn save<'a, C>(self, ctx: &'a C)
                ->
                    Result<teaql_runtime::GraphNode,
                    crate::TeaqlRepositoryError<C::TaskStatusRepository<'a>>>
                where C: crate::TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_status_repository().map_err(|err|
                                teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
                crate::TeaqlEntityRepository::save_entity_graph(&repository,
                    self)
            }
        }
    }
    mod expression {
        use teaql_core::{SafeExpression, SmartList};
        pub struct TaskStatusExpression<R> {
            expression: SafeExpression<R, crate::TaskStatus>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            TaskStatusExpression<R> {
            #[inline]
            fn clone(&self) -> TaskStatusExpression<R> {
                TaskStatusExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> TaskStatusExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression: SafeExpression<R, crate::TaskStatus>)
                -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<crate::TaskStatus> {
                self.expression.eval()
            }
            pub fn get_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.id())
            }
            pub fn get_name(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.name())
            }
            pub fn get_code(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.code())
            }
            pub fn get_color(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.color())
            }
            pub fn get_display_order(self)
                -> SafeExpression<R, rust_decimal::Decimal> {
                self.expression.apply(|value| value.display_order())
            }
            pub fn get_progress(self)
                -> SafeExpression<R, rust_decimal::Decimal> {
                self.expression.apply(|value| value.progress())
            }
            pub fn get_version(self) -> SafeExpression<R, i64> {
                self.expression.apply(|value| value.version())
            }
            pub fn get_task_list(self) -> crate::TaskListExpression<R> {
                crate::TaskListExpression::new(self.expression.apply(|value|
                            value.task_list().clone()))
            }
        }
        pub struct TaskStatusListExpression<R> {
            expression: SafeExpression<R, SmartList<crate::TaskStatus>>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            TaskStatusListExpression<R> {
            #[inline]
            fn clone(&self) -> TaskStatusListExpression<R> {
                TaskStatusListExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> TaskStatusListExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression:
                    SafeExpression<R, SmartList<crate::TaskStatus>>) -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<SmartList<crate::TaskStatus>> {
                self.expression.eval()
            }
            pub fn size(self) -> SafeExpression<R, usize> {
                self.expression.size()
            }
            pub fn first(self) -> TaskStatusExpression<R> {
                TaskStatusExpression::new(self.expression.first())
            }
            pub fn get(self, index: usize) -> TaskStatusExpression<R> {
                TaskStatusExpression::new(self.expression.get(index))
            }
        }
    }
    mod request {
        use std::marker::PhantomData;
        use serde_json::Value as JsonValue;
        use teaql_core::{
            Aggregate, AggregateFunction, EntityDescriptor, Expr, Record,
            SelectQuery, SmartList,
        };
        use teaql_runtime::{RepositoryError, RuntimeError};
        use crate::request_support::*;
        impl EntityReference for crate::TaskStatus {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(&self)
            }
        }
        impl EntityReference for &crate::TaskStatus {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(self)
            }
        }
        pub struct TaskStatusRequest<R = crate::TaskStatus> {
            query: SelectQuery,
            relation_selections: Vec<RelationSelection>,
            relation_filters: Vec<RelationFilter>,
            child_enhancements: Vec<QuerySelection>,
            query_options: QueryOptions,
            marker: PhantomData<R>,
        }
        #[automatically_derived]
        impl<R: ::core::fmt::Debug> ::core::fmt::Debug for
            TaskStatusRequest<R> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["query", "relation_selections", "relation_filters",
                                "child_enhancements", "query_options", "marker"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.query, &self.relation_selections,
                                &self.relation_filters, &self.child_enhancements,
                                &self.query_options, &&self.marker];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "TaskStatusRequest", names, values)
            }
        }
        impl<R> Clone for TaskStatusRequest<R> {
            fn clone(&self) -> Self {
                Self {
                    query: self.query.clone(),
                    relation_selections: self.relation_selections.clone(),
                    relation_filters: self.relation_filters.clone(),
                    child_enhancements: self.child_enhancements.clone(),
                    query_options: self.query_options.clone(),
                    marker: PhantomData,
                }
            }
        }
        impl<R> TaskStatusRequest<R> {
            pub(crate) fn new() -> Self {
                Self {
                    query: SelectQuery::new("TaskStatus"),
                    relation_selections: Vec::new(),
                    relation_filters: Vec::new(),
                    child_enhancements: Vec::new(),
                    query_options: QueryOptions::default(),
                    marker: PhantomData,
                }
            }
            pub fn return_type<T>(self) -> TaskStatusRequest<T> {
                TaskStatusRequest {
                    query: self.query,
                    relation_selections: self.relation_selections,
                    relation_filters: self.relation_filters,
                    child_enhancements: self.child_enhancements,
                    query_options: self.query_options,
                    marker: PhantomData,
                }
            }
            pub fn query(&self) -> &SelectQuery { &self.query }
            pub fn relation_selections(&self) -> &[RelationSelection] {
                &self.relation_selections
            }
            pub fn relation_filters(&self) -> &[RelationFilter] {
                &self.relation_filters
            }
            pub fn child_enhancements(&self) -> &[QuerySelection] {
                &self.child_enhancements
            }
            pub fn query_options(&self) -> &QueryOptions {
                &self.query_options
            }
            pub fn into_query(self) -> SelectQuery { self.query }
            pub fn new_entity<C>(&self, ctx: &C) -> crate::TaskStatus where
                C: TeaqlRuntime + ?Sized {
                crate::TaskStatus::runtime_new(ctx.user_context().entity_root())
            }
            pub async fn execute_for_list<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let repository =
                    ctx.task_status_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_enhanced_entities_with_relation_aggregates::<R>(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_first<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let rows = self.limit(1).execute_for_list(ctx).await?;
                Ok(rows.into_iter().next())
            }
            pub async fn execute_for_one<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.execute_for_first(ctx).await
            }
            pub async fn execute_by_id<'a,
                C>(self, ctx: &'a C, id: impl Into<teaql_core::Value>)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.and_filter(Expr::eq("id",
                                id)).execute_for_first(ctx).await
            }
            pub async fn execute_for_page<'a,
                C>(self, ctx: &'a C, offset: u64, limit: u64)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let total_count = self.clone().execute_for_count(ctx).await?;
                let mut rows =
                    self.page_offset(offset,
                                    limit).execute_for_list(ctx).await?;
                rows.total_count = Some(total_count);
                Ok(rows)
            }
            pub async fn execute_for_count<'a, C>(self, ctx: &'a C)
                ->
                    Result<u64,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_status_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query;
                query.projection.clear();
                query.expr_projection.clear();
                query.order_by.clear();
                query.slice = None;
                query.relations.clear();
                query = query.count(COUNT_ALIAS);
                let rows = repository.fetch_all(&query)?;
                rows.first().and_then(|row|
                                row.get(COUNT_ALIAS)).and_then(teaql_core::Value::try_u64).ok_or_else(||
                        RepositoryError::Runtime(RuntimeError::Graph(::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("count result for TaskStatus is missing or not numeric"))
                                    }))))
            }
            pub async fn execute_for_exists<'a, C>(self, ctx: &'a C)
                ->
                    Result<bool,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_status_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query.limit(1);
                query.relations.clear();
                let rows = repository.fetch_all(&query)?;
                Ok(!rows.is_empty())
            }
            pub async fn execute_for_records<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<Record>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_status_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_smart_list_with_relation_aggregates(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_record<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<Record>,
                    TeaqlRepositoryError<C::TaskStatusRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let records = self.limit(1).execute_for_records(ctx).await?;
                Ok(records.into_iter().next())
            }
            pub fn filter(mut self, filter: Expr) -> Self {
                self.query = self.query.filter(filter);
                self
            }
            pub fn and_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.and_filter(filter);
                self
            }
            pub fn or_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.or_filter(filter);
                self
            }
            pub fn append_search_criteria(self, criteria: Expr) -> Self {
                self.and_filter(criteria)
            }
            pub fn filter_property(mut self, property1: impl AsRef<str>,
                operator: FieldOperator, property2: impl AsRef<str>) -> Self {
                self.query =
                    self.query.and_filter(field_operator_column_expr(property1.as_ref(),
                            operator, property2.as_ref()));
                self
            }
            pub fn with_deleted_rows(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self
            }
            pub fn deleted_rows_only(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self.query =
                    self.query.and_filter(Expr::lte("version", 0_i64));
                self
            }
            pub fn match_types(mut self,
                types: impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list(TYPE_FIELD,
                            types.into_iter().map(Into::into)));
                self
            }
            pub fn with_type_group(mut self) -> Self {
                self.query = self.query.project(TYPE_GROUP_FIELD);
                self
            }
            pub fn matching_any_of(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                let entity =
                    EntityDescriptor::new(selection.query.entity.clone());
                self.query =
                    self.query.and_filter(Expr::in_subquery("id", entity,
                            selection.query.clone(), "id"));
                self
            }
            pub fn match_any_of(self, request: impl Into<QuerySelection>)
                -> Self {
                self.matching_any_of(request)
            }
            pub fn enhance_child(mut self, request: impl Into<QuerySelection>)
                -> Self {
                self.child_enhancements.push(request.into());
                self
            }
            pub fn enhance_children_if_needed(self) -> Self {
                let request = self;
                request
            }
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.query_options.comment = Some(comment.into());
                self
            }
            pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment)
                -> Self {
                self.query_options.raw_sql = Some(raw_sql.into_sql());
                self
            }
            pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql_filter(mut self,
                raw_sql: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
                self
            }
            pub fn filter_with_json(self, json_expr: impl Into<String>)
                -> Self {
                self.merge_dynamic_json_expr(json_expr.into())
            }
            fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
                let json =
                    serde_json::from_str::<JsonValue>(&json_expr).unwrap_or_else(|_|
                            {
                                ::core::panicking::panic_fmt(format_args!("Input JSON format error: {0}",
                                        json_expr));
                            });
                self.merge_dynamic_json(&json)
            }
            fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
                let Some(object) = json.as_object() else { return self; };
                for (field, value) in object {
                    if field.starts_with('_') { continue; }
                    self = self.apply_dynamic_json_filter(field, value);
                }
                self =
                    self.apply_dynamic_json_order_by(object.get("_orderBy"));
                if let Some(offset) = dynamic_json_u64_field(object, "_start")
                    {
                    self = self.skip(offset);
                }
                if let Some(size) = dynamic_json_u64_field(object, "_size") {
                    self = self.limit(size);
                }
                if let Some(page_size) =
                        dynamic_json_u64_field(object, "_pageSize") {
                    self = self.limit(page_size);
                }
                if let Some(page_number) =
                        dynamic_json_u64_field(object, "_page") {
                    if page_number > 0 {
                        let size =
                            dynamic_json_u64_field(object,
                                        "_pageSize").or_else(||
                                        self.query.slice.as_ref().and_then(|slice|
                                                slice.limit)).unwrap_or(10);
                        let offset =
                            page_number.saturating_sub(1).saturating_mul(size);
                        self = self.page_offset(offset, size);
                    }
                }
                self
            }
            pub(crate) fn apply_dynamic_json_filter(self, field: &str,
                value: &JsonValue) -> Self {
                if let Some((head, tail)) = field.split_once('.') {
                    self.apply_dynamic_json_chain_filter(head, tail, value)
                } else if let Some(storage_field) =
                        Self::dynamic_json_self_field(field) {
                    self.and_filter(dynamic_json_filter_expr(storage_field,
                            value))
                } else { self }
            }
            fn apply_dynamic_json_order_by(mut self,
                order_by: Option<&JsonValue>) -> Self {
                match order_by {
                    Some(JsonValue::String(field)) => {
                        if let Some(storage_field) =
                                Self::dynamic_json_self_field(field) {
                            self.query = self.query.order_desc(storage_field);
                        }
                    }
                    Some(JsonValue::Object(order_by)) => {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                    Some(JsonValue::Array(order_bys)) => {
                        for order_by in order_bys {
                            if let Some(order_by) = order_by.as_object() {
                                self = self.apply_dynamic_json_single_order_by(order_by);
                            }
                        }
                    }
                    _ => {}
                }
                self
            }
            fn apply_dynamic_json_single_order_by(mut self,
                order_by: &serde_json::Map<String, JsonValue>) -> Self {
                let Some(field) =
                    order_by.get("field").and_then(JsonValue::as_str) else {
                        return self;
                    };
                let Some(storage_field) =
                    Self::dynamic_json_self_field(field) else { return self; };
                if order_by.get("useAsc").and_then(JsonValue::as_bool).unwrap_or(false)
                    {
                    self.query = self.query.order_asc(storage_field);
                } else { self.query = self.query.order_desc(storage_field); }
                self
            }
            fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
                match field {
                    "id" => Some("id"),
                    "name" => Some("name"),
                    "code" => Some("code"),
                    "color" => Some("color"),
                    "display_order" => Some("display_order"),
                    "progress" => Some("progress"),
                    "version" => Some("version"),
                    _ => None,
                }
            }
            fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str,
                value: &JsonValue) -> Self {
                let _ = (tail, value);
                match head {
                    "task_list" => {
                        self.with_task_list_matching(crate::Q::tasks_minimal().apply_dynamic_json_filter(tail,
                                value))
                    }
                    _ => self,
                }
            }
            pub fn create_property_as(self, property_name: impl Into<String>,
                raw_sql_segment: impl Into<String>) -> Self {
                self.unsafe_create_property_as(property_name,
                    UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn unsafe_create_property_as(mut self,
                property_name: impl Into<String>,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.dynamic_properties.push(RawDynamicProperty::new(property_name,
                        raw_sql_segment));
                self
            }
            pub fn limit(mut self, limit: u64) -> Self {
                self.query = self.query.limit(limit);
                self
            }
            pub fn skip(mut self, offset: u64) -> Self {
                self.query = self.query.offset(offset);
                self
            }
            pub fn offset_only(self, offset: u64) -> Self {
                self.skip(offset)
            }
            pub fn offset(self, offset: u64, size: u64) -> Self {
                self.page_offset(offset, size)
            }
            pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
                self.query = self.query.page(offset, limit);
                self
            }
            pub fn top(self, top_n: u64) -> Self { self.limit(top_n) }
            pub fn offset_size(self, offset: u64, size: u64) -> Self {
                self.offset(offset, size)
            }
            pub fn unlimited(mut self) -> Self {
                self.query.slice = None;
                self
            }
            pub fn page_number(self, page_number: u64, page_size: u64)
                -> Self {
                let offset =
                    page_number.saturating_sub(1).saturating_mul(page_size);
                self.page_offset(offset, page_size)
            }
            pub fn page_number_default(self, page_number: u64) -> Self {
                self.page_number(page_number, 10)
            }
            pub fn page(self, page_number: u64, page_size: u64) -> Self {
                self.page_number(page_number, page_size)
            }
            pub fn page_default(self, page_number: u64) -> Self {
                self.page_number_default(page_number)
            }
            pub fn select_self(mut self) -> Self {
                self.query = self.query.project("id");
                self.query = self.query.project("name");
                self.query = self.query.project("code");
                self.query = self.query.project("color");
                self.query = self.query.project("display_order");
                self.query = self.query.project("progress");
                self.query = self.query.project("version");
                self
            }
            pub fn select_self_fields(self) -> Self { self.select_self() }
            pub fn select_self_without_parent(self) -> Self {
                self.select_self_fields()
            }
            pub fn select_all(self) -> Self { self.select_self() }
            pub fn select_children(self) -> Self {
                let mut request = self.select_all();
                request = request.select_task_list();
                request
            }
            pub fn select_any(self) -> Self { self.select_children() }
            pub fn group_by(mut self, field: impl Into<String>) -> Self {
                self.query = self.query.group_by(field);
                self
            }
            pub fn aggregate_count(mut self, alias: impl Into<String>)
                -> Self {
                self.query = self.query.count(alias);
                self
            }
            pub fn aggregate_count_field(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.count_field(field, alias);
                self
            }
            pub fn aggregate_with_function(mut self, field: impl Into<String>,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.query =
                    self.query.aggregate(Aggregate::new(function, field,
                            alias));
                self
            }
            pub fn aggregate_sum(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.sum(field, alias);
                self
            }
            pub fn aggregate_avg(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.avg(field, alias);
                self
            }
            pub fn aggregate_min(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.min(field, alias);
                self
            }
            pub fn aggregate_max(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.max(field, alias);
                self
            }
            pub fn aggregate_stddev(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev(field, alias);
                self
            }
            pub fn aggregate_stddev_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev_pop(field, alias);
                self
            }
            pub fn aggregate_var_samp(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_samp(field, alias);
                self
            }
            pub fn aggregate_var_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_pop(field, alias);
                self
            }
            pub fn aggregate_bit_and(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_and(field, alias);
                self
            }
            pub fn aggregate_bit_or(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_or(field, alias);
                self
            }
            pub fn aggregate_bit_xor(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_xor(field, alias);
                self
            }
            pub fn enable_aggregation_cache(mut self) -> Self {
                self.query = self.query.enable_aggregation_cache();
                self
            }
            pub fn enable_aggregation_cache_for(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.enable_aggregation_cache_for(cache_expired_millis);
                self
            }
            pub fn propagate_aggregation_cache(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.propagate_aggregation_cache(cache_expired_millis);
                self
            }
            pub fn select_id(mut self) -> Self {
                self.query = self.query.project("id");
                self
            }
            pub fn project_id(self) -> Self { self.select_id() }
            pub fn select_id_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_id_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("id",
                        raw_sql_segment));
                self
            }
            pub fn group_by_id(self) -> Self { self.group_by("id") }
            pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("id");
                request.query =
                    request.query.project_expr(alias, Expr::column("id"));
                request
            }
            pub fn group_by_id_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("id").aggregate_with_function("id", alias,
                    function)
            }
            pub fn count_id(self) -> Self { self.count_id_as("id_count") }
            pub fn count_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("id", alias)
            }
            pub fn sum_id(self) -> Self { self.sum_id_as("sum_id") }
            pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("id", alias)
            }
            pub fn avg_id(self) -> Self { self.avg_id_as("avg_id") }
            pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("id", alias)
            }
            pub fn min_id(self) -> Self { self.min_id_as("min_id") }
            pub fn min_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("id", alias)
            }
            pub fn max_id(self) -> Self { self.max_id_as("max_id") }
            pub fn max_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("id", alias)
            }
            pub fn unselect_id(mut self) -> Self {
                self.query.projection.retain(|field| field != "id");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "id");
                self
            }
            pub fn with_id(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("id", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_id_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("id", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("id", value));
                self
            }
            pub fn with_id_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("id", value));
                self
            }
            pub fn with_id_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_id_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn order_by_id_asc(mut self) -> Self {
                self.query = self.query.order_asc("id");
                self
            }
            pub fn order_by_id_desc(mut self) -> Self {
                self.query = self.query.order_desc("id");
                self
            }
            pub fn order_by_id_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("id");
                self
            }
            pub fn order_by_id_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("id");
                self
            }
            pub fn select_name(mut self) -> Self {
                self.query = self.query.project("name");
                self
            }
            pub fn project_name(self) -> Self { self.select_name() }
            pub fn select_name_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_name_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("name",
                        raw_sql_segment));
                self
            }
            pub fn group_by_name(self) -> Self { self.group_by("name") }
            pub fn group_by_name_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("name");
                request.query =
                    request.query.project_expr(alias, Expr::column("name"));
                request
            }
            pub fn group_by_name_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("name").aggregate_with_function("name", alias,
                    function)
            }
            pub fn count_name(self) -> Self {
                self.count_name_as("name_count")
            }
            pub fn count_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("name", alias)
            }
            pub fn sum_name(self) -> Self { self.sum_name_as("sum_name") }
            pub fn sum_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("name", alias)
            }
            pub fn avg_name(self) -> Self { self.avg_name_as("avg_name") }
            pub fn avg_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("name", alias)
            }
            pub fn min_name(self) -> Self { self.min_name_as("min_name") }
            pub fn min_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("name", alias)
            }
            pub fn max_name(self) -> Self { self.max_name_as("max_name") }
            pub fn max_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("name", alias)
            }
            pub fn unselect_name(mut self) -> Self {
                self.query.projection.retain(|field| field != "name");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "name");
                self
            }
            pub fn with_name(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("name", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_name_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("name", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_name_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("name", value));
                self
            }
            pub fn with_name_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("name", value));
                self
            }
            pub fn with_name_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("name", value));
                self
            }
            pub fn with_name_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gte("name", value));
                self
            }
            pub fn with_name_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("name", value));
                self
            }
            pub fn with_name_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lte("name", value));
                self
            }
            pub fn with_name_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("name", lower, upper));
                self
            }
            pub fn with_name_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("name", range.start,
                            range.end));
                self
            }
            pub fn with_name_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("name",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_name_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("name",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_name_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("name", value));
                self
            }
            pub fn with_name_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("name", value));
                self
            }
            pub fn with_name_starting_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("name", value));
                self
            }
            pub fn with_name_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("name", value));
                self
            }
            pub fn with_name_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("name", value));
                self
            }
            pub fn with_name_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("name", value));
                self
            }
            pub fn with_name_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("name", value));
                self
            }
            pub fn with_name_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("name", value));
                self
            }
            pub fn with_name_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("name", value));
                self
            }
            pub fn with_name_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("name"));
                self
            }
            pub fn with_name_is_known(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_not_null("name"));
                self
            }
            pub fn order_by_name_asc(mut self) -> Self {
                self.query = self.query.order_asc("name");
                self
            }
            pub fn order_by_name_desc(mut self) -> Self {
                self.query = self.query.order_desc("name");
                self
            }
            pub fn order_by_name_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("name");
                self
            }
            pub fn order_by_name_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("name");
                self
            }
            pub fn select_code(mut self) -> Self {
                self.query = self.query.project("code");
                self
            }
            pub fn project_code(self) -> Self { self.select_code() }
            pub fn select_code_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_code_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_code_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("code",
                        raw_sql_segment));
                self
            }
            pub fn group_by_code(self) -> Self { self.group_by("code") }
            pub fn group_by_code_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("code");
                request.query =
                    request.query.project_expr(alias, Expr::column("code"));
                request
            }
            pub fn group_by_code_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("code").aggregate_with_function("code", alias,
                    function)
            }
            pub fn count_code(self) -> Self {
                self.count_code_as("code_count")
            }
            pub fn count_code_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("code", alias)
            }
            pub fn sum_code(self) -> Self { self.sum_code_as("sum_code") }
            pub fn sum_code_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("code", alias)
            }
            pub fn avg_code(self) -> Self { self.avg_code_as("avg_code") }
            pub fn avg_code_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("code", alias)
            }
            pub fn min_code(self) -> Self { self.min_code_as("min_code") }
            pub fn min_code_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("code", alias)
            }
            pub fn max_code(self) -> Self { self.max_code_as("max_code") }
            pub fn max_code_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("code", alias)
            }
            pub fn unselect_code(mut self) -> Self {
                self.query.projection.retain(|field| field != "code");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "code");
                self
            }
            pub fn with_code(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("code", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_code_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("code", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_code_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("code", value));
                self
            }
            pub fn with_code_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("code", value));
                self
            }
            pub fn with_code_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("code", value));
                self
            }
            pub fn with_code_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gte("code", value));
                self
            }
            pub fn with_code_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("code", value));
                self
            }
            pub fn with_code_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lte("code", value));
                self
            }
            pub fn with_code_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("code", lower, upper));
                self
            }
            pub fn with_code_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("code", range.start,
                            range.end));
                self
            }
            pub fn with_code_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("code",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_code_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("code",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_code_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("code", value));
                self
            }
            pub fn with_code_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("code", value));
                self
            }
            pub fn with_code_starting_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("code", value));
                self
            }
            pub fn with_code_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("code", value));
                self
            }
            pub fn with_code_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("code", value));
                self
            }
            pub fn with_code_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("code", value));
                self
            }
            pub fn with_code_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("code", value));
                self
            }
            pub fn with_code_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("code", value));
                self
            }
            pub fn with_code_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("code", value));
                self
            }
            pub fn with_code_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("code"));
                self
            }
            pub fn with_code_is_known(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_not_null("code"));
                self
            }
            pub fn order_by_code_asc(mut self) -> Self {
                self.query = self.query.order_asc("code");
                self
            }
            pub fn order_by_code_desc(mut self) -> Self {
                self.query = self.query.order_desc("code");
                self
            }
            pub fn order_by_code_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("code");
                self
            }
            pub fn order_by_code_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("code");
                self
            }
            pub fn select_color(mut self) -> Self {
                self.query = self.query.project("color");
                self
            }
            pub fn project_color(self) -> Self { self.select_color() }
            pub fn select_color_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_color_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_color_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("color",
                        raw_sql_segment));
                self
            }
            pub fn group_by_color(self) -> Self { self.group_by("color") }
            pub fn group_by_color_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("color");
                request.query =
                    request.query.project_expr(alias, Expr::column("color"));
                request
            }
            pub fn group_by_color_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("color").aggregate_with_function("color", alias,
                    function)
            }
            pub fn count_color(self) -> Self {
                self.count_color_as("color_count")
            }
            pub fn count_color_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("color", alias)
            }
            pub fn sum_color(self) -> Self { self.sum_color_as("sum_color") }
            pub fn sum_color_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("color", alias)
            }
            pub fn avg_color(self) -> Self { self.avg_color_as("avg_color") }
            pub fn avg_color_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("color", alias)
            }
            pub fn min_color(self) -> Self { self.min_color_as("min_color") }
            pub fn min_color_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("color", alias)
            }
            pub fn max_color(self) -> Self { self.max_color_as("max_color") }
            pub fn max_color_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("color", alias)
            }
            pub fn unselect_color(mut self) -> Self {
                self.query.projection.retain(|field| field != "color");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "color");
                self
            }
            pub fn with_color(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("color", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_color_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("color", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_color_is(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::eq("color", value));
                self
            }
            pub fn with_color_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("color", value));
                self
            }
            pub fn with_color_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("color", value));
                self
            }
            pub fn with_color_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gte("color", value));
                self
            }
            pub fn with_color_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("color", value));
                self
            }
            pub fn with_color_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lte("color", value));
                self
            }
            pub fn with_color_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("color", lower, upper));
                self
            }
            pub fn with_color_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("color", range.start,
                            range.end));
                self
            }
            pub fn with_color_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("color",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_color_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("color",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_color_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("color", value));
                self
            }
            pub fn with_color_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("color", value));
                self
            }
            pub fn with_color_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("color", value));
                self
            }
            pub fn with_color_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("color", value));
                self
            }
            pub fn with_color_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("color", value));
                self
            }
            pub fn with_color_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("color", value));
                self
            }
            pub fn with_color_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("color", value));
                self
            }
            pub fn with_color_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("color", value));
                self
            }
            pub fn with_color_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("color", value));
                self
            }
            pub fn with_color_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("color"));
                self
            }
            pub fn with_color_is_known(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("color"));
                self
            }
            pub fn order_by_color_asc(mut self) -> Self {
                self.query = self.query.order_asc("color");
                self
            }
            pub fn order_by_color_desc(mut self) -> Self {
                self.query = self.query.order_desc("color");
                self
            }
            pub fn order_by_color_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("color");
                self
            }
            pub fn order_by_color_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("color");
                self
            }
            pub fn select_display_order(mut self) -> Self {
                self.query = self.query.project("display_order");
                self
            }
            pub fn project_display_order(self) -> Self {
                self.select_display_order()
            }
            pub fn select_display_order_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_display_order_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_display_order_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("display_order",
                        raw_sql_segment));
                self
            }
            pub fn select_display_order_with_function(self,
                function: AggregateFunction) -> Self {
                self.select_display_order_as_with_function("display_order",
                    function)
            }
            pub fn select_display_order_as_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.aggregate_with_function("display_order", alias, function)
            }
            pub fn group_by_display_order(self) -> Self {
                self.group_by("display_order")
            }
            pub fn group_by_display_order_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("display_order");
                request.query =
                    request.query.project_expr(alias,
                        Expr::column("display_order"));
                request
            }
            pub fn group_by_display_order_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("display_order").aggregate_with_function("display_order",
                    alias, function)
            }
            pub fn count_display_order(self) -> Self {
                self.count_display_order_as("display_order_count")
            }
            pub fn count_display_order_as(self, alias: impl Into<String>)
                -> Self {
                self.aggregate_count_field("display_order", alias)
            }
            pub fn sum_display_order(self) -> Self {
                self.sum_display_order_as("sum_display_order")
            }
            pub fn sum_display_order_as(self, alias: impl Into<String>)
                -> Self {
                self.aggregate_sum("display_order", alias)
            }
            pub fn avg_display_order(self) -> Self {
                self.avg_display_order_as("avg_display_order")
            }
            pub fn avg_display_order_as(self, alias: impl Into<String>)
                -> Self {
                self.aggregate_avg("display_order", alias)
            }
            pub fn min_display_order(self) -> Self {
                self.min_display_order_as("min_display_order")
            }
            pub fn min_display_order_as(self, alias: impl Into<String>)
                -> Self {
                self.aggregate_min("display_order", alias)
            }
            pub fn max_display_order(self) -> Self {
                self.max_display_order_as("max_display_order")
            }
            pub fn max_display_order_as(self, alias: impl Into<String>)
                -> Self {
                self.aggregate_max("display_order", alias)
            }
            pub fn standard_deviation_display_order(self) -> Self {
                self.standard_deviation_display_order_as("stdDev_display_order")
            }
            pub fn standard_deviation_display_order_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_stddev("display_order", alias)
            }
            pub fn square_root_of_population_standard_deviation_display_order(self)
                -> Self {
                self.square_root_of_population_standard_deviation_display_order_as("stdDevPop_display_order")
            }
            pub fn square_root_of_population_standard_deviation_display_order_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_stddev_pop("display_order", alias)
            }
            pub fn sample_variance_display_order(self) -> Self {
                self.sample_variance_display_order_as("varSamp_display_order")
            }
            pub fn sample_variance_display_order_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_var_samp("display_order", alias)
            }
            pub fn sample_population_variance_display_order(self) -> Self {
                self.sample_population_variance_display_order_as("varPop_display_order")
            }
            pub fn sample_population_variance_display_order_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_var_pop("display_order", alias)
            }
            pub fn unselect_display_order(mut self) -> Self {
                self.query.projection.retain(|field|
                        field != "display_order");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "display_order");
                self
            }
            pub fn with_display_order(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("display_order",
                            operator, values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_display_order_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("display_order", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_display_order_is(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::eq("display_order", value));
                self
            }
            pub fn with_display_order_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::ne("display_order", value));
                self
            }
            pub fn with_display_order_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gt("display_order", value));
                self
            }
            pub fn with_display_order_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gte("display_order", value));
                self
            }
            pub fn with_display_order_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lt("display_order", value));
                self
            }
            pub fn with_display_order_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lte("display_order", value));
                self
            }
            pub fn with_display_order_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("display_order", lower,
                            upper));
                self
            }
            pub fn with_display_order_between_range<T>(mut self,
                range: DateRange<T>) -> Self where
                T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("display_order",
                            range.start, range.end));
                self
            }
            pub fn with_display_order_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("display_order",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_display_order_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("display_order",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_display_order_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lt("display_order", value));
                self
            }
            pub fn with_display_order_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gt("display_order", value));
                self
            }
            pub fn with_display_order_is_unknown(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_null("display_order"));
                self
            }
            pub fn with_display_order_is_known(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("display_order"));
                self
            }
            pub fn order_by_display_order_asc(mut self) -> Self {
                self.query = self.query.order_asc("display_order");
                self
            }
            pub fn order_by_display_order_desc(mut self) -> Self {
                self.query = self.query.order_desc("display_order");
                self
            }
            pub fn order_by_display_order_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("display_order");
                self
            }
            pub fn order_by_display_order_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("display_order");
                self
            }
            pub fn select_progress(mut self) -> Self {
                self.query = self.query.project("progress");
                self
            }
            pub fn project_progress(self) -> Self { self.select_progress() }
            pub fn select_progress_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_progress_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_progress_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("progress",
                        raw_sql_segment));
                self
            }
            pub fn select_progress_with_function(self,
                function: AggregateFunction) -> Self {
                self.select_progress_as_with_function("progress", function)
            }
            pub fn select_progress_as_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.aggregate_with_function("progress", alias, function)
            }
            pub fn group_by_progress(self) -> Self {
                self.group_by("progress")
            }
            pub fn group_by_progress_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("progress");
                request.query =
                    request.query.project_expr(alias, Expr::column("progress"));
                request
            }
            pub fn group_by_progress_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("progress").aggregate_with_function("progress",
                    alias, function)
            }
            pub fn count_progress(self) -> Self {
                self.count_progress_as("progress_count")
            }
            pub fn count_progress_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("progress", alias)
            }
            pub fn sum_progress(self) -> Self {
                self.sum_progress_as("sum_progress")
            }
            pub fn sum_progress_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("progress", alias)
            }
            pub fn avg_progress(self) -> Self {
                self.avg_progress_as("avg_progress")
            }
            pub fn avg_progress_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("progress", alias)
            }
            pub fn min_progress(self) -> Self {
                self.min_progress_as("min_progress")
            }
            pub fn min_progress_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("progress", alias)
            }
            pub fn max_progress(self) -> Self {
                self.max_progress_as("max_progress")
            }
            pub fn max_progress_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("progress", alias)
            }
            pub fn standard_deviation_progress(self) -> Self {
                self.standard_deviation_progress_as("stdDev_progress")
            }
            pub fn standard_deviation_progress_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_stddev("progress", alias)
            }
            pub fn square_root_of_population_standard_deviation_progress(self)
                -> Self {
                self.square_root_of_population_standard_deviation_progress_as("stdDevPop_progress")
            }
            pub fn square_root_of_population_standard_deviation_progress_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_stddev_pop("progress", alias)
            }
            pub fn sample_variance_progress(self) -> Self {
                self.sample_variance_progress_as("varSamp_progress")
            }
            pub fn sample_variance_progress_as(self, alias: impl Into<String>)
                -> Self {
                self.aggregate_var_samp("progress", alias)
            }
            pub fn sample_population_variance_progress(self) -> Self {
                self.sample_population_variance_progress_as("varPop_progress")
            }
            pub fn sample_population_variance_progress_as(self,
                alias: impl Into<String>) -> Self {
                self.aggregate_var_pop("progress", alias)
            }
            pub fn unselect_progress(mut self) -> Self {
                self.query.projection.retain(|field| field != "progress");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "progress");
                self
            }
            pub fn with_progress(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("progress",
                            operator, values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_progress_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("progress", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_progress_is(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::eq("progress", value));
                self
            }
            pub fn with_progress_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::ne("progress", value));
                self
            }
            pub fn with_progress_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gt("progress", value));
                self
            }
            pub fn with_progress_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gte("progress", value));
                self
            }
            pub fn with_progress_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lt("progress", value));
                self
            }
            pub fn with_progress_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lte("progress", value));
                self
            }
            pub fn with_progress_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("progress", lower,
                            upper));
                self
            }
            pub fn with_progress_between_range<T>(mut self,
                range: DateRange<T>) -> Self where
                T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("progress", range.start,
                            range.end));
                self
            }
            pub fn with_progress_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("progress",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_progress_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("progress",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_progress_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lt("progress", value));
                self
            }
            pub fn with_progress_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gt("progress", value));
                self
            }
            pub fn with_progress_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("progress"));
                self
            }
            pub fn with_progress_is_known(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("progress"));
                self
            }
            pub fn order_by_progress_asc(mut self) -> Self {
                self.query = self.query.order_asc("progress");
                self
            }
            pub fn order_by_progress_desc(mut self) -> Self {
                self.query = self.query.order_desc("progress");
                self
            }
            pub fn order_by_progress_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("progress");
                self
            }
            pub fn order_by_progress_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("progress");
                self
            }
            pub fn select_version(mut self) -> Self {
                self.query = self.query.project("version");
                self
            }
            pub fn project_version(self) -> Self { self.select_version() }
            pub fn select_version_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_version_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_version_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("version",
                        raw_sql_segment));
                self
            }
            pub fn group_by_version(self) -> Self { self.group_by("version") }
            pub fn group_by_version_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("version");
                request.query =
                    request.query.project_expr(alias, Expr::column("version"));
                request
            }
            pub fn group_by_version_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("version").aggregate_with_function("version",
                    alias, function)
            }
            pub fn count_version(self) -> Self {
                self.count_version_as("version_count")
            }
            pub fn count_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("version", alias)
            }
            pub fn sum_version(self) -> Self {
                self.sum_version_as("sum_version")
            }
            pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("version", alias)
            }
            pub fn avg_version(self) -> Self {
                self.avg_version_as("avg_version")
            }
            pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("version", alias)
            }
            pub fn min_version(self) -> Self {
                self.min_version_as("min_version")
            }
            pub fn min_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("version", alias)
            }
            pub fn max_version(self) -> Self {
                self.max_version_as("max_version")
            }
            pub fn max_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("version", alias)
            }
            pub fn unselect_version(mut self) -> Self {
                self.query.projection.retain(|field| field != "version");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "version");
                self
            }
            pub fn order_by_version_asc(mut self) -> Self {
                self.query = self.query.order_asc("version");
                self
            }
            pub fn order_by_version_desc(mut self) -> Self {
                self.query = self.query.order_desc("version");
                self
            }
            pub fn order_by_version_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("version");
                self
            }
            pub fn order_by_version_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("version");
                self
            }
            pub fn id_is_value_1001(self) -> Self { self.with_id_is("1001") }
            pub fn with_id_is_value_1001(self) -> Self {
                self.with_id_is("1001")
            }
            pub fn with_id_is_not_value_1001(self) -> Self {
                self.with_id_is_not("1001")
            }
            pub fn id_is_value_1002(self) -> Self { self.with_id_is("1002") }
            pub fn with_id_is_value_1002(self) -> Self {
                self.with_id_is("1002")
            }
            pub fn with_id_is_not_value_1002(self) -> Self {
                self.with_id_is_not("1002")
            }
            pub fn id_is_value_1003(self) -> Self { self.with_id_is("1003") }
            pub fn with_id_is_value_1003(self) -> Self {
                self.with_id_is("1003")
            }
            pub fn with_id_is_not_value_1003(self) -> Self {
                self.with_id_is_not("1003")
            }
            pub fn name_is_planned(self) -> Self {
                self.with_name_is("Planned")
            }
            pub fn with_name_is_planned(self) -> Self {
                self.with_name_is("Planned")
            }
            pub fn with_name_is_not_planned(self) -> Self {
                self.with_name_is_not("Planned")
            }
            pub fn name_is_process(self) -> Self {
                self.with_name_is("Process")
            }
            pub fn with_name_is_process(self) -> Self {
                self.with_name_is("Process")
            }
            pub fn with_name_is_not_process(self) -> Self {
                self.with_name_is_not("Process")
            }
            pub fn name_is_done(self) -> Self { self.with_name_is("Done") }
            pub fn with_name_is_done(self) -> Self {
                self.with_name_is("Done")
            }
            pub fn with_name_is_not_done(self) -> Self {
                self.with_name_is_not("Done")
            }
            pub fn code_is_planne_d(self) -> Self {
                self.with_code_is("PLANNED")
            }
            pub fn with_code_is_planne_d(self) -> Self {
                self.with_code_is("PLANNED")
            }
            pub fn with_code_is_not_planne_d(self) -> Self {
                self.with_code_is_not("PLANNED")
            }
            pub fn code_is_proces_s(self) -> Self {
                self.with_code_is("PROCESS")
            }
            pub fn with_code_is_proces_s(self) -> Self {
                self.with_code_is("PROCESS")
            }
            pub fn with_code_is_not_proces_s(self) -> Self {
                self.with_code_is_not("PROCESS")
            }
            pub fn code_is_don_e(self) -> Self { self.with_code_is("DONE") }
            pub fn with_code_is_don_e(self) -> Self {
                self.with_code_is("DONE")
            }
            pub fn with_code_is_not_don_e(self) -> Self {
                self.with_code_is_not("DONE")
            }
            pub fn color_is_value_94a3b8(self) -> Self {
                self.with_color_is("#94A3B8")
            }
            pub fn with_color_is_value_94a3b8(self) -> Self {
                self.with_color_is("#94A3B8")
            }
            pub fn with_color_is_not_value_94a3b8(self) -> Self {
                self.with_color_is_not("#94A3B8")
            }
            pub fn color_is_f59e0_b(self) -> Self {
                self.with_color_is("#F59E0B")
            }
            pub fn with_color_is_f59e0_b(self) -> Self {
                self.with_color_is("#F59E0B")
            }
            pub fn with_color_is_not_f59e0_b(self) -> Self {
                self.with_color_is_not("#F59E0B")
            }
            pub fn color_is_value_16a34_a(self) -> Self {
                self.with_color_is("#16A34A")
            }
            pub fn with_color_is_value_16a34_a(self) -> Self {
                self.with_color_is("#16A34A")
            }
            pub fn with_color_is_not_value_16a34_a(self) -> Self {
                self.with_color_is_not("#16A34A")
            }
            pub fn display_order_is_value_10(self) -> Self {
                self.with_display_order_is("10")
            }
            pub fn with_display_order_is_value_10(self) -> Self {
                self.with_display_order_is("10")
            }
            pub fn with_display_order_is_not_value_10(self) -> Self {
                self.with_display_order_is_not("10")
            }
            pub fn display_order_is_value_20(self) -> Self {
                self.with_display_order_is("20")
            }
            pub fn with_display_order_is_value_20(self) -> Self {
                self.with_display_order_is("20")
            }
            pub fn with_display_order_is_not_value_20(self) -> Self {
                self.with_display_order_is_not("20")
            }
            pub fn display_order_is_value_30(self) -> Self {
                self.with_display_order_is("30")
            }
            pub fn with_display_order_is_value_30(self) -> Self {
                self.with_display_order_is("30")
            }
            pub fn with_display_order_is_not_value_30(self) -> Self {
                self.with_display_order_is_not("30")
            }
            pub fn progress_is_value_0(self) -> Self {
                self.with_progress_is("0")
            }
            pub fn with_progress_is_value_0(self) -> Self {
                self.with_progress_is("0")
            }
            pub fn with_progress_is_not_value_0(self) -> Self {
                self.with_progress_is_not("0")
            }
            pub fn progress_is_value_50(self) -> Self {
                self.with_progress_is("50")
            }
            pub fn with_progress_is_value_50(self) -> Self {
                self.with_progress_is("50")
            }
            pub fn with_progress_is_not_value_50(self) -> Self {
                self.with_progress_is_not("50")
            }
            pub fn progress_is_value_100(self) -> Self {
                self.with_progress_is("100")
            }
            pub fn with_progress_is_value_100(self) -> Self {
                self.with_progress_is("100")
            }
            pub fn with_progress_is_not_value_100(self) -> Self {
                self.with_progress_is_not("100")
            }
            pub fn have_tasks(self) -> Self {
                self.with_task_list_matching(SelectQuery::new("Task"))
            }
            pub fn have_no_tasks(self) -> Self {
                self.without_task_list_matching(SelectQuery::new("Task"))
            }
            pub fn with_task_list_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::in_subquery("id",
                            <crate::Task as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "status_id"));
                self.relation_filters.push(RelationFilter::new("task_list",
                        selection));
                self
            }
            pub fn without_task_list_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::not_in_subquery("id",
                            <crate::Task as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "status_id"));
                self.relation_filters.push(RelationFilter::new("task_list",
                        selection));
                self
            }
            pub fn select_task_list(mut self) -> Self {
                self.query = self.query.relation("task_list");
                self
            }
            pub fn select_task_list_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.relation_query("task_list",
                        selection.clone().into_query());
                self.relation_selections.push(RelationSelection::new("task_list",
                        selection));
                self
            }
            pub fn count_tasks(self) -> Self {
                self.count_tasks_as("count_tasks")
            }
            pub fn count_tasks_as(self, alias: impl Into<String>) -> Self {
                self.count_tasks_with(alias, crate::Q::tasks().unlimited())
            }
            pub fn count_tasks_with(mut self, alias: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query_options.relation_aggregates.push(RelationAggregate::new("task_list",
                        alias, selection, true));
                self
            }
            pub fn stats_from_tasks(self, request: impl Into<QuerySelection>)
                -> Self {
                self.stats_from_tasks_as("refinements", request)
            }
            pub fn stats_from_tasks_as(mut self, alias: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query_options.relation_aggregates.push(RelationAggregate::new("task_list",
                        alias, selection, false));
                self
            }
            pub fn group_by_tasks_with_details(self,
                request: impl Into<QuerySelection>) -> Self {
                self.stats_from_tasks(request)
            }
        }
        impl<R> Default for TaskStatusRequest<R> {
            fn default() -> Self { Self::new() }
        }
        impl<R> From<TaskStatusRequest<R>> for SelectQuery {
            fn from(request: TaskStatusRequest<R>) -> Self {
                QuerySelection::from(request).into_query()
            }
        }
        impl<R> From<TaskStatusRequest<R>> for QuerySelection {
            fn from(request: TaskStatusRequest<R>) -> Self {
                Self {
                    query: request.query,
                    relation_selections: request.relation_selections,
                    relation_filters: request.relation_filters,
                    child_enhancements: request.child_enhancements,
                    query_options: request.query_options,
                }
            }
        }
    }
    pub use behavior::*;
    pub use checker::*;
    pub use entity::TaskStatus;
    pub use expression::*;
    pub use request::*;
}
pub mod task {
    mod behavior {
        use teaql_runtime::RepositoryBehavior;
        pub struct TaskBehavior;
        #[automatically_derived]
        impl ::core::clone::Clone for TaskBehavior {
            #[inline]
            fn clone(&self) -> TaskBehavior { TaskBehavior }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TaskBehavior {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "TaskBehavior")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TaskBehavior {
            #[inline]
            fn default() -> TaskBehavior { TaskBehavior {} }
        }
        impl RepositoryBehavior for TaskBehavior {}
    }
    mod checker {
        use teaql_runtime::{
            CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker,
            UserContext,
        };
        pub trait TaskCheckerLogic: Send + Sync {
            fn check_and_fix_task(&self, _ctx: &UserContext,
                _entity: &mut crate::Task, _status: CheckObjectStatus,
                _location: &ObjectLocation, _results: &mut CheckResults) {}
            fn required(&self, value: bool, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if !value {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_option<V>(&self, value: Option<&V>, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.is_none() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_text(&self, value: &str, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.trim().is_empty() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn min_string_length(&self, value: &str, field: &str,
                min_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() < min_len {
                    results.push(teaql_runtime::CheckResult::min_str(location.clone().member(field),
                            min_len as u64, value.to_owned()));
                }
            }
            fn max_string_length(&self, value: &str, field: &str,
                max_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() > max_len {
                    results.push(teaql_runtime::CheckResult::max_str(location.clone().member(field),
                            max_len as u64, value.to_owned()));
                }
            }
        }
        pub struct NoopTaskChecker;
        #[automatically_derived]
        impl ::core::clone::Clone for NoopTaskChecker {
            #[inline]
            fn clone(&self) -> NoopTaskChecker { NoopTaskChecker }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NoopTaskChecker {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "NoopTaskChecker")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NoopTaskChecker {
            #[inline]
            fn default() -> NoopTaskChecker { NoopTaskChecker {} }
        }
        impl TaskCheckerLogic for NoopTaskChecker {}
        pub struct TaskChecker<L = NoopTaskChecker> {
            logic: L,
        }
        #[automatically_derived]
        impl<L: ::core::clone::Clone> ::core::clone::Clone for TaskChecker<L>
            {
            #[inline]
            fn clone(&self) -> TaskChecker<L> {
                TaskChecker {
                    logic: ::core::clone::Clone::clone(&self.logic),
                }
            }
        }
        #[automatically_derived]
        impl<L: ::core::fmt::Debug> ::core::fmt::Debug for TaskChecker<L> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "TaskChecker", "logic", &&self.logic)
            }
        }
        impl Default for TaskChecker<NoopTaskChecker> {
            fn default() -> Self { Self { logic: NoopTaskChecker } }
        }
        impl<L> TaskChecker<L> where L: TaskCheckerLogic {
            pub fn new(logic: L) -> Self { Self { logic } }
        }
        impl<L> TypedChecker<crate::Task> for TaskChecker<L> where
            L: TaskCheckerLogic {
            fn check_and_fix_typed(&self, ctx: &UserContext,
                entity: &mut crate::Task, status: CheckObjectStatus,
                location: &ObjectLocation, results: &mut CheckResults) {
                self.logic.check_and_fix_task(ctx, entity, status, location,
                    results);
            }
        }
    }
    mod entity {
        use std::collections::BTreeMap;
        use teaql_core::SmartList;
        use teaql_macros::TeaqlEntity;
        #[teaql(entity = "Task", table = "task_data")]
        pub struct Task {
            #[teaql(id)]
            id: u64,
            name: String,
            #[teaql(version)]
            version: i64,
            #[teaql(column = "status")]
            status_id: u64,
            #[teaql(column = "platform")]
            platform_id: u64,
            #[teaql(relation(target = "TaskStatus", local_key = "status_id",
            foreign_key = "id"))]
            status: Option<crate::TaskStatus>,
            #[teaql(relation(target = "Platform", local_key = "platform_id",
            foreign_key = "id"))]
            platform: Option<crate::Platform>,
            #[teaql(relation(target = "TaskExecutionLog", local_key = "id",
            foreign_key = "task_id", many))]
            task_execution_log_list: SmartList<crate::TaskExecutionLog>,
            #[teaql(dynamic)]
            dynamic: BTreeMap<String, teaql_core::Value>,
            #[teaql(skip)]
            root: teaql_runtime::EntityRoot,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Task {
            #[inline]
            fn clone(&self) -> Task {
                Task {
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    version: ::core::clone::Clone::clone(&self.version),
                    status_id: ::core::clone::Clone::clone(&self.status_id),
                    platform_id: ::core::clone::Clone::clone(&self.platform_id),
                    status: ::core::clone::Clone::clone(&self.status),
                    platform: ::core::clone::Clone::clone(&self.platform),
                    task_execution_log_list: ::core::clone::Clone::clone(&self.task_execution_log_list),
                    dynamic: ::core::clone::Clone::clone(&self.dynamic),
                    root: ::core::clone::Clone::clone(&self.root),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Task {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["id", "name", "version", "status_id", "platform_id",
                                "status", "platform", "task_execution_log_list", "dynamic",
                                "root"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.id, &self.name, &self.version, &self.status_id,
                                &self.platform_id, &self.status, &self.platform,
                                &self.task_execution_log_list, &self.dynamic, &&self.root];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Task",
                    names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Task { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Task {
            #[inline]
            fn eq(&self, other: &Task) -> bool {
                self.id == other.id && self.version == other.version &&
                                                self.status_id == other.status_id &&
                                            self.platform_id == other.platform_id &&
                                        self.name == other.name && self.status == other.status &&
                                self.platform == other.platform &&
                            self.task_execution_log_list ==
                                other.task_execution_log_list &&
                        self.dynamic == other.dynamic && self.root == other.root
            }
        }
        impl ::teaql_core::TeaqlEntity for Task {
            fn entity_descriptor() -> ::teaql_core::EntityDescriptor {
                let mut descriptor =
                    ::teaql_core::EntityDescriptor::new("Task").table_name("task_data");
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("id",
                                        ::teaql_core::DataType::U64).column_name("id").not_null().id());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("name",
                                    ::teaql_core::DataType::Text).column_name("name").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("version",
                                        ::teaql_core::DataType::I64).column_name("version").not_null().version());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("status_id",
                                    ::teaql_core::DataType::U64).column_name("status").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("platform_id",
                                    ::teaql_core::DataType::U64).column_name("platform").not_null());
                descriptor =
                    descriptor.relation(::teaql_core::RelationDescriptor::new("status",
                                    "TaskStatus").local_key("status_id").foreign_key("id"));
                descriptor =
                    descriptor.relation(::teaql_core::RelationDescriptor::new("platform",
                                    "Platform").local_key("platform_id").foreign_key("id"));
                descriptor =
                    descriptor.relation(::teaql_core::RelationDescriptor::new("task_execution_log_list",
                                        "TaskExecutionLog").local_key("id").foreign_key("task_id").many());
                descriptor
            }
        }
        impl ::teaql_core::Entity for Task {
            fn from_record(record: ::teaql_core::Record)
                -> Result<Self, ::teaql_core::EntityError> {
                Ok(Self {
                        id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Task",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "id", other))
                                                                        }))),
                                                })
                                        })()?,
                        name: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("name") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Task",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "name", other))
                                                                        }))),
                                                })
                                        })()?,
                        version: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("version") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::I64(v) => *v,
                                                    ::teaql_core::Value::U64(v) =>
                                                        i64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: u64 out of i64 range",
                                                                                        "version"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Task",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "version", other))
                                                                        }))),
                                                })
                                        })()?,
                        status_id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("status_id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "status_id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "status_id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Task",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "status_id", other))
                                                                        }))),
                                                })
                                        })()?,
                        platform_id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("platform_id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "platform_id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("Task",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "platform_id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("Task",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "platform_id", other))
                                                                        }))),
                                                })
                                        })()?,
                        status: match record.get("status") {
                            Some(::teaql_core::Value::Object(record)) => {
                                Some(<crate::TaskStatus as
                                                ::teaql_core::Entity>::from_record(record.clone())?)
                            }
                            Some(::teaql_core::Value::Null) | None => None,
                            other => {
                                return Err(::teaql_core::EntityError::new("Task",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("invalid relation field {0}: {1:?}",
                                                            "status", other))
                                                })))
                            }
                        },
                        platform: match record.get("platform") {
                            Some(::teaql_core::Value::Object(record)) => {
                                Some(<crate::Platform as
                                                ::teaql_core::Entity>::from_record(record.clone())?)
                            }
                            Some(::teaql_core::Value::Null) | None => None,
                            other => {
                                return Err(::teaql_core::EntityError::new("Task",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("invalid relation field {0}: {1:?}",
                                                            "platform", other))
                                                })))
                            }
                        },
                        task_execution_log_list: match record.get("task_execution_log_list")
                            {
                            Some(::teaql_core::Value::List(values)) =>
                                ::teaql_core::SmartList::from(values.iter().map(|value|
                                                    match value {
                                                        ::teaql_core::Value::Object(record) => {
                                                            <crate::TaskExecutionLog as
                                                                    ::teaql_core::Entity>::from_record(record.clone())
                                                        }
                                                        other =>
                                                            Err(::teaql_core::EntityError::new("Task",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid relation list item {0}: {1:?}",
                                                                                    "task_execution_log_list", other))
                                                                        }))),
                                                    }).collect::<Result<Vec<_>, _>>()?),
                            Some(::teaql_core::Value::Null) | None =>
                                ::teaql_core::SmartList::default(),
                            other => {
                                return Err(::teaql_core::EntityError::new("Task",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("invalid relation field {0}: {1:?}",
                                                            "task_execution_log_list", other))
                                                })))
                            }
                        },
                        dynamic: {
                            let known_fields =
                                ["id", "name", "version", "status_id", "platform_id",
                                        "status", "platform", "task_execution_log_list"];
                            record.iter().filter(|(key, _)|
                                            !known_fields.contains(&key.as_str())).map(|(key, value)|
                                        (key.clone(), value.clone())).collect()
                        },
                        root: Default::default(),
                    })
            }
            fn into_record(self) -> ::teaql_core::Record {
                let mut record = ::teaql_core::Record::new();
                record.insert("id".to_owned(), (self.id).into());
                record.insert("name".to_owned(), (self.name).into());
                record.insert("version".to_owned(), (self.version).into());
                record.insert("status_id".to_owned(),
                    (self.status_id).into());
                record.insert("platform_id".to_owned(),
                    (self.platform_id).into());
                record.insert("status".to_owned(),
                    match self.status {
                        Some(entity) =>
                            ::teaql_core::Value::object(entity.into_record()),
                        None => ::teaql_core::Value::Null,
                    });
                record.insert("platform".to_owned(),
                    match self.platform {
                        Some(entity) =>
                            ::teaql_core::Value::object(entity.into_record()),
                        None => ::teaql_core::Value::Null,
                    });
                record.insert("task_execution_log_list".to_owned(),
                    ::teaql_core::Value::List((self.task_execution_log_list).data.into_iter().map(|entity|
                                    ::teaql_core::Value::object(entity.into_record())).collect()));
                for (key, value) in self.dynamic {
                    record.insert(key, value);
                }
                record
            }
            fn on_loaded(&mut self, _context: &dyn std::any::Any) {}
        }
        impl ::teaql_core::IdentifiableEntity for Task {
            fn id_value(&self) -> ::teaql_core::Value {
                ::teaql_core::Value::U64((*&self.id).into())
            }
        }
        impl ::teaql_core::VersionedEntity for Task {
            fn version(&self) -> i64 { self.version }
        }
        impl Task {
            pub fn with_id(id: u64) -> teaql_core::Value {
                teaql_core::Value::U64(id)
            }
            pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot)
                -> Self {
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
                }
            }
            pub fn entity_key(&self) -> teaql_runtime::EntityKey {
                teaql_runtime::EntityKey::new("Task", self.id)
            }
            pub fn attach_root_recursive(&mut self,
                root: teaql_runtime::EntityRoot) {
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
            pub fn id(&self) -> u64 {
                self.changed_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.id)
            }
            pub fn update_id(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.id = value.try_u64().unwrap_or(self.id.clone());
                self.root.set(self.entity_key(), "id", value);
                self
            }
            pub fn changed_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "id")
            }
            pub fn name(&self) -> String {
                self.changed_name().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.name.clone())
            }
            pub fn update_name(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.name =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.name.clone());
                self.root.set(self.entity_key(), "name", value);
                self
            }
            pub fn changed_name(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "name")
            }
            pub fn version(&self) -> i64 {
                self.changed_version().and_then(|value|
                            value.try_i64()).unwrap_or(self.version)
            }
            pub fn update_version(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.version =
                    value.try_i64().unwrap_or(self.version.clone());
                self.root.set(self.entity_key(), "version", value);
                self
            }
            pub fn changed_version(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "version")
            }
            pub fn status_id(&self) -> u64 {
                self.changed_status_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.status_id)
            }
            pub(crate) fn update_status_id(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.status_id =
                    value.try_u64().unwrap_or(self.status_id.clone());
                self.root.set(self.entity_key(), "status_id", value);
                self
            }
            pub fn changed_status_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "status_id")
            }
            pub fn platform_id(&self) -> u64 {
                self.changed_platform_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.platform_id)
            }
            pub fn update_platform_id(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.platform_id =
                    value.try_u64().unwrap_or(self.platform_id.clone());
                self.root.set(self.entity_key(), "platform_id", value);
                self
            }
            pub fn changed_platform_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "platform_id")
            }
            pub fn update_status_to_planned(&mut self) -> &mut Self {
                self.update_status_id(1001_u64)
            }
            pub fn status_is_planned(&self) -> bool {
                self.status_id() == 1001_u64
            }
            pub fn update_status_to_process(&mut self) -> &mut Self {
                self.update_status_id(1002_u64)
            }
            pub fn status_is_process(&self) -> bool {
                self.status_id() == 1002_u64
            }
            pub fn update_status_to_done(&mut self) -> &mut Self {
                self.update_status_id(1003_u64)
            }
            pub fn status_is_done(&self) -> bool {
                self.status_id() == 1003_u64
            }
            pub fn status(&self) -> Option<&crate::TaskStatus> {
                self.status.as_ref()
            }
            pub fn platform(&self) -> Option<&crate::Platform> {
                self.platform.as_ref()
            }
            pub fn task_execution_log_list(&self)
                -> &SmartList<crate::TaskExecutionLog> {
                &self.task_execution_log_list
            }
            pub fn mark_as_delete(&mut self) -> &mut Self {
                self.root.mark_as_delete(self.entity_key());
                self
            }
            pub fn set_comment(&mut self, comment: impl Into<String>)
                -> &mut Self {
                self.root.set_comment(comment);
                self
            }
            pub async fn save<'a, C>(self, ctx: &'a C)
                ->
                    Result<teaql_runtime::GraphNode,
                    crate::TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: crate::TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_repository().map_err(|err|
                                teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
                crate::TeaqlEntityRepository::save_entity_graph(&repository,
                    self)
            }
        }
    }
    mod expression {
        use teaql_core::{SafeExpression, SmartList};
        pub struct TaskExpression<R> {
            expression: SafeExpression<R, crate::Task>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            TaskExpression<R> {
            #[inline]
            fn clone(&self) -> TaskExpression<R> {
                TaskExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> TaskExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression: SafeExpression<R, crate::Task>) -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<crate::Task> {
                self.expression.eval()
            }
            pub fn get_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.id())
            }
            pub fn get_name(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.name())
            }
            pub fn get_version(self) -> SafeExpression<R, i64> {
                self.expression.apply(|value| value.version())
            }
            pub fn get_status_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.status_id())
            }
            pub fn get_platform_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.platform_id())
            }
            pub fn get_status(self) -> crate::TaskStatusExpression<R> {
                crate::TaskStatusExpression::new(self.expression.apply_optional(|value|
                            value.status().cloned()))
            }
            pub fn get_platform(self) -> crate::PlatformExpression<R> {
                crate::PlatformExpression::new(self.expression.apply_optional(|value|
                            value.platform().cloned()))
            }
            pub fn status_is_planned(self) -> SafeExpression<R, bool> {
                self.expression.apply(|value| value.status_is_planned())
            }
            pub fn status_is_process(self) -> SafeExpression<R, bool> {
                self.expression.apply(|value| value.status_is_process())
            }
            pub fn status_is_done(self) -> SafeExpression<R, bool> {
                self.expression.apply(|value| value.status_is_done())
            }
            pub fn get_task_execution_log_list(self)
                -> crate::TaskExecutionLogListExpression<R> {
                crate::TaskExecutionLogListExpression::new(self.expression.apply(|value|
                            value.task_execution_log_list().clone()))
            }
        }
        pub struct TaskListExpression<R> {
            expression: SafeExpression<R, SmartList<crate::Task>>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            TaskListExpression<R> {
            #[inline]
            fn clone(&self) -> TaskListExpression<R> {
                TaskListExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> TaskListExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression: SafeExpression<R, SmartList<crate::Task>>)
                -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<SmartList<crate::Task>> {
                self.expression.eval()
            }
            pub fn size(self) -> SafeExpression<R, usize> {
                self.expression.size()
            }
            pub fn first(self) -> TaskExpression<R> {
                TaskExpression::new(self.expression.first())
            }
            pub fn get(self, index: usize) -> TaskExpression<R> {
                TaskExpression::new(self.expression.get(index))
            }
        }
    }
    mod request {
        use std::marker::PhantomData;
        use serde_json::Value as JsonValue;
        use teaql_core::{
            Aggregate, AggregateFunction, EntityDescriptor, Expr, Record,
            SelectQuery, SmartList,
        };
        use teaql_runtime::{RepositoryError, RuntimeError};
        use crate::request_support::*;
        impl EntityReference for crate::Task {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(&self)
            }
        }
        impl EntityReference for &crate::Task {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(self)
            }
        }
        pub struct TaskRequest<R = crate::Task> {
            query: SelectQuery,
            relation_selections: Vec<RelationSelection>,
            relation_filters: Vec<RelationFilter>,
            child_enhancements: Vec<QuerySelection>,
            query_options: QueryOptions,
            marker: PhantomData<R>,
        }
        #[automatically_derived]
        impl<R: ::core::fmt::Debug> ::core::fmt::Debug for TaskRequest<R> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["query", "relation_selections", "relation_filters",
                                "child_enhancements", "query_options", "marker"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.query, &self.relation_selections,
                                &self.relation_filters, &self.child_enhancements,
                                &self.query_options, &&self.marker];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "TaskRequest", names, values)
            }
        }
        impl<R> Clone for TaskRequest<R> {
            fn clone(&self) -> Self {
                Self {
                    query: self.query.clone(),
                    relation_selections: self.relation_selections.clone(),
                    relation_filters: self.relation_filters.clone(),
                    child_enhancements: self.child_enhancements.clone(),
                    query_options: self.query_options.clone(),
                    marker: PhantomData,
                }
            }
        }
        impl<R> TaskRequest<R> {
            pub(crate) fn new() -> Self {
                Self {
                    query: SelectQuery::new("Task"),
                    relation_selections: Vec::new(),
                    relation_filters: Vec::new(),
                    child_enhancements: Vec::new(),
                    query_options: QueryOptions::default(),
                    marker: PhantomData,
                }
            }
            pub fn return_type<T>(self) -> TaskRequest<T> {
                TaskRequest {
                    query: self.query,
                    relation_selections: self.relation_selections,
                    relation_filters: self.relation_filters,
                    child_enhancements: self.child_enhancements,
                    query_options: self.query_options,
                    marker: PhantomData,
                }
            }
            pub fn query(&self) -> &SelectQuery { &self.query }
            pub fn relation_selections(&self) -> &[RelationSelection] {
                &self.relation_selections
            }
            pub fn relation_filters(&self) -> &[RelationFilter] {
                &self.relation_filters
            }
            pub fn child_enhancements(&self) -> &[QuerySelection] {
                &self.child_enhancements
            }
            pub fn query_options(&self) -> &QueryOptions {
                &self.query_options
            }
            pub fn into_query(self) -> SelectQuery { self.query }
            pub fn new_entity<C>(&self, ctx: &C) -> crate::Task where
                C: TeaqlRuntime + ?Sized {
                crate::Task::runtime_new(ctx.user_context().entity_root())
            }
            pub async fn execute_for_list<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let repository =
                    ctx.task_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_enhanced_entities_with_relation_aggregates::<R>(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_first<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let rows = self.limit(1).execute_for_list(ctx).await?;
                Ok(rows.into_iter().next())
            }
            pub async fn execute_for_one<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.execute_for_first(ctx).await
            }
            pub async fn execute_by_id<'a,
                C>(self, ctx: &'a C, id: impl Into<teaql_core::Value>)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.and_filter(Expr::eq("id",
                                id)).execute_for_first(ctx).await
            }
            pub async fn execute_for_page<'a,
                C>(self, ctx: &'a C, offset: u64, limit: u64)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let total_count = self.clone().execute_for_count(ctx).await?;
                let mut rows =
                    self.page_offset(offset,
                                    limit).execute_for_list(ctx).await?;
                rows.total_count = Some(total_count);
                Ok(rows)
            }
            pub async fn execute_for_count<'a, C>(self, ctx: &'a C)
                -> Result<u64, TeaqlRepositoryError<C::TaskRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query;
                query.projection.clear();
                query.expr_projection.clear();
                query.order_by.clear();
                query.slice = None;
                query.relations.clear();
                query = query.count(COUNT_ALIAS);
                let rows = repository.fetch_all(&query)?;
                rows.first().and_then(|row|
                                row.get(COUNT_ALIAS)).and_then(teaql_core::Value::try_u64).ok_or_else(||
                        RepositoryError::Runtime(RuntimeError::Graph(::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("count result for Task is missing or not numeric"))
                                    }))))
            }
            pub async fn execute_for_exists<'a, C>(self, ctx: &'a C)
                -> Result<bool, TeaqlRepositoryError<C::TaskRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query.limit(1);
                query.relations.clear();
                let rows = repository.fetch_all(&query)?;
                Ok(!rows.is_empty())
            }
            pub async fn execute_for_records<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<Record>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_smart_list_with_relation_aggregates(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_record<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<Record>,
                    TeaqlRepositoryError<C::TaskRepository<'a>>> where
                C: TeaqlRuntime + ?Sized {
                let records = self.limit(1).execute_for_records(ctx).await?;
                Ok(records.into_iter().next())
            }
            pub fn filter(mut self, filter: Expr) -> Self {
                self.query = self.query.filter(filter);
                self
            }
            pub fn and_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.and_filter(filter);
                self
            }
            pub fn or_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.or_filter(filter);
                self
            }
            pub fn append_search_criteria(self, criteria: Expr) -> Self {
                self.and_filter(criteria)
            }
            pub fn filter_property(mut self, property1: impl AsRef<str>,
                operator: FieldOperator, property2: impl AsRef<str>) -> Self {
                self.query =
                    self.query.and_filter(field_operator_column_expr(property1.as_ref(),
                            operator, property2.as_ref()));
                self
            }
            pub fn with_deleted_rows(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self
            }
            pub fn deleted_rows_only(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self.query =
                    self.query.and_filter(Expr::lte("version", 0_i64));
                self
            }
            pub fn match_types(mut self,
                types: impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list(TYPE_FIELD,
                            types.into_iter().map(Into::into)));
                self
            }
            pub fn with_type_group(mut self) -> Self {
                self.query = self.query.project(TYPE_GROUP_FIELD);
                self
            }
            pub fn matching_any_of(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                let entity =
                    EntityDescriptor::new(selection.query.entity.clone());
                self.query =
                    self.query.and_filter(Expr::in_subquery("id", entity,
                            selection.query.clone(), "id"));
                self
            }
            pub fn match_any_of(self, request: impl Into<QuerySelection>)
                -> Self {
                self.matching_any_of(request)
            }
            pub fn enhance_child(mut self, request: impl Into<QuerySelection>)
                -> Self {
                self.child_enhancements.push(request.into());
                self
            }
            pub fn enhance_children_if_needed(self) -> Self {
                let request = self;
                request
            }
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.query_options.comment = Some(comment.into());
                self
            }
            pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment)
                -> Self {
                self.query_options.raw_sql = Some(raw_sql.into_sql());
                self
            }
            pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql_filter(mut self,
                raw_sql: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
                self
            }
            pub fn filter_with_json(self, json_expr: impl Into<String>)
                -> Self {
                self.merge_dynamic_json_expr(json_expr.into())
            }
            fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
                let json =
                    serde_json::from_str::<JsonValue>(&json_expr).unwrap_or_else(|_|
                            {
                                ::core::panicking::panic_fmt(format_args!("Input JSON format error: {0}",
                                        json_expr));
                            });
                self.merge_dynamic_json(&json)
            }
            fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
                let Some(object) = json.as_object() else { return self; };
                for (field, value) in object {
                    if field.starts_with('_') { continue; }
                    self = self.apply_dynamic_json_filter(field, value);
                }
                self =
                    self.apply_dynamic_json_order_by(object.get("_orderBy"));
                if let Some(offset) = dynamic_json_u64_field(object, "_start")
                    {
                    self = self.skip(offset);
                }
                if let Some(size) = dynamic_json_u64_field(object, "_size") {
                    self = self.limit(size);
                }
                if let Some(page_size) =
                        dynamic_json_u64_field(object, "_pageSize") {
                    self = self.limit(page_size);
                }
                if let Some(page_number) =
                        dynamic_json_u64_field(object, "_page") {
                    if page_number > 0 {
                        let size =
                            dynamic_json_u64_field(object,
                                        "_pageSize").or_else(||
                                        self.query.slice.as_ref().and_then(|slice|
                                                slice.limit)).unwrap_or(10);
                        let offset =
                            page_number.saturating_sub(1).saturating_mul(size);
                        self = self.page_offset(offset, size);
                    }
                }
                self
            }
            pub(crate) fn apply_dynamic_json_filter(self, field: &str,
                value: &JsonValue) -> Self {
                if let Some((head, tail)) = field.split_once('.') {
                    self.apply_dynamic_json_chain_filter(head, tail, value)
                } else if let Some(storage_field) =
                        Self::dynamic_json_self_field(field) {
                    self.and_filter(dynamic_json_filter_expr(storage_field,
                            value))
                } else { self }
            }
            fn apply_dynamic_json_order_by(mut self,
                order_by: Option<&JsonValue>) -> Self {
                match order_by {
                    Some(JsonValue::String(field)) => {
                        if let Some(storage_field) =
                                Self::dynamic_json_self_field(field) {
                            self.query = self.query.order_desc(storage_field);
                        }
                    }
                    Some(JsonValue::Object(order_by)) => {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                    Some(JsonValue::Array(order_bys)) => {
                        for order_by in order_bys {
                            if let Some(order_by) = order_by.as_object() {
                                self = self.apply_dynamic_json_single_order_by(order_by);
                            }
                        }
                    }
                    _ => {}
                }
                self
            }
            fn apply_dynamic_json_single_order_by(mut self,
                order_by: &serde_json::Map<String, JsonValue>) -> Self {
                let Some(field) =
                    order_by.get("field").and_then(JsonValue::as_str) else {
                        return self;
                    };
                let Some(storage_field) =
                    Self::dynamic_json_self_field(field) else { return self; };
                if order_by.get("useAsc").and_then(JsonValue::as_bool).unwrap_or(false)
                    {
                    self.query = self.query.order_asc(storage_field);
                } else { self.query = self.query.order_desc(storage_field); }
                self
            }
            fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
                match field {
                    "id" => Some("id"),
                    "name" => Some("name"),
                    "version" => Some("version"),
                    "status" | "status_id" => Some("status_id"),
                    "platform" | "platform_id" => Some("platform_id"),
                    _ => None,
                }
            }
            fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str,
                value: &JsonValue) -> Self {
                let _ = (tail, value);
                match head {
                    "status" => {
                        self.with_status_matching(crate::Q::task_status_minimal().apply_dynamic_json_filter(tail,
                                value))
                    }
                    "platform" => {
                        self.with_platform_matching(crate::Q::platforms_minimal().apply_dynamic_json_filter(tail,
                                value))
                    }
                    "task_execution_log_list" => {
                        self.with_task_execution_log_list_matching(crate::Q::task_execution_logs_minimal().apply_dynamic_json_filter(tail,
                                value))
                    }
                    _ => self,
                }
            }
            pub fn create_property_as(self, property_name: impl Into<String>,
                raw_sql_segment: impl Into<String>) -> Self {
                self.unsafe_create_property_as(property_name,
                    UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn unsafe_create_property_as(mut self,
                property_name: impl Into<String>,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.dynamic_properties.push(RawDynamicProperty::new(property_name,
                        raw_sql_segment));
                self
            }
            pub fn limit(mut self, limit: u64) -> Self {
                self.query = self.query.limit(limit);
                self
            }
            pub fn skip(mut self, offset: u64) -> Self {
                self.query = self.query.offset(offset);
                self
            }
            pub fn offset_only(self, offset: u64) -> Self {
                self.skip(offset)
            }
            pub fn offset(self, offset: u64, size: u64) -> Self {
                self.page_offset(offset, size)
            }
            pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
                self.query = self.query.page(offset, limit);
                self
            }
            pub fn top(self, top_n: u64) -> Self { self.limit(top_n) }
            pub fn offset_size(self, offset: u64, size: u64) -> Self {
                self.offset(offset, size)
            }
            pub fn unlimited(mut self) -> Self {
                self.query.slice = None;
                self
            }
            pub fn page_number(self, page_number: u64, page_size: u64)
                -> Self {
                let offset =
                    page_number.saturating_sub(1).saturating_mul(page_size);
                self.page_offset(offset, page_size)
            }
            pub fn page_number_default(self, page_number: u64) -> Self {
                self.page_number(page_number, 10)
            }
            pub fn page(self, page_number: u64, page_size: u64) -> Self {
                self.page_number(page_number, page_size)
            }
            pub fn page_default(self, page_number: u64) -> Self {
                self.page_number_default(page_number)
            }
            pub fn select_self(mut self) -> Self {
                self.query = self.query.project("id");
                self.query = self.query.project("name");
                self.query = self.query.project("version");
                self.query = self.query.project("status_id");
                self.query = self.query.project("platform_id");
                self
            }
            pub fn select_self_fields(self) -> Self { self.select_self() }
            pub fn select_self_without_parent(self) -> Self {
                self.select_self_fields()
            }
            pub fn select_all(self) -> Self {
                let mut request = self.select_self();
                request = request.select_status();
                request = request.select_platform();
                request
            }
            pub fn select_children(self) -> Self {
                let mut request = self.select_all();
                request = request.select_task_execution_log_list();
                request
            }
            pub fn select_any(self) -> Self { self.select_children() }
            pub fn group_by(mut self, field: impl Into<String>) -> Self {
                self.query = self.query.group_by(field);
                self
            }
            pub fn aggregate_count(mut self, alias: impl Into<String>)
                -> Self {
                self.query = self.query.count(alias);
                self
            }
            pub fn aggregate_count_field(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.count_field(field, alias);
                self
            }
            pub fn aggregate_with_function(mut self, field: impl Into<String>,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.query =
                    self.query.aggregate(Aggregate::new(function, field,
                            alias));
                self
            }
            pub fn aggregate_sum(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.sum(field, alias);
                self
            }
            pub fn aggregate_avg(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.avg(field, alias);
                self
            }
            pub fn aggregate_min(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.min(field, alias);
                self
            }
            pub fn aggregate_max(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.max(field, alias);
                self
            }
            pub fn aggregate_stddev(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev(field, alias);
                self
            }
            pub fn aggregate_stddev_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev_pop(field, alias);
                self
            }
            pub fn aggregate_var_samp(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_samp(field, alias);
                self
            }
            pub fn aggregate_var_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_pop(field, alias);
                self
            }
            pub fn aggregate_bit_and(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_and(field, alias);
                self
            }
            pub fn aggregate_bit_or(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_or(field, alias);
                self
            }
            pub fn aggregate_bit_xor(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_xor(field, alias);
                self
            }
            pub fn enable_aggregation_cache(mut self) -> Self {
                self.query = self.query.enable_aggregation_cache();
                self
            }
            pub fn enable_aggregation_cache_for(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.enable_aggregation_cache_for(cache_expired_millis);
                self
            }
            pub fn propagate_aggregation_cache(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.propagate_aggregation_cache(cache_expired_millis);
                self
            }
            pub fn select_id(mut self) -> Self {
                self.query = self.query.project("id");
                self
            }
            pub fn project_id(self) -> Self { self.select_id() }
            pub fn select_id_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_id_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("id",
                        raw_sql_segment));
                self
            }
            pub fn group_by_id(self) -> Self { self.group_by("id") }
            pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("id");
                request.query =
                    request.query.project_expr(alias, Expr::column("id"));
                request
            }
            pub fn group_by_id_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("id").aggregate_with_function("id", alias,
                    function)
            }
            pub fn count_id(self) -> Self { self.count_id_as("id_count") }
            pub fn count_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("id", alias)
            }
            pub fn sum_id(self) -> Self { self.sum_id_as("sum_id") }
            pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("id", alias)
            }
            pub fn avg_id(self) -> Self { self.avg_id_as("avg_id") }
            pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("id", alias)
            }
            pub fn min_id(self) -> Self { self.min_id_as("min_id") }
            pub fn min_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("id", alias)
            }
            pub fn max_id(self) -> Self { self.max_id_as("max_id") }
            pub fn max_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("id", alias)
            }
            pub fn unselect_id(mut self) -> Self {
                self.query.projection.retain(|field| field != "id");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "id");
                self
            }
            pub fn with_id(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("id", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_id_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("id", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("id", value));
                self
            }
            pub fn with_id_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("id", value));
                self
            }
            pub fn with_id_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_id_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn order_by_id_asc(mut self) -> Self {
                self.query = self.query.order_asc("id");
                self
            }
            pub fn order_by_id_desc(mut self) -> Self {
                self.query = self.query.order_desc("id");
                self
            }
            pub fn order_by_id_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("id");
                self
            }
            pub fn order_by_id_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("id");
                self
            }
            pub fn select_name(mut self) -> Self {
                self.query = self.query.project("name");
                self
            }
            pub fn project_name(self) -> Self { self.select_name() }
            pub fn select_name_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_name_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("name",
                        raw_sql_segment));
                self
            }
            pub fn group_by_name(self) -> Self { self.group_by("name") }
            pub fn group_by_name_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("name");
                request.query =
                    request.query.project_expr(alias, Expr::column("name"));
                request
            }
            pub fn group_by_name_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("name").aggregate_with_function("name", alias,
                    function)
            }
            pub fn count_name(self) -> Self {
                self.count_name_as("name_count")
            }
            pub fn count_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("name", alias)
            }
            pub fn sum_name(self) -> Self { self.sum_name_as("sum_name") }
            pub fn sum_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("name", alias)
            }
            pub fn avg_name(self) -> Self { self.avg_name_as("avg_name") }
            pub fn avg_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("name", alias)
            }
            pub fn min_name(self) -> Self { self.min_name_as("min_name") }
            pub fn min_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("name", alias)
            }
            pub fn max_name(self) -> Self { self.max_name_as("max_name") }
            pub fn max_name_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("name", alias)
            }
            pub fn unselect_name(mut self) -> Self {
                self.query.projection.retain(|field| field != "name");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "name");
                self
            }
            pub fn with_name(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("name", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_name_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("name", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_name_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("name", value));
                self
            }
            pub fn with_name_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("name", value));
                self
            }
            pub fn with_name_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("name", value));
                self
            }
            pub fn with_name_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gte("name", value));
                self
            }
            pub fn with_name_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("name", value));
                self
            }
            pub fn with_name_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lte("name", value));
                self
            }
            pub fn with_name_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("name", lower, upper));
                self
            }
            pub fn with_name_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("name", range.start,
                            range.end));
                self
            }
            pub fn with_name_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("name",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_name_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("name",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_name_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("name", value));
                self
            }
            pub fn with_name_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("name", value));
                self
            }
            pub fn with_name_starting_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("name", value));
                self
            }
            pub fn with_name_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("name", value));
                self
            }
            pub fn with_name_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("name", value));
                self
            }
            pub fn with_name_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("name", value));
                self
            }
            pub fn with_name_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("name", value));
                self
            }
            pub fn with_name_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("name", value));
                self
            }
            pub fn with_name_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("name", value));
                self
            }
            pub fn with_name_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("name"));
                self
            }
            pub fn with_name_is_known(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_not_null("name"));
                self
            }
            pub fn order_by_name_asc(mut self) -> Self {
                self.query = self.query.order_asc("name");
                self
            }
            pub fn order_by_name_desc(mut self) -> Self {
                self.query = self.query.order_desc("name");
                self
            }
            pub fn order_by_name_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("name");
                self
            }
            pub fn order_by_name_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("name");
                self
            }
            pub fn select_version(mut self) -> Self {
                self.query = self.query.project("version");
                self
            }
            pub fn project_version(self) -> Self { self.select_version() }
            pub fn select_version_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_version_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_version_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("version",
                        raw_sql_segment));
                self
            }
            pub fn group_by_version(self) -> Self { self.group_by("version") }
            pub fn group_by_version_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("version");
                request.query =
                    request.query.project_expr(alias, Expr::column("version"));
                request
            }
            pub fn group_by_version_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("version").aggregate_with_function("version",
                    alias, function)
            }
            pub fn count_version(self) -> Self {
                self.count_version_as("version_count")
            }
            pub fn count_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("version", alias)
            }
            pub fn sum_version(self) -> Self {
                self.sum_version_as("sum_version")
            }
            pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("version", alias)
            }
            pub fn avg_version(self) -> Self {
                self.avg_version_as("avg_version")
            }
            pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("version", alias)
            }
            pub fn min_version(self) -> Self {
                self.min_version_as("min_version")
            }
            pub fn min_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("version", alias)
            }
            pub fn max_version(self) -> Self {
                self.max_version_as("max_version")
            }
            pub fn max_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("version", alias)
            }
            pub fn unselect_version(mut self) -> Self {
                self.query.projection.retain(|field| field != "version");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "version");
                self
            }
            pub fn order_by_version_asc(mut self) -> Self {
                self.query = self.query.order_asc("version");
                self
            }
            pub fn order_by_version_desc(mut self) -> Self {
                self.query = self.query.order_desc("version");
                self
            }
            pub fn order_by_version_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("version");
                self
            }
            pub fn order_by_version_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("version");
                self
            }
            /// Please use `with_status_is` instead
            pub(crate) fn filter_by_status(mut self,
                value: impl EntityReference) -> Self {
                self.query =
                    self.query.and_filter(Expr::eq("status_id",
                            value.entity_id_value()));
                self
            }
            /// Complex relation filter for `status`.
            ///
            /// **Usage Priority:**
            ///
            /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
            ///    - [`Self::with_status_is_xxx`]
            ///
            ///    This gives the best code readability.
            ///
            /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
            ///
            /// # Example
            /// ```rust
            /// // Only use when building dynamic queries
            /// let dynamic_query = crate::Q::task_status_minimal().filter(...);
            /// let request = crate::Q::tasks().with_status_matching(dynamic_query);
            /// ```
            pub fn with_status_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::in_subquery("status_id",
                            <crate::TaskStatus as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "id"));
                self.relation_filters.push(RelationFilter::new("status",
                        selection));
                self
            }
            /// Complex relation filter for `status`.
            ///
            /// **Usage Priority:**
            ///
            /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
            ///    - [`Self::with_status_is_not_xxx`]
            ///
            ///    This gives the best code readability.
            ///
            /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
            ///
            /// # Example
            /// ```rust
            /// // Only use when building dynamic queries
            /// let dynamic_query = crate::Q::task_status_minimal().filter(...);
            /// let request = crate::Q::tasks().without_status_matching(dynamic_query);
            /// ```
            pub fn without_status_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::not_in_subquery("status_id",
                            <crate::TaskStatus as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "id"));
                self.relation_filters.push(RelationFilter::new("status",
                        selection));
                self
            }
            pub fn have_status(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("status_id"));
                self
            }
            pub fn have_no_status(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_null("status_id"));
                self
            }
            pub fn group_by_status(self) -> Self {
                self.group_by("status_id")
            }
            pub fn group_by_status_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("status_id");
                request.query =
                    request.query.project_expr(alias,
                        Expr::column("status_id"));
                request
            }
            pub fn group_by_status_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("status_id").aggregate_with_function("status_id",
                    alias, function)
            }
            pub fn group_by_status_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                self.query = self.query.group_by("status_id");
                self.query_options.object_group_bys.push(ObjectGroupBy::new("status",
                        "status_id", request));
                self
            }
            pub fn group_by_status_with_details(self) -> Self {
                self.group_by_status_with_details_from(crate::Q::task_status().unlimited())
            }
            pub fn group_by_status_with_details_from(self,
                request: impl Into<QuerySelection>) -> Self {
                self.group_by_status_with(request)
            }
            pub fn roll_up_to_status(self) -> Self {
                self.roll_up_to_status_with(crate::Q::task_status().unlimited())
            }
            pub fn roll_up_to_status_with(self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.with_status_matching(selection.clone()).group_by_status_with(selection)
            }
            pub fn count_status(self) -> Self {
                self.count_status_as("status_count")
            }
            pub fn count_status_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("status_id", alias)
            }
            pub fn unselect_status(mut self) -> Self {
                self.query.projection.retain(|field| field != "status_id");
                self.query.relations.retain(|relation|
                        relation.name != "status");
                self
            }
            pub fn filter_by_platform(mut self, value: impl EntityReference)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::eq("platform_id",
                            value.entity_id_value()));
                self
            }
            pub fn with_platform_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::in_subquery("platform_id",
                            <crate::Platform as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "id"));
                self.relation_filters.push(RelationFilter::new("platform",
                        selection));
                self
            }
            pub fn without_platform_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::not_in_subquery("platform_id",
                            <crate::Platform as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "id"));
                self.relation_filters.push(RelationFilter::new("platform",
                        selection));
                self
            }
            pub fn have_platform(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("platform_id"));
                self
            }
            pub fn have_no_platform(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_null("platform_id"));
                self
            }
            pub fn group_by_platform(self) -> Self {
                self.group_by("platform_id")
            }
            pub fn group_by_platform_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("platform_id");
                request.query =
                    request.query.project_expr(alias,
                        Expr::column("platform_id"));
                request
            }
            pub fn group_by_platform_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("platform_id").aggregate_with_function("platform_id",
                    alias, function)
            }
            pub fn group_by_platform_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                self.query = self.query.group_by("platform_id");
                self.query_options.object_group_bys.push(ObjectGroupBy::new("platform",
                        "platform_id", request));
                self
            }
            pub fn group_by_platform_with_details(self) -> Self {
                self.group_by_platform_with_details_from(crate::Q::platforms().unlimited())
            }
            pub fn group_by_platform_with_details_from(self,
                request: impl Into<QuerySelection>) -> Self {
                self.group_by_platform_with(request)
            }
            pub fn roll_up_to_platform(self) -> Self {
                self.roll_up_to_platform_with(crate::Q::platforms().unlimited())
            }
            pub fn roll_up_to_platform_with(self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.with_platform_matching(selection.clone()).group_by_platform_with(selection)
            }
            pub fn count_platform(self) -> Self {
                self.count_platform_as("platform_count")
            }
            pub fn count_platform_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("platform_id", alias)
            }
            pub fn unselect_platform(mut self) -> Self {
                self.query.projection.retain(|field| field != "platform_id");
                self.query.relations.retain(|relation|
                        relation.name != "platform");
                self
            }
            pub fn status_is_planned(self) -> Self {
                self.filter_by_status(1001_u64)
            }
            pub fn with_status_is_planned(self) -> Self {
                self.filter_by_status(1001_u64)
            }
            pub fn with_status_is_not_planned(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::ne("status_id", 1001_u64));
                self
            }
            pub fn status_is_process(self) -> Self {
                self.filter_by_status(1002_u64)
            }
            pub fn with_status_is_process(self) -> Self {
                self.filter_by_status(1002_u64)
            }
            pub fn with_status_is_not_process(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::ne("status_id", 1002_u64));
                self
            }
            pub fn status_is_done(self) -> Self {
                self.filter_by_status(1003_u64)
            }
            pub fn with_status_is_done(self) -> Self {
                self.filter_by_status(1003_u64)
            }
            pub fn with_status_is_not_done(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::ne("status_id", 1003_u64));
                self
            }
            pub fn select_status(mut self) -> Self {
                self.query = self.query.relation("status");
                self
            }
            pub fn select_status_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.relation_query("status",
                        selection.clone().into_query());
                self.relation_selections.push(RelationSelection::new("status",
                        selection));
                self
            }
            pub fn facet_by_status_as(self, facet_name: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                self.facet_by_status_as_with_options(facet_name, request,
                    true)
            }
            pub fn facet_by_status_as_with_options(mut self,
                facet_name: impl Into<String>,
                request: impl Into<QuerySelection>, include_all_facets: bool)
                -> Self {
                self.query_options.facets.push(FacetRequest::new(facet_name,
                        "status", request, include_all_facets));
                self
            }
            pub fn select_platform(mut self) -> Self {
                self.query = self.query.relation("platform");
                self
            }
            pub fn select_platform_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.relation_query("platform",
                        selection.clone().into_query());
                self.relation_selections.push(RelationSelection::new("platform",
                        selection));
                self
            }
            pub fn facet_by_platform_as(self, facet_name: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                self.facet_by_platform_as_with_options(facet_name, request,
                    true)
            }
            pub fn facet_by_platform_as_with_options(mut self,
                facet_name: impl Into<String>,
                request: impl Into<QuerySelection>, include_all_facets: bool)
                -> Self {
                self.query_options.facets.push(FacetRequest::new(facet_name,
                        "platform", request, include_all_facets));
                self
            }
            pub fn have_task_execution_logs(self) -> Self {
                self.with_task_execution_log_list_matching(SelectQuery::new("TaskExecutionLog"))
            }
            pub fn have_no_task_execution_logs(self) -> Self {
                self.without_task_execution_log_list_matching(SelectQuery::new("TaskExecutionLog"))
            }
            pub fn with_task_execution_log_list_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::in_subquery("id",
                            <crate::TaskExecutionLog as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "task_id"));
                self.relation_filters.push(RelationFilter::new("task_execution_log_list",
                        selection));
                self
            }
            pub fn without_task_execution_log_list_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::not_in_subquery("id",
                            <crate::TaskExecutionLog as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "task_id"));
                self.relation_filters.push(RelationFilter::new("task_execution_log_list",
                        selection));
                self
            }
            pub fn select_task_execution_log_list(mut self) -> Self {
                self.query = self.query.relation("task_execution_log_list");
                self
            }
            pub fn select_task_execution_log_list_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.relation_query("task_execution_log_list",
                        selection.clone().into_query());
                self.relation_selections.push(RelationSelection::new("task_execution_log_list",
                        selection));
                self
            }
            pub fn count_task_execution_logs(self) -> Self {
                self.count_task_execution_logs_as("count_task_execution_logs")
            }
            pub fn count_task_execution_logs_as(self,
                alias: impl Into<String>) -> Self {
                self.count_task_execution_logs_with(alias,
                    crate::Q::task_execution_logs().unlimited())
            }
            pub fn count_task_execution_logs_with(mut self,
                alias: impl Into<String>, request: impl Into<QuerySelection>)
                -> Self {
                let selection = request.into();
                self.query_options.relation_aggregates.push(RelationAggregate::new("task_execution_log_list",
                        alias, selection, true));
                self
            }
            pub fn stats_from_task_execution_logs(self,
                request: impl Into<QuerySelection>) -> Self {
                self.stats_from_task_execution_logs_as("refinements", request)
            }
            pub fn stats_from_task_execution_logs_as(mut self,
                alias: impl Into<String>, request: impl Into<QuerySelection>)
                -> Self {
                let selection = request.into();
                self.query_options.relation_aggregates.push(RelationAggregate::new("task_execution_log_list",
                        alias, selection, false));
                self
            }
            pub fn group_by_task_execution_logs_with_details(self,
                request: impl Into<QuerySelection>) -> Self {
                self.stats_from_task_execution_logs(request)
            }
        }
        impl<R> Default for TaskRequest<R> {
            fn default() -> Self { Self::new() }
        }
        impl<R> From<TaskRequest<R>> for SelectQuery {
            fn from(request: TaskRequest<R>) -> Self {
                QuerySelection::from(request).into_query()
            }
        }
        impl<R> From<TaskRequest<R>> for QuerySelection {
            fn from(request: TaskRequest<R>) -> Self {
                Self {
                    query: request.query,
                    relation_selections: request.relation_selections,
                    relation_filters: request.relation_filters,
                    child_enhancements: request.child_enhancements,
                    query_options: request.query_options,
                }
            }
        }
    }
    pub use behavior::*;
    pub use checker::*;
    pub use entity::Task;
    pub use expression::*;
    pub use request::*;
}
pub mod task_execution_log {
    mod behavior {
        use teaql_runtime::RepositoryBehavior;
        pub struct TaskExecutionLogBehavior;
        #[automatically_derived]
        impl ::core::clone::Clone for TaskExecutionLogBehavior {
            #[inline]
            fn clone(&self) -> TaskExecutionLogBehavior {
                TaskExecutionLogBehavior
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TaskExecutionLogBehavior {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f,
                    "TaskExecutionLogBehavior")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for TaskExecutionLogBehavior {
            #[inline]
            fn default() -> TaskExecutionLogBehavior {
                TaskExecutionLogBehavior {}
            }
        }
        impl RepositoryBehavior for TaskExecutionLogBehavior {}
    }
    mod checker {
        use teaql_runtime::{
            CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker,
            UserContext,
        };
        pub trait TaskExecutionLogCheckerLogic: Send + Sync {
            fn check_and_fix_task_execution_log(&self, _ctx: &UserContext,
                _entity: &mut crate::TaskExecutionLog,
                _status: CheckObjectStatus, _location: &ObjectLocation,
                _results: &mut CheckResults) {}
            fn required(&self, value: bool, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if !value {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_option<V>(&self, value: Option<&V>, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.is_none() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn required_text(&self, value: &str, field: &str,
                location: &ObjectLocation, results: &mut CheckResults) {
                if value.trim().is_empty() {
                    results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
                }
            }
            fn min_string_length(&self, value: &str, field: &str,
                min_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() < min_len {
                    results.push(teaql_runtime::CheckResult::min_str(location.clone().member(field),
                            min_len as u64, value.to_owned()));
                }
            }
            fn max_string_length(&self, value: &str, field: &str,
                max_len: usize, location: &ObjectLocation,
                results: &mut CheckResults) {
                if value.chars().count() > max_len {
                    results.push(teaql_runtime::CheckResult::max_str(location.clone().member(field),
                            max_len as u64, value.to_owned()));
                }
            }
        }
        pub struct NoopTaskExecutionLogChecker;
        #[automatically_derived]
        impl ::core::clone::Clone for NoopTaskExecutionLogChecker {
            #[inline]
            fn clone(&self) -> NoopTaskExecutionLogChecker {
                NoopTaskExecutionLogChecker
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NoopTaskExecutionLogChecker {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f,
                    "NoopTaskExecutionLogChecker")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NoopTaskExecutionLogChecker {
            #[inline]
            fn default() -> NoopTaskExecutionLogChecker {
                NoopTaskExecutionLogChecker {}
            }
        }
        impl TaskExecutionLogCheckerLogic for NoopTaskExecutionLogChecker {}
        pub struct TaskExecutionLogChecker<L = NoopTaskExecutionLogChecker> {
            logic: L,
        }
        #[automatically_derived]
        impl<L: ::core::clone::Clone> ::core::clone::Clone for
            TaskExecutionLogChecker<L> {
            #[inline]
            fn clone(&self) -> TaskExecutionLogChecker<L> {
                TaskExecutionLogChecker {
                    logic: ::core::clone::Clone::clone(&self.logic),
                }
            }
        }
        #[automatically_derived]
        impl<L: ::core::fmt::Debug> ::core::fmt::Debug for
            TaskExecutionLogChecker<L> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "TaskExecutionLogChecker", "logic", &&self.logic)
            }
        }
        impl Default for TaskExecutionLogChecker<NoopTaskExecutionLogChecker>
            {
            fn default() -> Self {
                Self { logic: NoopTaskExecutionLogChecker }
            }
        }
        impl<L> TaskExecutionLogChecker<L> where
            L: TaskExecutionLogCheckerLogic {
            pub fn new(logic: L) -> Self { Self { logic } }
        }
        impl<L> TypedChecker<crate::TaskExecutionLog> for
            TaskExecutionLogChecker<L> where L: TaskExecutionLogCheckerLogic {
            fn check_and_fix_typed(&self, ctx: &UserContext,
                entity: &mut crate::TaskExecutionLog,
                status: CheckObjectStatus, location: &ObjectLocation,
                results: &mut CheckResults) {
                self.logic.check_and_fix_task_execution_log(ctx, entity,
                    status, location, results);
            }
        }
    }
    mod entity {
        use std::collections::BTreeMap;
        use teaql_macros::TeaqlEntity;
        #[teaql(entity = "TaskExecutionLog", table =
        "task_execution_log_data")]
        pub struct TaskExecutionLog {
            #[teaql(id)]
            id: u64,
            action: String,
            detail: String,
            #[teaql(version)]
            version: i64,
            #[teaql(column = "task")]
            task_id: u64,
            #[teaql(relation(target = "Task", local_key = "task_id",
            foreign_key = "id"))]
            task: Option<crate::Task>,
            #[teaql(dynamic)]
            dynamic: BTreeMap<String, teaql_core::Value>,
            #[teaql(skip)]
            root: teaql_runtime::EntityRoot,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TaskExecutionLog {
            #[inline]
            fn clone(&self) -> TaskExecutionLog {
                TaskExecutionLog {
                    id: ::core::clone::Clone::clone(&self.id),
                    action: ::core::clone::Clone::clone(&self.action),
                    detail: ::core::clone::Clone::clone(&self.detail),
                    version: ::core::clone::Clone::clone(&self.version),
                    task_id: ::core::clone::Clone::clone(&self.task_id),
                    task: ::core::clone::Clone::clone(&self.task),
                    dynamic: ::core::clone::Clone::clone(&self.dynamic),
                    root: ::core::clone::Clone::clone(&self.root),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TaskExecutionLog {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["id", "action", "detail", "version", "task_id", "task",
                                "dynamic", "root"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.id, &self.action, &self.detail, &self.version,
                                &self.task_id, &self.task, &self.dynamic, &&self.root];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "TaskExecutionLog", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TaskExecutionLog { }
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TaskExecutionLog {
            #[inline]
            fn eq(&self, other: &TaskExecutionLog) -> bool {
                self.id == other.id && self.version == other.version &&
                                        self.task_id == other.task_id && self.action == other.action
                                && self.detail == other.detail && self.task == other.task &&
                        self.dynamic == other.dynamic && self.root == other.root
            }
        }
        impl ::teaql_core::TeaqlEntity for TaskExecutionLog {
            fn entity_descriptor() -> ::teaql_core::EntityDescriptor {
                let mut descriptor =
                    ::teaql_core::EntityDescriptor::new("TaskExecutionLog").table_name("task_execution_log_data");
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("id",
                                        ::teaql_core::DataType::U64).column_name("id").not_null().id());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("action",
                                    ::teaql_core::DataType::Text).column_name("action").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("detail",
                                    ::teaql_core::DataType::Text).column_name("detail").not_null());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("version",
                                        ::teaql_core::DataType::I64).column_name("version").not_null().version());
                descriptor =
                    descriptor.property(::teaql_core::PropertyDescriptor::new("task_id",
                                    ::teaql_core::DataType::U64).column_name("task").not_null());
                descriptor =
                    descriptor.relation(::teaql_core::RelationDescriptor::new("task",
                                    "Task").local_key("task_id").foreign_key("id"));
                descriptor
            }
        }
        impl ::teaql_core::Entity for TaskExecutionLog {
            fn from_record(record: ::teaql_core::Record)
                -> Result<Self, ::teaql_core::EntityError> {
                Ok(Self {
                        id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("TaskExecutionLog",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("TaskExecutionLog",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskExecutionLog",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "id", other))
                                                                        }))),
                                                })
                                        })()?,
                        action: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("action") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskExecutionLog",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "action", other))
                                                                        }))),
                                                })
                                        })()?,
                        detail: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("detail") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::Text(v) => v.clone(),
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskExecutionLog",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "detail", other))
                                                                        }))),
                                                })
                                        })()?,
                        version: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("version") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::I64(v) => *v,
                                                    ::teaql_core::Value::U64(v) =>
                                                        i64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("TaskExecutionLog",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: u64 out of i64 range",
                                                                                        "version"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskExecutionLog",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "version", other))
                                                                        }))),
                                                })
                                        })()?,
                        task_id: (|| -> Result<_, ::teaql_core::EntityError>
                                        {
                                            Ok(match match record.get("task_id") {
                                                        Some(::teaql_core::Value::Null) | None => {
                                                            return Ok(::core::default::Default::default());
                                                        }
                                                        Some(value) => value,
                                                    } {
                                                    ::teaql_core::Value::U64(v) => *v,
                                                    ::teaql_core::Value::I64(v) =>
                                                        u64::try_from(*v).map_err(|_|
                                                                    ::teaql_core::EntityError::new("TaskExecutionLog",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: negative i64 cannot map to u64",
                                                                                        "task_id"))
                                                                            })))?,
                                                    ::teaql_core::Value::Decimal(v) =>
                                                        ::teaql_core::Value::Decimal(*v).try_u64().ok_or_else(||
                                                                    ::teaql_core::EntityError::new("TaskExecutionLog",
                                                                        ::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(format_args!("invalid field {0}: decimal cannot map exactly to u64",
                                                                                        "task_id"))
                                                                            })))?,
                                                    other =>
                                                        return Err(::teaql_core::EntityError::new("TaskExecutionLog",
                                                                    ::alloc::__export::must_use({
                                                                            ::alloc::fmt::format(format_args!("invalid field {0}: {1:?}",
                                                                                    "task_id", other))
                                                                        }))),
                                                })
                                        })()?,
                        task: match record.get("task") {
                            Some(::teaql_core::Value::Object(record)) => {
                                Some(<crate::Task as
                                                ::teaql_core::Entity>::from_record(record.clone())?)
                            }
                            Some(::teaql_core::Value::Null) | None => None,
                            other => {
                                return Err(::teaql_core::EntityError::new("TaskExecutionLog",
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("invalid relation field {0}: {1:?}",
                                                            "task", other))
                                                })))
                            }
                        },
                        dynamic: {
                            let known_fields =
                                ["id", "action", "detail", "version", "task_id", "task"];
                            record.iter().filter(|(key, _)|
                                            !known_fields.contains(&key.as_str())).map(|(key, value)|
                                        (key.clone(), value.clone())).collect()
                        },
                        root: Default::default(),
                    })
            }
            fn into_record(self) -> ::teaql_core::Record {
                let mut record = ::teaql_core::Record::new();
                record.insert("id".to_owned(), (self.id).into());
                record.insert("action".to_owned(), (self.action).into());
                record.insert("detail".to_owned(), (self.detail).into());
                record.insert("version".to_owned(), (self.version).into());
                record.insert("task_id".to_owned(), (self.task_id).into());
                record.insert("task".to_owned(),
                    match self.task {
                        Some(entity) =>
                            ::teaql_core::Value::object(entity.into_record()),
                        None => ::teaql_core::Value::Null,
                    });
                for (key, value) in self.dynamic {
                    record.insert(key, value);
                }
                record
            }
            fn on_loaded(&mut self, _context: &dyn std::any::Any) {}
        }
        impl ::teaql_core::IdentifiableEntity for TaskExecutionLog {
            fn id_value(&self) -> ::teaql_core::Value {
                ::teaql_core::Value::U64((*&self.id).into())
            }
        }
        impl ::teaql_core::VersionedEntity for TaskExecutionLog {
            fn version(&self) -> i64 { self.version }
        }
        impl TaskExecutionLog {
            pub fn with_id(id: u64) -> teaql_core::Value {
                teaql_core::Value::U64(id)
            }
            pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot)
                -> Self {
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
            pub fn attach_root_recursive(&mut self,
                root: teaql_runtime::EntityRoot) {
                self.root = root.clone();
                if let Some(entity) = &mut self.task {
                    entity.attach_root_recursive(root.clone());
                }
            }
            pub fn id(&self) -> u64 {
                self.changed_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.id)
            }
            pub fn update_id(&mut self, value: impl Into<teaql_core::Value>)
                -> &mut Self {
                let value = value.into();
                self.id = value.try_u64().unwrap_or(self.id.clone());
                self.root.set(self.entity_key(), "id", value);
                self
            }
            pub fn changed_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "id")
            }
            pub fn action(&self) -> String {
                self.changed_action().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.action.clone())
            }
            pub fn update_action(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.action =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.action.clone());
                self.root.set(self.entity_key(), "action", value);
                self
            }
            pub fn changed_action(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "action")
            }
            pub fn detail(&self) -> String {
                self.changed_detail().and_then(|value|
                            value.try_text().map(|value|
                                    value.to_owned())).unwrap_or_else(|| self.detail.clone())
            }
            pub fn update_detail(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.detail =
                    value.try_text().map(|value|
                                value.to_owned()).unwrap_or_else(|| self.detail.clone());
                self.root.set(self.entity_key(), "detail", value);
                self
            }
            pub fn changed_detail(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "detail")
            }
            pub fn version(&self) -> i64 {
                self.changed_version().and_then(|value|
                            value.try_i64()).unwrap_or(self.version)
            }
            pub fn update_version(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.version =
                    value.try_i64().unwrap_or(self.version.clone());
                self.root.set(self.entity_key(), "version", value);
                self
            }
            pub fn changed_version(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "version")
            }
            pub fn task_id(&self) -> u64 {
                self.changed_task_id().and_then(|value|
                            value.try_u64()).unwrap_or(self.task_id)
            }
            pub fn update_task_id(&mut self,
                value: impl Into<teaql_core::Value>) -> &mut Self {
                let value = value.into();
                self.task_id =
                    value.try_u64().unwrap_or(self.task_id.clone());
                self.root.set(self.entity_key(), "task_id", value);
                self
            }
            pub fn changed_task_id(&self) -> Option<teaql_core::Value> {
                self.root.get(&self.entity_key(), "task_id")
            }
            pub fn task(&self) -> Option<&crate::Task> { self.task.as_ref() }
            pub fn mark_as_delete(&mut self) -> &mut Self {
                self.root.mark_as_delete(self.entity_key());
                self
            }
            pub fn set_comment(&mut self, comment: impl Into<String>)
                -> &mut Self {
                self.root.set_comment(comment);
                self
            }
            pub async fn save<'a, C>(self, ctx: &'a C)
                ->
                    Result<teaql_runtime::GraphNode,
                    crate::TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: crate::TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_execution_log_repository().map_err(|err|
                                teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
                crate::TeaqlEntityRepository::save_entity_graph(&repository,
                    self)
            }
        }
    }
    mod expression {
        use teaql_core::{SafeExpression, SmartList};
        pub struct TaskExecutionLogExpression<R> {
            expression: SafeExpression<R, crate::TaskExecutionLog>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            TaskExecutionLogExpression<R> {
            #[inline]
            fn clone(&self) -> TaskExecutionLogExpression<R> {
                TaskExecutionLogExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> TaskExecutionLogExpression<R> where R: Send + Sync + 'static {
            pub fn new(expression: SafeExpression<R, crate::TaskExecutionLog>)
                -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<crate::TaskExecutionLog> {
                self.expression.eval()
            }
            pub fn get_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.id())
            }
            pub fn get_action(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.action())
            }
            pub fn get_detail(self) -> SafeExpression<R, String> {
                self.expression.apply(|value| value.detail())
            }
            pub fn get_version(self) -> SafeExpression<R, i64> {
                self.expression.apply(|value| value.version())
            }
            pub fn get_task_id(self) -> SafeExpression<R, u64> {
                self.expression.apply(|value| value.task_id())
            }
            pub fn get_task(self) -> crate::TaskExpression<R> {
                crate::TaskExpression::new(self.expression.apply_optional(|value|
                            value.task().cloned()))
            }
        }
        pub struct TaskExecutionLogListExpression<R> {
            expression: SafeExpression<R, SmartList<crate::TaskExecutionLog>>,
        }
        #[automatically_derived]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for
            TaskExecutionLogListExpression<R> {
            #[inline]
            fn clone(&self) -> TaskExecutionLogListExpression<R> {
                TaskExecutionLogListExpression {
                    expression: ::core::clone::Clone::clone(&self.expression),
                }
            }
        }
        impl<R> TaskExecutionLogListExpression<R> where R: Send + Sync +
            'static {
            pub fn new(expression:
                    SafeExpression<R, SmartList<crate::TaskExecutionLog>>)
                -> Self {
                Self { expression }
            }
            pub fn eval(&self) -> Option<SmartList<crate::TaskExecutionLog>> {
                self.expression.eval()
            }
            pub fn size(self) -> SafeExpression<R, usize> {
                self.expression.size()
            }
            pub fn first(self) -> TaskExecutionLogExpression<R> {
                TaskExecutionLogExpression::new(self.expression.first())
            }
            pub fn get(self, index: usize) -> TaskExecutionLogExpression<R> {
                TaskExecutionLogExpression::new(self.expression.get(index))
            }
        }
    }
    mod request {
        use std::marker::PhantomData;
        use serde_json::Value as JsonValue;
        use teaql_core::{
            Aggregate, AggregateFunction, EntityDescriptor, Expr, Record,
            SelectQuery, SmartList,
        };
        use teaql_runtime::{RepositoryError, RuntimeError};
        use crate::request_support::*;
        impl EntityReference for crate::TaskExecutionLog {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(&self)
            }
        }
        impl EntityReference for &crate::TaskExecutionLog {
            fn entity_id_value(self) -> teaql_core::Value {
                teaql_core::IdentifiableEntity::id_value(self)
            }
        }
        pub struct TaskExecutionLogRequest<R = crate::TaskExecutionLog> {
            query: SelectQuery,
            relation_selections: Vec<RelationSelection>,
            relation_filters: Vec<RelationFilter>,
            child_enhancements: Vec<QuerySelection>,
            query_options: QueryOptions,
            marker: PhantomData<R>,
        }
        #[automatically_derived]
        impl<R: ::core::fmt::Debug> ::core::fmt::Debug for
            TaskExecutionLogRequest<R> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
                -> ::core::fmt::Result {
                let names: &'static _ =
                    &["query", "relation_selections", "relation_filters",
                                "child_enhancements", "query_options", "marker"];
                let values: &[&dyn ::core::fmt::Debug] =
                    &[&self.query, &self.relation_selections,
                                &self.relation_filters, &self.child_enhancements,
                                &self.query_options, &&self.marker];
                ::core::fmt::Formatter::debug_struct_fields_finish(f,
                    "TaskExecutionLogRequest", names, values)
            }
        }
        impl<R> Clone for TaskExecutionLogRequest<R> {
            fn clone(&self) -> Self {
                Self {
                    query: self.query.clone(),
                    relation_selections: self.relation_selections.clone(),
                    relation_filters: self.relation_filters.clone(),
                    child_enhancements: self.child_enhancements.clone(),
                    query_options: self.query_options.clone(),
                    marker: PhantomData,
                }
            }
        }
        impl<R> TaskExecutionLogRequest<R> {
            pub(crate) fn new() -> Self {
                Self {
                    query: SelectQuery::new("TaskExecutionLog"),
                    relation_selections: Vec::new(),
                    relation_filters: Vec::new(),
                    child_enhancements: Vec::new(),
                    query_options: QueryOptions::default(),
                    marker: PhantomData,
                }
            }
            pub fn return_type<T>(self) -> TaskExecutionLogRequest<T> {
                TaskExecutionLogRequest {
                    query: self.query,
                    relation_selections: self.relation_selections,
                    relation_filters: self.relation_filters,
                    child_enhancements: self.child_enhancements,
                    query_options: self.query_options,
                    marker: PhantomData,
                }
            }
            pub fn query(&self) -> &SelectQuery { &self.query }
            pub fn relation_selections(&self) -> &[RelationSelection] {
                &self.relation_selections
            }
            pub fn relation_filters(&self) -> &[RelationFilter] {
                &self.relation_filters
            }
            pub fn child_enhancements(&self) -> &[QuerySelection] {
                &self.child_enhancements
            }
            pub fn query_options(&self) -> &QueryOptions {
                &self.query_options
            }
            pub fn into_query(self) -> SelectQuery { self.query }
            pub fn new_entity<C>(&self, ctx: &C) -> crate::TaskExecutionLog
                where C: TeaqlRuntime + ?Sized {
                crate::TaskExecutionLog::runtime_new(ctx.user_context().entity_root())
            }
            pub async fn execute_for_list<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let repository =
                    ctx.task_execution_log_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_enhanced_entities_with_relation_aggregates::<R>(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_first<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let rows = self.limit(1).execute_for_list(ctx).await?;
                Ok(rows.into_iter().next())
            }
            pub async fn execute_for_one<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.execute_for_first(ctx).await
            }
            pub async fn execute_by_id<'a,
                C>(self, ctx: &'a C, id: impl Into<teaql_core::Value>)
                ->
                    Result<Option<R>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                self.and_filter(Expr::eq("id",
                                id)).execute_for_first(ctx).await
            }
            pub async fn execute_for_page<'a,
                C>(self, ctx: &'a C, offset: u64, limit: u64)
                ->
                    Result<SmartList<R>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized, R: teaql_core::Entity {
                let total_count = self.clone().execute_for_count(ctx).await?;
                let mut rows =
                    self.page_offset(offset,
                                    limit).execute_for_list(ctx).await?;
                rows.total_count = Some(total_count);
                Ok(rows)
            }
            pub async fn execute_for_count<'a, C>(self, ctx: &'a C)
                ->
                    Result<u64,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_execution_log_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query;
                query.projection.clear();
                query.expr_projection.clear();
                query.order_by.clear();
                query.slice = None;
                query.relations.clear();
                query = query.count(COUNT_ALIAS);
                let rows = repository.fetch_all(&query)?;
                rows.first().and_then(|row|
                                row.get(COUNT_ALIAS)).and_then(teaql_core::Value::try_u64).ok_or_else(||
                        RepositoryError::Runtime(RuntimeError::Graph(::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("count result for TaskExecutionLog is missing or not numeric"))
                                    }))))
            }
            pub async fn execute_for_exists<'a, C>(self, ctx: &'a C)
                ->
                    Result<bool,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_execution_log_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let mut query = self.query.limit(1);
                query.relations.clear();
                let rows = repository.fetch_all(&query)?;
                Ok(!rows.is_empty())
            }
            pub async fn execute_for_records<'a, C>(self, ctx: &'a C)
                ->
                    Result<SmartList<Record>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let repository =
                    ctx.task_execution_log_repository().map_err(|err|
                                RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
                let query_options = self.query_options.clone();
                let outer_query = self.query.clone();
                let relation_aggregates =
                    runtime_relation_aggregates(&query_options);
                let query =
                    apply_runtime_metadata(self.query, &query_options,
                        &self.child_enhancements);
                let mut rows =
                    repository.fetch_smart_list_with_relation_aggregates(&query,
                            &relation_aggregates)?;
                let facets =
                    execute_facets(ctx, &outer_query,
                                &query_options).map_err(RepositoryError::Runtime)?;
                attach_facets(&mut rows, facets);
                Ok(rows)
            }
            pub async fn execute_for_record<'a, C>(self, ctx: &'a C)
                ->
                    Result<Option<Record>,
                    TeaqlRepositoryError<C::TaskExecutionLogRepository<'a>>>
                where C: TeaqlRuntime + ?Sized {
                let records = self.limit(1).execute_for_records(ctx).await?;
                Ok(records.into_iter().next())
            }
            pub fn filter(mut self, filter: Expr) -> Self {
                self.query = self.query.filter(filter);
                self
            }
            pub fn and_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.and_filter(filter);
                self
            }
            pub fn or_filter(mut self, filter: Expr) -> Self {
                self.query = self.query.or_filter(filter);
                self
            }
            pub fn append_search_criteria(self, criteria: Expr) -> Self {
                self.and_filter(criteria)
            }
            pub fn filter_property(mut self, property1: impl AsRef<str>,
                operator: FieldOperator, property2: impl AsRef<str>) -> Self {
                self.query =
                    self.query.and_filter(field_operator_column_expr(property1.as_ref(),
                            operator, property2.as_ref()));
                self
            }
            pub fn with_deleted_rows(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self
            }
            pub fn deleted_rows_only(mut self) -> Self {
                self.query.filter =
                    remove_default_live_filter(self.query.filter);
                self.query =
                    self.query.and_filter(Expr::lte("version", 0_i64));
                self
            }
            pub fn match_types(mut self,
                types: impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list(TYPE_FIELD,
                            types.into_iter().map(Into::into)));
                self
            }
            pub fn with_type_group(mut self) -> Self {
                self.query = self.query.project(TYPE_GROUP_FIELD);
                self
            }
            pub fn matching_any_of(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                let entity =
                    EntityDescriptor::new(selection.query.entity.clone());
                self.query =
                    self.query.and_filter(Expr::in_subquery("id", entity,
                            selection.query.clone(), "id"));
                self
            }
            pub fn match_any_of(self, request: impl Into<QuerySelection>)
                -> Self {
                self.matching_any_of(request)
            }
            pub fn enhance_child(mut self, request: impl Into<QuerySelection>)
                -> Self {
                self.child_enhancements.push(request.into());
                self
            }
            pub fn enhance_children_if_needed(self) -> Self {
                let request = self;
                request
            }
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.query_options.comment = Some(comment.into());
                self
            }
            pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment)
                -> Self {
                self.query_options.raw_sql = Some(raw_sql.into_sql());
                self
            }
            pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
                self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
            }
            pub fn unsafe_raw_sql_filter(mut self,
                raw_sql: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
                self
            }
            pub fn filter_with_json(self, json_expr: impl Into<String>)
                -> Self {
                self.merge_dynamic_json_expr(json_expr.into())
            }
            fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
                let json =
                    serde_json::from_str::<JsonValue>(&json_expr).unwrap_or_else(|_|
                            {
                                ::core::panicking::panic_fmt(format_args!("Input JSON format error: {0}",
                                        json_expr));
                            });
                self.merge_dynamic_json(&json)
            }
            fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
                let Some(object) = json.as_object() else { return self; };
                for (field, value) in object {
                    if field.starts_with('_') { continue; }
                    self = self.apply_dynamic_json_filter(field, value);
                }
                self =
                    self.apply_dynamic_json_order_by(object.get("_orderBy"));
                if let Some(offset) = dynamic_json_u64_field(object, "_start")
                    {
                    self = self.skip(offset);
                }
                if let Some(size) = dynamic_json_u64_field(object, "_size") {
                    self = self.limit(size);
                }
                if let Some(page_size) =
                        dynamic_json_u64_field(object, "_pageSize") {
                    self = self.limit(page_size);
                }
                if let Some(page_number) =
                        dynamic_json_u64_field(object, "_page") {
                    if page_number > 0 {
                        let size =
                            dynamic_json_u64_field(object,
                                        "_pageSize").or_else(||
                                        self.query.slice.as_ref().and_then(|slice|
                                                slice.limit)).unwrap_or(10);
                        let offset =
                            page_number.saturating_sub(1).saturating_mul(size);
                        self = self.page_offset(offset, size);
                    }
                }
                self
            }
            pub(crate) fn apply_dynamic_json_filter(self, field: &str,
                value: &JsonValue) -> Self {
                if let Some((head, tail)) = field.split_once('.') {
                    self.apply_dynamic_json_chain_filter(head, tail, value)
                } else if let Some(storage_field) =
                        Self::dynamic_json_self_field(field) {
                    self.and_filter(dynamic_json_filter_expr(storage_field,
                            value))
                } else { self }
            }
            fn apply_dynamic_json_order_by(mut self,
                order_by: Option<&JsonValue>) -> Self {
                match order_by {
                    Some(JsonValue::String(field)) => {
                        if let Some(storage_field) =
                                Self::dynamic_json_self_field(field) {
                            self.query = self.query.order_desc(storage_field);
                        }
                    }
                    Some(JsonValue::Object(order_by)) => {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                    Some(JsonValue::Array(order_bys)) => {
                        for order_by in order_bys {
                            if let Some(order_by) = order_by.as_object() {
                                self = self.apply_dynamic_json_single_order_by(order_by);
                            }
                        }
                    }
                    _ => {}
                }
                self
            }
            fn apply_dynamic_json_single_order_by(mut self,
                order_by: &serde_json::Map<String, JsonValue>) -> Self {
                let Some(field) =
                    order_by.get("field").and_then(JsonValue::as_str) else {
                        return self;
                    };
                let Some(storage_field) =
                    Self::dynamic_json_self_field(field) else { return self; };
                if order_by.get("useAsc").and_then(JsonValue::as_bool).unwrap_or(false)
                    {
                    self.query = self.query.order_asc(storage_field);
                } else { self.query = self.query.order_desc(storage_field); }
                self
            }
            fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
                match field {
                    "id" => Some("id"),
                    "action" => Some("action"),
                    "detail" => Some("detail"),
                    "version" => Some("version"),
                    "task" | "task_id" => Some("task_id"),
                    _ => None,
                }
            }
            fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str,
                value: &JsonValue) -> Self {
                let _ = (tail, value);
                match head {
                    "task" => {
                        self.with_task_matching(crate::Q::tasks_minimal().apply_dynamic_json_filter(tail,
                                value))
                    }
                    _ => self,
                }
            }
            pub fn create_property_as(self, property_name: impl Into<String>,
                raw_sql_segment: impl Into<String>) -> Self {
                self.unsafe_create_property_as(property_name,
                    UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn unsafe_create_property_as(mut self,
                property_name: impl Into<String>,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.dynamic_properties.push(RawDynamicProperty::new(property_name,
                        raw_sql_segment));
                self
            }
            pub fn limit(mut self, limit: u64) -> Self {
                self.query = self.query.limit(limit);
                self
            }
            pub fn skip(mut self, offset: u64) -> Self {
                self.query = self.query.offset(offset);
                self
            }
            pub fn offset_only(self, offset: u64) -> Self {
                self.skip(offset)
            }
            pub fn offset(self, offset: u64, size: u64) -> Self {
                self.page_offset(offset, size)
            }
            pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
                self.query = self.query.page(offset, limit);
                self
            }
            pub fn top(self, top_n: u64) -> Self { self.limit(top_n) }
            pub fn offset_size(self, offset: u64, size: u64) -> Self {
                self.offset(offset, size)
            }
            pub fn unlimited(mut self) -> Self {
                self.query.slice = None;
                self
            }
            pub fn page_number(self, page_number: u64, page_size: u64)
                -> Self {
                let offset =
                    page_number.saturating_sub(1).saturating_mul(page_size);
                self.page_offset(offset, page_size)
            }
            pub fn page_number_default(self, page_number: u64) -> Self {
                self.page_number(page_number, 10)
            }
            pub fn page(self, page_number: u64, page_size: u64) -> Self {
                self.page_number(page_number, page_size)
            }
            pub fn page_default(self, page_number: u64) -> Self {
                self.page_number_default(page_number)
            }
            pub fn select_self(mut self) -> Self {
                self.query = self.query.project("id");
                self.query = self.query.project("action");
                self.query = self.query.project("detail");
                self.query = self.query.project("version");
                self.query = self.query.project("task_id");
                self
            }
            pub fn select_self_fields(self) -> Self { self.select_self() }
            pub fn select_self_without_parent(self) -> Self {
                self.select_self_fields()
            }
            pub fn select_all(self) -> Self {
                let mut request = self.select_self();
                request = request.select_task();
                request
            }
            pub fn select_children(self) -> Self { self.select_all() }
            pub fn select_any(self) -> Self { self.select_children() }
            pub fn group_by(mut self, field: impl Into<String>) -> Self {
                self.query = self.query.group_by(field);
                self
            }
            pub fn aggregate_count(mut self, alias: impl Into<String>)
                -> Self {
                self.query = self.query.count(alias);
                self
            }
            pub fn aggregate_count_field(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.count_field(field, alias);
                self
            }
            pub fn aggregate_with_function(mut self, field: impl Into<String>,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.query =
                    self.query.aggregate(Aggregate::new(function, field,
                            alias));
                self
            }
            pub fn aggregate_sum(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.sum(field, alias);
                self
            }
            pub fn aggregate_avg(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.avg(field, alias);
                self
            }
            pub fn aggregate_min(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.min(field, alias);
                self
            }
            pub fn aggregate_max(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.max(field, alias);
                self
            }
            pub fn aggregate_stddev(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev(field, alias);
                self
            }
            pub fn aggregate_stddev_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.stddev_pop(field, alias);
                self
            }
            pub fn aggregate_var_samp(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_samp(field, alias);
                self
            }
            pub fn aggregate_var_pop(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.var_pop(field, alias);
                self
            }
            pub fn aggregate_bit_and(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_and(field, alias);
                self
            }
            pub fn aggregate_bit_or(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_or(field, alias);
                self
            }
            pub fn aggregate_bit_xor(mut self, field: impl Into<String>,
                alias: impl Into<String>) -> Self {
                self.query = self.query.bit_xor(field, alias);
                self
            }
            pub fn enable_aggregation_cache(mut self) -> Self {
                self.query = self.query.enable_aggregation_cache();
                self
            }
            pub fn enable_aggregation_cache_for(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.enable_aggregation_cache_for(cache_expired_millis);
                self
            }
            pub fn propagate_aggregation_cache(mut self,
                cache_expired_millis: u64) -> Self {
                self.query =
                    self.query.propagate_aggregation_cache(cache_expired_millis);
                self
            }
            pub fn select_id(mut self) -> Self {
                self.query = self.query.project("id");
                self
            }
            pub fn project_id(self) -> Self { self.select_id() }
            pub fn select_id_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_id_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("id",
                        raw_sql_segment));
                self
            }
            pub fn group_by_id(self) -> Self { self.group_by("id") }
            pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("id");
                request.query =
                    request.query.project_expr(alias, Expr::column("id"));
                request
            }
            pub fn group_by_id_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("id").aggregate_with_function("id", alias,
                    function)
            }
            pub fn count_id(self) -> Self { self.count_id_as("id_count") }
            pub fn count_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("id", alias)
            }
            pub fn sum_id(self) -> Self { self.sum_id_as("sum_id") }
            pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("id", alias)
            }
            pub fn avg_id(self) -> Self { self.avg_id_as("avg_id") }
            pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("id", alias)
            }
            pub fn min_id(self) -> Self { self.min_id_as("min_id") }
            pub fn min_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("id", alias)
            }
            pub fn max_id(self) -> Self { self.max_id_as("max_id") }
            pub fn max_id_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("id", alias)
            }
            pub fn unselect_id(mut self) -> Self {
                self.query.projection.retain(|field| field != "id");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "id");
                self
            }
            pub fn with_id(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("id", operator,
                            values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_id_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("id", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>)
                -> Self {
                self.query = self.query.and_filter(Expr::eq("id", value));
                self
            }
            pub fn with_id_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("id", value));
                self
            }
            pub fn with_id_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_id_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("id",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn order_by_id_asc(mut self) -> Self {
                self.query = self.query.order_asc("id");
                self
            }
            pub fn order_by_id_desc(mut self) -> Self {
                self.query = self.query.order_desc("id");
                self
            }
            pub fn order_by_id_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("id");
                self
            }
            pub fn order_by_id_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("id");
                self
            }
            pub fn select_action(mut self) -> Self {
                self.query = self.query.project("action");
                self
            }
            pub fn project_action(self) -> Self { self.select_action() }
            pub fn select_action_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_action_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_action_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("action",
                        raw_sql_segment));
                self
            }
            pub fn group_by_action(self) -> Self { self.group_by("action") }
            pub fn group_by_action_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("action");
                request.query =
                    request.query.project_expr(alias, Expr::column("action"));
                request
            }
            pub fn group_by_action_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("action").aggregate_with_function("action",
                    alias, function)
            }
            pub fn count_action(self) -> Self {
                self.count_action_as("action_count")
            }
            pub fn count_action_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("action", alias)
            }
            pub fn sum_action(self) -> Self {
                self.sum_action_as("sum_action")
            }
            pub fn sum_action_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("action", alias)
            }
            pub fn avg_action(self) -> Self {
                self.avg_action_as("avg_action")
            }
            pub fn avg_action_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("action", alias)
            }
            pub fn min_action(self) -> Self {
                self.min_action_as("min_action")
            }
            pub fn min_action_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("action", alias)
            }
            pub fn max_action(self) -> Self {
                self.max_action_as("max_action")
            }
            pub fn max_action_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("action", alias)
            }
            pub fn unselect_action(mut self) -> Self {
                self.query.projection.retain(|field| field != "action");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "action");
                self
            }
            pub fn with_action(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("action",
                            operator, values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_action_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("action", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_action_is(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::eq("action", value));
                self
            }
            pub fn with_action_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("action", value));
                self
            }
            pub fn with_action_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("action", value));
                self
            }
            pub fn with_action_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gte("action", value));
                self
            }
            pub fn with_action_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("action", value));
                self
            }
            pub fn with_action_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lte("action", value));
                self
            }
            pub fn with_action_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("action", lower,
                            upper));
                self
            }
            pub fn with_action_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("action", range.start,
                            range.end));
                self
            }
            pub fn with_action_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("action",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_action_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("action",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_action_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("action", value));
                self
            }
            pub fn with_action_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("action", value));
                self
            }
            pub fn with_action_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("action", value));
                self
            }
            pub fn with_action_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("action",
                            value));
                self
            }
            pub fn with_action_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("action", value));
                self
            }
            pub fn with_action_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("action", value));
                self
            }
            pub fn with_action_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("action", value));
                self
            }
            pub fn with_action_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("action", value));
                self
            }
            pub fn with_action_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("action", value));
                self
            }
            pub fn with_action_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("action"));
                self
            }
            pub fn with_action_is_known(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("action"));
                self
            }
            pub fn order_by_action_asc(mut self) -> Self {
                self.query = self.query.order_asc("action");
                self
            }
            pub fn order_by_action_desc(mut self) -> Self {
                self.query = self.query.order_desc("action");
                self
            }
            pub fn order_by_action_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("action");
                self
            }
            pub fn order_by_action_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("action");
                self
            }
            pub fn select_detail(mut self) -> Self {
                self.query = self.query.project("detail");
                self
            }
            pub fn project_detail(self) -> Self { self.select_detail() }
            pub fn select_detail_raw(self, raw_sql_segment: impl Into<String>)
                -> Self {
                self.select_detail_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_detail_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("detail",
                        raw_sql_segment));
                self
            }
            pub fn group_by_detail(self) -> Self { self.group_by("detail") }
            pub fn group_by_detail_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("detail");
                request.query =
                    request.query.project_expr(alias, Expr::column("detail"));
                request
            }
            pub fn group_by_detail_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("detail").aggregate_with_function("detail",
                    alias, function)
            }
            pub fn count_detail(self) -> Self {
                self.count_detail_as("detail_count")
            }
            pub fn count_detail_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("detail", alias)
            }
            pub fn sum_detail(self) -> Self {
                self.sum_detail_as("sum_detail")
            }
            pub fn sum_detail_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("detail", alias)
            }
            pub fn avg_detail(self) -> Self {
                self.avg_detail_as("avg_detail")
            }
            pub fn avg_detail_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("detail", alias)
            }
            pub fn min_detail(self) -> Self {
                self.min_detail_as("min_detail")
            }
            pub fn min_detail_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("detail", alias)
            }
            pub fn max_detail(self) -> Self {
                self.max_detail_as("max_detail")
            }
            pub fn max_detail_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("detail", alias)
            }
            pub fn unselect_detail(mut self) -> Self {
                self.query.projection.retain(|field| field != "detail");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "detail");
                self
            }
            pub fn with_detail(mut self, operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(field_operator_expr("detail",
                            operator, values.into_iter().map(Into::into).collect()));
                self
            }
            pub fn create_detail_criteria(operator: FieldOperator,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Expr {
                field_operator_expr("detail", operator,
                    values.into_iter().map(Into::into).collect())
            }
            pub fn with_detail_is(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::eq("detail", value));
                self
            }
            pub fn with_detail_is_not(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::ne("detail", value));
                self
            }
            pub fn with_detail_greater_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("detail", value));
                self
            }
            pub fn with_detail_greater_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::gte("detail", value));
                self
            }
            pub fn with_detail_less_than(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("detail", value));
                self
            }
            pub fn with_detail_less_than_or_equal_to(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::lte("detail", value));
                self
            }
            pub fn with_detail_between(mut self,
                lower: impl Into<teaql_core::Value>,
                upper: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::between("detail", lower,
                            upper));
                self
            }
            pub fn with_detail_between_range<T>(mut self, range: DateRange<T>)
                -> Self where T: Into<teaql_core::Value> {
                self.query =
                    self.query.and_filter(Expr::between("detail", range.start,
                            range.end));
                self
            }
            pub fn with_detail_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::in_list("detail",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_detail_not_in(mut self,
                values:
                    impl IntoIterator<Item = impl Into<teaql_core::Value>>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::not_in_list("detail",
                            values.into_iter().map(Into::into)));
                self
            }
            pub fn with_detail_containing(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::contain("detail", value));
                self
            }
            pub fn with_detail_not_containing(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_contain("detail", value));
                self
            }
            pub fn with_detail_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::begin_with("detail", value));
                self
            }
            pub fn with_detail_not_starting_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_begin_with("detail",
                            value));
                self
            }
            pub fn with_detail_ending_with(mut self, value: impl Into<String>)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::end_with("detail", value));
                self
            }
            pub fn with_detail_not_ending_with(mut self,
                value: impl Into<String>) -> Self {
                self.query =
                    self.query.and_filter(Expr::not_end_with("detail", value));
                self
            }
            pub fn with_detail_sounding_like(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query =
                    self.query.and_filter(Expr::sound_like("detail", value));
                self
            }
            pub fn with_detail_before(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::lt("detail", value));
                self
            }
            pub fn with_detail_after(mut self,
                value: impl Into<teaql_core::Value>) -> Self {
                self.query = self.query.and_filter(Expr::gt("detail", value));
                self
            }
            pub fn with_detail_is_unknown(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("detail"));
                self
            }
            pub fn with_detail_is_known(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("detail"));
                self
            }
            pub fn order_by_detail_asc(mut self) -> Self {
                self.query = self.query.order_asc("detail");
                self
            }
            pub fn order_by_detail_desc(mut self) -> Self {
                self.query = self.query.order_desc("detail");
                self
            }
            pub fn order_by_detail_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("detail");
                self
            }
            pub fn order_by_detail_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("detail");
                self
            }
            pub fn select_version(mut self) -> Self {
                self.query = self.query.project("version");
                self
            }
            pub fn project_version(self) -> Self { self.select_version() }
            pub fn select_version_raw(self,
                raw_sql_segment: impl Into<String>) -> Self {
                self.select_version_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
            }
            pub fn select_version_unsafe_raw(mut self,
                raw_sql_segment: UnsafeRawSqlSegment) -> Self {
                self.query_options.raw_projections.push(RawProjection::new("version",
                        raw_sql_segment));
                self
            }
            pub fn group_by_version(self) -> Self { self.group_by("version") }
            pub fn group_by_version_as(self, alias: impl Into<String>)
                -> Self {
                let alias = alias.into();
                let mut request = self.group_by("version");
                request.query =
                    request.query.project_expr(alias, Expr::column("version"));
                request
            }
            pub fn group_by_version_with_function(self,
                alias: impl Into<String>, function: AggregateFunction)
                -> Self {
                self.group_by("version").aggregate_with_function("version",
                    alias, function)
            }
            pub fn count_version(self) -> Self {
                self.count_version_as("version_count")
            }
            pub fn count_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("version", alias)
            }
            pub fn sum_version(self) -> Self {
                self.sum_version_as("sum_version")
            }
            pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_sum("version", alias)
            }
            pub fn avg_version(self) -> Self {
                self.avg_version_as("avg_version")
            }
            pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_avg("version", alias)
            }
            pub fn min_version(self) -> Self {
                self.min_version_as("min_version")
            }
            pub fn min_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_min("version", alias)
            }
            pub fn max_version(self) -> Self {
                self.max_version_as("max_version")
            }
            pub fn max_version_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_max("version", alias)
            }
            pub fn unselect_version(mut self) -> Self {
                self.query.projection.retain(|field| field != "version");
                self.query_options.raw_projections.retain(|projection|
                        projection.property_name != "version");
                self
            }
            pub fn order_by_version_asc(mut self) -> Self {
                self.query = self.query.order_asc("version");
                self
            }
            pub fn order_by_version_desc(mut self) -> Self {
                self.query = self.query.order_desc("version");
                self
            }
            pub fn order_by_version_asc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_asc("version");
                self
            }
            pub fn order_by_version_desc_using_gbk(mut self) -> Self {
                self.query = self.query.order_gbk_desc("version");
                self
            }
            pub fn filter_by_task(mut self, value: impl EntityReference)
                -> Self {
                self.query =
                    self.query.and_filter(Expr::eq("task_id",
                            value.entity_id_value()));
                self
            }
            pub fn with_task_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::in_subquery("task_id",
                            <crate::Task as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "id"));
                self.relation_filters.push(RelationFilter::new("task",
                        selection));
                self
            }
            pub fn without_task_matching(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.and_filter(Expr::not_in_subquery("task_id",
                            <crate::Task as
                                    teaql_core::TeaqlEntity>::entity_descriptor(),
                            selection.query.clone(), "id"));
                self.relation_filters.push(RelationFilter::new("task",
                        selection));
                self
            }
            pub fn have_task(mut self) -> Self {
                self.query =
                    self.query.and_filter(Expr::is_not_null("task_id"));
                self
            }
            pub fn have_no_task(mut self) -> Self {
                self.query = self.query.and_filter(Expr::is_null("task_id"));
                self
            }
            pub fn group_by_task(self) -> Self { self.group_by("task_id") }
            pub fn group_by_task_as(self, alias: impl Into<String>) -> Self {
                let alias = alias.into();
                let mut request = self.group_by("task_id");
                request.query =
                    request.query.project_expr(alias, Expr::column("task_id"));
                request
            }
            pub fn group_by_task_with_function(self, alias: impl Into<String>,
                function: AggregateFunction) -> Self {
                self.group_by("task_id").aggregate_with_function("task_id",
                    alias, function)
            }
            pub fn group_by_task_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                self.query = self.query.group_by("task_id");
                self.query_options.object_group_bys.push(ObjectGroupBy::new("task",
                        "task_id", request));
                self
            }
            pub fn group_by_task_with_details(self) -> Self {
                self.group_by_task_with_details_from(crate::Q::tasks().unlimited())
            }
            pub fn group_by_task_with_details_from(self,
                request: impl Into<QuerySelection>) -> Self {
                self.group_by_task_with(request)
            }
            pub fn roll_up_to_task(self) -> Self {
                self.roll_up_to_task_with(crate::Q::tasks().unlimited())
            }
            pub fn roll_up_to_task_with(self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.with_task_matching(selection.clone()).group_by_task_with(selection)
            }
            pub fn count_task(self) -> Self {
                self.count_task_as("task_count")
            }
            pub fn count_task_as(self, alias: impl Into<String>) -> Self {
                self.aggregate_count_field("task_id", alias)
            }
            pub fn unselect_task(mut self) -> Self {
                self.query.projection.retain(|field| field != "task_id");
                self.query.relations.retain(|relation|
                        relation.name != "task");
                self
            }
            pub fn select_task(mut self) -> Self {
                self.query = self.query.relation("task");
                self
            }
            pub fn select_task_with(mut self,
                request: impl Into<QuerySelection>) -> Self {
                let selection = request.into();
                self.query =
                    self.query.relation_query("task",
                        selection.clone().into_query());
                self.relation_selections.push(RelationSelection::new("task",
                        selection));
                self
            }
            pub fn facet_by_task_as(self, facet_name: impl Into<String>,
                request: impl Into<QuerySelection>) -> Self {
                self.facet_by_task_as_with_options(facet_name, request, true)
            }
            pub fn facet_by_task_as_with_options(mut self,
                facet_name: impl Into<String>,
                request: impl Into<QuerySelection>, include_all_facets: bool)
                -> Self {
                self.query_options.facets.push(FacetRequest::new(facet_name,
                        "task", request, include_all_facets));
                self
            }
        }
        impl<R> Default for TaskExecutionLogRequest<R> {
            fn default() -> Self { Self::new() }
        }
        impl<R> From<TaskExecutionLogRequest<R>> for SelectQuery {
            fn from(request: TaskExecutionLogRequest<R>) -> Self {
                QuerySelection::from(request).into_query()
            }
        }
        impl<R> From<TaskExecutionLogRequest<R>> for QuerySelection {
            fn from(request: TaskExecutionLogRequest<R>) -> Self {
                Self {
                    query: request.query,
                    relation_selections: request.relation_selections,
                    relation_filters: request.relation_filters,
                    child_enhancements: request.child_enhancements,
                    query_options: request.query_options,
                }
            }
        }
    }
    pub use behavior::*;
    pub use checker::*;
    pub use entity::TaskExecutionLog;
    pub use expression::*;
    pub use request::*;
}
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use platform::*;
pub use task_status::*;
pub use task::*;
pub use task_execution_log::*;
