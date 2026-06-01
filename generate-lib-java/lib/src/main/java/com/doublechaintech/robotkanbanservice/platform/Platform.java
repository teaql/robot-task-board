package com.doublechaintech.robotkanbanservice.platform;

import cn.hutool.core.util.ObjectUtil;
import cn.hutool.core.util.StrUtil;
import com.doublechaintech.robotkanbanservice.task.Task;
import io.teaql.data.BaseEntity;
import io.teaql.data.EntityStatus;
import io.teaql.data.RemoteInput;
import io.teaql.data.SmartList;
import java.time.LocalDateTime;

public class Platform extends BaseEntity implements RemoteInput {
    public static String INTERNAL_TYPE = "Platform";

    public static final String NAME_PROPERTY = "name";
    public static final String FOUNDED_PROPERTY = "founded";
    public static final String TASK_LIST_PROPERTY = "taskList";
    private String name;
    private LocalDateTime founded;
    private SmartList<Task> taskList;

    public String getName(){
        return this.name;
    }

    /**
         * @deprecated
         * Please use updateName in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setName(String name){
        this.name = name;
    }
    public LocalDateTime getFounded(){
        return this.founded;
    }

    /**
         * @deprecated
         * Please use updateFounded in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setFounded(LocalDateTime founded){
        this.founded = founded;
    }
    public SmartList<Task> getTaskList(){
        return this.taskList;
    }
    public void setTaskList(SmartList<Task> taskList){
        this.taskList = taskList;
    }
    public Platform updateName(String name){
        name = StrUtil.trim(name);
        if(ObjectUtil.equal(this.name, name)){
            return this;
        }
        handleUpdate(NAME_PROPERTY, getName(), name);
        setName(name);
        return this;
    }
    public Platform updateFounded(LocalDateTime founded){
        if(ObjectUtil.equal(this.founded, founded)){
            return this;
        }
        handleUpdate(FOUNDED_PROPERTY, getFounded(), founded);
        setFounded(founded);
        return this;
    }
    public Platform addTask(Task task){
        if (task == null){
            return this;
        }

        if(null == this.taskList){
            this.taskList = new SmartList<>();
        }

        this.taskList.add(task);
        task.cacheRelation(Task.PLATFORM_PROPERTY, this);
        return this;
    }

    public static Platform refer(Long id){
        Platform refer = new Platform();
        refer.setId(id);
        refer.set$status(EntityStatus.REFER);
        return refer;
    }
    @Override
    public String typeName(){
        return INTERNAL_TYPE;
    }

    public Platform comment(String comment){
        this.setComment(comment);
        return this;
    }

}