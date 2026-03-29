# Defects & Gaps Analysis

## 1. CRITICAL: `reduce()` panics instead of returning `Result`

**Files:** `src/runner/reduce.rs:55,67,80,93` + 6 `.unwrap()` calls on lines 36,45,57,69,82,95

The reducer — the core state machine — has 4 `panic!()` calls and 6 `.unwrap()` calls. Any invalid event crashes the process. This is the single most important function in an event-sourced system and it cannot fail gracefully.

**Fix:** Change signature to `Result<DefaultExecutionState, ExecutionStateError>`, propagate errors through `controller.rs`.

**Leverage:** HIGH — touches every event processed, and fixing it forces proper error handling up through the controller.

---

## 2. CRITICAL: Processor silently discards handler errors

**File:** `src/runner/processor.rs:26`
```rust
Event::complete(activity_id.to_string(), Some(result.unwrap()))
```

When an activity handler returns `Err(...)`, this panics instead of emitting `Event::failed()` or `Event::error()`. The `invariant_violation()` helper (line 36) was written for exactly this purpose but is never called.

**Fix:** Match on `result` — `Ok` → `Event::complete`, `Err` → `Event::failed`.

---

## 3. CRITICAL: Shell handler has unsafe double-unwrap

**File:** `src/activities/shell.rs:33,39`
```rust
let config = get_config(config.0.unwrap()).unwrap();  // double unwrap
// ...
.output().unwrap()  // process execution can fail
```

Shell commands that fail to execute, or activities with missing config, crash the entire system. The handler already returns `Result<Value, Vec<String>>` — the error path just isn't used.

---

## 4. HIGH: Validation functions are never called

**Files:** `src/api/activities/handlers.rs`, `src/runner/processor.rs`

`SyncActivityHandler` and `AsyncActivityHandler` both carry `validate_config` and `validate_input` fields, and activity implementations (shell, echo, fixed_output) all define validators. But the processor never calls them before executing the handler. This means invalid configs reach the handler and cause panics (see #3).

---

## 5. HIGH: Async activity execution is defined but not implemented

The type system is fully built out (`AsyncReady` → `AsyncRunning` → `AsyncCompleted/Failed/Error`), the registry stores `AsyncActivityHandler`s, the scheduler considers async activities runnable, but the processor only handles sync handlers. `registry.get_sync_module()` is the only lookup path.

**Contradiction:** The scheduler returns async activities as runnable, the processor can't execute them, and the result would be a `SystemError` event that the reducer ignores (see #6).

---

## 6. HIGH: System events are silently ignored

**File:** `src/runner/reduce.rs:17-20`
```rust
Event::System(_) => {
    // TODO - consider how system events affect execution state
    execution_state  // passed through unchanged
}
```

When the processor returns a system error (e.g., activity module not found), the reducer does nothing with it. The execution state doesn't reflect the error, the activity stays in `Running` state forever, and the controller loops infinitely (scheduler keeps returning the same runnable activity).

---

## 7. HIGH: Inconsistent error types across validation

- `ValidateConfig` returns `Result<(), ActivityError>`
- `ValidateInput` returns `Result<(), String>`

These serve the same purpose but use different error types, making unified error handling impossible if validation is ever wired up.

---

## 8. MEDIUM: `ExecutionState.activity_states` is public

**File:** `src/api/execution/model.rs:15` — has an inline comment acknowledging this needs to be private. External code can mutate activity states directly, bypassing the event stream, which violates the core event-sourcing invariant.

---

## 9. MEDIUM: Dead code and unused infrastructure

| Item | File | Issue |
|------|------|-------|
| `loop_fn` field | `controller.rs:16` | Defined, initialized, never read |
| `event_handlers` field | `controller.rs:17` | Defined, initialized, never read |
| `invariant_violation()` | `processor.rs:36` | Written for error handling, never called |
| `main.rs` | `src/main.rs` | 10 unused imports, empty `fn main()` |
| `Rc<RefCell<EventStream>>` | `controller.rs` | Acknowledged as "unnecessary lock risk" in `docs/todo.md` |

The `loop_fn` / `event_handlers` pattern suggests a planned hook/plugin system that was never implemented. Either implement it or remove it.

---

## 10. MEDIUM: Minimal test coverage with critical gaps

**18 tests total.** Well-tested: restore (6), activity handlers (4), events (2). Poorly tested or untested:

- **Processor:** 1 empty placeholder test (`test_executor() {}`)
- **Shell handler:** 0 tests
- **Controller:** 1 basic happy-path test, no error scenarios
- **Error paths:** Almost entirely untested
- **Async activities:** Only 1 test (add + start), no completion/failure
- **No integration tests directory**

---

## 11. LOW: Architectural design gaps documented but unaddressed

- `docs/architecture.md` describes recovery/durability as a goal but no persistence mechanism exists
- `docs/todo.md` flags the event persistence API as problematic
- The "command" concept in the glossary is marked "unimplemented"
- No retry, compensation, or timeout mechanisms

---

## 12. LOW: Minor code quality issues

- Scheduler uses manual `match` instead of `.map()` (`scheduler.rs:11-14`)
- Registry uses `unwrap_or_else(Vec::new)` instead of `unwrap_or_default()` (`registry.rs`)
- Needless borrow in processor (`&activity.kind()`)
- Duplicate TODO comments in echo.rs and fixed_output.rs

---

## Highest-Leverage Improvements (Ordered)

1. **Make `reduce()` return `Result`** — Forces proper error handling through the entire execution pipeline. This is the foundation everything else depends on.

2. **Fix processor to handle handler errors** — Wire up the `Ok`/`Err` match and use the existing `invariant_violation()` helper. Also call validators before execution.

3. **Handle system events in the reducer** — At minimum, mark affected activities as errored so the controller doesn't infinite-loop.

4. **Fix shell handler error handling** — Replace unwraps with `?` operator. The function already returns `Result`.

5. **Wire up validation** — Add `validate_config`/`validate_input` calls in the processor before handler execution. Unify the error types first.

6. **Clean up dead code** — Remove `loop_fn`, `event_handlers`, `invariant_violation`, and the empty `main.rs` imports. Reduces cognitive overhead for anyone reading the code.

7. **Add tests for error paths** — The happy paths work. The failure modes are where the bugs are (as items 1-4 demonstrate).
