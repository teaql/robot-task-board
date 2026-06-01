package com.doublechaintech.robotkanbanservice;

import io.teaql.data.criteria.Operator;

public class Q  {
  public static com.doublechaintech.robotkanbanservice.platform.PlatformRequest<com.doublechaintech.robotkanbanservice.platform.Platform> platforms(){
      return new com.doublechaintech.robotkanbanservice.platform.PlatformRequest(com.doublechaintech.robotkanbanservice.platform.Platform.class).selectSelf().withVersion(Operator.GREATER_THAN, 0l);
  }
  public static com.doublechaintech.robotkanbanservice.platform.PlatformRequest<com.doublechaintech.robotkanbanservice.platform.Platform> platformsWithMinimalFields(){
      return new com.doublechaintech.robotkanbanservice.platform.PlatformRequest(com.doublechaintech.robotkanbanservice.platform.Platform.class).withVersion(Operator.GREATER_THAN, 0l);
  }


  public static com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest<com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus> taskStatuses(){
      return new com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.class).selectSelf().withVersion(Operator.GREATER_THAN, 0l);
  }
  public static com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest<com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus> taskStatusesWithMinimalFields(){
      return new com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.class).withVersion(Operator.GREATER_THAN, 0l);
  }


  public static com.doublechaintech.robotkanbanservice.task.TaskRequest<com.doublechaintech.robotkanbanservice.task.Task> tasks(){
      return new com.doublechaintech.robotkanbanservice.task.TaskRequest(com.doublechaintech.robotkanbanservice.task.Task.class).selectSelf().withVersion(Operator.GREATER_THAN, 0l);
  }
  public static com.doublechaintech.robotkanbanservice.task.TaskRequest<com.doublechaintech.robotkanbanservice.task.Task> tasksWithMinimalFields(){
      return new com.doublechaintech.robotkanbanservice.task.TaskRequest(com.doublechaintech.robotkanbanservice.task.Task.class).withVersion(Operator.GREATER_THAN, 0l);
  }


  public static com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest<com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog> taskExecutionLogs(){
      return new com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.class).selectSelf().withVersion(Operator.GREATER_THAN, 0l);
  }
  public static com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest<com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog> taskExecutionLogsWithMinimalFields(){
      return new com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.class).withVersion(Operator.GREATER_THAN, 0l);
  }


}