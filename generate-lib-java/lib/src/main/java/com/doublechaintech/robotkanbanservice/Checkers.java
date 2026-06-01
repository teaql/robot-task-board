package com.doublechaintech.robotkanbanservice;

import com.doublechaintech.robotkanbanservice.platform.PlatformChecker;
import com.doublechaintech.robotkanbanservice.task.TaskChecker;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLogChecker;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatusChecker;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class Checkers{
    @Bean
    @ConditionalOnMissingBean(name = "platformChecker")
    public PlatformChecker platformChecker(){
        return new PlatformChecker();
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskStatusChecker")
    public TaskStatusChecker taskStatusChecker(){
        return new TaskStatusChecker();
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskChecker")
    public TaskChecker taskChecker(){
        return new TaskChecker();
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskExecutionLogChecker")
    public TaskExecutionLogChecker taskExecutionLogChecker(){
        return new TaskExecutionLogChecker();
    }

}