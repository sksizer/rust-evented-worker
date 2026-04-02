use crate::api::activities::{Activity, ActivityId, SyncActivity, SyncNew};
use crate::api::execution::{DefaultExecutionState, ExecutionGraphRelation};
use petgraph::Graph;
use std::collections::HashMap;

/// Rebuilds the execution graph from the current activity_map and re-evaluates
/// readiness for activities whose dependencies may have changed.
pub(in crate::runner::reduce) fn update_graph(
    mut execution_state: DefaultExecutionState,
) -> DefaultExecutionState {
    // 1. Rebuild graph from scratch — it's derived state from activity_map

    let mut graph = Graph::new();
    let mut node_indices: HashMap<ActivityId, petgraph::graph::NodeIndex> = HashMap::new();

    // Update ActivityID-GraphIDX Map
    for id in execution_state.activity_to_graph_map.keys() {
        // Add ActivityIds as Nodes
        let idx = graph.add_node(id.clone());
        // Add ActivityID -> Graph Idx to map
        node_indices.insert(id.clone(), idx);
    }

    for activity in execution_state.activity_to_graph_map.values() {
        if let Some(dependencies) = &activity.core().depends_on {
            let dependent_idx = node_indices[activity.id()];
            for dep_id in dependencies {
                if let Some(&dep_idx) = node_indices.get(dep_id) {
                    // Create an edge from a dependency to the dependent: A - Precedes B
                    graph.add_edge(dep_idx, dependent_idx, ExecutionGraphRelation::Precedes);
                }
            }
        }
    }

    execution_state.activity_graph = graph;

    // 2. Re-evaluate readiness for activities that aren't running or terminal
    let ids_to_check: Vec<ActivityId> = execution_state
        .activity_to_graph_map
        .keys()
        .cloned()
        .collect();

    for id in ids_to_check {
        let activity = &execution_state.activity_to_graph_map[&id];

        let should_evaluate = matches!(
            activity,
            Activity::Sync(
                SyncActivity::New(_)
                    | SyncActivity::UnfulfilledDependencies(_)
                    | SyncActivity::Ready(_)
            )
        );
        if !should_evaluate {
            continue;
        }

        let core = activity.core().clone();
        let deps_met = match &core.depends_on {
            None => true,
            Some(deps) => deps.iter().all(|dep_id| {
                execution_state
                    .activity_to_graph_map
                    .get(dep_id)
                    .map(|a| a.is_completed())
                    .unwrap_or(false)
            }),
        };

        let new_activity = if deps_met {
            Activity::from(SyncActivity::from(SyncNew::new(core).make_ready(None)))
        } else {
            Activity::from(SyncActivity::UnfulfilledDependencies(SyncNew::new(core)))
        };

        execution_state
            .activity_to_graph_map
            .insert(id, new_activity);
    }

    execution_state
}
