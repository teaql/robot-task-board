use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait AttributeCheckerLogic: Send + Sync {
    fn check_and_fix_attribute(
        &self,
        _ctx: &UserContext,
        _entity: &mut crate::Attribute,
        _status: CheckObjectStatus,
        _location: &ObjectLocation,
        _results: &mut CheckResults,
    ) {
    }

    fn required(
        &self,
        value: bool,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if !value {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn required_option<V>(
        &self,
        value: Option<&V>,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.is_none() {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn required_text(
        &self,
        value: &str,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.trim().is_empty() {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn min_string_length(
        &self,
        value: &str,
        field: &str,
        min_len: usize,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.chars().count() < min_len {
            results.push(teaql_runtime::CheckResult::min_str(
                location.clone().member(field),
                min_len as u64,
                value.to_owned(),
            ));
        }
    }

    fn max_string_length(
        &self,
        value: &str,
        field: &str,
        max_len: usize,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.chars().count() > max_len {
            results.push(teaql_runtime::CheckResult::max_str(
                location.clone().member(field),
                max_len as u64,
                value.to_owned(),
            ));
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NoopAttributeChecker;

impl AttributeCheckerLogic for NoopAttributeChecker {}

#[derive(Clone, Debug)]
pub struct AttributeChecker<L = NoopAttributeChecker> {
    logic: L,
}

impl Default for AttributeChecker<NoopAttributeChecker> {
    fn default() -> Self {
        Self {
            logic: NoopAttributeChecker,
        }
    }
}

impl<L> AttributeChecker<L>
where
    L: AttributeCheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::Attribute> for AttributeChecker<L>
where
    L: AttributeCheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        ctx: &UserContext,
        entity: &mut crate::Attribute,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        self.logic
            .check_and_fix_attribute(ctx, entity, status, location, results);
    }
}