package com.doublechaintech.robotkanbanservice;

import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.task.TaskRequest;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import com.doublechaintech.robotkanbanservice.platform.Platform;
import io.teaql.data.TQLContext;
import io.teaql.data.UserContext;
import io.teaql.data.Repository;
import io.teaql.data.meta.EntityMetaFactory;
import io.teaql.data.meta.PropertyDescriptor;
import io.teaql.data.meta.EntityDescriptor;
import io.teaql.data.sql.SQLRepository;
import io.teaql.data.sql.GenericSQLProperty;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class TestController {

  @Autowired private EntityMetaFactory factory;

  @GetMapping("/test/dump")
  public Object dump(@TQLContext UserContext context) {
    EntityDescriptor taskDesc = factory.resolveEntityDescriptor(Task.INTERNAL_TYPE);
    StringBuilder sb = new StringBuilder();
    Repository repo = context.resolveRepository(Task.INTERNAL_TYPE);
    sb.append("Repository for Task: " + (repo != null ? repo.getClass().getName() : "null") + "\n");
    for (PropertyDescriptor prop : taskDesc.getProperties()) {
      if (prop instanceof GenericSQLProperty) {
         GenericSQLProperty sp = (GenericSQLProperty) prop;
         sb.append(sp.getName() + " -> " + sp.getColumnName() + " (" + sp.getTableName() + ")\n");
      } else {
         sb.append(prop.getName() + " -> NOT GenericSQLProperty (" + prop.getClass().getName() + ")\n");
      }
    }
    sb.append("\n=================\n");
    EntityDescriptor logDesc = factory.resolveEntityDescriptor("TaskExecutionLog");
    if (logDesc != null) {
      for (PropertyDescriptor prop : logDesc.getProperties()) {
        if (prop instanceof GenericSQLProperty) {
           GenericSQLProperty sp = (GenericSQLProperty) prop;
           sb.append(sp.getName() + " -> " + sp.getColumnName() + " (" + sp.getTableName() + ")\n");
        } else {
           sb.append(prop.getName() + " -> NOT GenericSQLProperty (" + prop.getClass().getName() + ")\n");
        }
      }
    }
    return sb.toString();
  }

  @GetMapping("/test/query")
  public Object queryTasks(@TQLContext UserContext context) {
    try {
      TaskRequest Q = new TaskRequest(Task.class);
      Q.selectName().selectStatus().selectPlatform().unlimited();
      Object tasks = Q.executeForList(context);
      io.teaql.data.SmartList<Task> taskList = (io.teaql.data.SmartList<Task>) tasks;
      return taskList.stream().map(task -> {
        java.util.Map<String, Object> map = new java.util.HashMap<>();
        map.put("id", task.getId());
        map.put("name", task.getName());
        if (task.getStatus() != null) map.put("statusId", task.getStatus().getId());
        if (task.getPlatform() != null) map.put("platformId", task.getPlatform().getId());
        return map;
      }).collect(java.util.stream.Collectors.toList());
    } catch (Exception e) {
      e.printStackTrace();
      return "Failed to query tasks: " + e.getMessage();
    }
  }

  @GetMapping("/test/modify")
  public Object modifyTask(@TQLContext UserContext context) {
    try {
      Task task = new Task();
      task.setName("Test Task " + System.currentTimeMillis());
      TaskStatus status = new TaskStatus();
      status.setId(1L);
      status.set$status(io.teaql.data.EntityStatus.REFER);
      Platform platform = new Platform();
      platform.setId(1L);
      platform.set$status(io.teaql.data.EntityStatus.REFER);
      task.setStatus(status);
      task.setPlatform(platform);
      context.info("BEFORE SAVE: status=" + status.get$status());
      context.info("BEFORE SAVE: task hash=" + System.identityHashCode(task));
      context.info("BEFORE SAVE: task.getStatus()=" + task.getStatus());
      context.checkAndFix(task);
      java.util.Map<String, Object> response = new java.util.HashMap<>();
      
      io.teaql.data.graph.GraphMutationPlan plan = io.teaql.data.graph.GraphMutationEngine.planGraph(context, task);
      java.util.List<Object> traceOutput = new java.util.ArrayList<>();
      for (io.teaql.data.graph.GraphMutationBatch batch : plan.getBatches()) {
          for (io.teaql.data.graph.GraphMutationBatch.Item item : batch.getItems()) {
              if (item.getScopeToken() != null) {
                  java.util.List<io.teaql.data.graph.TraceNode> chain = item.getScopeToken().recoverTraceChain();
                  java.util.List<String> chainStr = new java.util.ArrayList<>();
                  for (io.teaql.data.graph.TraceNode node : chain) {
                      chainStr.add(node.getEntityType() + "[" + node.getEntityId() + "](" + node.getComment() + ")");
                  }
                  java.util.Map<String, Object> traceInfo = new java.util.HashMap<>();
                  traceInfo.put("entity", batch.getEntity());
                  traceInfo.put("operation", batch.getKind().name());
                  traceInfo.put("traceChain", chainStr);
                  traceOutput.add(traceInfo);
              } else {
                  java.util.Map<String, Object> traceInfo = new java.util.HashMap<>();
                  traceInfo.put("entity", batch.getEntity());
                  traceInfo.put("operation", batch.getKind().name());
                  traceInfo.put("traceChain", "No TraceScopeToken");
                  traceOutput.add(traceInfo);
              }
          }
      }
      response.put("traces", traceOutput);

      context.saveGraph(task);
      context.info("AFTER SAVE: status=" + status.get$status());
      response.put("message", "Saved task successfully!");
      return response;
    } catch (Exception e) {
      e.printStackTrace();
      return "Failed to save task: " + e.getMessage();
    }
  }

  @GetMapping("/test/modify_complex")
  public Object modifyComplexTask(@TQLContext UserContext context) {
    try {
      TaskRequest Q = new TaskRequest(Task.class);
      Q.selectName().selectStatus().selectPlatform().unlimited();
      Object tasksObj = Q.executeForList(context);
      io.teaql.data.SmartList<Task> taskList = (io.teaql.data.SmartList<Task>) tasksObj;

      Task taskToUpdate;
      if (taskList.isEmpty()) {
          taskToUpdate = new Task();
          taskToUpdate.setName("Complex Task " + System.currentTimeMillis());
          TaskStatus status = new TaskStatus(); status.setId(1L); status.set$status(io.teaql.data.EntityStatus.REFER);
          Platform platform = new Platform(); platform.setId(1L); platform.set$status(io.teaql.data.EntityStatus.REFER);
          taskToUpdate.setStatus(status);
          taskToUpdate.setPlatform(platform);
          taskToUpdate.setComment("Creating initial task for complex update");
          context.saveGraph(taskToUpdate);
      } else {
          taskToUpdate = taskList.get(0);
          taskToUpdate.set$status(io.teaql.data.EntityStatus.UPDATED);
      }

      taskToUpdate.setName("Updated Task " + System.currentTimeMillis());
      taskToUpdate.setComment("Updating task name");

      com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog log = new com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog();
      log.setAction("UPDATE_NAME");
      log.setDetail("Execution log at " + System.currentTimeMillis());
      log.setComment("Adding execution log");

      log.updateTask(taskToUpdate);
      if (taskToUpdate.getTaskExecutionLogList() == null) {
          taskToUpdate.setTaskExecutionLogList(new io.teaql.data.SmartList<>());
      }
      taskToUpdate.getTaskExecutionLogList().add(log);

      context.info("DEBUG: log is Entity? " + (log instanceof io.teaql.data.Entity));
      context.info("DEBUG: log list size: " + taskToUpdate.getTaskExecutionLogList().size());
      context.info("DEBUG: list is java.util.List? " + (taskToUpdate.getTaskExecutionLogList() instanceof java.util.List));
      context.info("DEBUG: list name property matches? " + taskToUpdate.getProperty("taskExecutionLogList"));

      java.util.Map<String, Object> response = new java.util.HashMap<>();
      io.teaql.data.graph.GraphMutationPlan plan = io.teaql.data.graph.GraphMutationEngine.planGraph(context, taskToUpdate);
      java.util.List<Object> traceOutput = new java.util.ArrayList<>();
      for (io.teaql.data.graph.GraphMutationBatch batch : plan.getBatches()) {
          for (io.teaql.data.graph.GraphMutationBatch.Item item : batch.getItems()) {
              if (item.getScopeToken() != null) {
                  java.util.List<io.teaql.data.graph.TraceNode> chain = item.getScopeToken().recoverTraceChain();
                  java.util.List<String> chainStr = new java.util.ArrayList<>();
                  for (io.teaql.data.graph.TraceNode node : chain) {
                      chainStr.add(node.getEntityType() + "[" + node.getEntityId() + "](" + node.getComment() + ")");
                  }
                  java.util.Map<String, Object> traceInfo = new java.util.HashMap<>();
                  traceInfo.put("entity", batch.getEntity());
                  traceInfo.put("operation", batch.getKind().name());
                  traceInfo.put("traceChain", chainStr);
                  traceOutput.add(traceInfo);
              }
          }
      }
      response.put("traces", traceOutput);

      context.saveGraph(taskToUpdate);
      response.put("message", "Updated task and added execution log successfully!");
      return response;
    } catch (Exception e) {
      e.printStackTrace();
      return "Failed to complex modify task: " + e.getMessage();
    }
  }
}
