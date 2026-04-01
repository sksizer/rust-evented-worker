use serde_json::Value;
pub type ActivityId = String;
pub type ActivityKind = String;



/// The core of an activity or an activity event
#[derive(Clone, Debug)]
pub struct ActivityCore {
    pub id: ActivityId,
    pub kind: ActivityKind,
    /// Static Configuration Value for the Activity
    pub config: Option<Value>,

    // RELATIONSHIPS
    /// Indicates a partial ordering amongst related activities
    pub depends_on: Option<Vec<ActivityId>>,

    // RETRY TRACKING
    /// Current attempt number (starts at 0, incremented on each retry)
    pub attempt: u32,
    /// Total number of times this activity has failed
    pub failure_count: u32,
    /// Total number of times this activity has errored
    pub error_count: u32,
}
