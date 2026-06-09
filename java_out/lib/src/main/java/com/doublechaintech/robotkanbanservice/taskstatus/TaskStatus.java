package com.doublechaintech.robotkanbanservice.taskstatus;

import cn.hutool.core.util.ObjectUtil;
import cn.hutool.core.util.StrUtil;
import com.doublechaintech.robotkanbanservice.task.Task;
import io.teaql.data.BaseEntity;
import io.teaql.data.EntityStatus;
import io.teaql.data.RemoteInput;
import io.teaql.data.SmartList;
import java.math.BigDecimal;

public class TaskStatus extends BaseEntity implements RemoteInput {
    public static String INTERNAL_TYPE = "TaskStatus";

    public static final String NAME_PROPERTY = "name";
    public static final String CODE_PROPERTY = "code";
    public static final String COLOR_PROPERTY = "color";
    public static final String DISPLAY_ORDER_PROPERTY = "displayOrder";
    public static final String PROGRESS_PROPERTY = "progress";
    public static final String TASK_LIST_PROPERTY = "taskList";
    private String name;
    private String code;
    private String color;
    private BigDecimal displayOrder;
    private BigDecimal progress;
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
    public String getCode(){
        return this.code;
    }

    /**
         * @deprecated
         * Please use updateCode in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setCode(String code){
        this.code = code;
    }
    public String getColor(){
        return this.color;
    }

    /**
         * @deprecated
         * Please use updateColor in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setColor(String color){
        this.color = color;
    }
    public BigDecimal getDisplayOrder(){
        return this.displayOrder;
    }

    /**
         * @deprecated
         * Please use updateDisplayOrder in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setDisplayOrder(BigDecimal displayOrder){
        this.displayOrder = displayOrder;
    }
    public BigDecimal getProgress(){
        return this.progress;
    }

    /**
         * @deprecated
         * Please use updateProgress in your custom code instead, setter is for framework internal use only.
    */
    @Deprecated
    public void setProgress(BigDecimal progress){
        this.progress = progress;
    }
    public SmartList<Task> getTaskList(){
        return this.taskList;
    }
    public void setTaskList(SmartList<Task> taskList){
        this.taskList = taskList;
    }
    public TaskStatus updateName(String name){
        name = StrUtil.trim(name);
        if(ObjectUtil.equal(this.name, name)){
            return this;
        }
        handleUpdate(NAME_PROPERTY, getName(), name);
        setName(name);
        return this;
    }
    public TaskStatus updateCode(String code){
        code = StrUtil.trim(code);
        if(ObjectUtil.equal(this.code, code)){
            return this;
        }
        handleUpdate(CODE_PROPERTY, getCode(), code);
        setCode(code);
        return this;
    }
    public TaskStatus updateColor(String color){
        color = StrUtil.trim(color);
        if(ObjectUtil.equal(this.color, color)){
            return this;
        }
        handleUpdate(COLOR_PROPERTY, getColor(), color);
        setColor(color);
        return this;
    }
    public TaskStatus updateDisplayOrder(BigDecimal displayOrder){
        if(ObjectUtil.equal(this.displayOrder, displayOrder)){
            return this;
        }
        handleUpdate(DISPLAY_ORDER_PROPERTY, getDisplayOrder(), displayOrder);
        setDisplayOrder(displayOrder);
        return this;
    }
    public TaskStatus updateProgress(BigDecimal progress){
        if(ObjectUtil.equal(this.progress, progress)){
            return this;
        }
        handleUpdate(PROGRESS_PROPERTY, getProgress(), progress);
        setProgress(progress);
        return this;
    }
    public TaskStatus addTask(Task task){
        if (task == null){
            return this;
        }

        if(null == this.taskList){
            this.taskList = new SmartList<>();
        }

        this.taskList.add(task);
        task.cacheRelation(Task.STATUS_PROPERTY, this);
        return this;
    }

    public static TaskStatus refer(Long id){
        TaskStatus refer = new TaskStatus();
        refer.setId(id);
        refer.set$status(EntityStatus.REFER);
        return refer;
    }
    @Override
    public String typeName(){
        return INTERNAL_TYPE;
    }

    public TaskStatus comment(String comment){
        this.setComment(comment);
        return this;
    }

}