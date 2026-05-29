use std::collections::BTreeMap;
use crate::TeaqlRuntime;
use crate::Q;

pub enum DemoDataScale {
    Tiny,
    Small,
    Medium,
}

pub struct DemoDataPlan {
    pub scale: DemoDataScale,
    pub seed: u64,
}

impl DemoDataPlan {
    pub fn small() -> Self {
        Self {
            scale: DemoDataScale::Small,
            seed: 0,
        }
    }
}

pub struct DemoDataReport {
    pub generated: BTreeMap<&'static str, usize>,
    pub skipped: Vec<DemoDataSkipped>,
}

pub struct DemoDataSkipped {
    pub entity: &'static str,
    pub reason: String,
}

pub struct DemoDataState {
    pub plan: DemoDataPlan,
    pub references: BTreeMap<&'static str, Vec<u64>>,
    pub generated: BTreeMap<&'static str, usize>,
    pub skipped: Vec<DemoDataSkipped>,
}

impl DemoDataState {
    pub fn new(plan: DemoDataPlan) -> Self {
        Self {
            plan,
            references: BTreeMap::new(),
            generated: BTreeMap::new(),
            skipped: Vec::new(),
        }
    }

    pub fn add_reference(&mut self, entity: &'static str, id: u64) {
        self.references.entry(entity).or_default().push(id);
    }

    pub fn ids(&self, entity: &'static str) -> &[u64] {
        self.references.get(entity).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn pick_id(&self, entity: &'static str, salt: usize) -> Option<u64> {
        let ids = self.ids(entity);
        if ids.is_empty() {
            None
        } else {
            Some(ids[salt % ids.len()])
        }
    }

    pub fn pick_unused_id(&self, entity: &'static str, salt: usize, used: &std::collections::HashSet<u64>) -> Option<u64> {
        let ids = self.ids(entity);
        if ids.is_empty() {
            return None;
        }

        let best_id = ids[salt % ids.len()];
        if !used.contains(&best_id) {
            return Some(best_id);
        }

        for id in ids {
            if !used.contains(id) {
                return Some(*id);
            }
        }

        Some(best_id)
    }

    pub fn record_generated(&mut self, entity: &'static str) {
        *self.generated.entry(entity).or_default() += 1;
    }

    pub fn record_skipped(&mut self, entity: &'static str, reason: String) {
        self.skipped.push(DemoDataSkipped { entity, reason });
    }

    pub fn into_report(self) -> DemoDataReport {
        DemoDataReport {
            generated: self.generated,
            skipped: self.skipped,
        }
    }
}

pub async fn generate_demo_data<C>(
    ctx: &C,
    plan: DemoDataPlan,
) -> Result<DemoDataReport, String>
where
    C: TeaqlRuntime + ?Sized,
{
    let mut state = DemoDataState::new(plan);

    load_root_platforms(ctx, &mut state).await?; //depth: 0


    generate_task_status(ctx, &mut state).await?;

    generate_tasks(ctx, &mut state).await?;


    Ok(state.into_report())
}

async fn load_root_platforms<C>(
    ctx: &C,
    state: &mut DemoDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized,
{
    let list = Q::platforms().execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("platform", item.id());
    }
    Ok(())
}


async fn generate_task_status<C>(
    ctx: &C,
    state: &mut DemoDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized,
{
    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        DemoDataScale::Tiny => base_fanout,
        DemoDataScale::Small => base_fanout * 5,
        DemoDataScale::Medium => base_fanout * 50,
    };

    for i in 0..fanout {
        let mut entity = Q::task_status().new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("platform", i as usize, &used_refs) {
                    entity.update_platform_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "Planned", i + 1));

                entity.update_code(format!("{} {}", "PLANNED", i + 1));



        let saved_node = entity.save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("task_status");

        if let Some(id_value) = saved_node.id() {
            let id = match id_value {
                teaql_core::Value::U64(id) => *id,
                teaql_core::Value::I64(id) => *id as u64,
                _ => 0,
            };
            if id > 0 {
                state.add_reference("task_status", id);
            }
        }
    }

    Ok(())
}


async fn generate_tasks<C>(
    ctx: &C,
    state: &mut DemoDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized,
{
    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        DemoDataScale::Tiny => base_fanout,
        DemoDataScale::Small => base_fanout * 5,
        DemoDataScale::Medium => base_fanout * 50,
    };

    for i in 0..fanout {
        let mut entity = Q::tasks().new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("task_status", i as usize, &used_refs) {
                    entity.update_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("platform", i as usize, &used_refs) {
                    entity.update_platform_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "Task Name", i + 1));



        entity.save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("task");

    }

    Ok(())
}
