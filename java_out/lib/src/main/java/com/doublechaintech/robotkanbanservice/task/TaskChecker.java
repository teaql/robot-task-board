package com.doublechaintech.robotkanbanservice.task;

import cn.hutool.core.util.ObjectUtil;
import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.platform.PlatformChecker;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogChecker;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusChecker;
import io.teaql.data.UserContext;
import io.teaql.data.checker.Checker;
import io.teaql.data.checker.ObjectLocation;

public class TaskChecker implements Checker<Task>{

    public String type(){
        return Task.INTERNAL_TYPE;
    }

    public void checkAndFix(UserContext _ctx, Task task, ObjectLocation _parentLocation){
        if(needCheck(_ctx, task)){
            markAsChecked(_ctx, task);
            doCheck(_ctx, task, _parentLocation);
        }
    }

    public void doCheck(UserContext _ctx, Task task, ObjectLocation _parentLocation){
      if(ObjectUtil.isNull(task)){
         return;
      }
      if(task.newItem()){
      }else if(task.updateItem()){
      }
      checkName(_ctx, task.getProperty(Task.NAME_PROPERTY), newLocation(_parentLocation, Task.NAME_PROPERTY));
      checkStatus(_ctx, task.getProperty(Task.STATUS_PROPERTY), newLocation(_parentLocation, Task.STATUS_PROPERTY));
      checkPlatform(_ctx, task.getProperty(Task.PLATFORM_PROPERTY), newLocation(_parentLocation, Task.PLATFORM_PROPERTY));
      for(int i = 0; task.getTaskExecutionLogList() != null && i < task.getTaskExecutionLogList().size(); i++){
         TaskExecutionLog taskExecutionLog = task.getTaskExecutionLogList().get(i);
         _ctx.getBean(TaskExecutionLogChecker.class).checkAndFix(_ctx, taskExecutionLog, newLocation(_parentLocation, Task.TASK_EXECUTION_LOG_LIST_PROPERTY, i));
      }
    }

    public void checkName(UserContext _ctx, String name, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, name);
    if(ObjectUtil.isNull(name)){
        return;
    }
    minStringCheck(_ctx, _parentLocation, 1, name);
    maxStringCheck(_ctx, _parentLocation, 200, name);

    }
    public void checkStatus(UserContext _ctx, TaskStatus status, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, status);
    if(ObjectUtil.isNull(status)){
        return;
    }
    _ctx.getBean(TaskStatusChecker.class).checkAndFix(_ctx, status, _parentLocation);
    }
    public void checkPlatform(UserContext _ctx, Platform platform, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, platform);
    if(ObjectUtil.isNull(platform)){
        return;
    }
    _ctx.getBean(PlatformChecker.class).checkAndFix(_ctx, platform, _parentLocation);
    }
}