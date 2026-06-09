package com.doublechaintech.robotkanbanservice.taskexecutionlog;

import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskExpression;
import io.teaql.data.UserContext;
import io.teaql.data.value.BaseEntityExpression;
import io.teaql.data.value.Expression;
import io.teaql.data.value.ExpressionAdaptor;
import java.util.function.Function;

public class TaskExecutionLogExpression<T, E, U extends TaskExecutionLog> extends ExpressionAdaptor<T, E, U> implements BaseEntityExpression<T, U> {
    public TaskExecutionLogExpression(Expression<T, U> expression){
        super(expression);
    }

    public TaskExecutionLogExpression(Expression<T, E> expression, Function<E, U> function){
        super(expression, function);
    }

     public TaskExecutionLogExpression<T, U, U> updateId(Long id){
        return new TaskExecutionLogExpression(this, $it -> {((TaskExecutionLog)$it).setId(id); return this;});
     }

     public TaskExecutionLogExpression<T, U, U> save(UserContext userContext){
        return new TaskExecutionLogExpression(this, $it -> ((TaskExecutionLog)$it).save(userContext));
     }


    public TaskExpression<T, U, Task> getTask(){
       return new TaskExpression(this, $it ->  ((TaskExecutionLog)$it).getTask());
    }

    public TaskExecutionLogExpression<T, U, U> updateTask(Task task){
       return new TaskExecutionLogExpression(this, $it ->  ((TaskExecutionLog)$it).updateTask(task));
    }

    public Expression<T, String> getAction(){
       return apply(TaskExecutionLog::getAction);
    }
    public TaskExecutionLogExpression<T, U, U> updateAction(String action){
       return new TaskExecutionLogExpression(this, $it ->  ((TaskExecutionLog)$it).updateAction(action));
    }

    public Expression<T, String> getDetail(){
       return apply(TaskExecutionLog::getDetail);
    }
    public TaskExecutionLogExpression<T, U, U> updateDetail(String detail){
       return new TaskExecutionLogExpression(this, $it ->  ((TaskExecutionLog)$it).updateDetail(detail));
    }

}