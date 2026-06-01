package com.doublechaintech.robotkanbanservice.taskexecutionlog;

import cn.hutool.core.util.ObjectUtil;
import cn.hutool.core.util.StrUtil;
import com.doublechaintech.robotkanbanservice.task.Task;
import io.teaql.data.BaseEntity;
import io.teaql.data.EntityStatus;
import io.teaql.data.RemoteInput;

public class TaskExecutionLog extends BaseEntity implements RemoteInput {
    public static String INTERNAL_TYPE = "TaskExecutionLog";

    public static final String TASK_PROPERTY = "task";
    public static final String ACTION_PROPERTY = "action";
    public static final String DETAIL_PROPERTY = "detail";
    private Task task;
    private String action;
    private String detail;

    public Task getTask(){
        return this.task;
    }

    /**
         * @deprecated
         * Please use updateTask in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setTask(Task task){
        this.task = task;
    }
    public String getAction(){
        return this.action;
    }

    /**
         * @deprecated
         * Please use updateAction in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setAction(String action){
        this.action = action;
    }
    public String getDetail(){
        return this.detail;
    }

    /**
         * @deprecated
         * Please use updateDetail in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setDetail(String detail){
        this.detail = detail;
    }
    public TaskExecutionLog updateTask(Task task){
        if(ObjectUtil.equal(this.task, task)){
            return this;
        }
        handleUpdate(TASK_PROPERTY, getTask(), task);
        setTask(task);
        return this;
    }
    public TaskExecutionLog updateAction(String action){
        action = StrUtil.trim(action);
        if(ObjectUtil.equal(this.action, action)){
            return this;
        }
        handleUpdate(ACTION_PROPERTY, getAction(), action);
        setAction(action);
        return this;
    }
    public TaskExecutionLog updateDetail(String detail){
        detail = StrUtil.trim(detail);
        if(ObjectUtil.equal(this.detail, detail)){
            return this;
        }
        handleUpdate(DETAIL_PROPERTY, getDetail(), detail);
        setDetail(detail);
        return this;
    }

    public static TaskExecutionLog refer(Long id){
        TaskExecutionLog refer = new TaskExecutionLog();
        refer.setId(id);
        refer.set$status(EntityStatus.REFER);
        return refer;
    }
    @Override
    public String typeName(){
        return INTERNAL_TYPE;
    }

    public TaskExecutionLog comment(String comment){
        this.setComment(comment);
        return this;
    }

}