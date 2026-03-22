use crate::api::steps::{StepId, StepKind};


// An event indicates when something HAS happened — and should result in some state change
#[derive(Clone)]
pub enum StepEvent {
    Add(StepId, StepKind),
    Start(StepId),
    Complete(StepId, Option<String>),
    Failed(StepId, Option<String>),
    Error(StepId, Option<String>),
}

pub type EventStream = Vec<StepEvent>;


/// Interface to an event source
pub trait EventSource {
    fn get_events_for(&self, id: &str) -> EventStream;
    fn save_step_event(&mut self, event: StepEvent);
}