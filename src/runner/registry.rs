use crate::api::steps::{AsyncStepModule, SyncStepModule};
pub struct Registry {
    /// Step modules that handle synchronous operations
    sync_definitions: Vec<SyncStepModule>,
    async_definitions:Vec<AsyncStepModule>
}

impl Registry {
    pub fn new(sync_step_modules: Option<Vec<SyncStepModule>>, async_step_modules:Option<Vec<AsyncStepModule>>) -> Registry {
        Registry {
            sync_definitions: sync_step_modules.unwrap_or_else(Vec::new),
            async_definitions:async_step_modules.unwrap_or_else(Vec::new)
        }
    }

    pub fn register_sync(&mut self, step: SyncStepModule) {
        self.sync_definitions.push(step);
    }

    pub fn get_sync_module(&self, step_kind:&str) -> Option<&SyncStepModule> {
        self.sync_definitions.iter().find(|s| s.id == step_kind)
    }
}

