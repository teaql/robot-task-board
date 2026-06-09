package com.doublechaintech.robotkanbanservice.task;

import cn.hutool.core.util.ObjectUtil;
import cn.hutool.core.util.StrUtil;
import com.doublechaintech.robotkanbanservice.Constants;
import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import io.teaql.data.BaseEntity;
import io.teaql.data.EntityStatus;
import io.teaql.data.RemoteInput;
import io.teaql.data.SmartList;

public class Task extends BaseEntity implements RemoteInput {
    public static String INTERNAL_TYPE = "Task";

    public static final String NAME_PROPERTY = "name";
    public static final String STATUS_PROPERTY = "status";
    public static final String PLATFORM_PROPERTY = "platform";
    public static final String TASK_EXECUTION_LOG_LIST_PROPERTY = "taskExecutionLogList";
    private String name;
    private TaskStatus status;
    private Platform platform;
    private SmartList<TaskExecutionLog> taskExecutionLogList;

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
    public TaskStatus getStatus(){
        return this.status;
    }

    /**
         * @deprecated
         * Please use updateStatus in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setStatus(TaskStatus status){
        this.status = status;
    }
    public Platform getPlatform(){
        return this.platform;
    }

    /**
         * @deprecated
         * Please use updatePlatform in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setPlatform(Platform platform){
        this.platform = platform;
    }
    public SmartList<TaskExecutionLog> getTaskExecutionLogList(){
        return this.taskExecutionLogList;
    }
    public void setTaskExecutionLogList(SmartList<TaskExecutionLog> taskExecutionLogList){
        this.taskExecutionLogList = taskExecutionLogList;
    }
    public Task updateName(String name){
        name = StrUtil.trim(name);
        if(ObjectUtil.equal(this.name, name)){
            return this;
        }
        handleUpdate(NAME_PROPERTY, getName(), name);
        setName(name);
        return this;
    }
    private Task updateStatus(TaskStatus status){
        if(ObjectUtil.equal(this.status, status)){
            return this;
        }
        handleUpdate(STATUS_PROPERTY, getStatus(), status);
        setStatus(status);
        return this;
    }
    public Task updatePlatform(Platform platform){
        if(ObjectUtil.equal(this.platform, platform)){
            return this;
        }
        handleUpdate(PLATFORM_PROPERTY, getPlatform(), platform);
        setPlatform(platform);
        return this;
    }
    public Task addTaskExecutionLog(TaskExecutionLog taskExecutionLog){
        if (taskExecutionLog == null){
            return this;
        }

        if(null == this.taskExecutionLogList){
            this.taskExecutionLogList = new SmartList<>();
        }

        this.taskExecutionLogList.add(taskExecutionLog);
        taskExecutionLog.cacheRelation(TaskExecutionLog.TASK_PROPERTY, this);
        return this;
    }
    public boolean isStatusPlanned(){
        return ObjectUtil.equals(getStatus(), Constants.TASK_STATUS_PLANNED);
    }

    public Task updateStatusToPlanned(){
        return updateStatus(Constants.TASK_STATUS_PLANNED);
    }
    public boolean isStatusReady(){
        return ObjectUtil.equals(getStatus(), Constants.TASK_STATUS_READY);
    }

    public Task updateStatusToReady(){
        return updateStatus(Constants.TASK_STATUS_READY);
    }
    public boolean isStatusExecuting(){
        return ObjectUtil.equals(getStatus(), Constants.TASK_STATUS_EXECUTING);
    }

    public Task updateStatusToExecuting(){
        return updateStatus(Constants.TASK_STATUS_EXECUTING);
    }
    public boolean isStatusVerified(){
        return ObjectUtil.equals(getStatus(), Constants.TASK_STATUS_VERIFIED);
    }

    public Task updateStatusToVerified(){
        return updateStatus(Constants.TASK_STATUS_VERIFIED);
    }

    public static Task refer(Long id){
        Task refer = new Task();
        refer.setId(id);
        refer.set$status(EntityStatus.REFER);
        return refer;
    }
    @Override
    public String typeName(){
        return INTERNAL_TYPE;
    }

    public Task comment(String comment){
        this.setComment(comment);
        return this;
    }

}