package com.doublechaintech.robotkanbanservice.taskstatus;

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
import java.math.BigDecimal;

public class TaskStatusRequest<T extends TaskStatus> extends BaseRequest<T> {

    /**
     * @deprecated AI agents and business code must use the generated Q facade
     *             instead of constructing request builders directly.
     */
    @Deprecated
    public TaskStatusRequest(Class<T> returnType){
        super(returnType);
        selectId();
        selectVersion();
    }

    public TaskStatusRequest<T> comment(String comment){
         super.internalComment(comment);
         return this;
    }

    public TaskStatusRequest<T> returnType(Class<? extends T> returnType){
        super.setReturnType(returnType);
        return this;
    }

    public TaskStatusRequest<T> enableAggregationCache(long cacheExpiredMillis){
        super.enableAggregationCache();
        super.aggregateCacheTime(cacheExpiredMillis);
        return this;
    }

    public TaskStatusRequest<T> enableAggregationCache(){
        return enableAggregationCache(0l);
    }


    public TaskStatusRequest<T> propagateAggregationCache(long cacheExpiredMillis){
        super.propagateAggregationCache(cacheExpiredMillis);
        return this;
    }

    public TaskStatusRequest<T> appendSearchCriteria(SearchCriteria searchCriteria){
        return (TaskStatusRequest<T>)super.appendSearchCriteria(searchCriteria);
    }

    public TaskStatusRequest<T> filter(String property1, Operator operator, String property2){
        return appendSearchCriteria(new TwoOperatorCriteria(operator, new PropertyReference(property1), new PropertyReference(property2)));
    }

    public TaskStatusRequest<T> rawSql(String rawSql){
        super.setRawSql(rawSql);
        return this;
    }

    public TaskStatusRequest<T> rawSqlFilter(String rawSql){
        return appendSearchCriteria(new RawSql(rawSql));
    }

    public TaskStatusRequest<T> matchingAnyOf(TaskStatusRequest taskStatus){
        super.internalMatchAny(taskStatus);
        return this;
    }

    public TaskStatusRequest<T> enhanceChildrenIfNeeded(){
        return this;
    }

    public TaskStatusRequest<T> withDeletedRows(){
        super.withDeletedRows();
        return this;
    }

    public TaskStatusRequest<T> deletedRowsOnly(){
        super.deletedRowsOnly();
        return this;
    }

    public TaskStatusRequest<T> selectSelf(){
        super.selectSelf();
        return selectId().selectName().selectCode().selectColor().selectDisplayOrder().selectProgress().selectVersion();
    }

    public TaskStatusRequest<T> selectSelfFields(){
        return selectSelf();
    }

    public TaskStatusRequest<T> selectAll(){
        super.selectAll();
        return selectId().selectName().selectCode().selectColor().selectDisplayOrder().selectProgress().selectVersion();
    }

    public TaskStatusRequest<T> selectChildren(){
        super.selectAny();
        selectTaskList();
        return selectId().selectName().selectCode().selectColor().selectDisplayOrder().selectProgress().selectVersion();
    }
    public TaskStatusRequest<T> filterWithJson(String jsonExpr){
        super.internalFindWithJsonExpr(jsonExpr);
        return this;
    }


    public TaskStatusRequest<T> selectId(){
       selectProperty(TaskStatus.ID_PROPERTY);
       return this;
    }

    /**
     * fill the id with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  id) to fetch id property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectId(RawSql rawSqlSegment){
       selectProperty(TaskStatus.ID_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskStatusRequest<T> unselectId(){
       unselectProperty(TaskStatus.ID_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectName(){
       selectProperty(TaskStatus.NAME_PROPERTY);
       return this;
    }

    /**
     * fill the name with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  name) to fetch name property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectName(RawSql rawSqlSegment){
       selectProperty(TaskStatus.NAME_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskStatusRequest<T> unselectName(){
       unselectProperty(TaskStatus.NAME_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectCode(){
       selectProperty(TaskStatus.CODE_PROPERTY);
       return this;
    }

    /**
     * fill the code with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  code) to fetch code property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectCode(RawSql rawSqlSegment){
       selectProperty(TaskStatus.CODE_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskStatusRequest<T> unselectCode(){
       unselectProperty(TaskStatus.CODE_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectColor(){
       selectProperty(TaskStatus.COLOR_PROPERTY);
       return this;
    }

    /**
     * fill the color with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  color) to fetch color property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectColor(RawSql rawSqlSegment){
       selectProperty(TaskStatus.COLOR_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskStatusRequest<T> unselectColor(){
       unselectProperty(TaskStatus.COLOR_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectDisplayOrder(){
       selectProperty(TaskStatus.DISPLAY_ORDER_PROPERTY);
       return this;
    }

    /**
     * fill the displayOrder with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  displayOrder) to fetch displayOrder property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectDisplayOrder(RawSql rawSqlSegment){
       selectProperty(TaskStatus.DISPLAY_ORDER_PROPERTY, rawSqlSegment);
       return this;
    }

    /**
     * fill the displayOrder with customized aggrFunction, TEAQL uses ({aggrFunction}(displayOrder) AS displayOrder to fetch displayOrder property.
     * @param aggrFunction  aggrFunction
     */
    public TaskStatusRequest<T> selectDisplayOrder(AggrFunction aggrFunction){
       selectProperty(TaskStatus.DISPLAY_ORDER_PROPERTY, aggrFunction);
       return this;
    }


    public TaskStatusRequest<T> unselectDisplayOrder(){
       unselectProperty(TaskStatus.DISPLAY_ORDER_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectProgress(){
       selectProperty(TaskStatus.PROGRESS_PROPERTY);
       return this;
    }

    /**
     * fill the progress with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  progress) to fetch progress property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectProgress(RawSql rawSqlSegment){
       selectProperty(TaskStatus.PROGRESS_PROPERTY, rawSqlSegment);
       return this;
    }

    /**
     * fill the progress with customized aggrFunction, TEAQL uses ({aggrFunction}(progress) AS progress to fetch progress property.
     * @param aggrFunction  aggrFunction
     */
    public TaskStatusRequest<T> selectProgress(AggrFunction aggrFunction){
       selectProperty(TaskStatus.PROGRESS_PROPERTY, aggrFunction);
       return this;
    }


    public TaskStatusRequest<T> unselectProgress(){
       unselectProperty(TaskStatus.PROGRESS_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectVersion(){
       selectProperty(TaskStatus.VERSION_PROPERTY);
       return this;
    }

    /**
     * fill the version with customized rawSqlSegment, TEAQL uses ({rawSqlSegment} AS  version) to fetch version property.
     * @param rawSqlSegment  customized rawSqlSegment
     */
    public TaskStatusRequest<T> selectVersion(RawSql rawSqlSegment){
       selectProperty(TaskStatus.VERSION_PROPERTY, rawSqlSegment);
       return this;
    }



    public TaskStatusRequest<T> unselectVersion(){
       unselectProperty(TaskStatus.VERSION_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> selectTaskList(){
       return selectTaskListWith(Q.tasks().selectSelf());
    }

    public TaskStatusRequest<T> selectTaskListWith(TaskRequest taskList){
       enhanceRelation(TaskStatus.TASK_LIST_PROPERTY, taskList);
       return this;
    }

    public TaskStatusRequest<T> withId(Operator operator, Object... values){
       return appendSearchCriteria(createIdCriteria(operator, values));
    }

    public SearchCriteria createIdCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.ID_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withIdIs(Long id){
       return withId(Operator.EQUAL, id);
    }
    public TaskStatusRequest<T> withIdIn(Long... id){
       return withId(Operator.EQUAL, (Object[])id);
    }



    public TaskStatusRequest<T> filterByName(String... name){
      if (name == null || name.length == 0) {
        throw new RepositoryException("filterByName parameter name cannot be empty");
      }
      return appendSearchCriteria(createNameCriteria(Operator.EQUAL, (Object[])name));
    }

    public TaskStatusRequest<T> withName(Operator operator, Object... values){
       return appendSearchCriteria(createNameCriteria(operator, values));
    }

    public TaskStatusRequest<T> withNameIsUnknown(){
       return withName(Operator.IS_NULL);
    }

    public TaskStatusRequest<T> withNameIsKnown(){
       return withName(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createNameCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.NAME_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withNameGreaterThan(String name){
       return withName(Operator.GREATER_THAN, name);
    }

    public TaskStatusRequest<T> withNameGreaterThanOrEqualTo(String name){
       return withName(Operator.GREATER_THAN_OR_EQUAL, name);
    }

    public TaskStatusRequest<T> withNameLessThan(String name){
       return withName(Operator.LESS_THAN, name);
    }

    public TaskStatusRequest<T> withNameLessThanOrEqualTo(String name){
       return withName(Operator.LESS_THAN_OR_EQUAL, name);
    }

    public TaskStatusRequest<T> withNameBetween(String startOfName, String endOfName){
       return withName(Operator.BETWEEN, startOfName, endOfName);
    }
    public TaskStatusRequest<T> withNameStartingWith(String name){
       return withName(Operator.BEGIN_WITH, name);
    }
    public TaskStatusRequest<T> withNameContaining(String name){
       return withName(Operator.CONTAIN, name);
    }

    public TaskStatusRequest<T> withNameEndingWith(String name){
       return withName(Operator.END_WITH, name);
    }

    public TaskStatusRequest<T> withNameIs(String name){
       return withName(Operator.EQUAL, name);
    }

    public TaskStatusRequest<T> withNameSoundingLike(String name){
       return withName(Operator.SOUNDS_LIKE, name);
    }



    public TaskStatusRequest<T> filterByCode(String... code){
      if (code == null || code.length == 0) {
        throw new RepositoryException("filterByCode parameter code cannot be empty");
      }
      return appendSearchCriteria(createCodeCriteria(Operator.EQUAL, (Object[])code));
    }

    public TaskStatusRequest<T> withCode(Operator operator, Object... values){
       return appendSearchCriteria(createCodeCriteria(operator, values));
    }

    public TaskStatusRequest<T> withCodeIsUnknown(){
       return withCode(Operator.IS_NULL);
    }

    public TaskStatusRequest<T> withCodeIsKnown(){
       return withCode(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createCodeCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.CODE_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withCodeGreaterThan(String code){
       return withCode(Operator.GREATER_THAN, code);
    }

    public TaskStatusRequest<T> withCodeGreaterThanOrEqualTo(String code){
       return withCode(Operator.GREATER_THAN_OR_EQUAL, code);
    }

    public TaskStatusRequest<T> withCodeLessThan(String code){
       return withCode(Operator.LESS_THAN, code);
    }

    public TaskStatusRequest<T> withCodeLessThanOrEqualTo(String code){
       return withCode(Operator.LESS_THAN_OR_EQUAL, code);
    }

    public TaskStatusRequest<T> withCodeBetween(String startOfCode, String endOfCode){
       return withCode(Operator.BETWEEN, startOfCode, endOfCode);
    }
    public TaskStatusRequest<T> withCodeStartingWith(String code){
       return withCode(Operator.BEGIN_WITH, code);
    }
    public TaskStatusRequest<T> withCodeContaining(String code){
       return withCode(Operator.CONTAIN, code);
    }

    public TaskStatusRequest<T> withCodeEndingWith(String code){
       return withCode(Operator.END_WITH, code);
    }

    public TaskStatusRequest<T> withCodeIs(String code){
       return withCode(Operator.EQUAL, code);
    }

    public TaskStatusRequest<T> withCodeSoundingLike(String code){
       return withCode(Operator.SOUNDS_LIKE, code);
    }



    public TaskStatusRequest<T> filterByColor(String... color){
      if (color == null || color.length == 0) {
        throw new RepositoryException("filterByColor parameter color cannot be empty");
      }
      return appendSearchCriteria(createColorCriteria(Operator.EQUAL, (Object[])color));
    }

    public TaskStatusRequest<T> withColor(Operator operator, Object... values){
       return appendSearchCriteria(createColorCriteria(operator, values));
    }

    public TaskStatusRequest<T> withColorIsUnknown(){
       return withColor(Operator.IS_NULL);
    }

    public TaskStatusRequest<T> withColorIsKnown(){
       return withColor(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createColorCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.COLOR_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withColorGreaterThan(String color){
       return withColor(Operator.GREATER_THAN, color);
    }

    public TaskStatusRequest<T> withColorGreaterThanOrEqualTo(String color){
       return withColor(Operator.GREATER_THAN_OR_EQUAL, color);
    }

    public TaskStatusRequest<T> withColorLessThan(String color){
       return withColor(Operator.LESS_THAN, color);
    }

    public TaskStatusRequest<T> withColorLessThanOrEqualTo(String color){
       return withColor(Operator.LESS_THAN_OR_EQUAL, color);
    }

    public TaskStatusRequest<T> withColorBetween(String startOfColor, String endOfColor){
       return withColor(Operator.BETWEEN, startOfColor, endOfColor);
    }
    public TaskStatusRequest<T> withColorStartingWith(String color){
       return withColor(Operator.BEGIN_WITH, color);
    }
    public TaskStatusRequest<T> withColorContaining(String color){
       return withColor(Operator.CONTAIN, color);
    }

    public TaskStatusRequest<T> withColorEndingWith(String color){
       return withColor(Operator.END_WITH, color);
    }

    public TaskStatusRequest<T> withColorIs(String color){
       return withColor(Operator.EQUAL, color);
    }

    public TaskStatusRequest<T> withColorSoundingLike(String color){
       return withColor(Operator.SOUNDS_LIKE, color);
    }



    public TaskStatusRequest<T> filterByDisplayOrder(BigDecimal... displayOrder){
      if (displayOrder == null || displayOrder.length == 0) {
        throw new RepositoryException("filterByDisplayOrder parameter displayOrder cannot be empty");
      }
      return appendSearchCriteria(createDisplayOrderCriteria(Operator.EQUAL, (Object[])displayOrder));
    }

    public TaskStatusRequest<T> withDisplayOrder(Operator operator, Object... values){
       return appendSearchCriteria(createDisplayOrderCriteria(operator, values));
    }

    public TaskStatusRequest<T> withDisplayOrderIsUnknown(){
       return withDisplayOrder(Operator.IS_NULL);
    }

    public TaskStatusRequest<T> withDisplayOrderIsKnown(){
       return withDisplayOrder(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createDisplayOrderCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.DISPLAY_ORDER_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withDisplayOrderGreaterThan(BigDecimal displayOrder){
       return withDisplayOrder(Operator.GREATER_THAN, displayOrder);
    }

    public TaskStatusRequest<T> withDisplayOrderGreaterThanOrEqualTo(BigDecimal displayOrder){
       return withDisplayOrder(Operator.GREATER_THAN_OR_EQUAL, displayOrder);
    }

    public TaskStatusRequest<T> withDisplayOrderLessThan(BigDecimal displayOrder){
       return withDisplayOrder(Operator.LESS_THAN, displayOrder);
    }

    public TaskStatusRequest<T> withDisplayOrderLessThanOrEqualTo(BigDecimal displayOrder){
       return withDisplayOrder(Operator.LESS_THAN_OR_EQUAL, displayOrder);
    }

    public TaskStatusRequest<T> withDisplayOrderBetween(BigDecimal startOfDisplayOrder, BigDecimal endOfDisplayOrder){
       return withDisplayOrder(Operator.BETWEEN, startOfDisplayOrder, endOfDisplayOrder);
    }



    public TaskStatusRequest<T> filterByProgress(BigDecimal... progress){
      if (progress == null || progress.length == 0) {
        throw new RepositoryException("filterByProgress parameter progress cannot be empty");
      }
      return appendSearchCriteria(createProgressCriteria(Operator.EQUAL, (Object[])progress));
    }

    public TaskStatusRequest<T> withProgress(Operator operator, Object... values){
       return appendSearchCriteria(createProgressCriteria(operator, values));
    }

    public TaskStatusRequest<T> withProgressIsUnknown(){
       return withProgress(Operator.IS_NULL);
    }

    public TaskStatusRequest<T> withProgressIsKnown(){
       return withProgress(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createProgressCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.PROGRESS_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withProgressGreaterThan(BigDecimal progress){
       return withProgress(Operator.GREATER_THAN, progress);
    }

    public TaskStatusRequest<T> withProgressGreaterThanOrEqualTo(BigDecimal progress){
       return withProgress(Operator.GREATER_THAN_OR_EQUAL, progress);
    }

    public TaskStatusRequest<T> withProgressLessThan(BigDecimal progress){
       return withProgress(Operator.LESS_THAN, progress);
    }

    public TaskStatusRequest<T> withProgressLessThanOrEqualTo(BigDecimal progress){
       return withProgress(Operator.LESS_THAN_OR_EQUAL, progress);
    }

    public TaskStatusRequest<T> withProgressBetween(BigDecimal startOfProgress, BigDecimal endOfProgress){
       return withProgress(Operator.BETWEEN, startOfProgress, endOfProgress);
    }



    public TaskStatusRequest<T> filterByVersion(Long... version){
      if (version == null || version.length == 0) {
        throw new RepositoryException("filterByVersion parameter version cannot be empty");
      }
      return appendSearchCriteria(createVersionCriteria(Operator.EQUAL, (Object[])version));
    }

    public TaskStatusRequest<T> withVersion(Operator operator, Object... values){
       return appendSearchCriteria(createVersionCriteria(operator, values));
    }

    public TaskStatusRequest<T> withVersionIsUnknown(){
       return withVersion(Operator.IS_NULL);
    }

    public TaskStatusRequest<T> withVersionIsKnown(){
       return withVersion(Operator.IS_NOT_NULL);
    }

    public SearchCriteria createVersionCriteria(Operator operator, Object... values) {
        return createBasicSearchCriteria(TaskStatus.VERSION_PROPERTY, operator, values);
    }

    public TaskStatusRequest<T> withVersionGreaterThan(Long version){
       return withVersion(Operator.GREATER_THAN, version);
    }

    public TaskStatusRequest<T> withVersionGreaterThanOrEqualTo(Long version){
       return withVersion(Operator.GREATER_THAN_OR_EQUAL, version);
    }

    public TaskStatusRequest<T> withVersionLessThan(Long version){
       return withVersion(Operator.LESS_THAN, version);
    }

    public TaskStatusRequest<T> withVersionLessThanOrEqualTo(Long version){
       return withVersion(Operator.LESS_THAN_OR_EQUAL, version);
    }

    public TaskStatusRequest<T> withVersionBetween(Long startOfVersion, Long endOfVersion){
       return withVersion(Operator.BETWEEN, startOfVersion, endOfVersion);
    }

    public TaskStatusRequest<T> withTaskListMatching(TaskRequest taskRequest){
        return appendSearchCriteria(new SubQuerySearchCriteria(TaskStatus.ID_PROPERTY, taskRequest, Task.STATUS_PROPERTY));
    }

    public TaskStatusRequest<T> withoutTaskListMatching(TaskRequest taskRequest){
        return appendSearchCriteria(SearchCriteria.not(new SubQuerySearchCriteria(TaskStatus.ID_PROPERTY, taskRequest, Task.STATUS_PROPERTY)));
    }

    public TaskStatusRequest<T> haveTasks(){
        return withTaskListMatching(Q.tasks().unlimited());
    }

    public TaskStatusRequest<T> haveNoTasks(){
        return withoutTaskListMatching(Q.tasks().unlimited());
    }

    public TaskStatusRequest<T> count(){
        super.count();
        return this;
    }
    public TaskStatusRequest<T> countAs(String retName){
        super.count(retName);
        return this;
    }
    public TaskStatusRequest minDisplayOrder(){
        return minDisplayOrderAs(prefix("minOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest minDisplayOrderAs(String retName){
        super.min(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest maxDisplayOrder(){
        return maxDisplayOrderAs(prefix("maxOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest maxDisplayOrderAs(String retName){
        super.max(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest sumDisplayOrder(){
        return sumDisplayOrderAs(prefix("sumOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest sumDisplayOrderAs(String retName){
        super.sum(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest avgDisplayOrder(){
        return avgDisplayOrderAs(prefix("avgOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest avgDisplayOrderAs(String retName){
        super.avg(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest standardDeviationDisplayOrder(){
        return standardDeviationDisplayOrderAs(prefix("standardDeviationOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest standardDeviationDisplayOrderAs(String retName){
        super.standardDeviation(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest squareRootOfPopulationStandardDeviationDisplayOrder(){
        return squareRootOfPopulationStandardDeviationDisplayOrderAs(prefix("squareRootOfPopulationStandardDeviationOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest squareRootOfPopulationStandardDeviationDisplayOrderAs(String retName){
        super.squareRootOfPopulationStandardDeviation(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest sampleVarianceDisplayOrder(){
        return sampleVarianceDisplayOrderAs(prefix("sampleVarianceOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest sampleVarianceDisplayOrderAs(String retName){
        super.sampleVariance(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest samplePopulationVarianceDisplayOrder(){
        return samplePopulationVarianceDisplayOrderAs(prefix("samplePopulationVarianceOf",TaskStatus.DISPLAY_ORDER_PROPERTY));
    }

    public TaskStatusRequest samplePopulationVarianceDisplayOrderAs(String retName){
        super.samplePopulationVariance(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
        return this;
    }
    public TaskStatusRequest minProgress(){
        return minProgressAs(prefix("minOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest minProgressAs(String retName){
        super.min(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest maxProgress(){
        return maxProgressAs(prefix("maxOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest maxProgressAs(String retName){
        super.max(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest sumProgress(){
        return sumProgressAs(prefix("sumOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest sumProgressAs(String retName){
        super.sum(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest avgProgress(){
        return avgProgressAs(prefix("avgOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest avgProgressAs(String retName){
        super.avg(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest standardDeviationProgress(){
        return standardDeviationProgressAs(prefix("standardDeviationOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest standardDeviationProgressAs(String retName){
        super.standardDeviation(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest squareRootOfPopulationStandardDeviationProgress(){
        return squareRootOfPopulationStandardDeviationProgressAs(prefix("squareRootOfPopulationStandardDeviationOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest squareRootOfPopulationStandardDeviationProgressAs(String retName){
        super.squareRootOfPopulationStandardDeviation(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest sampleVarianceProgress(){
        return sampleVarianceProgressAs(prefix("sampleVarianceOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest sampleVarianceProgressAs(String retName){
        super.sampleVariance(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest samplePopulationVarianceProgress(){
        return samplePopulationVarianceProgressAs(prefix("samplePopulationVarianceOf",TaskStatus.PROGRESS_PROPERTY));
    }

    public TaskStatusRequest samplePopulationVarianceProgressAs(String retName){
        super.samplePopulationVariance(retName, TaskStatus.PROGRESS_PROPERTY);
        return this;
    }
    public TaskStatusRequest<T> groupByTasksWithDetails(TaskRequest subRequest){
       aggregate(TaskStatus.TASK_LIST_PROPERTY, subRequest);
       return this;
    }

    public TaskStatusRequest<T> groupById(){
       groupBy(TaskStatus.ID_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByIdAs(String retName){
       groupBy(retName, TaskStatus.ID_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByIdWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.ID_PROPERTY, function);
       return this;
    }

    public TaskStatusRequest<T> groupByName(){
       groupBy(TaskStatus.NAME_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByNameAs(String retName){
       groupBy(retName, TaskStatus.NAME_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByNameWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.NAME_PROPERTY, function);
       return this;
    }

    public TaskStatusRequest<T> groupByCode(){
       groupBy(TaskStatus.CODE_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByCodeAs(String retName){
       groupBy(retName, TaskStatus.CODE_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByCodeWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.CODE_PROPERTY, function);
       return this;
    }

    public TaskStatusRequest<T> groupByColor(){
       groupBy(TaskStatus.COLOR_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByColorAs(String retName){
       groupBy(retName, TaskStatus.COLOR_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByColorWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.COLOR_PROPERTY, function);
       return this;
    }

    public TaskStatusRequest<T> groupByDisplayOrder(){
       groupBy(TaskStatus.DISPLAY_ORDER_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByDisplayOrderAs(String retName){
       groupBy(retName, TaskStatus.DISPLAY_ORDER_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByDisplayOrderWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.DISPLAY_ORDER_PROPERTY, function);
       return this;
    }

    public TaskStatusRequest<T> groupByProgress(){
       groupBy(TaskStatus.PROGRESS_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByProgressAs(String retName){
       groupBy(retName, TaskStatus.PROGRESS_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByProgressWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.PROGRESS_PROPERTY, function);
       return this;
    }

    public TaskStatusRequest<T> groupByVersion(){
       groupBy(TaskStatus.VERSION_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByVersionAs(String retName){
       groupBy(retName, TaskStatus.VERSION_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> groupByVersionWithFunction(String retName, AggrFunction function){
       groupBy(retName, TaskStatus.VERSION_PROPERTY, function);
       return this;
    }



    public TaskStatusRequest<T> orderByIdAscending(){
       addOrderByAscending(TaskStatus.ID_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByIdDescending(){
       addOrderByDescending(TaskStatus.ID_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByNameAscending(){
       addOrderByAscending(TaskStatus.NAME_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByNameDescending(){
       addOrderByDescending(TaskStatus.NAME_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> orderByNameAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(TaskStatus.NAME_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByNameDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(TaskStatus.NAME_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> orderByCodeAscending(){
       addOrderByAscending(TaskStatus.CODE_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByCodeDescending(){
       addOrderByDescending(TaskStatus.CODE_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> orderByCodeAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(TaskStatus.CODE_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByCodeDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(TaskStatus.CODE_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> orderByColorAscending(){
       addOrderByAscending(TaskStatus.COLOR_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByColorDescending(){
       addOrderByDescending(TaskStatus.COLOR_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> orderByColorAscendingUsingGBK(){
       addOrderByAscendingUsingGBK(TaskStatus.COLOR_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByColorDescendingUsingGBK(){
       addOrderByDescendingUsingGBK(TaskStatus.COLOR_PROPERTY);
       return this;
    }
    public TaskStatusRequest<T> orderByDisplayOrderAscending(){
       addOrderByAscending(TaskStatus.DISPLAY_ORDER_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByDisplayOrderDescending(){
       addOrderByDescending(TaskStatus.DISPLAY_ORDER_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByProgressAscending(){
       addOrderByAscending(TaskStatus.PROGRESS_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByProgressDescending(){
       addOrderByDescending(TaskStatus.PROGRESS_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByVersionAscending(){
       addOrderByAscending(TaskStatus.VERSION_PROPERTY);
       return this;
    }

    public TaskStatusRequest<T> orderByVersionDescending(){
       addOrderByDescending(TaskStatus.VERSION_PROPERTY);
       return this;
    }


    public TaskStatusRequest<T> statsFromTasksAs(String name, TaskRequest subRequest){
       return statsFromTasksAs(name, subRequest, false);
    }

    public TaskStatusRequest<T> statsFromTasksAs(String name, TaskRequest subRequest, boolean singleResult){
       subRequest.setPartitionProperty(Task.STATUS_PROPERTY);
       addAggregateDynamicProperty(name, subRequest, singleResult);
       return this;
    }

    public TaskStatusRequest<T> statsFromTasks(TaskRequest subRequest){
       return statsFromTasksAs(REFINEMENTS, subRequest);
    }
    public TaskStatusRequest<T> countTasks(){
        return countTasksAs("Count");
    }

    public TaskStatusRequest<T> countTasksAs(String name){
        return countTasksWith(name, Q.tasks().unlimited());
    }

    public TaskStatusRequest<T> countTasksWith(String name, TaskRequest subRequest){
        return statsFromTasksAs(name, subRequest.count(), true);
    }


    public TaskStatusRequest<T> createPropertyAs(String propertyName, RawSql rawSqlSegment){
        super.addSimpleDynamicProperty(propertyName, rawSqlSegment);
        return this;
    }

    /**
     * get topN records
     * @param topN  records number
     */
    public TaskStatusRequest<T> top(int topN) {
        super.top(topN);
        return this;
    }

    /**
     * get records from offset(inclusive) to offset+size(exclusive)
     * @param offset record offset
     * @param size records number
     */
    public TaskStatusRequest<T> offset(int offset, int size) {
        super.offset(offset, size);
        return this;
    }

    /**
     * retrieve all records
     */
    public TaskStatusRequest<T> unlimited() {
        super.unlimited();
        return this;
    }

    /**
     * get records of one page
     * @param pageNumber page number(1-based)
     * @param pageSize page size
     */
    public TaskStatusRequest<T> page(int pageNumber, int pageSize) {
        int offset = (pageNumber - 1) * pageSize;
        return offset(offset, pageSize);
   }

    /**
     * get records of one page, default page size is 10
     * @param pageNumber page number(1-based)
     */
    public TaskStatusRequest<T> page(int pageNumber) {
        return page(pageNumber, 10);
   }
}