use crate::api::activities::ActivityCore;
use chrono::{DateTime, Utc};
use serde_json::Value;

// --- Shared composites ---

#[derive(Clone, Debug)]
pub struct RanActivity {
    pub started_at: DateTime<Utc>,
    pub input: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct CompletedActivity {
    pub ran: RanActivity,
    pub output: Option<Value>,
}

pub type Failure = Option<Vec<String>>;

#[derive(Clone, Debug)]
pub struct FailedActivity {
    pub ran: RanActivity,
    pub failure: Failure,
}

// ============================================================
// SyncActivity typestate structs
// ============================================================

#[derive(Clone, Debug)]
pub struct SyncNew {
    pub core: ActivityCore,
}

#[derive(Clone, Debug)]
pub struct SyncReady {
    pub core: ActivityCore,
    pub input: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct SyncRunning {
    pub core: ActivityCore,
    pub ran: RanActivity,
}

#[derive(Clone, Debug)]
pub struct SyncCompleted {
    pub core: ActivityCore,
    pub completed: CompletedActivity,
}

#[derive(Clone, Debug)]
pub struct SyncFailed {
    pub core: ActivityCore,
    pub failed: FailedActivity,
}

#[derive(Clone, Debug)]
pub struct SyncError {
    pub core: ActivityCore,
    pub failed: FailedActivity,
}

// --- Creators ---

impl SyncNew {
    pub fn new(core: ActivityCore) -> Self {
        SyncNew { core }
    }
}

// --- Transitions ---

impl SyncNew {
    pub fn make_ready(self, input: Option<Value>) -> SyncReady {
        SyncReady {
            core: self.core,
            input,
        }
    }
}

impl SyncReady {
    pub fn start(self) -> SyncRunning {
        SyncRunning {
            core: self.core,
            ran: RanActivity {
                started_at: Utc::now(),
                input: self.input,
            },
        }
    }
}

impl SyncRunning {
    pub fn complete(self, output: Option<Value>) -> SyncCompleted {
        SyncCompleted {
            core: self.core,
            completed: CompletedActivity {
                ran: self.ran,
                output,
            },
        }
    }

    pub fn fail(self, failure: Failure) -> SyncFailed {
        SyncFailed {
            core: self.core,
            failed: FailedActivity {
                ran: self.ran,
                failure,
            },
        }
    }

    pub fn error(self, failure: Failure) -> SyncError {
        SyncError {
            core: self.core,
            failed: FailedActivity {
                ran: self.ran,
                failure,
            },
        }
    }
}

// ============================================================
// AsyncActivity typestate structs
// ============================================================

#[derive(Clone, Debug)]
pub struct AsyncReady {
    pub core: ActivityCore,
}

#[derive(Clone, Debug)]
pub struct AsyncRunning {
    pub core: ActivityCore,
    pub ran: RanActivity,
}

#[derive(Clone, Debug)]
pub struct AsyncCompleted {
    pub core: ActivityCore,
    pub completed: CompletedActivity,
}

#[derive(Clone, Debug)]
pub struct AsyncFailed {
    pub core: ActivityCore,
    pub failed: FailedActivity,
}

#[derive(Clone, Debug)]
pub struct AsyncError {
    pub core: ActivityCore,
    pub failed: FailedActivity,
}

// --- Creators ---

impl AsyncReady {
    pub fn new(core: ActivityCore) -> Self {
        AsyncReady { core }
    }
}

// --- Transitions ---

impl AsyncReady {
    pub fn start(self, input: Option<Value>) -> AsyncRunning {
        AsyncRunning {
            core: self.core,
            ran: RanActivity {
                started_at: Utc::now(),
                input,
            },
        }
    }
}

impl AsyncRunning {
    pub fn complete(self, output: Option<Value>) -> AsyncCompleted {
        AsyncCompleted {
            core: self.core,
            completed: CompletedActivity {
                ran: self.ran,
                output,
            },
        }
    }

    pub fn fail(self, failure: Failure) -> AsyncFailed {
        AsyncFailed {
            core: self.core,
            failed: FailedActivity {
                ran: self.ran,
                failure,
            },
        }
    }

    pub fn error(self, failure: Failure) -> AsyncError {
        AsyncError {
            core: self.core,
            failed: FailedActivity {
                ran: self.ran,
                failure,
            },
        }
    }
}

// ============================================================
// Activity enums — for when you need to hold any activity dynamically
// ============================================================

#[derive(Clone, Debug)]
pub enum SyncActivity {
    New(SyncNew),
    UnfulfilledDependencies(SyncNew),
    Ready(SyncReady),
    Running(SyncRunning),
    Completed(SyncCompleted),
    Failed(SyncFailed),
    Error(SyncError),
}

#[derive(Clone, Debug)]
pub enum AsyncActivity {
    // UnfulfilledDependencies(), TODO
    Ready(AsyncReady),
    Running(AsyncRunning),
    Completed(AsyncCompleted),
    Failed(AsyncFailed),
    Error(AsyncError),
}

#[derive(Clone, Debug)]
pub enum Activity {
    Sync(SyncActivity),
    Async(AsyncActivity),
}

// ============================================================
// Shared accessor trait + macro
// ============================================================

pub trait ActivityState {
    fn core(&self) -> &ActivityCore;
    fn input(&self) -> Option<&Value>;
}

macro_rules! impl_activity_state {
    ($t:ty, core_only) => {
        impl ActivityState for $t {
            fn core(&self) -> &ActivityCore {
                &self.core
            }
            fn input(&self) -> Option<&Value> {
                None
            }
        }
    };
    ($t:ty, direct_input) => {
        impl ActivityState for $t {
            fn core(&self) -> &ActivityCore {
                &self.core
            }
            fn input(&self) -> Option<&Value> {
                self.input.as_ref()
            }
        }
    };
    ($t:ty, ran) => {
        impl ActivityState for $t {
            fn core(&self) -> &ActivityCore {
                &self.core
            }
            fn input(&self) -> Option<&Value> {
                self.ran.input.as_ref()
            }
        }
    };
    ($t:ty, completed) => {
        impl ActivityState for $t {
            fn core(&self) -> &ActivityCore {
                &self.core
            }
            fn input(&self) -> Option<&Value> {
                self.completed.ran.input.as_ref()
            }
        }
    };
    ($t:ty, failed) => {
        impl ActivityState for $t {
            fn core(&self) -> &ActivityCore {
                &self.core
            }
            fn input(&self) -> Option<&Value> {
                self.failed.ran.input.as_ref()
            }
        }
    };
}

impl_activity_state!(SyncNew, core_only);
impl_activity_state!(SyncReady, direct_input);
impl_activity_state!(SyncRunning, ran);
impl_activity_state!(SyncCompleted, completed);
impl_activity_state!(SyncFailed, failed);
impl_activity_state!(SyncError, failed);

impl_activity_state!(AsyncReady, core_only);
impl_activity_state!(AsyncRunning, ran);
impl_activity_state!(AsyncCompleted, completed);
impl_activity_state!(AsyncFailed, failed);
impl_activity_state!(AsyncError, failed);

// --- Enum-level ActivityState impls ---

impl ActivityState for SyncActivity {
    fn core(&self) -> &ActivityCore {
        match self {
            SyncActivity::New(s) => s.core(),
            SyncActivity::UnfulfilledDependencies(s) => s.core(),
            SyncActivity::Ready(s) => s.core(),
            SyncActivity::Running(s) => s.core(),
            SyncActivity::Completed(s) => s.core(),
            SyncActivity::Failed(s) => s.core(),
            SyncActivity::Error(s) => s.core(),
        }
    }
    fn input(&self) -> Option<&Value> {
        match self {
            SyncActivity::New(s) => s.input(),
            SyncActivity::UnfulfilledDependencies(s) => s.input(),
            SyncActivity::Ready(s) => s.input(),
            SyncActivity::Running(s) => s.input(),
            SyncActivity::Completed(s) => s.input(),
            SyncActivity::Failed(s) => s.input(),
            SyncActivity::Error(s) => s.input(),
        }
    }
}

impl ActivityState for AsyncActivity {
    fn core(&self) -> &ActivityCore {
        match self {
            AsyncActivity::Ready(s) => s.core(),
            AsyncActivity::Running(s) => s.core(),
            AsyncActivity::Completed(s) => s.core(),
            AsyncActivity::Failed(s) => s.core(),
            AsyncActivity::Error(s) => s.core(),
        }
    }
    fn input(&self) -> Option<&Value> {
        match self {
            AsyncActivity::Ready(s) => s.input(),
            AsyncActivity::Running(s) => s.input(),
            AsyncActivity::Completed(s) => s.input(),
            AsyncActivity::Failed(s) => s.input(),
            AsyncActivity::Error(s) => s.input(),
        }
    }
}

// --- Activity enum convenience methods ---

impl Activity {
    pub fn core(&self) -> &ActivityCore {
        match self {
            Activity::Sync(s) => s.core(),
            Activity::Async(a) => a.core(),
        }
    }

    pub fn id(&self) -> &str {
        &self.core().id
    }
    pub fn kind(&self) -> &str {
        &self.core().kind
    }
    pub fn config(&self) -> Option<&Value> {
        self.core().config.as_ref()
    }

    pub fn input(&self) -> Option<&Value> {
        match self {
            Activity::Sync(s) => s.input(),
            Activity::Async(a) => a.input(),
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(
            self,
            Activity::Sync(
                SyncActivity::Completed(_) | SyncActivity::Failed(_) | SyncActivity::Error(_)
            ) | Activity::Async(
                AsyncActivity::Completed(_) | AsyncActivity::Failed(_) | AsyncActivity::Error(_)
            )
        )
    }

    pub fn is_runnable(&self) -> bool {
        matches!(
            self,
            Activity::Sync(SyncActivity::Ready(_))
                | Activity::Async(AsyncActivity::Ready(_)
                | AsyncActivity::Running(_))
        )
    }

    pub fn is_completed(&self) -> bool {
        matches!(
            self,
            Activity::Sync(SyncActivity::Completed(_))
                | Activity::Async(AsyncActivity::Completed(_))
        )
    }

    pub fn is_failed(&self) -> bool {
        matches!(
            self,
            Activity::Sync(SyncActivity::Failed(_)) | Activity::Async(AsyncActivity::Failed(_))
        )
    }

    pub fn is_error(&self) -> bool {
        matches!(
            self,
            Activity::Sync(SyncActivity::Error(_)) | Activity::Async(AsyncActivity::Error(_))
        )
    }
}

// --- From impls for ergonomic wrapping ---

impl From<SyncNew> for SyncActivity {
    fn from(s: SyncNew) -> Self {
        SyncActivity::New(s)
    }
}
impl From<SyncReady> for SyncActivity {
    fn from(s: SyncReady) -> Self {
        SyncActivity::Ready(s)
    }
}
impl From<SyncRunning> for SyncActivity {
    fn from(s: SyncRunning) -> Self {
        SyncActivity::Running(s)
    }
}
impl From<SyncCompleted> for SyncActivity {
    fn from(s: SyncCompleted) -> Self {
        SyncActivity::Completed(s)
    }
}
impl From<SyncFailed> for SyncActivity {
    fn from(s: SyncFailed) -> Self {
        SyncActivity::Failed(s)
    }
}
impl From<SyncError> for SyncActivity {
    fn from(s: SyncError) -> Self {
        SyncActivity::Error(s)
    }
}

impl From<AsyncReady> for AsyncActivity {
    fn from(s: AsyncReady) -> Self {
        AsyncActivity::Ready(s)
    }
}
impl From<AsyncRunning> for AsyncActivity {
    fn from(s: AsyncRunning) -> Self {
        AsyncActivity::Running(s)
    }
}
impl From<AsyncCompleted> for AsyncActivity {
    fn from(s: AsyncCompleted) -> Self {
        AsyncActivity::Completed(s)
    }
}
impl From<AsyncFailed> for AsyncActivity {
    fn from(s: AsyncFailed) -> Self {
        AsyncActivity::Failed(s)
    }
}
impl From<AsyncError> for AsyncActivity {
    fn from(s: AsyncError) -> Self {
        AsyncActivity::Error(s)
    }
}

impl From<SyncActivity> for Activity {
    fn from(s: SyncActivity) -> Self {
        Activity::Sync(s)
    }
}
impl From<AsyncActivity> for Activity {
    fn from(s: AsyncActivity) -> Self {
        Activity::Async(s)
    }
}
