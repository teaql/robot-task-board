package com.doublechaintech.robotkanbanservice.task;

import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.platform.PlatformRequest;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest;
import io.teaql.data.AggrFunction;
import io.teaql.data.BaseRequest;
import io.teaql.data.PropertyReference;
import io.teaql.data.RepositoryException;
import io.teaql.data.SearchCriteria;
import io.teaql.data.SubQuerySearchCriteria;
import io.teaql.data.criteria.Operator;
import io.teaql.data.criteria.RawSql;
import io.teaql.data.criteria.TwoOperatorCriteria;

public class TaskRequest<T extends Task> extends BaseRequest<T> {

    /**
     * @deprecated AI agents and business code must use the generated Q facade
     *             instead of constructing request builders directly.
     */
    @Deprecated
    public TaskRequest(Class<T> returnType){
        super(returnType);
        selectId();
        selectVersion();
    }

    public TaskRequest<T> comment(String comment){
         super.internalComment(comment);
         return this;
    }

    public TaskRequest<T> returnType(Class<? extends T> returnType){
        super.setReturnType(returnType);
        return this;
    }

    public TaskRequest<T> enableAggregationCache(long cacheExpiredMillis){
        super.enableAggregationCache();
        super.aggregateCacheTime(cacheExpiredMillis);
        return this;
    }

    public TaskRequest<T> enableAggregationCache(){
        return enableAggregationCache(0l);
    }


    public TaskRequest<T> propagateAggregationCache(long cacheExpiredMillis){
        super.propagateAggregationCache(cacheExpiredMillis);
        return this;
    }

    public TaskRequest<T> appendSearchCriteria(SearchCriteria searchCriteria){
        return (TaskRequest<T>)super.appendSearchCriteria(searchCriteria);
    }

    public TaskRequest<T> filter(String property1, Operator operator, String property2){
        return appendSearchCriteria(new TwoOperatorCriteria(operator, new PropertyReference(property1), new PropertyReference(property2)));
    }

    public TaskRequest<T> rawSql(String rawSql){
        super.setRawSql(rawSql);
        return this;
    }

    public TaskRequest<T> rawSqlFilter(String rawSql){
        return appendSearchCriteria(new RawSql(rawSql));
    }

    public TaskRequest<T> matchingAnyOf(TaskRequest task){
        super.internalMatchAny(task);
        return this;
    }

    public TaskRequest<T> enhanceChildrenIfNeeded(){
        return this;
    }

    public TaskRequest<T> withDeletedRows(){
        super.withDeletedRows();
        return this;
    }

    public TaskRequest<T> deletedRowsOnly(){
        super.deletedRowsOnly();
        return this;
    }

    public TaskRequest<T> selectSelf(){
        super.selectSelf();
        return selectId().selectName().selectStatusIdOnly().selectPlatformIdOnly().selectVersion();
    }

    public TaskRequest<T> selectSelfFields(){
        return selectSelf();
    }

    public TaskRequest<T> selectAll(){
        super.selectAll();
        return selectId().selectName().selectStatus().selectPlatform().selectVersion();
    }

    public TaskRequest<T> selectChildren(){
        super.selectAny();
        selectTaskExecutionLogList();
        return selectId().selectName().selectStatus().selectPlatform().selectVersion();
    }
    public TaskRequest<T> filterWithJson(String jsonExpr){
        super.internalFindWithJsonExpr(jsonExpr);
        return this;
    }


    public TaskRequest<T> selectId(){
       selectProperty(Task.ID_PROPERTY);
       return this;
    }

    /**
     * fill the id with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  id) to fetch id property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskRequest<T> selectId(RawSql rawSqlSegment){
       selectProperty(Task.ID_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskRequest<T> unselectId(){
       unselectProperty(Task.ID_PROPERTY);
       return this;
    }
    public TaskRequest<T> selectName(){
       selectProperty(Task.NAME_PROPERTY);
       return this;
    }

    /**
     * fill the name with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  name) to fetch name property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskRequest<T> selectName(RawSql rawSqlSegment){
       selectProperty(Task.NAME_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskRequest<T> unselectName(){
       unselectProperty(Task.NAME_PROPERTY);
       return this;
    }
    public TaskRequest<T> selectStatusIdOnly(){
       selectProperty(Task.STATUS_PROPERTY);
       return this;
    }

    public TaskRequest<T> selectStatus(){
        return selectStatusWith(Q.taskStatuses().unlimited().selectSelf());
    }

    public TaskRequest<T> selectStatusWith(TaskStatusRequest status){
       selectProperty(Task.STATUS_PROPERTY);
       enhanceRelation(Task.STATUS_PROPERTY, status);
       return this;
    }

    public TaskRequest<T> unselectStatus(){
       unselectProperty(Task.STATUS_PROPERTY);
       return this;
    }
    public TaskRequest<T> selectPlatformIdOnly(){
       selectProperty(Task.PLATFORM_PROPERTY);
       return this;
    }

    public TaskRequest<T> selectPlatform(){
        return selectPlatformWith(Q.platforms().unlimited().selectSelf());
    }

    public TaskRequest<T> selectPlatformWith(PlatformRequest platform){
       selectProperty(Task.PLATFORM_PROPERTY);
       enhanceRelation(Task.PLATFORM_PROPERTY, platform);
       return this;
    }

    public TaskRequest<T> unselectPlatform(){
       unselectProperty(Task.PLATFORM_PROPERTY);
       return this;
    }
    public TaskRequest<T> selectVersion(){
       selectProperty(Task.VERSION_PROPERTY);
       return this;
    }

    /**
     * fill the version with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  version) to fetch version property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskRequest<T> selectVersion(RawSql rawSqlSegment){
       selectProperty(Task.VERSION_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskRequest<T> unselectVersion(){
       unselectProperty(Task.VERSION_PROPERTY);
       return this;
    }
    public TaskRequest<T> selectTaskExecutionLogList(){
       return selectTaskExecutionLogListWith(Q.taskExecutionLogs().selectSelf());
    }

    public TaskRequest<T> selectTaskExecutionLogListWith(TaskExecutionLogRequest taskExecutionLogList){
       enhanceRelation(Task.TASK_EXECUTION_LOG_LIST_PROPERTY, taskExecutionLogList);
       return this;
    }

    public TaskRequest<T> withId(Operator operator, Object... values){
       return appendSearchCriteria(createIdCriteria(operator, values));
    }

    public SearchCriteria createIdCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Task.ID_PROPERTY, operator, values);
    }

    public TaskRequest<T> withIdIs(Long id){
       return withId(Operator.EQUAL, id);
    }
    public TaskRequest<T> withIdIn(Long... id){
       return withId(Operator.EQUAL, (Object[])id);
    }



    public TaskRequest<T> filterByName(String... name){
      if (name == null || name.length == 0) {
        throw new RepositoryException("filterByName parameter name cannot be empty");
      }
      return appendSearchCriteria(createNameCriteria(Operator.EQUAL, (Object[])name));
    }

    public TaskRequest<T> withName(Operator operator, Object... values){
       return appendSearchCriteria(createNameCriteria(operator, values));
    }

    public TaskRequest<T> withNameIsUnknown(){
       return withName(Operator.IS_NULL);
    }

    public TaskRequest<T> withNameIsKnown(){
       return withName(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createNameCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Task.NAME_PROPERTY, operator, values);
    }

    public TaskRequest<T> withNameGreaterThan(String name){
       return withName(Operator.GREATER_THAN, name);
    }

    public TaskRequest<T> withNameGreaterThanOrEqualTo(String name){
       return withName(Operator.GREATER_THAN_OR_EQUAL, name);
    }

    public TaskRequest<T> withNameLessThan(String name){
       return withName(Operator.LESS_THAN, name);
    }

    public TaskRequest<T> withNameLessThanOrEqualTo(String name){
       return withName(Operator.LESS_THAN_OR_EQUAL, name);
    }

    public TaskRequest<T> withNameBetween(String startOfName, String endOfName){
       return withName(Operator.BETWEEN, startOfName, endOfName);
    }
    public TaskRequest<T> withNameStartingWith(String name){
       return withName(Operator.BEGIN_WITH, name);
    }
    public TaskRequest<T> withNameContaining(String name){
       return withName(Operator.CONTAIN, name);
    }

    public TaskRequest<T> withNameEndingWith(String name){
       return withName(Operator.END_WITH, name);
    }

    public TaskRequest<T> withNameIs(String name){
       return withName(Operator.EQUAL, name);
    }

    public TaskRequest<T> withNameSoundingLike(String name){
       return withName(Operator.SOUNDS_LIKE, name);
    }



    public TaskRequest<T> filterByStatus(TaskStatus... status){
      if (status == null || status.length == 0) {
        throw new RepositoryException("filterByStatus parameter status cannot be empty");
      }
      return appendSearchCriteria(createStatusCriteria(Operator.EQUAL, (Object[])status));
    }

    public TaskRequest<T> withStatus(Operator operator, Object... values){
       return appendSearchCriteria(createStatusCriteria(operator, values));
    }

    public TaskRequest<T> withStatusIsUnknown(){
       return withStatus(Operator.IS_NULL);
    }

    public TaskRequest<T> withStatusIsKnown(){
       return withStatus(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createStatusCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Task.STATUS_PROPERTY, operator, values);
    }

    public TaskRequest<T> filterByStatus(Long status){
      if(status == null){
         return this;
      }
      return withStatus(Operator.EQUAL, status);
    }
    public TaskRequest<T> withStatusMatching(TaskStatusRequest status){
       return appendSearchCriteria(new SubQuerySearchCriteria(Task.STATUS_PROPERTY, status, TaskStatus.ID_PROPERTY));
    }

    public TaskRequest<T> filterByPlatform(Platform... platform){
      if (platform == null || platform.length == 0) {
        throw new RepositoryException("filterByPlatform parameter platform cannot be empty");
      }
      return appendSearchCriteria(createPlatformCriteria(Operator.EQUAL, (Object[])platform));
    }

    public TaskRequest<T> withPlatform(Operator operator, Object... values){
       return appendSearchCriteria(createPlatformCriteria(operator, values));
    }

    public TaskRequest<T> withPlatformIsUnknown(){
       return withPlatform(Operator.IS_NULL);
    }

    public TaskRequest<T> withPlatformIsKnown(){
       return withPlatform(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createPlatformCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Task.PLATFORM_PROPERTY, operator, values);
    }

    public TaskRequest<T> filterByPlatform(Long platform){
      if(platform == null){
         return this;
      }
      return withPlatform(Operator.EQUAL, platform);
    }
    public TaskRequest<T> withPlatformMatching(PlatformRequest platform){
       return appendSearchCriteria(new SubQuerySearchCriteria(Task.PLATFORM_PROPERTY, platform, Platform.ID_PROPERTY));
    }

    public TaskRequest<T> filterByVersion(Long... version){
      if (version == null || version.length == 0) {
        throw new RepositoryException("filterByVersion parameter version cannot be empty");
      }
      return appendSearchCriteria(createVersionCriteria(Operator.EQUAL, (Object[])version));
    }

    public TaskRequest<T> withVersion(Operator operator, Object... values){
       return appendSearchCriteria(createVersionCriteria(operator, values));
    }

    public TaskRequest<T> withVersionIsUnknown(){
       return withVersion(Operator.IS_NULL);
    }

    public TaskRequest<T> withVersionIsKnown(){
       return withVersion(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createVersionCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Task.VERSION_PROPERTY, operator, values);
    }

    public TaskRequest<T> withVersionGreaterThan(Long version){
       return withVersion(Operator.GREATER_THAN, version);
    }

    public TaskRequest<T> withVersionGreaterThanOrEqualTo(Long version){
       return withVersion(Operator.GREATER_THAN_OR_EQUAL, version);
    }

    public TaskRequest<T> withVersionLessThan(Long version){
       return withVersion(Operator.LESS_THAN, version);
    }

    public TaskRequest<T> withVersionLessThanOrEqualTo(Long version){
       return withVersion(Operator.LESS_THAN_OR_EQUAL, version);
    }

    public TaskRequest<T> withVersionBetween(Long startOfVersion, Long endOfVersion){
       return withVersion(Operator.BETWEEN, startOfVersion, endOfVersion);
    }

    public TaskRequest<T> withTaskExecutionLogListMatching(TaskExecutionLogRequest taskExecutionLogRequest){
        return appendSearchCriteria(new SubQuerySearchCriteria(Task.ID_PROPERTY, taskExecutionLogRequest, TaskExecutionLog.TASK_PROPERTY));
    }

    public TaskRequest<T> withoutTaskExecutionLogListMatching(TaskExecutionLogRequest taskExecutionLogRequest){
        return appendSearchCriteria(SearchCriteria.not(new SubQuerySearchCriteria(Task.ID_PROPERTY, taskExecutionLogRequest, TaskExecutionLog.TASK_PROPERTY)));
    }

    public TaskRequest<T> haveTaskExecutionLogs(){
        return withTaskExecutionLogListMatching(Q.taskExecutionLogs().unlimited());
    }

    public TaskRequest<T> haveNoTaskExecutionLogs(){
        return withoutTaskExecutionLogListMatching(Q.taskExecutionLogs().unlimited());
    }

    public TaskRequest<T> count(){
        super.count();
        return this;
    }
    public TaskRequest<T> countAs(String retName){
        super.count(retName);
        return this;
    }
    public TaskRequest<T> groupByStatusWithDetails(){
       return groupByStatusWithDetails(Q.taskStatuses().unlimited());
    }

    public TaskRequest<T> groupByStatusWithDetails(TaskStatusRequest subRequest){
       aggregate(Task.STATUS_PROPERTY, subRequest);
       return this;
    }

    public TaskRequest<T> groupByPlatformWithDetails(){
       return groupByPlatformWithDetails(Q.platforms().unlimited());
    }

    public TaskRequest<T> groupByPlatformWithDetails(PlatformRequest subRequest){
       aggregate(Task.PLATFORM_PROPERTY, subRequest);
       return this;
    }


    public TaskRequest<T> groupByTaskExecutionLogsWithDetails(TaskExecutionLogRequest subRequest){
       aggregate(Task.TASK_EXECUTION_LOG_LIST_PROPERTY, subRequest);
       return this;
    }

    public TaskRequest<T> groupById(){
       groupBy(Task.ID_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByIdAs(String retName){
       groupBy(retName, Task.ID_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByIdWithFunction(String retName, AggrFunction function){
       groupBy(retName, Task.ID_PROPERTY, function);
       return this;
    }

    public TaskRequest<T> groupByName(){
       groupBy(Task.NAME_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByNameAs(String retName){
       groupBy(retName, Task.NAME_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByNameWithFunction(String retName, AggrFunction function){
       groupBy(retName, Task.NAME_PROPERTY, function);
       return this;
    }
    public TaskRequest<T> groupByStatusWith(TaskStatusRequest subRequest){
       groupBy(Task.STATUS_PROPERTY, subRequest);
       return this;
    }
    public TaskRequest<T> groupByStatus(){
       groupBy(Task.STATUS_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByStatusAs(String retName){
       groupBy(retName, Task.STATUS_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByStatusWithFunction(String retName, AggrFunction function){
       groupBy(retName, Task.STATUS_PROPERTY, function);
       return this;
    }
    public TaskRequest<T> groupByPlatformWith(PlatformRequest subRequest){
       groupBy(Task.PLATFORM_PROPERTY, subRequest);
       return this;
    }
    public TaskRequest<T> groupByPlatform(){
       groupBy(Task.PLATFORM_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByPlatformAs(String retName){
       groupBy(retName, Task.PLATFORM_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByPlatformWithFunction(String retName, AggrFunction function){
       groupBy(retName, Task.PLATFORM_PROPERTY, function);
       return this;
    }

    public TaskRequest<T> groupByVersion(){
       groupBy(Task.VERSION_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByVersionAs(String retName){
       groupBy(retName, Task.VERSION_PROPERTY);
       return this;
    }

    public TaskRequest<T> groupByVersionWithFunction(String retName, AggrFunction function){
       groupBy(retName, Task.VERSION_PROPERTY, function);
       return this;
    }

    public TaskRequest<T> withStatusIsPlanned(){
       filterByStatus(com.doublechaintech.robotkanbanservice.Constants.TASK_STATUS_PLANNED);
       return this;
    }


    public TaskRequest<T> withStatusIsReady(){
       filterByStatus(com.doublechaintech.robotkanbanservice.Constants.TASK_STATUS_READY);
       return this;
    }


    public TaskRequest<T> withStatusIsExecuting(){
       filterByStatus(com.doublechaintech.robotkanbanservice.Constants.TASK_STATUS_EXECUTING);
       return this;
    }


    public TaskRequest<T> withStatusIsVerified(){
       filterByStatus(com.doublechaintech.robotkanbanservice.Constants.TASK_STATUS_VERIFIED);
       return this;
    }




    public TaskRequest<T> orderByIdAscending(){
       addOrderByAscending(Task.ID_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByIdDescending(){
       addOrderByDescending(Task.ID_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByNameAscending(){
       addOrderByAscending(Task.NAME_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByNameDescending(){
       addOrderByDescending(Task.NAME_PROPERTY);
       return this;
    }
    public TaskRequest<T> orderByNameAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(Task.NAME_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByNameDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(Task.NAME_PROPERTY);
       return this;
    }
    public TaskRequest<T> orderByStatusAscending(){
       addOrderByAscending(Task.STATUS_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByStatusDescending(){
       addOrderByDescending(Task.STATUS_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByPlatformAscending(){
       addOrderByAscending(Task.PLATFORM_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByPlatformDescending(){
       addOrderByDescending(Task.PLATFORM_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByVersionAscending(){
       addOrderByAscending(Task.VERSION_PROPERTY);
       return this;
    }

    public TaskRequest<T> orderByVersionDescending(){
       addOrderByDescending(Task.VERSION_PROPERTY);
       return this;
    }


    public TaskRequest<T> statsFromTaskExecutionLogsAs(String name, TaskExecutionLogRequest subRequest){
       return statsFromTaskExecutionLogsAs(name, subRequest, false);
    }

    public TaskRequest<T> statsFromTaskExecutionLogsAs(String name, TaskExecutionLogRequest subRequest, boolean singleResult){
       subRequest.setPartitionProperty(TaskExecutionLog.TASK_PROPERTY);
       addAggregateDynamicProperty(name, subRequest, singleResult);
       return this;
    }

    public TaskRequest<T> statsFromTaskExecutionLogs(TaskExecutionLogRequest subRequest){
       return statsFromTaskExecutionLogsAs(REFINEMENTS, subRequest);
    }
    public TaskStatusRequest rollUpToStatus(){
       TaskStatusRequest status = Q.taskStatuses().unlimited();
       this.withStatusMatching(status)
           .groupByStatusWith(status);
       return status;
    }

    public PlatformRequest rollUpToPlatform(){
       PlatformRequest platform = Q.platforms().unlimited();
       this.withPlatformMatching(platform)
           .groupByPlatformWith(platform);
       return platform;
    }


    public TaskRequest<T> countTaskExecutionLogs(){
        return countTaskExecutionLogsAs("Count");
    }

    public TaskRequest<T> countTaskExecutionLogsAs(String name){
        return countTaskExecutionLogsWith(name, Q.taskExecutionLogs().unlimited());
    }

    public TaskRequest<T> countTaskExecutionLogsWith(String name, TaskExecutionLogRequest subRequest){
        return statsFromTaskExecutionLogsAs(name, subRequest.count(), true);
    }

   public TaskRequest<T> facetByStatusAs(String facetName, TaskStatusRequest status){
       return facetByStatusAs(facetName, status, true);
   }

   public TaskRequest<T> facetByStatusAs(String facetName, TaskStatusRequest status, boolean includeAllFacets){
       addFacet(facetName, Task.STATUS_PROPERTY, status, includeAllFacets);
       return this;
   }
   public TaskRequest<T> facetByPlatformAs(String facetName, PlatformRequest platform){
       return facetByPlatformAs(facetName, platform, true);
   }

   public TaskRequest<T> facetByPlatformAs(String facetName, PlatformRequest platform, boolean includeAllFacets){
       addFacet(facetName, Task.PLATFORM_PROPERTY, platform, includeAllFacets);
       return this;
   }

    public TaskRequest<T> createPropertyAs(String propertyName, RawSql rawSqlSegment){
        super.addSimpleDynamicProperty(propertyName, rawSqlSegment);
        return this;
    }

    /**
     * get topN records
     * @param topN  records number
     */
    public TaskRequest<T> top(int topN) {
        super.top(topN);
        return this;
    }

    /**
     * get records from offset(inclusive) to offset+size(exclusive)
     * @param offset record offset
     * @param size records number
     */
    public TaskRequest<T> offset(int offset, int size) {
        super.offset(offset, size);
        return this;
    }

    /**
     * retrieve all records
     */
    public TaskRequest<T> unlimited() {
        super.unlimited();
        return this;
    }

    /**
     * get records of one page
     * @param pageNumber page number(1-based)
     * @param pageSize page size
     */
    public TaskRequest<T> page(int pageNumber, int pageSize) {
        int offset = (pageNumber - 1) * pageSize;
        return offset(offset, pageSize);
   }

    /**
     * get records of one page, default page size is 10
     * @param pageNumber page number(1-based)
     */
    public TaskRequest<T> page(int pageNumber) {
        return page(pageNumber, 10);
   }
}