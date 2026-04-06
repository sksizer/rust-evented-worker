use crate::api::activities::ActivityId;

#[allow(dead_code)]
pub enum Schedule {
    NoReadyActivity,                   // Pending activities but nothing in a runnable state
    Activity(Option<Vec<ActivityId>>), // List of ready to run activities
    // Error, // Runner/Controller is in error state
    Complete, // Activities present and all have been executed
}
