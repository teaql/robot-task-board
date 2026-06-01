package com.doublechaintech.robotkanbanservice.platform;

import com.doublechain.common.DateRange;
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
import java.time.LocalDateTime;
import java.util.Date;

public class PlatformRequest<T extends Platform> extends BaseRequest<T> {

    /**
     * @deprecated AI agents and business code must use the generated Q facade
     *             instead of constructing request builders directly.
     */
    @Deprecated
    public PlatformRequest(Class<T> returnType){
        super(returnType);
        selectId();
        selectVersion();
    }

    public PlatformRequest<T> comment(String comment){
         super.internalComment(comment);
         return this;
    }

    public PlatformRequest<T> returnType(Class<? extends T> returnType){
        super.setReturnType(returnType);
        return this;
    }

    public PlatformRequest<T> enableAggregationCache(long cacheExpiredMillis){
        super.enableAggregationCache();
        super.aggregateCacheTime(cacheExpiredMillis);
        return this;
    }

    public PlatformRequest<T> enableAggregationCache(){
        return enableAggregationCache(0l);
    }


    public PlatformRequest<T> propagateAggregationCache(long cacheExpiredMillis){
        super.propagateAggregationCache(cacheExpiredMillis);
        return this;
    }

    public PlatformRequest<T> appendSearchCriteria(SearchCriteria searchCriteria){
        return (PlatformRequest<T>)super.appendSearchCriteria(searchCriteria);
    }

    public PlatformRequest<T> filter(String property1, Operator operator, String property2){
        return appendSearchCriteria(new TwoOperatorCriteria(operator, new PropertyReference(property1), new PropertyReference(property2)));
    }

    public PlatformRequest<T> rawSql(String rawSql){
        super.setRawSql(rawSql);
        return this;
    }

    public PlatformRequest<T> rawSqlFilter(String rawSql){
        return appendSearchCriteria(new RawSql(rawSql));
    }

    public PlatformRequest<T> matchingAnyOf(PlatformRequest platform){
        super.internalMatchAny(platform);
        return this;
    }

    public PlatformRequest<T> enhanceChildrenIfNeeded(){
        return this;
    }

    public PlatformRequest<T> withDeletedRows(){
        super.withDeletedRows();
        return this;
    }

    public PlatformRequest<T> deletedRowsOnly(){
        super.deletedRowsOnly();
        return this;
    }

    public PlatformRequest<T> selectSelf(){
        super.selectSelf();
        return selectId().selectName().selectFounded().selectVersion();
    }

    public PlatformRequest<T> selectSelfFields(){
        return selectSelf();
    }

    public PlatformRequest<T> selectAll(){
        super.selectAll();
        return selectId().selectName().selectFounded().selectVersion();
    }

    public PlatformRequest<T> selectChildren(){
        super.selectAny();
        selectTaskList();
        return selectId().selectName().selectFounded().selectVersion();
    }
    public PlatformRequest<T> filterWithJson(String jsonExpr){
        super.internalFindWithJsonExpr(jsonExpr);
        return this;
    }


    public PlatformRequest<T> selectId(){
       selectProperty(Platform.ID_PROPERTY);
       return this;
    }

    /**
     * fill the id with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  id) to fetch id property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public PlatformRequest<T> selectId(RawSql rawSqlSegment){
       selectProperty(Platform.ID_PROPERTY, rawSqlSegment);
       return this;
    }



    public PlatformRequest<T> unselectId(){
       unselectProperty(Platform.ID_PROPERTY);
       return this;
    }
    public PlatformRequest<T> selectName(){
       selectProperty(Platform.NAME_PROPERTY);
       return this;
    }

    /**
     * fill the name with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  name) to fetch name property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public PlatformRequest<T> selectName(RawSql rawSqlSegment){
       selectProperty(Platform.NAME_PROPERTY, rawSqlSegment);
       return this;
    }



    public PlatformRequest<T> unselectName(){
       unselectProperty(Platform.NAME_PROPERTY);
       return this;
    }
    public PlatformRequest<T> selectFounded(){
       selectProperty(Platform.FOUNDED_PROPERTY);
       return this;
    }

    /**
     * fill the founded with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  founded) to fetch founded property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public PlatformRequest<T> selectFounded(RawSql rawSqlSegment){
       selectProperty(Platform.FOUNDED_PROPERTY, rawSqlSegment);
       return this;
    }



    public PlatformRequest<T> unselectFounded(){
       unselectProperty(Platform.FOUNDED_PROPERTY);
       return this;
    }
    public PlatformRequest<T> selectVersion(){
       selectProperty(Platform.VERSION_PROPERTY);
       return this;
    }

    /**
     * fill the version with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  version) to fetch version property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public PlatformRequest<T> selectVersion(RawSql rawSqlSegment){
       selectProperty(Platform.VERSION_PROPERTY, rawSqlSegment);
       return this;
    }



    public PlatformRequest<T> unselectVersion(){
       unselectProperty(Platform.VERSION_PROPERTY);
       return this;
    }
    public PlatformRequest<T> selectTaskList(){
       return selectTaskListWith(Q.tasks().selectSelf());
    }

    public PlatformRequest<T> selectTaskListWith(TaskRequest taskList){
       enhanceRelation(Platform.TASK_LIST_PROPERTY, taskList);
       return this;
    }

    public PlatformRequest<T> withId(Operator operator, Object... values){
       return appendSearchCriteria(createIdCriteria(operator, values));
    }

    public SearchCriteria createIdCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Platform.ID_PROPERTY, operator, values);
    }

    public PlatformRequest<T> withIdIs(Long id){
       return withId(Operator.EQUAL, id);
    }
    public PlatformRequest<T> withIdIn(Long... id){
       return withId(Operator.EQUAL, (Object[])id);
    }



    public PlatformRequest<T> filterByName(String... name){
      if (name == null || name.length == 0) {
        throw new RepositoryException("filterByName parameter name cannot be empty");
      }
      return appendSearchCriteria(createNameCriteria(Operator.EQUAL, (Object[])name));
    }

    public PlatformRequest<T> withName(Operator operator, Object... values){
       return appendSearchCriteria(createNameCriteria(operator, values));
    }

    public PlatformRequest<T> withNameIsUnknown(){
       return withName(Operator.IS_NULL);
    }

    public PlatformRequest<T> withNameIsKnown(){
       return withName(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createNameCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Platform.NAME_PROPERTY, operator, values);
    }

    public PlatformRequest<T> withNameGreaterThan(String name){
       return withName(Operator.GREATER_THAN, name);
    }

    public PlatformRequest<T> withNameGreaterThanOrEqualTo(String name){
       return withName(Operator.GREATER_THAN_OR_EQUAL, name);
    }

    public PlatformRequest<T> withNameLessThan(String name){
       return withName(Operator.LESS_THAN, name);
    }

    public PlatformRequest<T> withNameLessThanOrEqualTo(String name){
       return withName(Operator.LESS_THAN_OR_EQUAL, name);
    }

    public PlatformRequest<T> withNameBetween(String startOfName, String endOfName){
       return withName(Operator.BETWEEN, startOfName, endOfName);
    }
    public PlatformRequest<T> withNameStartingWith(String name){
       return withName(Operator.BEGIN_WITH, name);
    }
    public PlatformRequest<T> withNameContaining(String name){
       return withName(Operator.CONTAIN, name);
    }

    public PlatformRequest<T> withNameEndingWith(String name){
       return withName(Operator.END_WITH, name);
    }

    public PlatformRequest<T> withNameIs(String name){
       return withName(Operator.EQUAL, name);
    }

    public PlatformRequest<T> withNameSoundingLike(String name){
       return withName(Operator.SOUNDS_LIKE, name);
    }



    public PlatformRequest<T> filterByFounded(LocalDateTime... founded){
      if (founded == null || founded.length == 0) {
        throw new RepositoryException("filterByFounded parameter founded cannot be empty");
      }
      return appendSearchCriteria(createFoundedCriteria(Operator.EQUAL, (Object[])founded));
    }

    public PlatformRequest<T> withFounded(Operator operator, Object... values){
       return appendSearchCriteria(createFoundedCriteria(operator, values));
    }

    public PlatformRequest<T> withFoundedIsUnknown(){
       return withFounded(Operator.IS_NULL);
    }

    public PlatformRequest<T> withFoundedIsKnown(){
       return withFounded(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createFoundedCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Platform.FOUNDED_PROPERTY, operator, values);
    }

    public PlatformRequest<T> withFoundedGreaterThan(LocalDateTime founded){
       return withFounded(Operator.GREATER_THAN, founded);
    }

    public PlatformRequest<T> withFoundedGreaterThanOrEqualTo(LocalDateTime founded){
       return withFounded(Operator.GREATER_THAN_OR_EQUAL, founded);
    }

    public PlatformRequest<T> withFoundedLessThan(LocalDateTime founded){
       return withFounded(Operator.LESS_THAN, founded);
    }

    public PlatformRequest<T> withFoundedLessThanOrEqualTo(LocalDateTime founded){
       return withFounded(Operator.LESS_THAN_OR_EQUAL, founded);
    }

    public PlatformRequest<T> withFoundedBetween(LocalDateTime startOfFounded, LocalDateTime endOfFounded){
       return withFounded(Operator.BETWEEN, startOfFounded, endOfFounded);
    }
    public PlatformRequest<T> withFoundedBefore(LocalDateTime founded){
       return withFounded(Operator.LESS_THAN, founded);
    }

    public PlatformRequest<T> withFoundedBefore(Date founded){
       return withFounded(Operator.LESS_THAN, founded);
    }

    public PlatformRequest<T> withFoundedAfter(LocalDateTime founded){
       return withFounded(Operator.GREATER_THAN, founded);
    }

    public PlatformRequest<T> withFoundedAfter(Date founded){
       return withFounded(Operator.GREATER_THAN, founded);
    }

    public PlatformRequest<T> withFoundedBetween(Date startOfFounded, Date endOfFounded){
       return withFounded(Operator.BETWEEN, startOfFounded, endOfFounded);
    }

    public PlatformRequest<T> withFoundedBetween(DateRange foundedRange){
       return withFoundedBetween(foundedRange.getStartDate(), foundedRange.getEndDate());
    }


    public PlatformRequest<T> filterByVersion(Long... version){
      if (version == null || version.length == 0) {
        throw new RepositoryException("filterByVersion parameter version cannot be empty");
      }
      return appendSearchCriteria(createVersionCriteria(Operator.EQUAL, (Object[])version));
    }

    public PlatformRequest<T> withVersion(Operator operator, Object... values){
       return appendSearchCriteria(createVersionCriteria(operator, values));
    }

    public PlatformRequest<T> withVersionIsUnknown(){
       return withVersion(Operator.IS_NULL);
    }

    public PlatformRequest<T> withVersionIsKnown(){
       return withVersion(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createVersionCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(Platform.VERSION_PROPERTY, operator, values);
    }

    public PlatformRequest<T> withVersionGreaterThan(Long version){
       return withVersion(Operator.GREATER_THAN, version);
    }

    public PlatformRequest<T> withVersionGreaterThanOrEqualTo(Long version){
       return withVersion(Operator.GREATER_THAN_OR_EQUAL, version);
    }

    public PlatformRequest<T> withVersionLessThan(Long version){
       return withVersion(Operator.LESS_THAN, version);
    }

    public PlatformRequest<T> withVersionLessThanOrEqualTo(Long version){
       return withVersion(Operator.LESS_THAN_OR_EQUAL, version);
    }

    public PlatformRequest<T> withVersionBetween(Long startOfVersion, Long endOfVersion){
       return withVersion(Operator.BETWEEN, startOfVersion, endOfVersion);
    }

    public PlatformRequest<T> withTaskListMatching(TaskRequest taskRequest){
        return appendSearchCriteria(new SubQuerySearchCriteria(Platform.ID_PROPERTY, taskRequest, Task.PLATFORM_PROPERTY));
    }

    public PlatformRequest<T> withoutTaskListMatching(TaskRequest taskRequest){
        return appendSearchCriteria(SearchCriteria.not(new SubQuerySearchCriteria(Platform.ID_PROPERTY, taskRequest, Task.PLATFORM_PROPERTY)));
    }

    public PlatformRequest<T> haveTasks(){
        return withTaskListMatching(Q.tasks().unlimited());
    }

    public PlatformRequest<T> haveNoTasks(){
        return withoutTaskListMatching(Q.tasks().unlimited());
    }

    public PlatformRequest<T> count(){
        super.count();
        return this;
    }
    public PlatformRequest<T> countAs(String retName){
        super.count(retName);
        return this;
    }
    public PlatformRequest<T> groupByTasksWithDetails(TaskRequest subRequest){
       aggregate(Platform.TASK_LIST_PROPERTY, subRequest);
       return this;
    }

    public PlatformRequest<T> groupById(){
       groupBy(Platform.ID_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByIdAs(String retName){
       groupBy(retName, Platform.ID_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByIdWithFunction(String retName, AggrFunction function){
       groupBy(retName, Platform.ID_PROPERTY, function);
       return this;
    }

    public PlatformRequest<T> groupByName(){
       groupBy(Platform.NAME_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByNameAs(String retName){
       groupBy(retName, Platform.NAME_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByNameWithFunction(String retName, AggrFunction function){
       groupBy(retName, Platform.NAME_PROPERTY, function);
       return this;
    }

    public PlatformRequest<T> groupByFounded(){
       groupBy(Platform.FOUNDED_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByFoundedAs(String retName){
       groupBy(retName, Platform.FOUNDED_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByFoundedWithFunction(String retName, AggrFunction function){
       groupBy(retName, Platform.FOUNDED_PROPERTY, function);
       return this;
    }

    public PlatformRequest<T> groupByVersion(){
       groupBy(Platform.VERSION_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByVersionAs(String retName){
       groupBy(retName, Platform.VERSION_PROPERTY);
       return this;
    }

    public PlatformRequest<T> groupByVersionWithFunction(String retName, AggrFunction function){
       groupBy(retName, Platform.VERSION_PROPERTY, function);
       return this;
    }



    public PlatformRequest<T> orderByIdAscending(){
       addOrderByAscending(Platform.ID_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByIdDescending(){
       addOrderByDescending(Platform.ID_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByNameAscending(){
       addOrderByAscending(Platform.NAME_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByNameDescending(){
       addOrderByDescending(Platform.NAME_PROPERTY);
       return this;
    }
    public PlatformRequest<T> orderByNameAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(Platform.NAME_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByNameDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(Platform.NAME_PROPERTY);
       return this;
    }
    public PlatformRequest<T> orderByFoundedAscending(){
       addOrderByAscending(Platform.FOUNDED_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByFoundedDescending(){
       addOrderByDescending(Platform.FOUNDED_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByVersionAscending(){
       addOrderByAscending(Platform.VERSION_PROPERTY);
       return this;
    }

    public PlatformRequest<T> orderByVersionDescending(){
       addOrderByDescending(Platform.VERSION_PROPERTY);
       return this;
    }


    public PlatformRequest<T> statsFromTasksAs(String name, TaskRequest subRequest){
       return statsFromTasksAs(name, subRequest, false);
    }

    public PlatformRequest<T> statsFromTasksAs(String name, TaskRequest subRequest, boolean singleResult){
       subRequest.setPartitionProperty(Task.PLATFORM_PROPERTY);
       addAggregateDynamicProperty(name, subRequest, singleResult);
       return this;
    }

    public PlatformRequest<T> statsFromTasks(TaskRequest subRequest){
       return statsFromTasksAs(REFINEMENTS, subRequest);
    }
    public PlatformRequest<T> countTasks(){
        return countTasksAs("Count");
    }

    public PlatformRequest<T> countTasksAs(String name){
        return countTasksWith(name, Q.tasks().unlimited());
    }

    public PlatformRequest<T> countTasksWith(String name, TaskRequest subRequest){
        return statsFromTasksAs(name, subRequest.count(), true);
    }


    public PlatformRequest<T> createPropertyAs(String propertyName, RawSql rawSqlSegment){
        super.addSimpleDynamicProperty(propertyName, rawSqlSegment);
        return this;
    }

    /**
     * get topN records
     * @param topN  records number
     */
    public PlatformRequest<T> top(int topN) {
        super.top(topN);
        return this;
    }

    /**
     * get records from offset(inclusive) to offset+size(exclusive)
     * @param offset record offset
     * @param size records number
     */
    public PlatformRequest<T> offset(int offset, int size) {
        super.offset(offset, size);
        return this;
    }

    /**
     * retrieve all records
     */
    public PlatformRequest<T> unlimited() {
        super.unlimited();
        return this;
    }

    /**
     * get records of one page
     * @param pageNumber page number(1-based)
     * @param pageSize page size
     */
    public PlatformRequest<T> page(int pageNumber, int pageSize) {
        int offset = (pageNumber - 1) * pageSize;
        return offset(offset, pageSize);
   }

    /**
     * get records of one page, default page size is 10
     * @param pageNumber page number(1-based)
     */
    public PlatformRequest<T> page(int pageNumber) {
        return page(pageNumber, 10);
   }
}