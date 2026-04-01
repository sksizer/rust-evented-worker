# Event Storage Design

## Problem

The original design had three overlapping, partially-implemented concepts:

- `EventSource` trait — combined read + write in one trait
- `EventSink` trait — referenced in `InMemoryEventSink` but the trait itself didn't exist in `api/`
- `EventStream` — `Vec<StepEvent>`, the only thing actually wired to the controller

The result: one real thing (the vec) and two phantom things going nowhere.

## Concepts

Two distinct concerns exist in this system:

**Durability** — loading the prior event history to restore state, and persisting new events as they are produced. This is what makes the system recoverable.

**Notification** — reacting to events as they are produced (callbacks, side effects, monitoring). Already handled by `on_event` on the controller.

These should not be conflated.

## Options Considered

### Option A — Function injection (chosen)
Pass load + save as functions directly to the controller. No trait, no dedicated struct required initially.

The controller calls `load()` once at startup to restore state, and `save(event)` each time a new event is produced during the loop.

### Option B — `EventLog` trait
A single trait with `load(&self) -> EventStream` and `append(&mut self, event: &StepEvent)`. Natural swap point for a DB backend. Preferred if/when a second storage implementation is needed.

### Option C — Push model (controller as receiver)
External systems push commands/events into the controller via a `submit()` or `dispatch()` method rather than the controller polling an event source. The right model if async step completions or external triggers (webhooks, queues) need to arrive mid-workflow. Kept in mind for later.

### Option D — Explicit log + notification split
Separate `EventLog` trait for durability and `EventListener` type for reactivity. Most principled, most ceremony. Premature at current scale.

## Decision

**Option A** for now. Reasons:
- Single workflow, no concurrent consumers
- No DB yet — memory-only
- Avoids committing to a trait shape before we know the DB access pattern
- The existing `EventStream` (vec) already handles the load side; we only need to add a save hook

**Upgrade path:** When a DB backend arrives, extract into an `EventLog` trait (Option B). The function-based approach and the trait approach have identical call sites from the controller's perspective — the swap is mechanical.

**Future push model:** If external events need to trigger workflow advancement (async callbacks, webhooks, message queues), revisit Option C. The function/trait boundary here doesn't block that — the controller's intake API is a separate concern from its storage API.

## Implementation

The controller gains an optional save function. Load remains the existing `EventStream` parameter (a `Vec<StepEvent>` passed at construction). The save function is called once per produced event inside the loop.

Storage implementations live in `src/infra/` (renamed from `src/impl/`).
