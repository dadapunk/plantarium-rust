# Phase 1: Persistence & Core CRUD - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix critical data loss bug and complete missing CRUD operations:
- Wire SQLite to load/save functions
- Implement garden deletion
- Fix broken journal edit button
- Enable WAL mode for SQLite
- Handle errors gracefully

</domain>

<decisions>
## Implementation Decisions

### Persistence Approach
- **D-01:** Use hybrid approach (SQLite + localStorage) — SQLite for persistent storage on desktop, localStorage as fallback for demo data and web builds

### Error Handling
- **D-02:** Graceful degradation — Log errors to console, show toast notification, continue with in-memory data if SQLite fails

### Journal Edit Fix
- **D-03:** Just fix the button — Wire up existing edit button to trigger editing mode (minimal fix)

### the agent's Discretion
- WAL mode enablement (can be done during implementation)
- Garden deletion confirmation UX (can decide during implementation)
- Soft-delete vs hard-delete for gardens (can decide during implementation)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Storage Layer
- `src/storage/sqlite.rs` — SQLite implementation with schema
- `src/storage/mod.rs` — StorageProvider trait

### State Management
- `src/app_state/state.rs` — Global signals and load/save functions
- `.planning/research/ARCHITECTURE.md` — Repository pattern guidance

### Existing Code Patterns
- `.planning/codebase/CONVENTIONS.md` — Naming conventions
- `.planning/codebase/ARCHITECTURE.md` — Architecture patterns

[If no external specs: "No external specs — requirements fully captured in decisions above"]

</canonical_refs>

<codebase_context>
## Existing Code Insights

### Reusable Assets
- `src/storage/sqlite.rs` — Already has complete schema and SqliteStorage struct
- `src/app_state/state.rs` — load_from_storage() and save_to_storage() functions with clear structure

### Established Patterns
- Global signals (GARDENS, BEDS, PLANTS, TASKS, etc.) are the single source of truth
- Storage layer wraps persistence, state wraps business logic
- #[cfg(target_arch = "wasm32")] used for web vs desktop branching

### Integration Points
- load_from_storage() called in main.rs on startup
- save_to_storage() called after every state mutation (17 call sites in state.rs)
- SqliteStorage implements StorageProvider trait

</codebase_context>

<specifics>
## Specific Ideas

- Use hybrid persistence: SQLite for desktop, localStorage for web/demo data
- Graceful error handling: toast notification + console log, continue with in-memory
- WAL mode should be enabled in SqliteStorage::init()

</specifics>

<deferred>
## Deferred Ideas

### Garden Deletion UX
- Confirmation dialog vs immediate delete
- Soft-delete (mark deleted_at) vs hard-delete
- Cascade delete beds/plants or warn user
**Reason:** Can be decided during implementation

### Todo Backlog
- None — Phase 1 scope is clear

</deferred>

---

*Phase: 01-persistence-core-crud*
*Context gathered: 2026-03-26*