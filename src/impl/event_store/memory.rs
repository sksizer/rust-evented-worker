use std::collections::HashMap;
use crate::api::events::{EventSource, EventStream};
use crate::api::steps::StepEvent;

pub struct MemoryEventStore {
    events: HashMap<String, EventStream>,
}

impl MemoryEventStore {
    pub fn new() -> Self {
        MemoryEventStore {
            events: HashMap::new(),
        }
    }
}

impl EventSource for MemoryEventStore {
    fn get_events_for(&self, id: &str) -> EventStream {
        self.events.get(id).cloned().unwrap_or_default()
    }

    fn save_step_event(&mut self, event: StepEvent) {
        let id = event.step_id().clone();
        self.events.entry(id).or_default().push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_store_returns_empty_stream() {
        let store = MemoryEventStore::new();
        let events = store.get_events_for("nonexistent");
        assert!(events.is_empty());
    }

    #[test]
    fn save_and_retrieve_events() {
        let mut store = MemoryEventStore::new();
        store.save_step_event(StepEvent::add_sync("1", "alpha", None));
        store.save_step_event(StepEvent::complete("1", None));

        let events = store.get_events_for("1");
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn events_are_isolated_by_id() {
        let mut store = MemoryEventStore::new();
        store.save_step_event(StepEvent::add_sync("1", "alpha", None));
        store.save_step_event(StepEvent::add_sync("2", "beta", None));

        assert_eq!(store.get_events_for("1").len(), 1);
        assert_eq!(store.get_events_for("2").len(), 1);
    }
}
