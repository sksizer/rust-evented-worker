use crate::api::steps::StepCore;
use serde_json::Value;
use chrono::{DateTime, Utc};

// --- Shared composites ---

#[derive(Clone, Debug)]
pub struct RanStep {
    pub started_at: DateTime<Utc>,
    pub input: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct CompletedStep {
    pub ran: RanStep,
    pub output: Option<Value>,
}

pub type Failure = Option<Vec<String>>;

#[derive(Clone, Debug)]
pub struct FailedStep {
    pub ran: RanStep,
    pub failure: Failure,
}

// ============================================================
// SyncStep typestate structs
// ============================================================

#[derive(Clone, Debug)]
pub struct SyncNew { pub core: StepCore }

#[derive(Clone, Debug)]
pub struct SyncReady { pub core: StepCore, pub input: Option<Value> }

#[derive(Clone, Debug)]
pub struct SyncRunning { pub core: StepCore, pub ran: RanStep }

#[derive(Clone, Debug)]
pub struct SyncCompleted { pub core: StepCore, pub completed: CompletedStep }

#[derive(Clone, Debug)]
pub struct SyncFailed { pub core: StepCore, pub failed: FailedStep }

#[derive(Clone, Debug)]
pub struct SyncError { pub core: StepCore, pub failed: FailedStep }

// --- Creators ---

impl SyncNew {
    pub fn new(core: StepCore) -> Self {
        SyncNew { core }
    }
}

// --- Transitions ---

impl SyncNew {
    pub fn make_ready(self, input: Option<Value>) -> SyncReady {
        SyncReady { core: self.core, input }
    }
}

impl SyncReady {
    pub fn start(self) -> SyncRunning {
        SyncRunning {
            core: self.core,
            ran: RanStep { started_at: Utc::now(), input: self.input },
        }
    }
}

impl SyncRunning {
    pub fn complete(self, output: Option<Value>) -> SyncCompleted {
        SyncCompleted {
            core: self.core,
            completed: CompletedStep { ran: self.ran, output },
        }
    }

    pub fn fail(self, failure: Failure) -> SyncFailed {
        SyncFailed {
            core: self.core,
            failed: FailedStep { ran: self.ran, failure },
        }
    }

    pub fn error(self, failure: Failure) -> SyncError {
        SyncError {
            core: self.core,
            failed: FailedStep { ran: self.ran, failure },
        }
    }
}

// ============================================================
// AsyncStep typestate structs
// ============================================================

#[derive(Clone, Debug)]
pub struct AsyncReady { pub core: StepCore }

#[derive(Clone, Debug)]
pub struct AsyncRunning { pub core: StepCore, pub ran: RanStep }

#[derive(Clone, Debug)]
pub struct AsyncCompleted { pub core: StepCore, pub completed: CompletedStep }

#[derive(Clone, Debug)]
pub struct AsyncFailed { pub core: StepCore, pub failed: FailedStep }

#[derive(Clone, Debug)]
pub struct AsyncError { pub core: StepCore, pub failed: FailedStep }

// --- Creators ---

impl AsyncReady {
    pub fn new(core: StepCore) -> Self {
        AsyncReady { core }
    }
}

// --- Transitions ---

impl AsyncReady {
    pub fn start(self, input: Option<Value>) -> AsyncRunning {
        AsyncRunning {
            core: self.core,
            ran: RanStep { started_at: Utc::now(), input },
        }
    }
}

impl AsyncRunning {
    pub fn complete(self, output: Option<Value>) -> AsyncCompleted {
        AsyncCompleted {
            core: self.core,
            completed: CompletedStep { ran: self.ran, output },
        }
    }

    pub fn fail(self, failure: Failure) -> AsyncFailed {
        AsyncFailed {
            core: self.core,
            failed: FailedStep { ran: self.ran, failure },
        }
    }

    pub fn error(self, failure: Failure) -> AsyncError {
        AsyncError {
            core: self.core,
            failed: FailedStep { ran: self.ran, failure },
        }
    }
}

// ============================================================
// Step enums — for when you need to hold any step dynamically
// ============================================================

#[derive(Clone, Debug)]
pub enum SyncStep {
    New(SyncNew),
    Ready(SyncReady),
    Running(SyncRunning),
    Completed(SyncCompleted),
    Failed(SyncFailed),
    Error(SyncError),
}

#[derive(Clone, Debug)]
pub enum AsyncStep {
    Ready(AsyncReady),
    Running(AsyncRunning),
    Completed(AsyncCompleted),
    Failed(AsyncFailed),
    Error(AsyncError),
}

#[derive(Clone, Debug)]
pub enum Step {
    Sync(SyncStep),
    Async(AsyncStep),
}

// ============================================================
// Shared accessor trait + macro
// ============================================================

pub trait StepState {
    fn core(&self) -> &StepCore;
    fn input(&self) -> Option<&Value>;
}

macro_rules! impl_step_state {
    ($t:ty, core_only) => {
        impl StepState for $t {
            fn core(&self) -> &StepCore { &self.core }
            fn input(&self) -> Option<&Value> { None }
        }
    };
    ($t:ty, direct_input) => {
        impl StepState for $t {
            fn core(&self) -> &StepCore { &self.core }
            fn input(&self) -> Option<&Value> { self.input.as_ref() }
        }
    };
    ($t:ty, ran) => {
        impl StepState for $t {
            fn core(&self) -> &StepCore { &self.core }
            fn input(&self) -> Option<&Value> { self.ran.input.as_ref() }
        }
    };
    ($t:ty, completed) => {
        impl StepState for $t {
            fn core(&self) -> &StepCore { &self.core }
            fn input(&self) -> Option<&Value> { self.completed.ran.input.as_ref() }
        }
    };
    ($t:ty, failed) => {
        impl StepState for $t {
            fn core(&self) -> &StepCore { &self.core }
            fn input(&self) -> Option<&Value> { self.failed.ran.input.as_ref() }
        }
    };
}

impl_step_state!(SyncNew, core_only);
impl_step_state!(SyncReady, direct_input);
impl_step_state!(SyncRunning, ran);
impl_step_state!(SyncCompleted, completed);
impl_step_state!(SyncFailed, failed);
impl_step_state!(SyncError, failed);

impl_step_state!(AsyncReady, core_only);
impl_step_state!(AsyncRunning, ran);
impl_step_state!(AsyncCompleted, completed);
impl_step_state!(AsyncFailed, failed);
impl_step_state!(AsyncError, failed);

// --- Enum-level StepState impls ---

impl StepState for SyncStep {
    fn core(&self) -> &StepCore {
        match self {
            SyncStep::New(s) => s.core(),
            SyncStep::Ready(s) => s.core(),
            SyncStep::Running(s) => s.core(),
            SyncStep::Completed(s) => s.core(),
            SyncStep::Failed(s) => s.core(),
            SyncStep::Error(s) => s.core(),
        }
    }
    fn input(&self) -> Option<&Value> {
        match self {
            SyncStep::New(s) => s.input(),
            SyncStep::Ready(s) => s.input(),
            SyncStep::Running(s) => s.input(),
            SyncStep::Completed(s) => s.input(),
            SyncStep::Failed(s) => s.input(),
            SyncStep::Error(s) => s.input(),
        }
    }
}

impl StepState for AsyncStep {
    fn core(&self) -> &StepCore {
        match self {
            AsyncStep::Ready(s) => s.core(),
            AsyncStep::Running(s) => s.core(),
            AsyncStep::Completed(s) => s.core(),
            AsyncStep::Failed(s) => s.core(),
            AsyncStep::Error(s) => s.core(),
        }
    }
    fn input(&self) -> Option<&Value> {
        match self {
            AsyncStep::Ready(s) => s.input(),
            AsyncStep::Running(s) => s.input(),
            AsyncStep::Completed(s) => s.input(),
            AsyncStep::Failed(s) => s.input(),
            AsyncStep::Error(s) => s.input(),
        }
    }
}

// --- Step enum convenience methods ---

impl Step {
    pub fn core(&self) -> &StepCore {
        match self {
            Step::Sync(s) => s.core(),
            Step::Async(a) => a.core(),
        }
    }

    pub fn id(&self) -> &str { &self.core().id }
    pub fn kind(&self) -> &str { &self.core().kind }
    pub fn config(&self) -> Option<&Value> { self.core().config.as_ref() }

    pub fn input(&self) -> Option<&Value> {
        match self {
            Step::Sync(s) => s.input(),
            Step::Async(a) => a.input(),
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self,
            Step::Sync(SyncStep::Completed(_) | SyncStep::Failed(_) | SyncStep::Error(_)) |
            Step::Async(AsyncStep::Completed(_) | AsyncStep::Failed(_) | AsyncStep::Error(_))
        )
    }

    pub fn is_runnable(&self) -> bool {
        matches!(self,
            Step::Sync(SyncStep::Ready(_)) |
            Step::Async(AsyncStep::Ready(_) | AsyncStep::Running(_))
        )
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Step::Sync(SyncStep::Completed(_)) | Step::Async(AsyncStep::Completed(_)))
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Step::Sync(SyncStep::Failed(_)) | Step::Async(AsyncStep::Failed(_)))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Step::Sync(SyncStep::Error(_)) | Step::Async(AsyncStep::Error(_)))
    }
}

// --- From impls for ergonomic wrapping ---

impl From<SyncNew> for SyncStep { fn from(s: SyncNew) -> Self { SyncStep::New(s) } }
impl From<SyncReady> for SyncStep { fn from(s: SyncReady) -> Self { SyncStep::Ready(s) } }
impl From<SyncRunning> for SyncStep { fn from(s: SyncRunning) -> Self { SyncStep::Running(s) } }
impl From<SyncCompleted> for SyncStep { fn from(s: SyncCompleted) -> Self { SyncStep::Completed(s) } }
impl From<SyncFailed> for SyncStep { fn from(s: SyncFailed) -> Self { SyncStep::Failed(s) } }
impl From<SyncError> for SyncStep { fn from(s: SyncError) -> Self { SyncStep::Error(s) } }

impl From<AsyncReady> for AsyncStep { fn from(s: AsyncReady) -> Self { AsyncStep::Ready(s) } }
impl From<AsyncRunning> for AsyncStep { fn from(s: AsyncRunning) -> Self { AsyncStep::Running(s) } }
impl From<AsyncCompleted> for AsyncStep { fn from(s: AsyncCompleted) -> Self { AsyncStep::Completed(s) } }
impl From<AsyncFailed> for AsyncStep { fn from(s: AsyncFailed) -> Self { AsyncStep::Failed(s) } }
impl From<AsyncError> for AsyncStep { fn from(s: AsyncError) -> Self { AsyncStep::Error(s) } }

impl From<SyncStep> for Step { fn from(s: SyncStep) -> Self { Step::Sync(s) } }
impl From<AsyncStep> for Step { fn from(s: AsyncStep) -> Self { Step::Async(s) } }
