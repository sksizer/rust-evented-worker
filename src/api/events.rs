use serde_json::Value;
use crate::api::steps::{StepId, StepKind};

// An event indicates when something HAS happened — and should result in some state change
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StepEvent {
    AddSync(StepId, StepKind, Option<Value>),
    AddAsync(StepId, StepKind, Option<Value>),
    Start(StepId),
    Complete(StepId, Option<Value>),
    Failed(StepId, Option<String>),
    Error(StepId, Option<String>),
}

impl StepEvent {
    pub fn add_sync(id: impl ToString, kind: impl ToString, input: Option<Value>) -> Self {
        StepEvent::AddSync(id.to_string(), kind.to_string(), input)
    }
    pub fn add_async(id: impl ToString, kind: impl ToString, input: Option<Value>) -> Self {
        StepEvent::AddAsync(id.to_string(), kind.to_string(), input)
    }
}

pub type EventStream = Vec<StepEvent>;

/// Interface to an event source
pub trait EventSource {
    fn get_events_for(&self, id: &str) -> EventStream;
    fn save_step_event(&mut self, event: StepEvent);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_sync() {
        let step_event = StepEvent::add_async("1", "echo", None);
        assert_eq!(step_event, StepEvent::AddAsync("1".into(), "echo".into(), None));
    }

    #[test]
    fn test_add_async() {
        let step_event = StepEvent::add_async("1", "echo", None);
        assert_eq!(step_event, StepEvent::AddAsync("1".into(), "echo".into(), None));
    }
}