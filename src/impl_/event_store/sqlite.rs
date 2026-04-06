use crate::api::EventStore;
use crate::api::events::{Event, EventStream};

#[allow(dead_code)]
struct OnDiskConfig {
    path: String,
}

#[allow(dead_code)]
enum Config {
    InMemory,
    OnDisk(OnDiskConfig),
}

#[allow(dead_code)]
pub struct SqliteEventStore {}

impl EventStore for SqliteEventStore {
    fn persist(&mut self, _event: Event) -> Result<(), String> {
        todo!()
    }

    fn get_events(&self) -> EventStream {
        todo!()
    }
}

#[allow(dead_code)]
impl SqliteEventStore {
    fn new(_config: Config) -> SqliteEventStore {
        SqliteEventStore {}
    }
}
