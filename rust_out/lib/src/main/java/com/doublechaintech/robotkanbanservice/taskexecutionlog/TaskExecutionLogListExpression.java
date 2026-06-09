package com.doublechaintech.robotkanbanservice.taskexecutionlog;

import io.teaql.data.SmartList;
import io.teaql.data.value.Expression;
import io.teaql.data.value.SmartListExpression;
import java.util.function.Function;

public class TaskExecutionLogListExpression<T, E, U extends TaskExecutionLog> extends SmartListExpression<T, E, U> {
    public TaskExecutionLogListExpression(Expression<T, SmartList<U>> expression){
        super(expression);
    }

    public TaskExecutionLogListExpression(Expression<T, E> expression, Function<E, SmartList<U>> function){
        super(expression, function);
    }

    public TaskExecutionLogExpression<T, U, U> first() {
       return new TaskExecutionLogExpression(super.first());
    }

    public TaskExecutionLogExpression<T, U, U> get(int index) {
      return new TaskExecutionLogExpression(super.get(index));
    }
}