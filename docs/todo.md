*event persistence api*
I don't quite like the API for event persistence yet - seems like unnecessary lock risk to share a mutable object between outside and controller
=======
# Architectural TODOs

## P0 — Critical

- [ ] **Events not persisted**: Controller loop reduces events into local memory state but never writes them back to `EventSource`. Process crash loses all progress. Fix: emit events → persist to store → derive state from persisted log.
- [ ] **Reducer panics on invariant violations**: `reduce.rs` calls `panic!("Step not found in execution state")`. The reducer must return `Result<State, ReduceError>` to be composable and safe for replay scenarios.

## P1 — Architectural Contradictions

- [ ] **EventSource/EventSink split is incoherent**: `EventSource` is both reader and writer (violates CQRS intent). `EventSink` has no trait in `api/` — implementation without a contract. `MemoryEventStore` implements `EventSource` but is never used anywhere.
- [ ] **Validation framework is silently ignored**: `validate_config` and `validate_input` exist on every handler struct but `executor.rs` never calls them. Validators are dead code from the framework's perspective; handlers will panic or produce bad output instead.
- [ ] **Async step panics at runtime**: `Step::Async` execution hits `unimplemented!()` in `executor.rs`. Async steps can be added to execution state and scheduled, causing a silent runtime crash when picked up.

## P2 — Design Gaps

- [ ] **Input resolution is coupled to the controller**: `get_prior_output` lives in `controller.rs` but encodes a policy decision (how data flows between steps). If input chaining becomes more complex — fan-in, transforms, conditional routing — you'd be editing the orchestration loop to do it. Consider making it a separate, injectable concern. (The rest of the controller is genuinely thin and appropriate as a coordinator.)
- [ ] **Sync/Async step model asymmetry**: `SyncStep::Ready` carries `input: Option<Value>`; `AsyncStep::Ready` does not — input only exists ephemerally at start time for async. Both should follow the same transition pattern: `Ready → Starting(input) → Running → Terminal`.
- [ ] **No dependency model in scheduler**: `scheduler.rs` is a sequential left-to-right scan with no concept of step dependencies. A DAG model is needed to express "step C requires outputs from both A and B."
- [ ] **No compensation or resumability**: Execution failure is terminal. The event-sourcing model already supports resumability (replay + inject retry/override event) but no API exists for it.

## P3 — Code Quality

- [ ] **`serde_json::Value` as universal wire type**: No compile-time guarantees that step A's output matches step B's input. Consider generic `Input`/`Output` associated types on step handlers.
- [ ] **Dead code**: `complete_step.rs` defines a function never called (transitions handled inline in `reduce.rs`). `async_step.rs` is empty. `MemoryEventStore` is fully implemented but never instantiated. Delete all three.
- [ ] **`shell_handler` has unguarded `.unwrap()` calls**: Config deserialization and command execution both unwrap — a missing binary or malformed config panics the process instead of emitting a `Failed`/`Error` event.
- [ ] **Non-idiomatic module name**: `src/impl/` shadows the `impl` keyword. Rename to `infrastructure/`, `storage/`, or `adapters/`.
