# evented-worker

An event-sourced workflow engine for Rust. Define workflows as sequences of named steps, execute them with a `Controller`, and recover from any point in execution by replaying the event log.

## Concepts

- **Step** — a unit of work with a `kind` (handler name), optional config, and optional input/output
- **StepEvent** — an immutable record of something that happened (step added, started, completed, failed)
- **EventStream** — the ordered log of all events; the authoritative source of truth for execution state
- **Registry** — a catalog of named step handlers available at runtime
- **Controller** — orchestrates the execution loop: restores state, schedules steps, executes them, records results

State is never stored directly. It is always derived by replaying the event stream through the `reduce` function.

## Workspace

```
evented-worker/          # Main library crate
crates/serde-command/    # Serializable shell command builder
```

## Quick Start

### Controller (high-level)

Define an event log with your steps, hand it to a `Controller`, and call `start()`:

```rust
use evented_worker::runner::Controller;
use evented_worker::fixtures::get_registry;
use evented_worker::steps::shell::{StepParameters, get_step};
use serde_command::ShellCommand;
use std::cell::RefCell;
use std::rc::Rc;

let event_log = Rc::new(RefCell::new(vec![
    get_step(
        "format-and-lint",
        StepParameters {
            commands: vec![
                ShellCommand::new("cargo").arg("fmt"),
                ShellCommand::new("cargo").args(["clippy", "--fix"]),
            ],
        },
    ),
]));

let mut controller = Controller::new(get_registry(), event_log);
let state = controller.start();
```

Hook into the loop for observability:

```rust
// Called every iteration with current execution state
controller.on_loop(|state| {
    evented_worker::view::summarize::execution_state(&state);
});

// Called each time a new StepEvent is produced
controller.on_event(|event| {
    println!("event: {:?}", event);
});
```

### Low-level API

Use `scheduler`, `executor`, and `reduce` directly for full control:

```rust
use evented_worker::{runner, view};
use evented_worker::api::steps::StepEvent;
use evented_worker::runner::Registry;
use evented_worker::fixtures::{get_registry, get_test_step_modules};
use serde_json::json;

let registry = Registry::new(Some(get_test_step_modules()), None);
let event_stream = vec![
    StepEvent::add_sync("1", "echo", Some(json!({ "config": "hello" }))),
];

let mut state = runner::restore(&event_stream);

let step_id = runner::scheduler(&state).unwrap().id().to_string();
let input = runner::resolve_prior_output(&state, &step_id);
state = runner::reduce(state, &StepEvent::start(step_id, input));

let next_step = runner::scheduler(&state).unwrap();
let result_event = runner::executor(&registry, next_step);
state = runner::reduce(state, &result_event);

view::summarize::execution_state(&state);
```

### Chaining steps

Steps pass output to the next step automatically. The `Controller` uses `resolve_prior_output` to wire outputs as inputs:

```rust
let event_log = Rc::new(RefCell::new(vec![
    StepEvent::add_sync("0", "fixed_output", Some(json!({ "config": "DATA" }))),
    StepEvent::add_sync("1", "echo", None),  // receives output of step 0
    StepEvent::add_sync("2", "echo", None),  // receives output of step 1
]));
```

## How It Works

The execution loop follows this pattern each iteration:

```
EventStream (Vec<StepEvent>)
    ↓ restore()
DefaultExecutionState
    ↓ scheduler()     → pick next runnable step
    ↓ reduce()        → emit Start event
    ↓ executor()      → call handler → produce result StepEvent
    ↓ reduce()        → apply result to state
    [repeat until Finished or Failed]
```

Recovery works by replaying the event stream from the beginning. If a process crashes mid-execution, replaying the existing events restores exact state — already-completed steps are not re-executed.

## serde-command

A serializable wrapper for `std::process::Command`, useful when step configurations need to be stored in the event log as JSON:

```rust
use serde_command::ShellCommand;

let cmd = ShellCommand::new("cargo")
    .arg("build")
    .args(["--release", "--bin", "myapp"])
    .env("RUST_LOG", "debug")
    .working_dir("/path/to/project");

// Converts to std::process::Command
let mut process_cmd: std::process::Command = cmd.into();
```

Enable the `tokio` feature for async support:

```toml
[dependencies]
serde-command = { path = "crates/serde-command", features = ["tokio"] }
```

## Built-in Step Handlers

| Kind | Description |
|------|-------------|
| `shell` | Executes one or more `ShellCommand`s sequentially |
| `echo` | Passes input through unchanged (useful for testing/chaining) |
| `fixed_output` | Always emits a configured value as output |

## Writing a Custom Step Handler

```rust
use evented_worker::api::steps::{SyncStepHandler, StepConfig, StepInput};
use serde_json::{json, Value};

let handler = SyncStepHandler {
    name: "my-step".to_string(),
    id: "my-step".to_string(),
    description: "Does something useful".to_string(),
    validate_config: None,
    validate_input: None,
    handler: |config: StepConfig, input: StepInput| -> Result<Value, Vec<String>> {
        // config.0 and input.0 are Option<serde_json::Value>
        Ok(json!({ "result": "done" }))
    },
};
```

Register it in a `Registry` and pass the registry to the `Controller`.

## Running Examples

```sh
cargo run --example misc
cargo run --example quality_check
cargo run --example update_readme
```

## Status

Early-stage library. The core event-sourcing loop, synchronous step execution, and shell command integration are functional.

Known limitations:
- **Events not persisted**: the controller holds state in memory only — a crash loses all progress
- **Async step execution**: registered but hits `unimplemented!()` at runtime
- **Validators ignored**: `validate_config`/`validate_input` on handlers are never called by the executor
- **Sequential scheduling only**: no DAG-based dependency model; steps run left-to-right
- **No retry or compensation API**: failure is terminal (though the event log structure supports replay-based recovery)
