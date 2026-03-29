use serde_json::Value;
pub type ActivityId = String;
pub type ActivityKind = String;

/// The core of an activity or an activity event
#[derive(Clone, Debug)]
pub struct ActivityCore {
    pub id: ActivityId,
    pub kind: ActivityKind,
    pub config: Option<Value>,
}
