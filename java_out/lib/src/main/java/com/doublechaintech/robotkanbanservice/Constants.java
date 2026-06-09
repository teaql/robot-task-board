package com.doublechaintech.robotkanbanservice;

import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;

public interface Constants  {
  public static final long PLATFORM_ID = 1l;
  public static final Platform PLATFORM = Platform.refer(PLATFORM_ID);
  public static final long TASK_STATUS_ID = 1l;
  public static final TaskStatus TASK_STATUS = TaskStatus.refer(TASK_STATUS_ID);
  public static final long TASK_STATUS_PLANNED_ID = 1001l ;
  public static final TaskStatus TASK_STATUS_PLANNED = TaskStatus.refer(TASK_STATUS_PLANNED_ID);public static final long TASK_STATUS_READY_ID = 1002l ;
  public static final TaskStatus TASK_STATUS_READY = TaskStatus.refer(TASK_STATUS_READY_ID);public static final long TASK_STATUS_EXECUTING_ID = 1003l ;
  public static final TaskStatus TASK_STATUS_EXECUTING = TaskStatus.refer(TASK_STATUS_EXECUTING_ID);public static final long TASK_STATUS_VERIFIED_ID = 1004l ;
  public static final TaskStatus TASK_STATUS_VERIFIED = TaskStatus.refer(TASK_STATUS_VERIFIED_ID);
}