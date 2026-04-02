//! Models event sources
use crate::api::activities::{ActivityEvent, ActivityId, ActivityKind};
use serde_json::Value;

// --- System events ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SystemErrorData {
    pub activity_id: ActivityId,
    pub source: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SystemEvent {
    Error(SystemErrorData),
    /// Activity was encountered but no provider registered to handle it
    NoProvider(ActivityKind),
}

// --- Wrapper enum ---

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Event {
    Activity(ActivityEvent),
    System(SystemEvent),
}

impl Event {
    // Convenience constructors that produce Event::Activity variants
    pub fn add_sync(
        id: impl Into<String>,
        kind: impl Into<String>,
        config: Option<Value>,
        depends_on: Option<Vec<String>>,
    ) -> Self {
        Event::Activity(ActivityEvent::add_sync(id, kind, config, depends_on))
    }

    pub fn add_async(
        id: impl Into<String>,
        kind: impl Into<String>,
        config: Option<Value>,
    ) -> Self {
        Event::Activity(ActivityEvent::add_async(id, kind, config))
    }

    pub fn start(id: impl Into<String>) -> Self {
        Event::Activity(ActivityEvent::start(id))
    }

    pub fn complete(id: impl Into<String>, output: Option<Value>) -> Self {
        Event::Activity(ActivityEvent::complete(id, output))
    }

    pub fn failed(id: impl Into<String>, reason: Option<String>) -> Self {
        Event::Activity(ActivityEvent::failed(id, reason))
    }

    pub fn error(id: impl Into<String>, reason: Option<String>) -> Self {
        Event::Activity(ActivityEvent::error(id, reason))
    }

    pub fn system_error(data: SystemErrorData) -> Self {
        Event::System(SystemEvent::Error(data))
    }
}

impl From<ActivityEvent> for Event {
    fn from(e: ActivityEvent) -> Self {
        Event::Activity(e)
    }
}

impl From<SystemEvent> for Event {
    fn from(e: SystemEvent) -> Self {
        Event::System(e)
    }
}

pub type EventStream = Vec<Event>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_sync() {
        let event = Event::add_sync("1", "echo", None, None);
        assert_eq!(event, Event::add_sync("1", "echo", None, None));
    }

    #[test]
    fn test_add_async() {
        let event = Event::add_async("1", "echo", None);
        assert_eq!(event, Event::add_async("1", "echo", None));
    }
}
