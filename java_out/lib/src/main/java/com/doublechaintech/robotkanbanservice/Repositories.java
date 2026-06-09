package com.doublechaintech.robotkanbanservice;

import io.teaql.data.Repository;
import io.teaql.data.meta.EntityMetaFactory;
import io.teaql.data.sql.SQLRepository;
import javax.sql.DataSource;
import org.springframework.boot.autoconfigure.AutoConfigureAfter;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.DependsOn;

@Configuration
@AutoConfigureAfter(EntityMetaRegistry.class)
public class Repositories{
    @Bean
    @ConditionalOnMissingBean(name = "platformRepository")
    @DependsOn("entityMetaRegistry")
    public Repository<com.doublechaintech.robotkanbanservice.platform.Platform> platformRepository(EntityMetaFactory factory, DataSource dataSource){
        return new SQLRepository<>(factory.resolveEntityDescriptor(com.doublechaintech.robotkanbanservice.platform.Platform.INTERNAL_TYPE), dataSource);
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskStatusRepository")
    @DependsOn("entityMetaRegistry")
    public Repository<com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus> taskStatusRepository(EntityMetaFactory factory, DataSource dataSource){
        return new SQLRepository<>(factory.resolveEntityDescriptor(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.INTERNAL_TYPE), dataSource);
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskRepository")
    @DependsOn("entityMetaRegistry")
    public Repository<com.doublechaintech.robotkanbanservice.task.Task> taskRepository(EntityMetaFactory factory, DataSource dataSource){
        return new SQLRepository<>(factory.resolveEntityDescriptor(com.doublechaintech.robotkanbanservice.task.Task.INTERNAL_TYPE), dataSource);
    }
    @Bean
    @ConditionalOnMissingBean(name = "taskExecutionLogRepository")
    @DependsOn("entityMetaRegistry")
    public Repository<com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog> taskExecutionLogRepository(EntityMetaFactory factory, DataSource dataSource){
        return new SQLRepository<>(factory.resolveEntityDescriptor(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.INTERNAL_TYPE), dataSource);
    }

}