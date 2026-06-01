package com.doublechaintech.robotkanbanservice;

import io.teaql.data.meta.EntityMetaFactory;
import io.teaql.data.sql.SQLEntityDescriptor;
import java.math.BigDecimal;
import java.time.LocalDateTime;
import org.springframework.beans.factory.InitializingBean;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;

@Configuration("entityMetaRegistry")
public class EntityMetaRegistry implements InitializingBean {
  @Autowired private EntityMetaFactory $factory;


  @Override
  public void afterPropertiesSet() throws Exception {
    registerPlatform();
    registerTaskStatus();
    registerTask();
    registerTaskExecutionLog();
  }

  public void setFactory(EntityMetaFactory factory){
    this.$factory = factory;
  }
  private void registerPlatform() {
      SQLEntityDescriptor entityDescriptor = new SQLEntityDescriptor();
      entityDescriptor.setType(com.doublechaintech.robotkanbanservice.platform.Platform.INTERNAL_TYPE);
      entityDescriptor.setTargetType(com.doublechaintech.robotkanbanservice.platform.Platform.class);
      entityDescriptor.with("name", "Platform")
      .with("module", "Task")
      .with("module_key", "task");

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.platform.Platform.ID_PROPERTY, Long.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.platform.Platform.NAME_PROPERTY, String.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.platform.Platform.FOUNDED_PROPERTY, LocalDateTime.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.platform.Platform.VERSION_PROPERTY, Long.class)
      ;

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.platform.Platform.ID_PROPERTY).with("isPassword", "false")
      .with("isVersion", "false")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "true")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.platform.Platform.NAME_PROPERTY).with("isPassword", "false")
      .with("max", "100")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("candidates", "Robot System")
      .with("sqlType", "VARCHAR(<max>)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.platform.Platform.FOUNDED_PROPERTY).with("isPassword", "false")
      .with("isVersion", "false")
      .with("javaType", "java.time.LocalDateTime")
      .with("candidates", "createTime()")
      .with("sqlType", "TIMESTAMP")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("mssql_sqlType", "dateTime")
      .with("isDateTime", "true")
      .with("createFunction", "now")
      .with("isDate", "true")
      .with("isString", "false")
      .with("graphqlType", "LocalTime")
      .with("isTime", "true")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.platform.Platform.VERSION_PROPERTY).with("isPassword", "false")
      .with("isVersion", "true")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "false")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      $factory.register(entityDescriptor);
  }
  private void registerTaskStatus() {
      SQLEntityDescriptor entityDescriptor = new SQLEntityDescriptor();
      entityDescriptor.setType(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.INTERNAL_TYPE);
      entityDescriptor.setTargetType(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.class);
      entityDescriptor.with("features", "status")
      .with("identifier", "code")
      .with("constant", "true")
      .with("name", "Task Status")
      .with("module", "Task")
      .with("module_key", "task");

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.ID_PROPERTY, Long.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.NAME_PROPERTY, String.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.CODE_PROPERTY, String.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.COLOR_PROPERTY, String.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.DISPLAY_ORDER_PROPERTY, BigDecimal.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.PROGRESS_PROPERTY, BigDecimal.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.VERSION_PROPERTY, Long.class)
      ;

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.ID_PROPERTY).with("isPassword", "false")
      .with("isVersion", "false")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("candidates", "1001,1002,1003,1004")
      .with("sqlType", "BIGINT")
      .with("isId", "true")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.NAME_PROPERTY).with("isPassword", "false")
      .with("max", "100")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("candidates", "Planned,Ready,Executing,Verified")
      .with("sqlType", "VARCHAR(<max>)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.CODE_PROPERTY).with("identifier", "true")
      .with("isPassword", "false")
      .with("max", "100")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("candidates", "PLANNED,READY,EXECUTING,VERIFIED")
      .with("sqlType", "VARCHAR(<max>)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.COLOR_PROPERTY).with("isPassword", "false")
      .with("max", "100")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("candidates", "#94A3B8,#3B82F6,#F59E0B,#16A34A")
      .with("sqlType", "VARCHAR(<max>)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.DISPLAY_ORDER_PROPERTY).with("isPassword", "false")
      .with("db2_sqlType", "decimal(19,7)")
      .with("isVersion", "false")
      .with("oracle_sqlType", "number(19,7)")
      .with("javaType", "java.math.BigDecimal")
      .with("candidates", "10,20,30,40")
      .with("sqlType", "NUMERIC(19,7)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "true")
      .with("isString", "false")
      .with("isDate", "false")
      .with("graphqlType", "BigDecimal")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.PROGRESS_PROPERTY).with("isPassword", "false")
      .with("db2_sqlType", "decimal(19,7)")
      .with("isVersion", "false")
      .with("oracle_sqlType", "number(19,7)")
      .with("javaType", "java.math.BigDecimal")
      .with("candidates", "0,25,50,100")
      .with("sqlType", "NUMERIC(19,7)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "true")
      .with("isString", "false")
      .with("isDate", "false")
      .with("graphqlType", "BigDecimal")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.VERSION_PROPERTY).with("isPassword", "false")
      .with("isVersion", "true")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "false")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      $factory.register(entityDescriptor);
  }
  private void registerTask() {
      SQLEntityDescriptor entityDescriptor = new SQLEntityDescriptor();
      entityDescriptor.setType(com.doublechaintech.robotkanbanservice.task.Task.INTERNAL_TYPE);
      entityDescriptor.setTargetType(com.doublechaintech.robotkanbanservice.task.Task.class);
      entityDescriptor.with("features", "custom")
      .with("name", "Task")
      .with("module", "Task")
      .with("module_key", "task");

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.task.Task.ID_PROPERTY, Long.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.task.Task.NAME_PROPERTY, String.class)
      ;

      entityDescriptor.addObjectProperty($factory, com.doublechaintech.robotkanbanservice.task.Task.STATUS_PROPERTY, com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.INTERNAL_TYPE, com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.TASK_LIST_PROPERTY, com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus.class)
      ;

      entityDescriptor.addObjectProperty($factory, com.doublechaintech.robotkanbanservice.task.Task.PLATFORM_PROPERTY, com.doublechaintech.robotkanbanservice.platform.Platform.INTERNAL_TYPE, com.doublechaintech.robotkanbanservice.platform.Platform.TASK_LIST_PROPERTY, com.doublechaintech.robotkanbanservice.platform.Platform.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.task.Task.VERSION_PROPERTY, Long.class)
      ;

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.task.Task.ID_PROPERTY).with("isPassword", "false")
      .with("isVersion", "false")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "true")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.task.Task.NAME_PROPERTY).with("isPassword", "false")
      .with("max", "200")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("sqlType", "VARCHAR(<max>)")
      .with("min", "1")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");



      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.task.Task.VERSION_PROPERTY).with("isPassword", "false")
      .with("isVersion", "true")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "false")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      $factory.register(entityDescriptor);
  }
  private void registerTaskExecutionLog() {
      SQLEntityDescriptor entityDescriptor = new SQLEntityDescriptor();
      entityDescriptor.setType(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.INTERNAL_TYPE);
      entityDescriptor.setTargetType(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.class);
      entityDescriptor.with("features", "custom")
      .with("name", "Task Execution Log")
      .with("module", "Task")
      .with("module_key", "task");

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.ID_PROPERTY, Long.class)
      ;

      entityDescriptor.addObjectProperty($factory, com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.TASK_PROPERTY, com.doublechaintech.robotkanbanservice.task.Task.INTERNAL_TYPE, com.doublechaintech.robotkanbanservice.task.Task.TASK_EXECUTION_LOG_LIST_PROPERTY, com.doublechaintech.robotkanbanservice.task.Task.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.ACTION_PROPERTY, String.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.DETAIL_PROPERTY, String.class)
      ;

      entityDescriptor.addSimpleProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.VERSION_PROPERTY, Long.class)
      ;

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.ID_PROPERTY).with("isPassword", "false")
      .with("isVersion", "false")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "true")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");


      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.ACTION_PROPERTY).with("isPassword", "false")
      .with("max", "100")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("sqlType", "VARCHAR(<max>)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.DETAIL_PROPERTY).with("isPassword", "false")
      .with("max", "100")
      .with("isVersion", "false")
      .with("javaType", "java.lang.String")
      .with("sqlType", "VARCHAR(<max>)")
      .with("isId", "false")
      .with("isBool", "false")
      .with("isBaseEntityField", "false")
      .with("isNumber", "false")
      .with("isString", "true")
      .with("isDate", "false")
      .with("graphqlType", "String")
      .with("isTime", "false")
      .with("isText", "false");

      entityDescriptor.findProperty(com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog.VERSION_PROPERTY).with("isPassword", "false")
      .with("isVersion", "true")
      .with("oracle_sqlType", "number(11)")
      .with("javaType", "java.lang.Long")
      .with("sqlType", "BIGINT")
      .with("isId", "false")
      .with("isBaseEntityField", "true")
      .with("isBool", "false")
      .with("isNumber", "false")
      .with("isString", "false")
      .with("isDate", "false")
      .with("snowflake_sqlType", "number")
      .with("graphqlType", "Long")
      .with("isTime", "false")
      .with("isText", "false");

      $factory.register(entityDescriptor);
  }
}