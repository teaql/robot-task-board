package com.doublechaintech.robotkanbanservice.sampledata;

import com.doublechaintech.robotkanbanservice.*;
import com.doublechaintech.robotkanbanservice.Q;
import com.doublechaintech.robotkanbanservice.platform.Platform;
import com.doublechaintech.robotkanbanservice.task.Task;
import com.doublechaintech.robotkanbanservice.taskexecutionlog.TaskExecutionLog;
import com.doublechaintech.robotkanbanservice.taskstatus.TaskStatus;
import io.teaql.data.UserContext;
import java.util.*;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
public class SampleDataService {

    public enum SampleDataScale {
        Tiny,
        Small,
        Medium
    }

    public static class SampleDataPlan {
        public SampleDataScale scale = SampleDataScale.Small;
        public long seed = 0;

        public static SampleDataPlan small() {
            SampleDataPlan plan = new SampleDataPlan();
            plan.scale = SampleDataScale.Small;
            plan.seed = 0;
            return plan;
        }
    }

    public static class SampleDataSkipped {
        public String entity;
        public String reason;
        public SampleDataSkipped(String entity, String reason) {
            this.entity = entity;
            this.reason = reason;
        }
    }

    public static class SampleDataReport {
        public Map<String, Integer> generated;
        public List<SampleDataSkipped> skipped;
        public SampleDataReport(Map<String, Integer> generated, List<SampleDataSkipped> skipped) {
            this.generated = generated;
            this.skipped = skipped;
        }
    }

    public static class SampleDataState {
        public SampleDataPlan plan;
        public Map<String, List<Long>> references = new TreeMap<>();
        public Map<String, Integer> generated = new TreeMap<>();
        public List<SampleDataSkipped> skipped = new ArrayList<>();

        public SampleDataState(SampleDataPlan plan) {
            this.plan = plan;
        }

        public void addReference(String entity, Long id) {
            references.computeIfAbsent(entity, k -> new ArrayList<>()).add(id);
        }

        public List<Long> getIds(String entity) {
            return references.getOrDefault(entity, Collections.emptyList());
        }

        public Long pickId(String entity, int salt) {
            List<Long> ids = getIds(entity);
            if (ids.isEmpty()) {
                return null;
            }
            return ids.get(salt % ids.size());
        }

        public Long pickUnusedId(String entity, int salt, Set<Long> used) {
            List<Long> ids = getIds(entity);
            if (ids.isEmpty()) {
                return null;
            }

            Long bestId = ids.get(salt % ids.size());
            if (!used.contains(bestId)) {
                return bestId;
            }

            for (Long id : ids) {
                if (!used.contains(id)) {
                    return id;
                }
            }

            return bestId;
        }

        public void recordGenerated(String entity) {
            generated.put(entity, generated.getOrDefault(entity, 0) + 1);
        }

        public void recordSkipped(String entity, String reason) {
            skipped.add(new SampleDataSkipped(entity, reason));
        }

        public SampleDataReport intoReport() {
            return new SampleDataReport(generated, skipped);
        }
    }

    @Transactional
    public SampleDataReport generateSampleData(UserContext ctx, SampleDataPlan plan) throws Exception {
        ctx.info("Starting sample data generation. Scale: " + plan.scale + ", Seed: " + plan.seed);
        SampleDataState state = new SampleDataState(plan);

                loadRootPlatform(ctx, state);
                loadRootTaskStatus(ctx, state);


                generateTask(ctx, state);
                generateTaskExecutionLog(ctx, state);

        SampleDataReport report = state.intoReport();
        ctx.info("Sample data generation completed successfully. Generated: " + report.generated.size() + " tables, Skipped: " + report.skipped.size() + " tables.");
        return report;
    }

        private void loadRootPlatform(UserContext ctx, SampleDataState state) throws Exception {
            io.teaql.data.SmartList<Platform> list = Q.platforms().unlimited().executeForList(ctx);
            for (Platform item : list) {
                if (item.getId() != null) {
                    state.addReference("Platform", item.getId());
                }
            }
        }

        private void loadRootTaskStatus(UserContext ctx, SampleDataState state) throws Exception {
            io.teaql.data.SmartList<TaskStatus> list = Q.taskStatuses().unlimited().executeForList(ctx);
            for (TaskStatus item : list) {
                if (item.getId() != null) {
                    state.addReference("Task Status", item.getId());
                }
            }
        }


        private void generateTask(UserContext ctx, SampleDataState state) throws Exception {
                    if (state.getIds("Task Status").isEmpty()) {
                        state.recordSkipped("Task", "Required dependency Task Status is missing in reference pool");
                        ctx.info("Skipped generating Task: Required dependency Task Status is missing in reference pool.");
                        return;
                    }

                    if (state.getIds("Platform").isEmpty()) {
                        state.recordSkipped("Task", "Required dependency Platform is missing in reference pool");
                        ctx.info("Skipped generating Task: Required dependency Platform is missing in reference pool.");
                        return;
                    }


            int objectFieldsCount = 0 + 1 + 1;
            int baseFanout = Math.max(1, objectFieldsCount) * 20;

            int fanout = SampleDataScale.Tiny == state.plan.scale ? baseFanout :
                         SampleDataScale.Small == state.plan.scale ? baseFanout * 5 :
                         baseFanout * 50;

            ctx.info("Generating sample data for Task (expected: " + fanout + ")...");

            for (int i = 0; i < fanout; i++) {
                Task entity = new Task();
                Set<Long> usedRefs = new HashSet<>();

                            {
                                Long refId = state.pickUnusedId("Task Status", i, usedRefs);
                                if (refId != null) {
                                    entity.setStatus(TaskStatus.refer(refId));
                                    usedRefs.add(refId);
                                }
                            }
                            {
                                Long refId = state.pickUnusedId("Platform", i, usedRefs);
                                if (refId != null) {
                                    entity.setPlatform(Platform.refer(refId));
                                    usedRefs.add(refId);
                                }
                            }
                            entity.setName("Task Name " + (i + 1));



                entity.save(ctx);
                state.recordGenerated("Task");

                if (i % 20 == 0) {
                    ctx.info("Generating Task: " + i + "/" + fanout);
                }
                if (entity.getId() != null) {
                    state.addReference("Task", entity.getId());
                }
            }
            ctx.info("Successfully generated sample records for Task.");
        }

        private void generateTaskExecutionLog(UserContext ctx, SampleDataState state) throws Exception {
                    if (state.getIds("Task").isEmpty()) {
                        state.recordSkipped("Task Execution Log", "Required dependency Task is missing in reference pool");
                        ctx.info("Skipped generating Task Execution Log: Required dependency Task is missing in reference pool.");
                        return;
                    }


            int objectFieldsCount = 0 + 1;
            int baseFanout = Math.max(1, objectFieldsCount) * 20;

            int fanout = SampleDataScale.Tiny == state.plan.scale ? baseFanout :
                         SampleDataScale.Small == state.plan.scale ? baseFanout * 5 :
                         baseFanout * 50;

            ctx.info("Generating sample data for Task Execution Log (expected: " + fanout + ")...");

            for (int i = 0; i < fanout; i++) {
                TaskExecutionLog entity = new TaskExecutionLog();
                Set<Long> usedRefs = new HashSet<>();

                            {
                                Long refId = state.pickUnusedId("Task", i, usedRefs);
                                if (refId != null) {
                                    entity.setTask(Task.refer(refId));
                                    usedRefs.add(refId);
                                }
                            }
                            entity.setAction("string() " + (i + 1));

                            entity.setDetail("string() " + (i + 1));



                entity.save(ctx);
                state.recordGenerated("Task Execution Log");

                if (i % 20 == 0) {
                    ctx.info("Generating Task Execution Log: " + i + "/" + fanout);
                }
            }
            ctx.info("Successfully generated sample records for Task Execution Log.");
        }
}