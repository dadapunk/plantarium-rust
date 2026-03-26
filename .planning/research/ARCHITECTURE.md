# Architecture Research — SQLite Integration for Dioxus Desktop

**Project:** Plantarium — Garden Management Application  
**Domain:** Dioxus Desktop + SQLite Persistence Architecture  
**Researched:** 2026-03-26  
**Confidence:** HIGH

## Executive Summary

The current architecture has a working SQLite schema but **is not wired to the state layer**. The critical gap is that `load_from_storage()` and `save_to_storage()` functions in `src/app_state/state.rs` do not call the SQLite storage implementation. Data persists to LocalStorage (web) or nothing (desktop), causing data loss between sessions.

The recommended fix follows the **Repository Pattern** — create repository structs for each entity (Garden, Bed, Plant, Task, etc.) that wrap database operations, then integrate these into the existing global signal state management. This preserves the current architecture while enabling SQLite persistence.

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      PRESENTATION LAYER                        │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │Dashboard │  │Garden    │  │BedEditor │  │Calendar  │  ...  │
│  │  Page    │  │Detail    │  │  Page    │  │  Page    │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
└───────┼─────────────┼─────────────┼─────────────┼─────────────┘
        │             │             │             │
        ▼             ▼             ▼             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      STATE MANAGEMENT LAYER                    │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Global Signals: GARDENS, BEDS, PLANTS, TASKS, EVENTS   │   │
│  │  Functions: create_garden(), create_bed(), save_to_storage│   │
│  └─────────────────────────┬────────────────────────────────┘   │
└────────────────────────────┼────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      PERSISTENCE LAYER (TO WIRE)               │
├─────────────────────────────────────────────────────────────────┤
│  ┌────────────────┐  ┌────────────────┐  ┌─────────────────┐    │
│  │ SqliteStorage │  │ StorageProvider│  │    AppState     │    │
│  │   (exists)    │◄─┤    (trait)     │  │   (struct)      │    │
│  └────────┬───────┘  └────────────────┘  └─────────────────┘    │
│           │                                                      │
│           ▼                                                      │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │                  REPOSITORY LAYER (TO ADD)                │    │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐         │    │
│  │  │ Garden  │ │  Bed    │ │ Plant   │ │  Task   │  ...   │    │
│  │  │  Repo   │ │  Repo   │ │  Repo   │ │  Repo   │         │    │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘         │    │
│  └──────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Responsibilities

| Component | Responsibility | Status | Implementation |
|-----------|----------------|--------|-----------------|
| **Pages** | Full-screen views, user interaction | Working | Dioxus components in `src/pages/` |
| **Components** | Reusable UI widgets | Working | Dioxus components in `src/components/` |
| **Global Signals** | Cross-component state sharing | Working | Dioxus `Signal::global` in `state.rs` |
| **State Functions** | Business logic (create, update, delete) | Working | Functions in `state.rs` |
| **Storage Provider** | Trait defining storage interface | Working | `StorageProvider` trait in `db.rs` |
| **SQLite Storage** | SQLite implementation | Exists, not wired | `SqliteStorage` in `sqlite.rs` |
| **Repositories** | Entity-specific DB operations | **Missing** | Need to implement |

---

## Current Data Flow (Broken)

### Application Initialization (Current — Broken)

```
main() 
    → App component 
    → load_from_storage()
        → [cfg(wasm32)] LocalStorage.get() ✓
        → [cfg(not(wasm32))] *PLANTS.write() = default_plants() ← Only plants loaded!
```

### State Mutation (Current — Broken)

```
User creates garden
    → create_garden() 
    → GARDENS.write().push(garden) 
    → save_to_storage()
        → [cfg(wasm32)] LocalStorage.set() ✓
        → [cfg(not(wasm32))] // NO-OP ← Data not saved!
```

**Problem:** Desktop builds (the primary target) have no persistence. The SQLite implementation exists in `src/storage/sqlite.rs` but is never called.

---

## Recommended Data Flow (Fixed)

### Application Initialization (Fixed)

```
main() 
    → App component 
    → load_from_storage()
        → [cfg(wasm32)] LocalStorage.get() (keep for web)
        → [cfg(not(wasm32))] 
            → SqliteStorage::new() 
            → SqliteStorage.load_all() 
                → GardenRepository.list_all() → GARDENS
                → BedRepository.list_all() → BEDS
                → PlantRepository.list_all() → PLANTS
                → ... (all entities)
```

### State Mutation Flow (Fixed)

```
User creates garden
    → create_garden() 
    → GARDENS.write().push(garden) 
    → save_to_storage()
        → [cfg(wasm32)] LocalStorage.set() (keep for web)
        → [cfg(not(target_arch = "wasm32"))] 
            → SqliteStorage.save_all()
                → GardenRepository.upsert(garden)
                → ... (batch saves)
```

---

## Architectural Patterns

### Pattern 1: Repository Pattern for Database Access

**What:** Encapsulate database operations behind struct methods for each entity type.

**When to use:** Always for SQLite integration — provides clean separation between storage and business logic.

**Trade-offs:**
- Pros: Testable, explicit queries, no ORM magic, simple to understand
- Cons: More boilerplate than ORM, manual query writing

**Example:**

```rust
// src/storage/repositories/garden_repo.rs
use crate::app_state::Garden;
use rusqlite::{params, Connection, Result};

pub struct GardenRepository<'a> {
    conn: &'a Connection,
}

impl<'a> GardenRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn upsert(&self, garden: &Garden) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO gardens 
             (id, name, min_bed_distance, bed_spacing, created_at, updated_at, deleted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                garden.base.id,
                garden.name,
                garden.min_bed_distance,
                garden.bed_spacing,
                garden.base.created_at,
                garden.base.updated_at,
                garden.base.deleted_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_all(&self) -> Result<Vec<Garden>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, min_bed_distance, bed_spacing, created_at, updated_at, deleted_at 
             FROM gardens WHERE deleted_at IS NULL ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(Garden {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                    deleted_at: row.get(6)?,
                },
                name: row.get(1)?,
                min_bed_distance: row.get(2)?,
                bed_spacing: row.get(3)?,
            })
        })?;
        
        rows.collect()
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        // Soft delete
        let now = chrono::Utc::now().timestamp_millis();
        self.conn.execute(
            "UPDATE gardens SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }
}
```

---

### Pattern 2: Global State + Storage Gateway

**What:** The global signals in `state.rs` serve as the single source of truth during runtime, with storage functions acting as the gateway for persistence.

**When to use:** Works well for Dioxus apps where reactive state drives UI updates.

**Trade-offs:**
- Pros: Simple mental model, signals handle reactivity automatically
- Cons: Must remember to call `save_to_storage()` after mutations

**Example:**

```rust
// src/app_state/state.rs — Modified save function

#[cfg(not(target_arch = "wasm32"))]
pub fn save_to_storage() {
    use crate::storage::SqliteStorage;
    use std::sync::OnceLock;
    
    // Lazily initialize storage (avoid creating at program start)
    static STORAGE: OnceLock<SqliteStorage> = OnceLock::new();
    
    let storage = STORAGE.get_or_init(|| {
        SqliteStorage::new().expect("Failed to initialize SQLite")
    });
    
    let state = AppState {
        gardens: GARDENS.read().clone(),
        beds: BEDS.read().clone(),
        plants: PLANTS.read().clone(),
        tasks: TASKS.read().clone(),
        events: EVENTS.read().clone(),
        journal: JOURNAL.read().clone(),
        plot_actions: PLOT_ACTIONS.read().clone(),
        bed_orders: BED_ORDERS.read().clone(),
    };
    
    let _ = storage.save_all(&state);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_from_storage() {
    use crate::storage::SqliteStorage;
    use std::sync::OnceLock;
    
    static STORAGE: OnceLock<SqliteStorage> = OnceLock::new();
    
    let storage = STORAGE.get_or_init(|| {
        SqliteStorage::new().expect("Failed to initialize SQLite")
    });
    
    if let Ok(state) = storage.load_all() {
        *GARDENS.write() = state.gardens;
        *BEDS.write() = state.beds;
        *PLANTS.write() = if state.plants.is_empty() {
            default_plants()
        } else {
            state.plants
        };
        *TASKS.write() = state.tasks;
        *EVENTS.write() = state.events;
        *JOURNAL.write() = state.journal;
        *PLOT_ACTIONS.write() = state.plot_actions;
        *BED_ORDERS.write() = state.bed_orders;
    } else {
        *PLANTS.write() = default_plants();
    }
}
```

---

### Pattern 3: Storage Abstraction with Feature Flags

**What:** Use Rust's `#[cfg]` to provide different storage backends for web (LocalStorage) vs desktop (SQLite).

**When to use:** Required for cross-platform Dioxus apps that need different persistence strategies per platform.

**Trade-offs:**
- Pros: Platform-appropriate storage, maintains SPA architecture
- Cons: Platform-specific code paths require testing both targets

**Example:**

```rust
// In state.rs — already implemented
#[cfg(target_arch = "wasm32")]
pub fn load_from_storage() {
    // Web: Use LocalStorage via gloo-storage
    if let Ok(data) = LocalStorage::get::<AppState>(STORAGE_KEY) {
        // ... load from LocalStorage
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_from_storage() {
    // Desktop: Use SQLite (new integration)
    // ...
}
```

---

## Data Flow Summary

### Flow 1: App Startup

```
┌────────────────┐    load_from_storage()    ┌────────────────┐
│   main.rs      │ ───────────────────────► │  state.rs      │
│   (launch)     │                          │ (global sigs)  │
└────────────────┘                          └───────┬────────┘
                                                    │
                                    ┌───────────────┼───────────────┐
                                    │               │               │
                               [wasm32]        [desktop]       [desktop]
                                    │               │               │
                                    ▼               ▼               ▼
                             LocalStorage    SqliteStorage   SqliteStorage
                             .get()          .new()          .load_all()
                                                                │
                                                                ▼
                                                     Repositories
                                                     (query DB)
```

### Flow 2: User Creates Entity

```
┌──────────────┐    create_garden()     ┌────────────────┐
│   UI         │ ────────────────────► │  state.rs      │
│  Component   │                        │ (mutation)     │
└──────────────┘                        └────────┬────────┘
                                                │
                                                ▼
                                       GARDENS.write()
                                                │
                                                ▼
                                       save_to_storage()
                                                │
                             ┌───────────────────┼───────────────────┐
                             │                   │                   │
                        [wasm32]            [desktop]            [desktop]
                             │                   │                   │
                             ▼                   ▼                   ▼
                      LocalStorage        SqliteStorage       SqliteStorage
                      .set()              .save_all()          .save_all()
                                                      │
                                                      ▼
                                             Repositories
                                             (INSERT/UPDATE)
```

### Flow 3: User Deletes Entity

```
┌──────────────┐    delete_garden()     ┌────────────────┐
│   UI         │ ────────────────────► │  state.rs      │
│  Component   │                        │ (mutation)     │
└──────────────┘                        └────────┬────────┘
                                                │
                                                ▼
                                       Filter from GARDENS
                                                │
                                                ▼
                                       save_to_storage() → soft-delete in DB
```

---

## Suggested Build Order

Based on dependencies, implement in this order:

### Phase 1: Core Wiring (Critical Path)

| Step | Component | Notes |
|------|-----------|-------|
| 1.1 | **Implement GardenRepository** | Simplest entity to start with |
| 1.2 | **Wire load_from_storage()** | Desktop: call SQLite, return AppState |
| 1.3 | **Wire save_to_storage()** | Desktop: call SQLite, pass AppState |
| 1.4 | **Test garden CRUD** | Create, read, update, delete with persistence |

### Phase 2: Remaining Entities

| Step | Component | Notes |
|------|-----------|-------|
| 2.1 | **Implement BedRepository** | Includes placed_plants |
| 2.2 | **Implement PlantRepository** | Read-only master data |
| 2.3 | **Implement TaskRepository** | Task + CalendarEvent |
| 2.4 | **Implement JournalRepository** | JournalEntry |
| 2.5 | **Implement PlotActionRepository** | History tracking |
| 2.6 | **Implement BedOrderRepository** | Order tracking per garden |

### Phase 3: Refinements

| Step | Component | Notes |
|------|-----------|-------|
| 3.1 | **Add migration system** | Use rusqlite_migration for schema changes |
| 3.2 | **Error handling** | Proper error propagation, user-facing messages |
| 3.3 | **Performance tuning** | WAL mode, batch writes if needed |

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Direct Database Access from UI Components

**What people do:** Call database queries directly inside Dioxus components or handlers.

**Why it's wrong:** 
- Breaks the layered architecture
- Makes testing difficult
- Couples UI to persistence details

**Do this instead:** Always go through state functions → global signals → storage layer.

---

### Anti-Pattern 2: Creating New Connection Per Operation

**What people do:** Open a new SQLite connection for each CRUD operation.

**Why it's wrong:** 
- Connection overhead adds latency
- Not thread-safe by default
- WAL mode benefits lost

**Do this instead:** Use a single connection stored in a `OnceLock` or passed through the storage layer, reused for all operations.

---

### Anti-Pattern 3: Ignoring Soft Deletes

**What people do:** Using `DELETE FROM table` SQL statements.

**Why it's wrong:** 
- Data loss is permanent
- Hard to implement "undo" features
- Audit trail destroyed

**Do this instead:** Use the existing `deleted_at` field pattern (already in schema). Update to current timestamp instead of deleting.

---

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| **0-100 gardens** | Current schema fine, single SQLite file |
| **100-1,000 gardens** | Add indexes on foreign keys, consider WAL mode benefits |
| **1,000+ gardens** | Consider splitting data file, add query optimization |

### First Bottleneck: Query Performance

As garden/bed count grows, basic queries will slow down. First fix: Add indexes on `garden_id`, `bed_id`, `date` columns.

```sql
CREATE INDEX IF NOT EXISTS idx_beds_garden ON beds(garden_id);
CREATE INDEX IF NOT EXISTS idx_tasks_date ON tasks(date);
CREATE INDEX IF NOT EXISTS idx_events_date ON calendar_events(date);
```

### Second Bottleneck: Load Time

With many entities, loading all at startup becomes slow. Future optimization: Lazy loading, pagination, or in-memory cache with SQLite as backing store.

---

## Integration Points

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| **Pages → State** | Direct function calls | e.g., `create_garden(name)` |
| **State → Storage** | `load_from_storage()` / `save_to_storage()` | New: SQLite implementation called here |
| **Storage → Repositories** | Repository struct methods | Each entity has its own repo |
| **Repositories → SQLite** | rusqlite Connection | Thread-safe via Mutex or thread-local |

### External Dependencies

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| **SQLite (bundled)** | rusqlite with `bundled` feature | No system SQLite needed |
| **Project directories** | `directories::ProjectDirs` | Platform-specific app data paths |
| **Logging** | `tracing` or `println!` | For debugging persistence issues |

---

## Sources

- **Context7:** Dioxus 0.7 documentation — "Working with Databases" (2026)
- **WebSearch:** "Dioxus Rust SQLite integration best practices 2025" — patterns for desktop apps
- **WebSearch:** "Building Cross-Platform Desktop Apps in Rust with Dioxus 0.6" (Aug 2025) — architecture patterns
- **Codebase:** Existing `src/storage/sqlite.rs` schema, `src/app_state/state.rs` state functions

**Confidence:** HIGH — Multiple sources confirm the repository pattern is standard for rusqlite desktop apps, and the existing architecture maps cleanly to this approach.

---

*Architecture research for: SQLite persistence integration in Dioxus desktop app*  
*Researched: 2026-03-26*  
*Phase: Fixing broken persistence (critical blocker)*