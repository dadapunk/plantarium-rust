# Phase 1: Persistence & Core CRUD - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 1-Persistence & Core CRUD
**Areas discussed:** Persistence approach, Error handling, Journal edit fix scope

---

## Persistence Approach

| Option | Description | Selected |
|--------|-------------|----------|
| SQLite directly | Load from SQLite on desktop startup, save to SQLite on every state mutation | |
| Hybrid (SQLite + localStorage) | SQLite for persistent storage on desktop, localStorage as fallback for demo data and web builds | ✓ |

**User's choice:** Hybrid (SQLite + localStorage)
**Notes:** Prefers keeping localStorage as fallback for demo data, web builds still use localStorage

---

## Error Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Graceful degradation | Log errors to console, show toast notification, continue with in-memory data | ✓ |
| Silent fail | Log errors silently and continue | |
| Fail hard | Show error dialog and prevent app from loading | |

**User's choice:** Graceful degradation (Recommended)
**Notes:** Wants users to see feedback when something goes wrong, but app shouldn't crash

---

## Journal Edit Fix

| Option | Description | Selected |
|--------|-------------|----------|
| Just fix the button | Wire up existing edit button to trigger editing mode | ✓ |
| Full improvement | Improve journal editing UI - inline editing, better save/cancel flow | |

**User's choice:** Just fix the button (Recommended)
**Notes:** Minimal fix to make existing functionality work

---

## the agent's Discretion

Areas where user deferred to the agent:
- WAL mode enablement (can be done during implementation)
- Garden deletion confirmation UX (can decide during implementation)
- Soft-delete vs hard-delete for gardens (can decide during implementation)

---

## Deferred Ideas

Ideas mentioned during discussion that were noted for future phases:
- Garden deletion UX details (confirmation, soft-delete, cascade) — deferred to implementation
- Full journal editing improvement — out of scope for Phase 1