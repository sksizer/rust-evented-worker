use serde_json::Value;
mod types;
pub use types::{StepCore, SyncStep, AsyncStep, Step, StepKind};
pub type StepId = String;

pub struct SyncStepModule {
    /// Name for the step
    pub name: String,
    pub id: String,
    pub description: String,
    pub handler: fn(Option<Value>) -> Option<Value>,
}

pub struct AsyncStepModule {
    /// Name for the step
    pub name: String,
    pub id: String,
    pub description: String,
    pub handler: fn(Option<Value>) -> String,
}