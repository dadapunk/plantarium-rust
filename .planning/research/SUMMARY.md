# Project Research Summary

**Project:** Plantarium — Garden Management Application
**Domain:** Rust Desktop Application (Dioxus + SQLite)
**Researched:** 2026-03-26
**Confidence:** HIGH

## Executive Summary

Plantarium is a personal garden management desktop application built with Dioxus and Rust. The app has a working SQLite implementation but suffers from a critical bug: **persistence is not wired to the state layer**. Data persists in memory during a session but is lost when the app restarts because `save_to_storage()` is a no-op for desktop builds. This is the single blocker preventing production-readiness.

Research indicates the fix is straightforward: wire the existing SQLite storage layer into the global signal state management using the repository pattern. The recommended stack is **rusqlite 0.38.0 with bundled SQLite** (upgrade from current 0.30) plus **rusqlite_migration 2.0+** for schema migrations. All ORM alternatives (SQLx, Diesel, SeaORM) are rejected as unnecessary overhead for a single-user desktop app.

The roadmap should prioritize: (1) fixing SQLite persistence wiring, (2) completing garden CRUD (add delete), (3) fixing broken journal edit, then (4) expanding the plant database. Differentiator features like companion planting and climate-aware scheduling should wait until core stability is achieved.

## Key Findings

### Recommended Stack

**Core technologies:**
- **rusqlite 0.38.0** — SQLite bindings with bundled feature — lightweight, synchronous, avoids system SQLite dependency
- **rusqlite_migration 2.0+** — schema migration management stored in SQLite user_version field
- **directories crate** — platform-specific app data paths (already in stack)

**Why not alternatives:**
- **SQLx**: Async-only, adds tokio dependency — unnecessary for single-user desktop
- **Diesel**: Full ORM with compile-time checks — overkill for simple schema
- **SeaORM**: Async-first, relationship-heavy — designed for web services
- **refinery**: rusqlite_migration is simpler and integrates better

See STACK.md for detailed version upgrades and configuration.

### Expected Features

**Must have (table stakes):**
- **Data persistence** — CRITICAL BUG: SQLite exists but not wired to state; without this users lose all data on restart
- **Garden CRUD** — delete is explicitly missing; users cannot remove test/old gardens
- **Visual bed editor** — drag-and-drop plant placement (already works)
- **Plant database** — minimal data currently; limits bed editor utility
- **Task management** — watering, fertilizing, pruning reminders (already works)
- **Calendar view** — shows sowing, watering, harvest events (already works)
- **Journal** — CRITICAL BUG: edit button is broken

**Should have (competitive):**
- **Companion planting** — warn about bad plant combinations, suggest good ones
- **Harvest logging** — record yields over time, compare seasons

**Defer (v2+):**
- Crop rotation (needs multi-season data)
- Climate-aware scheduling (needs location integration)
- Weather integration (out of scope per PROJECT.md)
- Cloud sync (explicitly out of scope)

See FEATURES.md for complete feature analysis and competitive landscape.

### Architecture Approach

The current architecture has a working SQLite schema but is not connected to the state layer. The fix follows the **Repository Pattern**: create repository structs for each entity (Garden, Bed, Plant, Task, etc.) that wrap database operations, then integrate into the existing global signal state management via `load_from_storage()` and `save_to_storage()` functions.

**Major components:**
1. **State Management** (state.rs) — Global signals (GARDENS, BEDS, PLANTS, TASKS, etc.) with create/update/delete functions
2. **Storage Layer** (storage/sqlite.rs) — SqliteStorage struct with schema and connection management
3. **Repository Layer** (to add) — Entity-specific database operations (GardenRepository, BedRepository, etc.)

Data flow: App startup calls `load_from_storage()` → Desktop: loads from SQLite via repositories → UI uses global signals. User actions call state functions → mutate signals → call `save_to_storage()` → Desktop: writes to SQLite via repositories.

See ARCHITECTURE.md for detailed data flow diagrams and code patterns.

### Critical Pitfalls

1. **Not wiring save operations to state mutations** — Current Plantarium bug: data appears in UI but disappears on restart because desktop save is no-op
2. **Database lock contention** — SQLite file-level locking causes "database is locked" errors without WAL mode and busy timeout
3. **Connection not thread-safe** — rusqlite::Connection needs Mutex wrapper or thread-local storage
4. **Missing error handling** — Using `.expect()` on database operations causes panics on failures
5. **Soft-delete filter inconsistency** — Some queries return deleted records, others filter correctly
6. **Database path issues** — Parent directories not created causes first-run crashes

See PITFALLS.md for complete pitfall analysis and prevention strategies.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Fix Persistence Wiring
**Rationale:** This is the critical blocker — without data survival, nothing else matters. The SQLite schema already exists; we just need to wire it.

**Delivers:**
- Repository pattern implementation for all entities
- `load_from_storage()` loads from SQLite on desktop
- `save_to_storage()` writes to SQLite on desktop
- WAL mode, proper error handling, thread-safe connections

**Addresses:**
- Fix SQLite persistence (FEATURES.md)
- Implement garden deletion (FEATURES.md)
- Fix journal edit button (FEATURES.md)

**Avoids:**
- Pitfall #4: Not wiring saves (current bug)
- Pitfall #1: Lock contention (WAL mode configured)
- Pitfall #2: Thread safety (Mutex wrapper)
- Pitfall #5: Error handling (proper Result handling)
- Pitfall #6: Soft-delete consistency (audit queries)
- Pitfall #3: Path issues (create parent dirs)
- Pitfall #8: Schema init (call on startup)

**Research flag:** This phase is well-documented — no additional research needed beyond implementation.

### Phase 2: Data Validation & Plant Database
**Rationale:** With persistence working, make the data quality better. Expand plant data to make bed editor valuable.

**Delivers:**
- Input validation for forms (dimensions, names, dates)
- Expanded plant database (50-100 common vegetables/herbs)
- Harvest logging capability

**Addresses:**
- Expand plant database (FEATURES.md)
- Add harvest logging (FEATURES.md)

**Avoids:**
- Pitfall #7: No input validation

**Research flag:** Plant database content needs domain research — what data to include.

### Phase 3: Performance & Differentiators
**Rationale:** Core is stable, now optimize and add competitive features.

**Delivers:**
- Pagination for large datasets
- Indexes for query optimization
- Companion planting guidance
- Migration system for schema evolution

**Addresses:**
- Companion planting (FEATURES.md)
- Crop rotation foundation (FEATURES.md)

**Avoids:**
- Pitfall #9: Unbounded growth (pagination)
- Future schema changes (migration system)

**Research flag:** Companion planting rules need horticultural research.

### Phase Ordering Rationale

- **Why Phase 1 first:** Data loss is a dealbreaker — nothing else matters if users lose all data on restart. The SQLite implementation exists, just needs wiring.
- **Why Phase 2 second:** With persistence fixed, data quality and utility become the priority. Plant database makes the bed editor valuable.
- **Why Phase 3 third:** Performance and differentiators are luxuries — only tackle after core is bulletproof.
- **Grouping:** Each phase builds on the previous — Phase 1 infrastructure enables Phase 2 data quality enables Phase 3 features.
- **Pitfall avoidance:** All critical pitfalls are addressed in Phase 1; deferrable ones in later phases.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | rusqlite is standard for Rust desktop apps; Context7 documentation confirms patterns |
| Features | HIGH | Multiple 2026 sources (Leaftide, Gardenize comparisons) validate table stakes |
| Architecture | HIGH | Repository pattern is well-documented for rusqlite; existing code maps cleanly |
| Pitfalls | HIGH | SQLite locking and thread safety are well-known issues with standard solutions |

**Overall confidence:** HIGH

### Gaps to Address

- **Plant database content**: What data fields (spacing, sun, water) for which plants — needs horticultural research
- **Companion planting rules**: What plant combinations work/don't work — needs domain expertise
- **Error recovery UX**: How to handle database failures gracefully — needs UI design

These gaps should be addressed during Phase 2 planning.

## Sources

### Primary (HIGH confidence)
- Context7: /rusqlite/rusqlite — rusqlite documentation and patterns
- Dioxus 0.7 documentation — "Working with Databases" for desktop integration
- SQLite pragma reference — WAL mode, busy timeout configuration

### Secondary (MEDIUM confidence)
- WebSearch: "Rust ORMs in 2026: Diesel vs SQLx vs SeaORM vs Rusqlite" (Feb 2026) — comparison for desktop use
- WebSearch: "Dioxus Rust SQLite integration best practices 2025" — architecture patterns

### Tertiary (LOW confidence)
- Leaftide garden app comparisons — feature requirements validation
- Market research: GrowVeg, Gardenize, Leaftide feature sets — competitive analysis

---

*Research completed: 2026-03-26*
*Ready for roadmap: yes*