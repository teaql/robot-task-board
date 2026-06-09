package com.doublechaintech.robotkanbanservice;

import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.platform.PlatformExpression;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskExpression;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogExpression;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusExpression;
import io.teaql.data.value.ValueExpression;

public class E  {
  public static PlatformExpression<Platform, Platform, Platform> platform(Platform platform){
      return new PlatformExpression(new ValueExpression(platform));
  }
  public static TaskStatusExpression<TaskStatus, TaskStatus, TaskStatus> taskStatus(TaskStatus taskStatus){
      return new TaskStatusExpression(new ValueExpression(taskStatus));
  }
  public static TaskExpression<Task, Task, Task> task(Task task){
      return new TaskExpression(new ValueExpression(task));
  }
  public static TaskExecutionLogExpression<TaskExecutionLog, TaskExecutionLog, TaskExecutionLog> taskExecutionLog(TaskExecutionLog taskExecutionLog){
      return new TaskExecutionLogExpression(new ValueExpression(taskExecutionLog));
  }
}