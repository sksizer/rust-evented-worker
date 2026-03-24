//! Models event sources
use crate::api::steps::StepEvent;

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
        let step_event = StepEvent::add_sync("1", "echo", None);
        assert_eq!(step_event, StepEvent::add_sync("1", "echo", None));
    }

    #[test]
    fn test_add_async() {
        let step_event = StepEvent::add_async("1", "echo", None);
        assert_eq!(step_event, StepEvent::add_async("1", "echo", None));
    }
}