package com.doublechaintech.robotkanbanservice.platform;

import cn.hutool.core.util.ObjectUtil;
import cn.hutool.core.util.ReflectUtil;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskChecker;
import io.teaql.data.UserContext;
import io.teaql.data.checker.Checker;
import io.teaql.data.checker.ObjectLocation;
import java.time.LocalDateTime;

public class PlatformChecker implements Checker<Platform>{

    public String type(){
        return Platform.INTERNAL_TYPE;
    }

    public void checkAndFix(UserContext _ctx, Platform platform, ObjectLocation _parentLocation){
        if(needCheck(_ctx, platform)){
            markAsChecked(_ctx, platform);
            doCheck(_ctx, platform, _parentLocation);
        }
    }

    public void doCheck(UserContext _ctx, Platform platform, ObjectLocation _parentLocation){
      if(ObjectUtil.isNull(platform)){
         return;
      }
      if(platform.newItem()){
        if(platform.getFounded() == null){
           platform.updateFounded(ReflectUtil.invoke(_ctx, "now"));
        }
      }else if(platform.updateItem()){
      }
      checkName(_ctx, platform.getProperty(Platform.NAME_PROPERTY), newLocation(_parentLocation, Platform.NAME_PROPERTY));
      checkFounded(_ctx, platform.getProperty(Platform.FOUNDED_PROPERTY), newLocation(_parentLocation, Platform.FOUNDED_PROPERTY));
      for(int i = 0; platform.getTaskList() != null && i < platform.getTaskList().size(); i++){
         Task task = platform.getTaskList().get(i);
         _ctx.getBean(TaskChecker.class).checkAndFix(_ctx, task, newLocation(_parentLocation, Platform.TASK_LIST_PROPERTY, i));
      }
    }

    public void checkName(UserContext _ctx, String name, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, name);
    if(ObjectUtil.isNull(name)){
        return;
    }
    maxStringCheck(_ctx, _parentLocation, 100, name);

    }
    public void checkFounded(UserContext _ctx, LocalDateTime founded, ObjectLocation _parentLocation){
    requiredCheck(_ctx, _parentLocation, founded);
    if(ObjectUtil.isNull(founded)){
        return;
    }
    }
}