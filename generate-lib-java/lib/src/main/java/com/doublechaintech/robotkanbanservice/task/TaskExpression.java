package com.doublechaintech.robotkanbanservice.task;

import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.platform.PlatformExpression;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogListExpression;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusExpression;
import io.teaql.data.UserContext;
import io.teaql.data.value.BaseEntityExpression;
import io.teaql.data.value.Expression;
import io.teaql.data.value.ExpressionAdaptor;
import java.util.function.Function;

public class TaskExpression<T, E, U extends Task> extends ExpressionAdaptor<T, E, U> implements BaseEntityExpression<T, U> {
    public TaskExpression(Expression<T, U> expression){
        super(expression);
    }

    public TaskExpression(Expression<T, E> expression, Function<E, U> function){
        super(expression, function);
    }

     public TaskExpression<T, U, U> updateId(Long id){
        return new TaskExpression(this, $it -> {((Task)$it).setId(id); return this;});
     }

     public TaskExpression<T, U, U> save(UserContext userContext){
        return new TaskExpression(this, $it -> ((Task)$it).save(userContext));
     }


    public Expression<T, String> getName(){
       return apply(Task::getName);
    }
    public TaskExpression<T, U, U> updateName(String name){
       return new TaskExpression(this, $it ->  ((Task)$it).updateName(name));
    }

    public TaskStatusExpression<T, U, TaskStatus> getStatus(){
       return new TaskStatusExpression(this, $it ->  ((Task)$it).getStatus());
    }

    public TaskExpression<T, U, U> updateStatusToPlanned(){
       return new TaskExpression(this, $it ->  ((Task)$it).updateStatusToPlanned());
    }
    public TaskExpression<T, U, U> updateStatusToReady(){
       return new TaskExpression(this, $it ->  ((Task)$it).updateStatusToReady());
    }
    public TaskExpression<T, U, U> updateStatusToExecuting(){
       return new TaskExpression(this, $it ->  ((Task)$it).updateStatusToExecuting());
    }
    public TaskExpression<T, U, U> updateStatusToVerified(){
       return new TaskExpression(this, $it ->  ((Task)$it).updateStatusToVerified());
    }

    public PlatformExpression<T, U, Platform> getPlatform(){
       return new PlatformExpression(this, $it ->  ((Task)$it).getPlatform());
    }

    public TaskExpression<T, U, U> updatePlatform(Platform platform){
       return new TaskExpression(this, $it ->  ((Task)$it).updatePlatform(platform));
    }

    public TaskExecutionLogListExpression<T, U, TaskExecutionLog> getTaskExecutionLogList(){
        return new TaskExecutionLogListExpression(this, $it ->  ((Task)$it).getTaskExecutionLogList());
    }
    public TaskExpression<T, U, U> addTaskExecutionLog(TaskExecutionLog taskExecutionLog){
       return new TaskExpression(this, $it ->  ((Task)$it).addTaskExecutionLog(taskExecutionLog));
    }
}