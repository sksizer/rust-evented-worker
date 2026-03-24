use serde_json::Value;
pub type StepId = String;
pub type StepKind = String;

/// The core of a step or a step event
#[derive(Clone, Debug)]
pub struct StepCore {
    pub id: StepId,
    pub kind: StepKind,
    pub config: Option<Value>,
}