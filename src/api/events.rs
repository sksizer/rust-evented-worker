//! Models event sources
use crate::api::steps::{StepEvent, StepId};
use serde_json::Value;

// --- System events ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SystemErrorData {
    pub step_id: StepId,
    pub source: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SystemEvent {
    Error(SystemErrorData),
}

// --- Wrapper enum ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Event {
    Step(StepEvent),
    System(SystemEvent),
}

impl Event {
    // Convenience constructors that produce Event::Step variants
    pub fn add_sync(id: impl Into<String>, kind: impl Into<String>, config: Option<Value>) -> Self {
        Event::Step(StepEvent::add_sync(id, kind, config))
    }

    pub fn add_async(id: impl Into<String>, kind: impl Into<String>, config: Option<Value>) -> Self {
        Event::Step(StepEvent::add_async(id, kind, config))
    }

    pub fn start(id: impl Into<String>) -> Self {
        Event::Step(StepEvent::start(id))
    }

    pub fn complete(id: impl Into<String>, output: Option<Value>) -> Self {
        Event::Step(StepEvent::complete(id, output))
    }

    pub fn failed(id: impl Into<String>, reason: Option<String>) -> Self {
        Event::Step(StepEvent::failed(id, reason))
    }

    pub fn error(id: impl Into<String>, reason: Option<String>) -> Self {
        Event::Step(StepEvent::error(id, reason))
    }

    pub fn system_error(data: SystemErrorData) -> Self {
        Event::System(SystemEvent::Error(data))
    }
}

impl From<StepEvent> for Event {
    fn from(e: StepEvent) -> Self { Event::Step(e) }
}

impl From<SystemEvent> for Event {
    fn from(e: SystemEvent) -> Self { Event::System(e) }
}

pub type EventStream = Vec<Event>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_sync() {
        let step_event = Event::add_sync("1", "echo", None);
        assert_eq!(step_event, Event::add_sync("1", "echo", None));
    }

    #[test]
    fn test_add_async() {
        let step_event = Event::add_async("1", "echo", None);
        assert_eq!(step_event, Event::add_async("1", "echo", None));
    }
}
