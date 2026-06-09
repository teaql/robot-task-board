package com.doublechaintech.robotkanbanservice.taskstatus;

import cn.hutool.core.util.ObjectUtil;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskChecker;
import io.teaql.data.UserContext;
import io.teaql.data.checker.Checker;
import io.teaql.data.checker.ObjectLocation;
import java.math.BigDecimal;

public class TaskStatusChecker implements Checker<TaskStatus>{

    public String type(){
        return TaskStatus.INTERNAL_TYPE;
    }

    public void checkAndFix(UserContext _ctx, TaskStatus taskStatus, ObjectLocation _parentLocation){
        if(needCheck(_ctx, taskStatus)){
            markAsChecked(_ctx, taskStatus);
            doCheck(_ctx, taskStatus, _parentLocation);
        }
    }

    public void doCheck(UserContext _ctx, TaskStatus taskStatus, ObjectLocation _parentLocation){
      if(ObjectUtil.isNull(taskStatus)){
         return;
      }
      if(taskStatus.newItem()){
      }else if(taskStatus.updateItem()){
      }
      checkName(_ctx, taskStatus.getProperty(TaskStatus.NAME_PROPERTY), newLocation(_parentLocation, TaskStatus.NAME_PROPERTY));
      checkCode(_ctx, taskStatus.getProperty(TaskStatus.CODE_PROPERTY), newLocation(_parentLocation, TaskStatus.CODE_PROPERTY));
      checkColor(_ctx, taskStatus.getProperty(TaskStatus.COLOR_PROPERTY), newLocation(_parentLocation, TaskStatus.COLOR_PROPERTY));
      checkDisplayOrder(_ctx, taskStatus.getProperty(TaskStatus.DISPLAY_ORDER_PROPERTY), newLocation(_parentLocation, TaskStatus.DISPLAY_ORDER_PROPERTY));
      checkProgress(_ctx, taskStatus.getProperty(TaskStatus.PROGRESS_PROPERTY), newLocation(_parentLocation, TaskStatus.PROGRESS_PROPERTY));
      for(int i = 0; taskStatus.getTaskList() != null && i < taskStatus.getTaskList().size(); i++){
         Task task = taskStatus.getTaskList().get(i);
         _ctx.getBean(TaskChecker.class).checkAndFix(_ctx, task, newLocation(_parentLocation, TaskStatus.TASK_LIST_PROPERTY, i));
      }
    }

    public void checkName(UserContext _ctx, String name, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, name);
    if(ObjectUtil.isNull(name)){
        return;
    }
    maxStringCheck(_ctx, _parentLocation, 100, name);

    }
    public void checkCode(UserContext _ctx, String code, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, code);
    if(ObjectUtil.isNull(code)){
        return;
    }
    maxStringCheck(_ctx, _parentLocation, 100, code);

    }
    public void checkColor(UserContext _ctx, String color, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, color);
    if(ObjectUtil.isNull(color)){
        return;
    }
    maxStringCheck(_ctx, _parentLocation, 100, color);

    }
    public void checkDisplayOrder(UserContext _ctx, BigDecimal displayOrder, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, displayOrder);
    if(ObjectUtil.isNull(displayOrder)){
        return;
    }
    }
    public void checkProgress(UserContext _ctx, BigDecimal progress, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, progress);
    if(ObjectUtil.isNull(progress)){
        return;
    }
    }
}