package com.doublechaintech.robotkanbanservice.platform;

import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskListExpression;
import io.teaql.data.UserContext;
import io.teaql.data.value.BaseEntityExpression;
import io.teaql.data.value.Expression;
import io.teaql.data.value.ExpressionAdaptor;
import java.time.LocalDateTime;
import java.util.function.Function;

public class PlatformExpression<T, E, U extends Platform> extends ExpressionAdaptor<T, E, U> implements BaseEntityExpression<T, U> {
    public PlatformExpression(Expression<T, U> expression){
        super(expression);
    }

    public PlatformExpression(Expression<T, E> expression, Function<E, U> function){
        super(expression, function);
    }

     public PlatformExpression<T, U, U> updateId(Long id){
        return new PlatformExpression(this, $it -> {((Platform)$it).setId(id); return this;});
     }

     public PlatformExpression<T, U, U> save(UserContext userContext){
        return new PlatformExpression(this, $it -> ((Platform)$it).save(userContext));
     }


    public Expression<T, String> getName(){
       return apply(Platform::getName);
    }
    public PlatformExpression<T, U, U> updateName(String name){
       return new PlatformExpression(this, $it ->  ((Platform)$it).updateName(name));
    }

    public Expression<T, LocalDateTime> getFounded(){
       return apply(Platform::getFounded);
    }
    public PlatformExpression<T, U, U> updateFounded(LocalDateTime founded){
       return new PlatformExpression(this, $it ->  ((Platform)$it).updateFounded(founded));
    }

    public TaskListExpression<T, U, Task> getTaskList(){
        return new TaskListExpression(this, $it ->  ((Platform)$it).getTaskList());
    }
    public PlatformExpression<T, U, U> addTask(Task task){
       return new PlatformExpression(this, $it ->  ((Platform)$it).addTask(task));
    }
}