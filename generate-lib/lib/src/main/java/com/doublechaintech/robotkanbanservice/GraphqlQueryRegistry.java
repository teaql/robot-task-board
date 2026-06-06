package com.doublechaintech.robotkanbanservice;

import com.doublechaintech.robotkanbanservice.platform.PlatformQuery;
import com.doublechaintech.robotkanbanservice.task.TaskQuery;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogQuery;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusQuery;
import org.springframework.boot.autoconfigure.AutoConfigureAfter;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
@Configuration
@AutoConfigureAfter(EntityMetaRegistry.class)
public class GraphqlQueryRegistry{
    @Bean
    @ConditionalOnMissingBean(name = "platformQuery")
    public PlatformQuery platforms(){
        return new PlatformQuery();
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskStatusQuery")
    public TaskStatusQuery taskStatuses(){
        return new TaskStatusQuery();
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskQuery")
    public TaskQuery tasks(){
        return new TaskQuery();
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskExecutionLogQuery")
    public TaskExecutionLogQuery taskExecutionLogs(){
        return new TaskExecutionLogQuery();
    }

    @Bean
    @ConditionalOnMissingBean(name = "rootQuery")
    public RootQuery rootQuery(){
        return new RootQuery();
    }
}