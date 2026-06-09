package com.doublechaintech.robotkanbanservice.task;

import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogRequest;
import io.teaql.data.UserContext;
import io.teaql.data.graphql.BaseQueryContainer;
import java.util.Map;

public class TaskQuery extends BaseQueryContainer {
     @Override
     protected String type() {
       return "Task";
     }

         public TaskExecutionLogRequest<TaskExecutionLog> taskExecutionLogList(UserContext userContext, Map<String, Object> parameters){
             TaskExecutionLogRequest<TaskExecutionLog> request = Q.taskExecutionLogs();
             if(parameters == null){
                 return request;
             }
             // filterWithJsonExpr was removed in teaql 1.196
             return request;
         }
}