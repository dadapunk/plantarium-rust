---
title: SQLite Persistence for Dioxus Desktop App
focus: stack
date: 2026-03-26
version: 1.0
---

# SQLite Persistence Stack Research

**Project:** Plantarium (Dioxus + Rust garden management app)
**Research Date:** 2026-03-26
**Question:** Best approach for implementing SQLite persistence in a Dioxus desktop app?

## Summary

For a Dioxus desktop application requiring SQLite persistence, the recommended approach is **rusqlite with bundled SQLite**, initialized in the platform-specific application data directory. This replaces the broken LocalStorage persistence and provides proper cross-session data persistence. The existing rusqlite 0.30 dependency should be upgraded to 0.38.x, and a migration system should be added to manage schema evolution.

## Recommended Stack

### Database Layer

| Library | Version | Purpose | Rationale |
|---------|---------|---------|------------|
| **rusqlite** | 0.38.0 | Core SQLite bindings | Lightweight, synchronous (appropriate for desktop), bundled feature avoids system SQLite dependency |
| **rusqlite_migration** | 2.0+ | Schema migrations | Simple, stores version in SQLite's user_version field, no external dependencies |

**Why rusqlite over alternatives:**

| Alternative | Verdict | Reason |
|-------------|---------|--------|
| **SQLx** | Not recommended | Async-only overhead unnecessary for single-user desktop app; adds tokio dependency |
| **Diesel** | Not recommended | ORM complexity not needed; compile-time SQL checking unnecessary for simple app |
| **SeaORM** | Not recommended | Heavy for local-only use case; async-first adds unnecessary complexity |
| **rusqlite** | **Recommended** | Lightweight, synchronous, bundled SQLite, perfect for desktop |

### Database Location

```rust
// Use directories crate (already in stack) for platform-specific paths
use directories::ProjectDirs;

fn get_db_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "plantarium", "Plantarium") {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir).ok();
        data_dir.join("plantarium.db")
    } else {
        PathBuf::from("plantarium.db") // Fallback to current directory
    }
}
```

### Current Problem (from PROJECT.md)

- **Critical bug:** SQLite persistence not wired up — data lost between sessions
- Current code uses LocalStorage for web, but desktop fallback doesn't save to SQLite
- Demo data hardcoded in state (no way to clear or persist user data)

## Implementation Pattern

### 1. Database Initialization (src/storage/sqlite.rs)

```rust
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&path)?;
        
        // Enable WAL mode for better concurrent read/write
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
        )?;
        
        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn initialize_schema(&self) -> Result<()> {
        // Tables: gardens, beds, plants, tasks, events, journal_entries
        // Use migrations for version management
    }
}
```

### 2. Migration Pattern

```rust
// src/storage/migrations.rs
use rusqlite_migration::{Migration, Schema};

fn get_migrations() -> Schema<'static> {
    Schema::from(vec![
        Migration::new(1, "initial schema")
            .with_up("CREATE TABLE gardens (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )")
            .with_up("CREATE TABLE beds (
                id TEXT PRIMARY KEY,
                garden_id TEXT NOT NULL,
                name TEXT NOT NULL,
                width REAL NOT NULL,
                height REAL NOT NULL,
                FOREIGN KEY (garden_id) REFERENCES gardens(id) ON DELETE CASCADE
            )"),
        // ... more tables
    ])
}
```

### 3. Repository Pattern for Data Access

```rust
// src/storage/repositories/garden_repo.rs
pub struct GardenRepository<'a> {
    conn: &'a Connection,
}

impl<'a> GardenRepository<'a> {
    pub fn create(&self, garden: &Garden) -> Result<()> {
        self.conn.execute(
            "INSERT INTO gardens (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (&garden.id, &garden.name, &garden.created_at, &garden.updated_at),
        )?;
        Ok(())
    }

    pub fn list_all(&self) -> Result<Vec<Garden>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at FROM gardens ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Garden {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;
        rows.collect()
    }
}
```

### 4. State Integration

```rust
// src/app_state/state.rs - Integrate with existing global signals

pub fn load_from_storage() {
    // 1. Initialize SQLite database
    let db = Database::new(get_db_path()).expect("Failed to open database");
    db.run_migrations().expect("Failed to run migrations");
    
    // 2. Load data into global signals
    let garden_repo = GardenRepository::new(&db.conn);
    if let Ok(gardens) = garden_repo.list_all() {
        GARDENS.set(gardens);
    }
    // ... load other entities
}

pub fn save_to_storage() {
    // Write to SQLite instead of LocalStorage
    // Called after any state mutation
}
```

## Configuration Changes

### Cargo.toml

```toml
[dependencies]
# Upgrade existing
rusqlite = { version = "0.38", features = ["bundled"] }

# Add new
rusqlite_migration = "2.0"

# Already present, keep
directories = "5.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

## What NOT To Use

| Library | Why Not |
|---------|---------|
| **SQLx** | Async runtime (tokio) adds unnecessary complexity for single-user desktop app |
| **Diesel** | Full ORM with compile-time query checking — overkill for simple schema; requires diesel_cli for migrations |
| **SeaORM** | Async-first, relationship-heavy ORM — designed for web services, not local desktop apps |
| **refinery** | Alternative migration tool — rusqlite_migration is simpler and stores version in SQLite user_version |
| **anyhow for errors** | rusqlite uses `Result<T>` with `rusqlite::Error` — stick to standard error handling pattern |

## Patterns to Follow

1. **Single database instance** — Create once at app startup, pass references to repositories
2. **WAL mode** — Enables concurrent reads while writing, better UX
3. **Foreign keys** — Enable `PRAGMA foreign_keys = ON` for cascade deletes
4. **Bundled SQLite** — Avoids system SQLite version issues
5. **Repository pattern** — Clean separation between storage and business logic

## Phase-Specific Notes

### Current Phase (Persistence Fix)

- Create database initialization in existing storage layer
- Implement schema for: gardens, beds, plants, placed_plants, tasks, events, journal_entries, plot_actions
- Wire up existing `save_to_storage()` to write to SQLite instead of LocalStorage
- Load data on app startup

### Future Phases

- Add migration system for schema evolution
- Consider transaction batching for bulk operations
- Optional: Add FTS (full-text search) for journal entries

## Sources

- Context7: /rusqlite/rusqlite (rusqlite documentation)
- WebSearch: "Rust ORMs in 2026: Diesel vs SQLx vs SeaORM vs Rusqlite" (Feb 2026, Medium) — comparison and recommendation for rusqlite for desktop
- WebSearch: "Rust - Embedding a SQLite database in a Tauri Application" (Jan 2025) — practical patterns for desktop app SQLite

---

*Research: 2026-03-26 — SQLite persistence for Dioxus desktop app*