package com.doublechaintech.robotkanbanservice.platform;

import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskRequest;
import io.teaql.data.UserContext;
import io.teaql.data.graphql.BaseQueryContainer;
import java.util.Map;

public class PlatformQuery extends BaseQueryContainer {
     @Override
     protected String type() {
       return "Platform";
     }

         public TaskRequest<Task> taskList(UserContext userContext, Map<String, Object> parameters){
             TaskRequest<Task> request = Q.tasks();
             if(parameters == null){
                 return request;
             }
             // filterWithJsonExpr was removed in teaql 1.196
             return request;
         }
}