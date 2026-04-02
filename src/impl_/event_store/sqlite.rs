use crate::api::EventStore;
use crate::api::events::{Event, EventStream};
use crate::in_memory::InMemoryEventStore;
use std::path::PathBuf;

struct OnDiskConfig {
    path: String,
}

enum Config {
    InMemory,
    OnDisk(OnDiskConfig),
}

pub struct SqliteEventStore {}

impl EventStore for SqliteEventStore {
    fn persist(&mut self, event: Event) -> Result<(), String> {
        todo!()
    }

    fn get_events(&self) -> EventStream {
        todo!()
    }
}

impl SqliteEventStore {
    pub fn new(config: Config) -> SqliteEventStore {
        SqliteEventStore {}
    }
}
