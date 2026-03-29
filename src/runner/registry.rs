use crate::api::activities::{AsyncActivityHandler, SyncActivityHandler};
pub struct Registry {
    /// Activity modules that handle synchronous operations
    sync_definitions: Vec<SyncActivityHandler>,
    async_definitions: Vec<AsyncActivityHandler>,
}

impl Registry {
    pub fn new(
        sync_activity_modules: Option<Vec<SyncActivityHandler>>,
        async_activity_modules: Option<Vec<AsyncActivityHandler>>,
    ) -> Registry {
        Registry {
            sync_definitions: sync_activity_modules.unwrap_or_else(Vec::new),
            async_definitions: async_activity_modules.unwrap_or_else(Vec::new),
        }
    }

    pub fn register_sync(&mut self, activity: SyncActivityHandler) -> Result<(), String> {
        if self.get_sync_module(&activity.id).is_some() {
            return Err(format!("Activity with id {} already exists", activity.id));
        }
        self.sync_definitions.push(activity);
        Ok(())
    }

    pub fn register_async(&mut self, activity: AsyncActivityHandler) -> Result<(), String> {
        if self.get_async_module(&activity.id).is_some() {
            return Err(format!("Activity with id {} already exists", activity.id));
        }
        self.async_definitions.push(activity);
        Ok(())
    }

    pub fn get_sync_module(&self, activity_kind: &str) -> Option<&SyncActivityHandler> {
        self.sync_definitions
            .iter()
            .find(|s| s.id == activity_kind)
    }

    pub fn get_async_module(&self, activity_kind: &str) -> Option<&AsyncActivityHandler> {
        self.async_definitions
            .iter()
            .find(|s| s.id == activity_kind)
    }
}
