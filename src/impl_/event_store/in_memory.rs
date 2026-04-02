use crate::api::EventStore;
use crate::api::events::{Event, EventStream};

pub struct InMemoryEventStore {
    events: Vec<Event>,
}

impl EventStore for InMemoryEventStore {
    fn persist(&mut self, event: Event) -> Result<(), String> {
        self.events.push(event);
        Ok(())
    }

    fn get_events(&self) -> EventStream {
        self.events.clone()
    }
}

impl InMemoryEventStore {
    pub fn new() -> InMemoryEventStore {
        InMemoryEventStore { events: vec![] }
    }

    pub fn from_events(events: Vec<Event>) -> InMemoryEventStore {
        InMemoryEventStore { events }
    }
}
