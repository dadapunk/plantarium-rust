---
title: Plantarium Codebase Concerns
focus: concerns
date: 2026-03-26
version: 0.1.0
---

# Codebase Concerns

**Analysis Date:** 2026-03-26

## Critical Issues

### SQLite Persistence Not Implemented

**Issue:** Database load and save queries are stub implementations.

- **Files:** `src/storage/sqlite.rs` (lines 147-164)
- **Impact:** Data is not persisted on desktop builds. All gardens, beds, tasks, and journal entries are lost when the app restarts.
- **Fix approach:** Implement the `load_all()` and `save_all()` methods with actual SQL queries. The schema exists but no data is read/written.

### No Desktop Data Persistence

**Issue:** Desktop builds have no persistence layer wired up.

- **Files:** `src/app_state/state.rs` (lines 241-244)
- **Impact:** `save_to_storage()` is a no-op for non-wasm32 targets. Every CRUD operation calls `save_to_storage()` which does nothing on desktop.
- **Current state:** Web builds use localStorage. Desktop builds use in-memory only.
- **Fix approach:** Initialize SQLite in main.rs, call `storage.init()` on startup, and call `storage.save_all()` after mutations.

---

## Bugs

### Journal Edit Button Non-Functional

**Issue:** Edit button in journal entries has no implementation.

- **Files:** `src/pages/journal.rs` (lines 98-101)
- **Trigger:** Click the "Editar" button on any journal entry
- **Symptom:** Nothing happens - the button click is a no-op
- **Fix approach:** Wire up the onclick to set `editing_id` signal with the entry's ID

### No Garden Delete Functionality

**Issue:** Garden deletion is not implemented.

- **Files:** `src/app_state/state.rs`
- **Missing:** `delete_garden()` function not defined
- **Impact:** Gardens cannot be removed once created
- **Fix approach:** Add `delete_garden(garden_id: &str)` function following the same pattern as `delete_bed()`

---

## Technical Debt

### Unwrap/Expect Usage Risk

**Issue:** Multiple locations use unsafe unwraps that could panic.

- **Files:**
  - `src/app_state/state.rs` (lines 274-275): `.unwrap()` on `last()` - panics if beds list is empty
  - `src/storage/sqlite.rs` (line 169): `.expect()` on SQLite initialization
  - `src/pages/garden_detail.rs` (lines 64, 70): `.unwrap_or()` silently uses fallback values

- **Impact:** App could panic on edge cases (empty data, disk errors, etc.)
- **Fix approach:** Replace with proper error handling using `match` or `if let`

### No Input Validation

**Issue:** Form inputs lack validation.

- **Files:** `src/pages/garden_detail.rs`, `src/pages/journal.rs`
- **Examples:**
  - Width/height can be negative or zero (lines 64, 70)
  - No max length on text fields
  - Date format not validated

- **Fix approach:** Add validation in the signal update handlers before accepting values

### Missing Test Coverage

**Issue:** No test files exist in the codebase.

- **Files:** None found (*.rs with #[test] modules)
- **Risk:** Data operations, storage logic, and state management have no automated verification
- **Fix approach:** Add unit tests for state.rs functions and storage module

### Unused CSS Files

**Issue:** Multiple CSS files not loaded by the app.

- **Files:**
  - `assets/ui-designs/plantarium-screens/1-tareas-modo-oscuro.css`
  - `assets/ui-designs/plantarium-screens/2-editor-bancales-mejorado.css`
  - `assets/ui-designs/plantarium-screens/3-dashboard-mejorado.css`
  - `assets/ui-designs/plantarium-screens/4-editor-bancales-modo-oscuro.css`
  - `assets/ui-designs/plantarium-screens/5-dashboard-modo-oscuro.css`
  - `assets/ui-designs/plantarium-screens/6-botanical-journal.css`
  - `assets/ui-designs/plantarium-screens/7-diario-modo-oscuro.css`
  - `assets/ui-designs/plantarium-screens/8-tasks-calendar.css`

- **Impact:** Dead code cluttering the project (~8 unused CSS files)
- **Fix approach:** Remove unused files or integrate into active theme

### Version Mismatch

**Issue:** Package version does not match CHANGELOG.

- **Files:**
  - `Cargo.toml` line 3: `version = "0.1.0"`
  - `CHANGELOG.md` line 5: `[0.2.0] - 2026-03-18`

- **Fix approach:** Update Cargo.toml version to "0.2.0"

---

## Security Considerations

### No User Authentication

**Issue:** No authentication system implemented.

- **Files:** N/A - not implemented
- **Risk:** Any user with local access can view/modify all data
- **Current mitigation:** Desktop-first (assumes single-user machine)
- **Recommendations:** Add basic auth for multi-user scenarios

### HTML Content in Journal

**Issue:** Journal content is rendered as plain text, not sanitized markdown.

- **Files:** `src/pages/journal.rs` (line 111)
- **Risk:** If markdown parsing is added, XSS vulnerabilities possible
- **Current state:** Using `<pre>` tag - content rendered literally
- **Recommendations:** If using pulldown-cmark, ensure HTML output is sanitized via ammonia (already in deps)

---

## Performance Considerations

### No Lazy Loading

**Issue:** All data loaded into memory on startup.

- **Files:** `src/app_state/state.rs`, `src/storage/sqlite.rs`
- **Current state:** Entire AppState loaded at once (if implemented)
- **Impact:** Poor scaling with large datasets
- **Improvement path:** Add pagination, lazy-load beds/plants on demand

### Inefficient Bed Rendering

**Issue:** Beds re-render on every state change.

- **Files:** `src/pages/garden_detail.rs`
- **Current state:** No memoization on bed list computation
- **Improvement path:** Use `use_memo` for computed bed lists

### Large Binary Size Target

**Issue:** Release profile optimized for size but actual size may exceed targets.

- **Files:** `Cargo.toml` (lines 23-28)
- **Target:** <15MB desktop, <10MB mobile
- **Risk:** Dioxus + SQLite may exceed these targets
- **Recommendations:** Measure actual binary size, adjust profile settings if needed

---

## Missing Critical Features

### No Data Export/Backup

**Issue:** No way to export user data.

- **Problem:** Users cannot back up gardens, tasks, or journal entries
- **Blocks:** No migration path if SQLite schema changes
- **Priority:** High - data loss risk

### No Data Import

**Issue:** No way to restore from backup.

- **Problem:** Cannot migrate from web version or restore after data loss
- **Priority:** High

### No Error Recovery

**Issue:** No graceful error handling for storage failures.

- **Files:** `src/storage/sqlite.rs`
- **Problem:** If database fails to open, app panics via `.expect()`
- **Priority:** High

---

## Fragile Areas

### GlobalSignal Mutation Without Locks

**Issue:** Concurrent writes to global signals could cause race conditions.

- **Files:** `src/app_state/state.rs` (lines 154-161)
- **Why fragile:** Multiple functions mutate signals without coordination
- **Safe modification:** Ensure single-threaded access (Dioxus desktop is single-threaded by default)
- **Test coverage:** No tests to verify thread-safety assumptions

### Soft-Delete Not Enforced

**Issue:** Queries filter by `deleted_at.is_none()` but not consistently enforced.

- **Files:** `src/app_state/state.rs` (multiple functions)
- **Why fragile:** Some queries filter deleted items, others don't
- **Examples:** `get_bed_by_id()` returns deleted beds; `get_garden_beds()` filters correctly
- **Safe modification:** Audit all query functions to ensure consistent filtering

---

## Dependencies at Risk

### gloo-storage Unused

**Issue:** gloo-storage is optional but unused in desktop builds.

- **Files:** `Cargo.toml` (line 12)
- **Risk:** Dead dependency for desktop target
- **Migration plan:** Remove from desktop, keep for web only

### rusqlite Bundled SQLite

**Issue:** Using bundled SQLite limits platform support.

- **Files:** `Cargo.toml` (line 15)
- **Risk:** May not support all platforms
- **Alternative:** Use system SQLite for better platform coverage

---

*Concerns audit: 2026-03-26*
