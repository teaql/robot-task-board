package com.doublechaintech.robotkanbanservice.taskexecutionlog;

import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskRequest;
import io.teaql.data.AggrFunction;
import io.teaql.data.BaseRequest;
import io.teaql.data.PropertyReference;
import io.teaql.data.RepositoryException;
import io.teaql.data.SearchCriteria;
import io.teaql.data.SubQuerySearchCriteria;
import io.teaql.data.criteria.Operator;
import io.teaql.data.criteria.RawSql;
import io.teaql.data.criteria.TwoOperatorCriteria;

public class TaskExecutionLogRequest<T extends TaskExecutionLog> extends BaseRequest<T> {

    /**
     * @deprecated AI agents and business code must use the generated Q facade
     *             instead of constructing request builders directly.
     */
    @Deprecated
    public TaskExecutionLogRequest(Class<T> returnType){
        super(returnType);
        selectId();
        selectVersion();
    }

    public TaskExecutionLogRequest<T> comment(String comment){
         super.internalComment(comment);
         return this;
    }

    public TaskExecutionLogRequest<T> returnType(Class<? extends T> returnType){
        super.setReturnType(returnType);
        return this;
    }

    public TaskExecutionLogRequest<T> enableAggregationCache(long cacheExpiredMillis){
        super.enableAggregationCache();
        super.aggregateCacheTime(cacheExpiredMillis);
        return this;
    }

    public TaskExecutionLogRequest<T> enableAggregationCache(){
        return enableAggregationCache(0l);
    }


    public TaskExecutionLogRequest<T> propagateAggregationCache(long cacheExpiredMillis){
        super.propagateAggregationCache(cacheExpiredMillis);
        return this;
    }

    public TaskExecutionLogRequest<T> appendSearchCriteria(SearchCriteria searchCriteria){
        return (TaskExecutionLogRequest<T>)super.appendSearchCriteria(searchCriteria);
    }

    public TaskExecutionLogRequest<T> filter(String property1, Operator operator, String property2){
        return appendSearchCriteria(new TwoOperatorCriteria(operator, new PropertyReference(property1), new PropertyReference(property2)));
    }

    public TaskExecutionLogRequest<T> rawSql(String rawSql){
        super.setRawSql(rawSql);
        return this;
    }

    public TaskExecutionLogRequest<T> rawSqlFilter(String rawSql){
        return appendSearchCriteria(new RawSql(rawSql));
    }

    public TaskExecutionLogRequest<T> matchingAnyOf(TaskExecutionLogRequest taskExecutionLog){
        super.internalMatchAny(taskExecutionLog);
        return this;
    }

    public TaskExecutionLogRequest<T> enhanceChildrenIfNeeded(){
        return this;
    }

    public TaskExecutionLogRequest<T> withDeletedRows(){
        super.withDeletedRows();
        return this;
    }

    public TaskExecutionLogRequest<T> deletedRowsOnly(){
        super.deletedRowsOnly();
        return this;
    }

    public TaskExecutionLogRequest<T> selectSelf(){
        super.selectSelf();
        return selectId().selectTaskIdOnly().selectAction().selectDetail().selectVersion();
    }

    public TaskExecutionLogRequest<T> selectSelfFields(){
        return selectSelf();
    }

    public TaskExecutionLogRequest<T> selectAll(){
        super.selectAll();
        return selectId().selectTask().selectAction().selectDetail().selectVersion();
    }

    public TaskExecutionLogRequest<T> selectChildren(){
        super.selectAny();
        return selectId().selectTask().selectAction().selectDetail().selectVersion();
    }
    public TaskExecutionLogRequest<T> filterWithJson(String jsonExpr){
        super.internalFindWithJsonExpr(jsonExpr);
        return this;
    }


    public TaskExecutionLogRequest<T> selectId(){
       selectProperty(TaskExecutionLog.ID_PROPERTY);
       return this;
    }

    /**
     * fill the id with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  id) to fetch id property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskExecutionLogRequest<T> selectId(RawSql rawSqlSegment){
       selectProperty(TaskExecutionLog.ID_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskExecutionLogRequest<T> unselectId(){
       unselectProperty(TaskExecutionLog.ID_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> selectTaskIdOnly(){
       selectProperty(TaskExecutionLog.TASK_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> selectTask(){
        return selectTaskWith(Q.tasks().unlimited().selectSelf());
    }

    public TaskExecutionLogRequest<T> selectTaskWith(TaskRequest task){
       selectProperty(TaskExecutionLog.TASK_PROPERTY);
       enhanceRelation(TaskExecutionLog.TASK_PROPERTY, task);
       return this;
    }

    public TaskExecutionLogRequest<T> unselectTask(){
       unselectProperty(TaskExecutionLog.TASK_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> selectAction(){
       selectProperty(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }

    /**
     * fill the action with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  action) to fetch action property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskExecutionLogRequest<T> selectAction(RawSql rawSqlSegment){
       selectProperty(TaskExecutionLog.ACTION_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskExecutionLogRequest<T> unselectAction(){
       unselectProperty(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> selectDetail(){
       selectProperty(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }

    /**
     * fill the detail with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  detail) to fetch detail property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskExecutionLogRequest<T> selectDetail(RawSql rawSqlSegment){
       selectProperty(TaskExecutionLog.DETAIL_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskExecutionLogRequest<T> unselectDetail(){
       unselectProperty(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> selectVersion(){
       selectProperty(TaskExecutionLog.VERSION_PROPERTY);
       return this;
    }

    /**
     * fill the version with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  version) to fetch version property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskExecutionLogRequest<T> selectVersion(RawSql rawSqlSegment){
       selectProperty(TaskExecutionLog.VERSION_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskExecutionLogRequest<T> unselectVersion(){
       unselectProperty(TaskExecutionLog.VERSION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> withId(Operator operator, Object... values){
       return appendSearchCriteria(createIdCriteria(operator, values));
    }

    public SearchCriteria createIdCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskExecutionLog.ID_PROPERTY, operator, values);
    }

    public TaskExecutionLogRequest<T> withIdIs(Long id){
       return withId(Operator.EQUAL, id);
    }
    public TaskExecutionLogRequest<T> withIdIn(Long... id){
       return withId(Operator.EQUAL, (Object[])id);
    }



    public TaskExecutionLogRequest<T> filterByTask(Task... task){
      if (task == null || task.length == 0) {
        throw new RepositoryException("filterByTask parameter task cannot be empty");
      }
      return appendSearchCriteria(createTaskCriteria(Operator.EQUAL, (Object[])task));
    }

    public TaskExecutionLogRequest<T> withTask(Operator operator, Object... values){
       return appendSearchCriteria(createTaskCriteria(operator, values));
    }

    public TaskExecutionLogRequest<T> withTaskIsUnknown(){
       return withTask(Operator.IS_NULL);
    }

    public TaskExecutionLogRequest<T> withTaskIsKnown(){
       return withTask(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createTaskCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskExecutionLog.TASK_PROPERTY, operator, values);
    }

    public TaskExecutionLogRequest<T> filterByTask(Long task){
      if(task == null){
         return this;
      }
      return withTask(Operator.EQUAL, task);
    }
    public TaskExecutionLogRequest<T> withTaskMatching(TaskRequest task){
       return appendSearchCriteria(new SubQuerySearchCriteria(TaskExecutionLog.TASK_PROPERTY, task, Task.ID_PROPERTY));
    }

    public TaskExecutionLogRequest<T> filterByAction(String... action){
      if (action == null || action.length == 0) {
        throw new RepositoryException("filterByAction parameter action cannot be empty");
      }
      return appendSearchCriteria(createActionCriteria(Operator.EQUAL, (Object[])action));
    }

    public TaskExecutionLogRequest<T> withAction(Operator operator, Object... values){
       return appendSearchCriteria(createActionCriteria(operator, values));
    }

    public TaskExecutionLogRequest<T> withActionIsUnknown(){
       return withAction(Operator.IS_NULL);
    }

    public TaskExecutionLogRequest<T> withActionIsKnown(){
       return withAction(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createActionCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskExecutionLog.ACTION_PROPERTY, operator, values);
    }

    public TaskExecutionLogRequest<T> withActionGreaterThan(String action){
       return withAction(Operator.GREATER_THAN, action);
    }

    public TaskExecutionLogRequest<T> withActionGreaterThanOrEqualTo(String action){
       return withAction(Operator.GREATER_THAN_OR_EQUAL, action);
    }

    public TaskExecutionLogRequest<T> withActionLessThan(String action){
       return withAction(Operator.LESS_THAN, action);
    }

    public TaskExecutionLogRequest<T> withActionLessThanOrEqualTo(String action){
       return withAction(Operator.LESS_THAN_OR_EQUAL, action);
    }

    public TaskExecutionLogRequest<T> withActionBetween(String startOfAction, String endOfAction){
       return withAction(Operator.BETWEEN, startOfAction, endOfAction);
    }
    public TaskExecutionLogRequest<T> withActionStartingWith(String action){
       return withAction(Operator.BEGIN_WITH, action);
    }
    public TaskExecutionLogRequest<T> withActionContaining(String action){
       return withAction(Operator.CONTAIN, action);
    }

    public TaskExecutionLogRequest<T> withActionEndingWith(String action){
       return withAction(Operator.END_WITH, action);
    }

    public TaskExecutionLogRequest<T> withActionIs(String action){
       return withAction(Operator.EQUAL, action);
    }

    public TaskExecutionLogRequest<T> withActionSoundingLike(String action){
       return withAction(Operator.SOUNDS_LIKE, action);
    }



    public TaskExecutionLogRequest<T> filterByDetail(String... detail){
      if (detail == null || detail.length == 0) {
        throw new RepositoryException("filterByDetail parameter detail cannot be empty");
      }
      return appendSearchCriteria(createDetailCriteria(Operator.EQUAL, (Object[])detail));
    }

    public TaskExecutionLogRequest<T> withDetail(Operator operator, Object... values){
       return appendSearchCriteria(createDetailCriteria(operator, values));
    }

    public TaskExecutionLogRequest<T> withDetailIsUnknown(){
       return withDetail(Operator.IS_NULL);
    }

    public TaskExecutionLogRequest<T> withDetailIsKnown(){
       return withDetail(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createDetailCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskExecutionLog.DETAIL_PROPERTY, operator, values);
    }

    public TaskExecutionLogRequest<T> withDetailGreaterThan(String detail){
       return withDetail(Operator.GREATER_THAN, detail);
    }

    public TaskExecutionLogRequest<T> withDetailGreaterThanOrEqualTo(String detail){
       return withDetail(Operator.GREATER_THAN_OR_EQUAL, detail);
    }

    public TaskExecutionLogRequest<T> withDetailLessThan(String detail){
       return withDetail(Operator.LESS_THAN, detail);
    }

    public TaskExecutionLogRequest<T> withDetailLessThanOrEqualTo(String detail){
       return withDetail(Operator.LESS_THAN_OR_EQUAL, detail);
    }

    public TaskExecutionLogRequest<T> withDetailBetween(String startOfDetail, String endOfDetail){
       return withDetail(Operator.BETWEEN, startOfDetail, endOfDetail);
    }
    public TaskExecutionLogRequest<T> withDetailStartingWith(String detail){
       return withDetail(Operator.BEGIN_WITH, detail);
    }
    public TaskExecutionLogRequest<T> withDetailContaining(String detail){
       return withDetail(Operator.CONTAIN, detail);
    }

    public TaskExecutionLogRequest<T> withDetailEndingWith(String detail){
       return withDetail(Operator.END_WITH, detail);
    }

    public TaskExecutionLogRequest<T> withDetailIs(String detail){
       return withDetail(Operator.EQUAL, detail);
    }

    public TaskExecutionLogRequest<T> withDetailSoundingLike(String detail){
       return withDetail(Operator.SOUNDS_LIKE, detail);
    }



    public TaskExecutionLogRequest<T> filterByVersion(Long... version){
      if (version == null || version.length == 0) {
        throw new RepositoryException("filterByVersion parameter version cannot be empty");
      }
      return appendSearchCriteria(createVersionCriteria(Operator.EQUAL, (Object[])version));
    }

    public TaskExecutionLogRequest<T> withVersion(Operator operator, Object... values){
       return appendSearchCriteria(createVersionCriteria(operator, values));
    }

    public TaskExecutionLogRequest<T> withVersionIsUnknown(){
       return withVersion(Operator.IS_NULL);
    }

    public TaskExecutionLogRequest<T> withVersionIsKnown(){
       return withVersion(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createVersionCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskExecutionLog.VERSION_PROPERTY, operator, values);
    }

    public TaskExecutionLogRequest<T> withVersionGreaterThan(Long version){
       return withVersion(Operator.GREATER_THAN, version);
    }

    public TaskExecutionLogRequest<T> withVersionGreaterThanOrEqualTo(Long version){
       return withVersion(Operator.GREATER_THAN_OR_EQUAL, version);
    }

    public TaskExecutionLogRequest<T> withVersionLessThan(Long version){
       return withVersion(Operator.LESS_THAN, version);
    }

    public TaskExecutionLogRequest<T> withVersionLessThanOrEqualTo(Long version){
       return withVersion(Operator.LESS_THAN_OR_EQUAL, version);
    }

    public TaskExecutionLogRequest<T> withVersionBetween(Long startOfVersion, Long endOfVersion){
       return withVersion(Operator.BETWEEN, startOfVersion, endOfVersion);
    }


    public TaskExecutionLogRequest<T> count(){
        super.count();
        return this;
    }
    public TaskExecutionLogRequest<T> countAs(String retName){
        super.count(retName);
        return this;
    }
    public TaskExecutionLogRequest<T> groupByTaskWithDetails(){
       return groupByTaskWithDetails(Q.tasks().unlimited());
    }

    public TaskExecutionLogRequest<T> groupByTaskWithDetails(TaskRequest subRequest){
       aggregate(TaskExecutionLog.TASK_PROPERTY, subRequest);
       return this;
    }





    public TaskExecutionLogRequest<T> groupById(){
       groupBy(TaskExecutionLog.ID_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByIdAs(String retName){
       groupBy(retName, TaskExecutionLog.ID_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByIdWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskExecutionLog.ID_PROPERTY, function);
       return this;
    }
    public TaskExecutionLogRequest<T> groupByTaskWith(TaskRequest subRequest){
       groupBy(TaskExecutionLog.TASK_PROPERTY, subRequest);
       return this;
    }
    public TaskExecutionLogRequest<T> groupByTask(){
       groupBy(TaskExecutionLog.TASK_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByTaskAs(String retName){
       groupBy(retName, TaskExecutionLog.TASK_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByTaskWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskExecutionLog.TASK_PROPERTY, function);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByAction(){
       groupBy(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByActionAs(String retName){
       groupBy(retName, TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByActionWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskExecutionLog.ACTION_PROPERTY, function);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByDetail(){
       groupBy(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByDetailAs(String retName){
       groupBy(retName, TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByDetailWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskExecutionLog.DETAIL_PROPERTY, function);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByVersion(){
       groupBy(TaskExecutionLog.VERSION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByVersionAs(String retName){
       groupBy(retName, TaskExecutionLog.VERSION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> groupByVersionWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskExecutionLog.VERSION_PROPERTY, function);
       return this;
    }



    public TaskExecutionLogRequest<T> orderByIdAscending(){
       addOrderByAscending(TaskExecutionLog.ID_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByIdDescending(){
       addOrderByDescending(TaskExecutionLog.ID_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByTaskAscending(){
       addOrderByAscending(TaskExecutionLog.TASK_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByTaskDescending(){
       addOrderByDescending(TaskExecutionLog.TASK_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByActionAscending(){
       addOrderByAscending(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByActionDescending(){
       addOrderByDescending(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> orderByActionAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByActionDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(TaskExecutionLog.ACTION_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> orderByDetailAscending(){
       addOrderByAscending(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByDetailDescending(){
       addOrderByDescending(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> orderByDetailAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByDetailDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(TaskExecutionLog.DETAIL_PROPERTY);
       return this;
    }
    public TaskExecutionLogRequest<T> orderByVersionAscending(){
       addOrderByAscending(TaskExecutionLog.VERSION_PROPERTY);
       return this;
    }

    public TaskExecutionLogRequest<T> orderByVersionDescending(){
       addOrderByDescending(TaskExecutionLog.VERSION_PROPERTY);
       return this;
    }


    public TaskRequest rollUpToTask(){
       TaskRequest task = Q.tasks().unlimited();
       this.withTaskMatching(task)
           .groupByTaskWith(task);
       return task;
    }





   public TaskExecutionLogRequest<T> facetByTaskAs(String facetName, TaskRequest task){
       return facetByTaskAs(facetName, task, true);
   }

   public TaskExecutionLogRequest<T> facetByTaskAs(String facetName, TaskRequest task, boolean includeAllFacets){
       addFacet(facetName, TaskExecutionLog.TASK_PROPERTY, task, includeAllFacets);
       return this;
   }

    public TaskExecutionLogRequest<T> createPropertyAs(String propertyName, RawSql rawSqlSegment){
        super.addSimpleDynamicProperty(propertyName, rawSqlSegment);
        return this;
    }

    /**
     * get topN records
     * @param topN  records number
     */
    public TaskExecutionLogRequest<T> top(int topN) {
        super.top(topN);
        return this;
    }

    /**
     * get records from offset(inclusive) to offset+size(exclusive)
     * @param offset record offset
     * @param size records number
     */
    public TaskExecutionLogRequest<T> offset(int offset, int size) {
        super.offset(offset, size);
        return this;
    }

    /**
     * retrieve all records
     */
    public TaskExecutionLogRequest<T> unlimited() {
        super.unlimited();
        return this;
    }

    /**
     * get records of one page
     * @param pageNumber page number(1-based)
     * @param pageSize page size
     */
    public TaskExecutionLogRequest<T> page(int pageNumber, int pageSize) {
        int offset = (pageNumber - 1) * pageSize;
        return offset(offset, pageSize);
   }

    /**
     * get records of one page, default page size is 10
     * @param pageNumber page number(1-based)
     */
    public TaskExecutionLogRequest<T> page(int pageNumber) {
        return page(pageNumber, 10);
   }
}