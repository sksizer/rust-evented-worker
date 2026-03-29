# evented-worker

An event-sourced workflow engine for Rust. Define workflows as sequences of named activities, execute them with a `Controller`, and recover from any point in execution by replaying the event log.

## Concepts

- **Activity** — a unit of work with a `kind` (handler name), optional config, and optional input/output
- **ActivityEvent** — an immutable record of something that happened (activity added, started, completed, failed, errored)
- **EventStream** — the ordered log of all events; the authoritative source of truth for execution state
- **Registry** — a catalog of named activity handlers available at runtime
- **Controller** — orchestrates the execution loop: restores state, schedules the next event, processes it, and records results

State is never stored directly. It is always derived by replaying the event stream through the `reduce` function.

## Workspace

```
evented-worker/          # Main library crate
crates/cmd-spec/    # Serializable shell command builder
```

## Quick Start

### Controller (high-level)

Define an event log with your activities, hand it to a `Controller`, and call `start()`:

```rust
use evented_worker::runner::Controller;
use evented_worker::fixtures::get_registry;
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use cmd_spec::ShellCommand;
use std::cell::RefCell;
use std::rc::Rc;

let event_log = Rc::new(RefCell::new(vec![
    get_activity(
        "format-and-lint",
        ActivityParameters {
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

### Low-level API

Use `scheduler`, `process`, and `reduce` directly for full control:

```rust
use evented_worker::{runner, view};
use evented_worker::api::activities::ActivityEvent;
use evented_worker::runner::Registry;
use evented_worker::fixtures::get_registry;
use serde_json::json;

let registry = get_registry();
let event_stream = vec![
    ActivityEvent::add_sync("1", "echo", Some(json!({ "config": "hello" }))),
];

let mut state = runner::restore(&event_stream);

// scheduler picks the next runnable activity and returns a Start event
let start_event = runner::scheduler(&state).unwrap();
state = runner::reduce(state, &start_event);

// process executes the handler and returns a result event (Complete, Failed, or Error)
let result_event = runner::process(&state, &registry, &start_event);
state = runner::reduce(state, &result_event);

view::summarize::execution_state(&state);
```

### Chaining activities

Activities pass output to the next activity automatically. Use `resolve_prior_output` to wire the output of one activity as input to the next:

```rust
let event_log = Rc::new(RefCell::new(vec![
    ActivityEvent::add_sync("0", "fixed_output", Some(json!({ "config": "DATA" }))),
    ActivityEvent::add_sync("1", "echo", None),  // receives output of activity 0
    ActivityEvent::add_sync("2", "echo", None),  // receives output of activity 1
]));
```

## How It Works

The execution loop follows this pattern each iteration:

```
EventStream (Vec<ActivityEvent>)
    ↓ restore()
DefaultExecutionState
    ↓ scheduler()     → pick next runnable activity, return Start event
    ↓ reduce()        → apply Start event to state
    ↓ process()       → call handler → produce result ActivityEvent
    ↓ reduce()        → apply result to state
    [repeat until no runnable activities remain]
```

### Event types

| Event | Meaning |
|-------|---------|
| `AddSync` / `AddAsync` | An activity was added to the workflow |
| `Start` | An activity began executing |
| `Complete` | An activity finished successfully (may carry output) |
| `Failed` | An activity failed (may carry a reason) |
| `Error` | An activity encountered an error (may carry a reason) |
| `SystemError` | An infrastructure-level error occurred |

Recovery works by replaying the event stream from the beginning. If a process crashes mid-execution, replaying the existing events restores exact state — already-completed activities are not re-executed.

## cmd_spec

A serializable wrapper for `std::process::Command`, useful when activity configurations need to be stored in the event log as JSON:

```rust
use cmd_spec::ShellCommand;

let cmd = ShellCommand::new("cargo")
    .arg("build")
    .args(["--release", "--bin", "myapp"])
    .env("RUST_LOG", "debug")
    .working_dir("/path/to/project");

// Converts to std::process::Command
let mut process_cmd: std::process::Command = cmd.into();
```

Enable the `tokio` feature for async execution via `tokio::process::Command`:

```toml
[dependencies]
cmd-spec = { path = "crates/cmd-spec", features = ["tokio"] }
```

## Built-in Activity Handlers

| Kind | Description |
|------|-------------|
| `shell` | Executes one or more `ShellCommand`s sequentially, returning their stdout |
| `echo` | Passes input through unchanged (useful for testing/chaining) |
| `fixed_output` | Always emits the configured value as output, ignoring input |

## Writing a Custom Activity Handler

```rust
use evented_worker::api::activities::{SyncActivityHandler, ActivityConfig, ActivityInput};
use serde_json::{json, Value};

let handler = SyncActivityHandler {
    name: "my-activity".to_string(),
    id: "my-activity".to_string(),
    description: "Does something useful".to_string(),
    validate_config: None,
    validate_input: None,
    handler: |config: ActivityConfig, input: ActivityInput| -> Result<Value, Vec<String>> {
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

Early-stage library. The core event-sourcing loop, synchronous activity execution, and shell command integration are functional.

Known limitations:
- **Events not persisted**: the controller holds state in memory only — a crash loses all progress
- **Async activity execution**: event types and state machine exist but processing is not yet implemented
- **Validators ignored**: `validate_config`/`validate_input` on handlers are defined but never called
- **Sequential scheduling only**: no DAG-based dependency model; activities run left-to-right
- **No retry or compensation API**: failure is terminal (though the event log structure supports replay-based recovery)
