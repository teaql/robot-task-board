package com.doublechaintech.robotkanbanservice.taskexecutionlog;

import cn.hutool.core.util.ObjectUtil;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskChecker;
import io.teaql.data.UserContext;
import io.teaql.data.checker.Checker;
import io.teaql.data.checker.ObjectLocation;

public class TaskExecutionLogChecker implements Checker<TaskExecutionLog>{

    public String type(){
        return TaskExecutionLog.INTERNAL_TYPE;
    }

    public void checkAndFix(UserContext _ctx, TaskExecutionLog taskExecutionLog, ObjectLocation _parentLocation){
        if(needCheck(_ctx, taskExecutionLog)){
            markAsChecked(_ctx, taskExecutionLog);
            doCheck(_ctx, taskExecutionLog, _parentLocation);
        }
    }

    public void doCheck(UserContext _ctx, TaskExecutionLog taskExecutionLog, ObjectLocation _parentLocation){
      if(ObjectUtil.isNull(taskExecutionLog)){
         return;
      }
      if(taskExecutionLog.newItem()){
      }else if(taskExecutionLog.updateItem()){
      }
      checkTask(_ctx, taskExecutionLog.getProperty(TaskExecutionLog.TASK_PROPERTY), newLocation(_parentLocation, TaskExecutionLog.TASK_PROPERTY));
      checkAction(_ctx, taskExecutionLog.getProperty(TaskExecutionLog.ACTION_PROPERTY), newLocation(_parentLocation, TaskExecutionLog.ACTION_PROPERTY));
      checkDetail(_ctx, taskExecutionLog.getProperty(TaskExecutionLog.DETAIL_PROPERTY), newLocation(_parentLocation, TaskExecutionLog.DETAIL_PROPERTY));
    }

    public void checkTask(UserContext _ctx, Task task, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, task);
    if(ObjectUtil.isNull(task)){
        return;
    }
    _ctx.getBean(TaskChecker.class).checkAndFix(_ctx, task, _parentLocation);
    }
    public void checkAction(UserContext _ctx, String action, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, action);
    if(ObjectUtil.isNull(action)){
        return;
    }
    maxStringCheck(_ctx, _parentLocation, 100, action);

    }
    public void checkDetail(UserContext _ctx, String detail, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, detail);
    if(ObjectUtil.isNull(detail)){
        return;
    }
    maxStringCheck(_ctx, _parentLocation, 100, detail);

    }
}