mod add_activity;
mod update_activity;
mod update_graph;

use log::trace;
use crate::api::events::Event;
use crate::api::execution::{DefaultExecutionState, ExecutionState};
use crate::api::activities::{Activity, ActivityCore, ActivityEvent, AsyncActivity, AsyncReady, SyncActivity, SyncNew, SyncReady};

use add_activity::append_activity_state;
pub use crate::runner::execution::get_execution_status::get_execution_status;
use update_activity::update;
use update_graph::update_graph;


pub struct ReduceState {
    pub execution_state: DefaultExecutionState,
    pub changed_activity: Option<Activity>,
}

/// Takes prior state + an event and returns an updated state
pub fn reduce(execution_state: DefaultExecutionState, event: &Event) -> DefaultExecutionState {
    match event {
        Event::Activity(activity_event) => reduce_activity(execution_state, activity_event),
        Event::System(_) => {
            // TODO - consider how system events affect execution state
            execution_state
        }
    }
}

fn reduce_activity(
    mut execution_state: DefaultExecutionState,
    activity_event: &ActivityEvent,
) -> DefaultExecutionState {
    match activity_event {
        ActivityEvent::AddSync(payload) => {
            let core = ActivityCore {
                id: payload.id.clone(),
                kind: payload.kind.clone(),
                config: payload.config.clone(),
                depends_on: payload.depends_on.clone(),
                attempt: 0,
                failure_count: 0,
                error_count: 0,
            };
            let activity = Activity::from(SyncActivity::from(SyncNew::new(core)));
            let state = append_activity_state(execution_state, activity).unwrap();
            update_graph(state)
        }
        ActivityEvent::AddAsync(payload) => {
            let core = ActivityCore {
                id: payload.id.clone(),
                kind: payload.kind.clone(),
                config: payload.config.clone(),
                depends_on: payload.depends_on.clone(),
                attempt: 0,
                failure_count: 0,
                error_count: 0,
            };
            let activity = Activity::from(AsyncActivity::from(AsyncReady::new(core)));
            let state = append_activity_state(execution_state, activity).unwrap();
            update_graph(state)
        }
        ActivityEvent::Start(id) => {
            let new_activity = match execution_state.get_activity_state(id) {
                Some(Activity::Sync(SyncActivity::Ready(ready))) => {
                    Activity::from(SyncActivity::from(ready.clone().start()))
                }
                Some(Activity::Async(AsyncActivity::Ready(ready))) => {
                    Activity::from(AsyncActivity::from(ready.clone().start(None)))
                }
                _ => panic!("Invalid activity state for Start event: {}", id),
            };
            update(execution_state, new_activity).unwrap()
        }
        ActivityEvent::Complete(payload) => {
            let new_activity = match execution_state.get_activity_state(&payload.id) {
                Some(Activity::Sync(SyncActivity::Running(running))) => {
                    Activity::from(SyncActivity::from(
                        running.clone().complete(payload.output.clone()),
                    ))
                }
                Some(Activity::Async(AsyncActivity::Running(running))) => {
                    Activity::from(AsyncActivity::from(
                        running.clone().complete(payload.output.clone()),
                    ))
                }
                _ => panic!(
                    "Invalid activity state for Complete event: {}",
                    payload.id
                ),
            };
            let state = update(execution_state, new_activity).unwrap();
            update_graph(state)
        }
        ActivityEvent::Failed(payload) => {
            trace!("reduce:Activity failed: {:?}", payload);
            let failure = payload.reason.as_ref().map(|r| vec![r.clone()]);
            let new_activity = match execution_state.get_activity_state(&payload.id) {
                Some(Activity::Sync(SyncActivity::Running(running))) => {
                    let mut running = running.clone();
                    running.core.failure_count += 1;
                    Activity::from(SyncActivity::from(running.fail(failure)))
                }
                Some(Activity::Async(AsyncActivity::Running(running))) => {
                    let mut running = running.clone();
                    running.core.failure_count += 1;
                    Activity::from(AsyncActivity::from(running.fail(failure)))
                }
                _ => panic!(
                    "Invalid activity state for Failed event: {}",
                    payload.id
                ),
            };
            update(execution_state, new_activity).unwrap()
        }
        ActivityEvent::Error(payload) => {
            trace!("reduce:Activity error: {:?}", payload);
            let failure = payload.reason.as_ref().map(|r| vec![r.clone()]);
            let new_activity = match execution_state.get_activity_state(&payload.id) {
                Some(Activity::Sync(SyncActivity::Running(running))) => {
                    let mut running = running.clone();
                    running.core.error_count += 1;
                    Activity::from(SyncActivity::from(running.error(failure)))
                }
                Some(Activity::Async(AsyncActivity::Running(running))) => {
                    let mut running = running.clone();
                    running.core.error_count += 1;
                    Activity::from(AsyncActivity::from(running.error(failure)))
                }
                _ => panic!(
                    "Invalid activity state for Error event: {}",
                    payload.id
                ),
            };
            update(execution_state, new_activity).unwrap()
        }
        ActivityEvent::Retry(id) => {
            trace!("reduce:Activity retry: {}", id);
            let new_activity = match execution_state.get_activity_state(id) {
                Some(Activity::Sync(SyncActivity::Failed(f))) => {
                    let mut core = f.core.clone();
                    core.attempt += 1;
                    Activity::from(SyncActivity::from(SyncReady { core, input: None }))
                }
                Some(Activity::Sync(SyncActivity::Error(e))) => {
                    let mut core = e.core.clone();
                    core.attempt += 1;
                    Activity::from(SyncActivity::from(SyncReady { core, input: None }))
                }
                Some(Activity::Async(AsyncActivity::Failed(f))) => {
                    let mut core = f.core.clone();
                    core.attempt += 1;
                    Activity::from(AsyncActivity::from(AsyncReady::new(core)))
                }
                Some(Activity::Async(AsyncActivity::Error(e))) => {
                    let mut core = e.core.clone();
                    core.attempt += 1;
                    Activity::from(AsyncActivity::from(AsyncReady::new(core)))
                }
                _ => panic!("Invalid activity state for Retry event: {}", id),
            };
            // Bypass update() since Retry intentionally revives a stopped execution state
            execution_state
                .activity_to_graph_map
                .insert(id.clone(), new_activity);
            execution_state
        }
    }
}
