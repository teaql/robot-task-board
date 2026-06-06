package com.doublechaintech.robotkanbanservice;

import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.platform.PlatformRequest;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskRequest;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusRequest;
import io.teaql.data.UserContext;
import io.teaql.data.graphql.RootQueryType;
import java.util.Map;
public class RootQuery extends RootQueryType{
    public PlatformRequest<Platform> platforms(UserContext userContext, Map<String, Object> parameters){
        PlatformRequest<Platform> request = Q.platforms();
        if(parameters == null){
            return request;
        }
        // filterWithJsonExpr was removed in teaql 1.196
        return request;
    }
    public TaskStatusRequest<TaskStatus> taskStatuses(UserContext userContext, Map<String, Object> parameters){
        TaskStatusRequest<TaskStatus> request = Q.taskStatuses();
        if(parameters == null){
            return request;
        }
        // filterWithJsonExpr was removed in teaql 1.196
        return request;
    }
    public TaskRequest<Task> tasks(UserContext userContext, Map<String, Object> parameters){
        TaskRequest<Task> request = Q.tasks();
        if(parameters == null){
            return request;
        }
        // filterWithJsonExpr was removed in teaql 1.196
        return request;
    }
    public TaskExecutionLogRequest<TaskExecutionLog> taskExecutionLogs(UserContext userContext, Map<String, Object> parameters){
        TaskExecutionLogRequest<TaskExecutionLog> request = Q.taskExecutionLogs();
        if(parameters == null){
            return request;
        }
        // filterWithJsonExpr was removed in teaql 1.196
        return request;
    }
}