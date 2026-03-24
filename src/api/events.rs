//! Models event sources
use crate::api::steps::StepEvent;

pub type EventStream = Vec<StepEvent>;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_sync() {
        let step_event = StepEvent::add_sync("1", "echo", None);
        assert_eq!(step_event, StepEvent::add_sync("1", "echo", None));
    }

    #[test]
    fn test_add_async() {
        let step_event = StepEvent::add_async("1", "echo", None);
        assert_eq!(step_event, StepEvent::add_async("1", "echo", None));
    }
}