package com.doublechaintech.robotkanbanservice.taskstatus;

import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskListExpression;
import io.teaql.data.UserContext;
import io.teaql.data.value.BaseEntityExpression;
import io.teaql.data.value.Expression;
import io.teaql.data.value.ExpressionAdaptor;
import java.math.BigDecimal;
import java.util.function.Function;

public class TaskStatusExpression<T, E, U extends TaskStatus> extends ExpressionAdaptor<T, E, U> implements BaseEntityExpression<T, U> {
    public TaskStatusExpression(Expression<T, U> expression){
        super(expression);
    }

    public TaskStatusExpression(Expression<T, E> expression, Function<E, U> function){
        super(expression, function);
    }

     public TaskStatusExpression<T, U, U> updateId(Long id){
        return new TaskStatusExpression(this, $it -> {((TaskStatus)$it).setId(id); return this;});
     }

     public TaskStatusExpression<T, U, U> save(UserContext userContext){
        return new TaskStatusExpression(this, $it -> ((TaskStatus)$it).save(userContext));
     }


    public Expression<T, String> getName(){
       return apply(TaskStatus::getName);
    }
    public TaskStatusExpression<T, U, U> updateName(String name){
       return new TaskStatusExpression(this, $it ->  ((TaskStatus)$it).updateName(name));
    }

    public Expression<T, String> getCode(){
       return apply(TaskStatus::getCode);
    }
    public TaskStatusExpression<T, U, U> updateCode(String code){
       return new TaskStatusExpression(this, $it ->  ((TaskStatus)$it).updateCode(code));
    }

    public Expression<T, String> getColor(){
       return apply(TaskStatus::getColor);
    }
    public TaskStatusExpression<T, U, U> updateColor(String color){
       return new TaskStatusExpression(this, $it ->  ((TaskStatus)$it).updateColor(color));
    }

    public Expression<T, BigDecimal> getDisplayOrder(){
       return apply(TaskStatus::getDisplayOrder);
    }
    public TaskStatusExpression<T, U, U> updateDisplayOrder(BigDecimal displayOrder){
       return new TaskStatusExpression(this, $it ->  ((TaskStatus)$it).updateDisplayOrder(displayOrder));
    }

    public Expression<T, BigDecimal> getProgress(){
       return apply(TaskStatus::getProgress);
    }
    public TaskStatusExpression<T, U, U> updateProgress(BigDecimal progress){
       return new TaskStatusExpression(this, $it ->  ((TaskStatus)$it).updateProgress(progress));
    }

    public TaskListExpression<T, U, Task> getTaskList(){
        return new TaskListExpression(this, $it ->  ((TaskStatus)$it).getTaskList());
    }
    public TaskStatusExpression<T, U, U> addTask(Task task){
       return new TaskStatusExpression(this, $it ->  ((TaskStatus)$it).addTask(task));
    }
}