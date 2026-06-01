package com.doublechaintech.robotkanbanservice.taskexecutionlog;

import io.teaql.data.graphql.BaseQueryContainer;

public class TaskExecutionLogQuery extends BaseQueryContainer {
     @Override
     protected String type() {
       return "TaskExecutionLog";
     }

}