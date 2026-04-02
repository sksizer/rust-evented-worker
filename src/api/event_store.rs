use crate::api::events::{Event, EventStream};

/// The (very) simple API for getting and saving events for the system.
pub trait EventStore {
    fn persist(&mut self, event: Event) -> Result<(), String>;
    fn get_events(&self) -> EventStream;
}
