# Storage Architecture

## Overview

Plantarium utiliza **SQLite** como motor de persistencia principal para desktop, con una capa de abstracción diseñada para facilitar la migración futura a PostgreSQL/cloud.

**Principios:**
- **Desktop-first:** SQLite sincrónico (sin async/await)
- **Storage abstraction:** Trait `StorageProvider` permite cambiar backend sin tocar UI
- **Sync over async:** Simplifica código, suficiente para MVP desktop
- **Cross-platform ready:** Mismo AppState funcionará en web/mobile

## Arquitectura

```
┌─────────────────────────────────────────┐
│         Frontend (Dioxus UI)            │
│                                         │
│  Pages → Components → AppState          │
│                    ↓                    │
│           GlobalSignals                 │
│         (GARDENS, BEDS, etc.)          │
│                    ↓                    │
│      CRUD Functions (state.rs)          │
│         create_garden()                 │
│         save_to_storage()               │
│         load_from_storage()             │
└────────────────────┼────────────────────┘
                     │
┌────────────────────┼────────────────────┐
│   Storage Abstraction Layer             │
│                    ↓                    │
│  ┌──────────────────────────────┐      │
│  │   StorageProvider trait      │      │
│  │   - init()                   │      │
│  │   - load_all() → AppState    │      │
│  │   - save_all(AppState)       │      │
│  └───────┬─────────────┬────────┘      │
│          │             │                │
│  ┌───────┴──────┐ ┌───┴──────────┐    │
│  │ SqliteStorage│ │ (Future)      │    │
│  │ (rusqlite)   │ │ WebStorage    │    │
│  │ ~/.plantarium│ │ (localStorage)│    │
│  └──────────────┘ └──────────────┘    │
└─────────────────────────────────────────┘
```

## Module Structure

```
src/storage/
├── mod.rs         # Re-exports: StorageError, StorageProvider, SqliteStorage
├── db.rs          # Trait StorageProvider + Error handling
└── sqlite.rs      # SqliteStorage implementation
```

### db.rs - Trait & Error

```rust
pub enum StorageError {
    DatabaseError(String),    // rusqlite errors
    IoError(io::Error),       // Filesystem errors
    Custom(String),           // Other errors
}

pub trait StorageProvider {
    fn init(&self) -> Result<(), StorageError>;
    fn load_all(&self) -> Result<AppState, StorageError>;
    fn save_all(&self, state: &AppState) -> Result<(), StorageError>;
}
```

**Features:**
- `From<rusqlite::Error>` implementation
- `From<io::Error>` implementation
- `Display` and `Error` traits
- Extensible for future backends

### sqlite.rs - Implementation

```rust
pub struct SqliteStorage {
    db_path: PathBuf,  // ~/.plantarium/data.db
}

impl SqliteStorage {
    pub fn new() -> Result<Self, StorageError>;
    fn get_db_path() -> Result<PathBuf, StorageError>;
    fn open_connection(&self) -> Result<Connection, StorageError>;
}

impl StorageProvider for SqliteStorage {
    fn init(&self) -> Result<(), StorageError>;      // Create tables
    fn load_all(&self) -> Result<AppState, StorageError>;  // Load from DB
    fn save_all(&self, state: &AppState) -> Result<(), StorageError>; // Save to DB
}
```

**Key features:**
- Uses `directories` crate for cross-platform paths
- Creates `~/.plantarium/` directory automatically
- Embedded SQL schema (no migration files)
- Sync operations (no async runtime needed)

## Database Schema

### Tablas

#### gardens
```sql
CREATE TABLE gardens (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    min_bed_distance INTEGER,
    bed_spacing INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER
);
```

#### beds
```sql
CREATE TABLE beds (
    id TEXT PRIMARY KEY,
    garden_id TEXT NOT NULL,
    name TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    x INTEGER,
    y INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER,
    FOREIGN KEY (garden_id) REFERENCES gardens(id)
);
```

#### plants
```sql
CREATE TABLE plants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    icon TEXT NOT NULL,
    family TEXT,
    species TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER
);
```

#### placed_plants
```sql
CREATE TABLE placed_plants (
    id TEXT PRIMARY KEY,
    bed_id TEXT NOT NULL,
    plant_id TEXT NOT NULL,
    x REAL NOT NULL,
    y REAL NOT NULL,
    harvested_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER,
    FOREIGN KEY (bed_id) REFERENCES beds(id),
    FOREIGN KEY (plant_id) REFERENCES plants(id)
);
```

#### tasks
```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    date TEXT NOT NULL,
    type TEXT NOT NULL,  -- JSON enum: Sowing, Watering, Harvest, Fertilizing, Custom
    completed INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER
);
```

#### journal_entries
```sql
CREATE TABLE journal_entries (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER
);
```

#### calendar_events
```sql
CREATE TABLE calendar_events (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    date TEXT NOT NULL,
    type TEXT NOT NULL,
    plant_id TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER
);
```

#### plot_actions
```sql
CREATE TABLE plot_actions (
    id TEXT PRIMARY KEY,
    bed_id TEXT NOT NULL,
    plant_id TEXT NOT NULL,
    action TEXT NOT NULL,  -- Planted, Sowed, Harvested, Removed
    quantity INTEGER NOT NULL,
    date TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    deleted_at INTEGER,
    FOREIGN KEY (bed_id) REFERENCES beds(id),
    FOREIGN KEY (plant_id) REFERENCES plants(id)
);
```

#### bed_orders
```sql
CREATE TABLE bed_orders (
    garden_id TEXT NOT NULL,
    bed_id TEXT NOT NULL,
    position INTEGER NOT NULL,
    PRIMARY KEY (garden_id, bed_id)
);
```

### Soft Delete Pattern

Todas las tablas principales usan `deleted_at INTEGER`:
- `NULL` = entidad activa
- `timestamp` = entidad eliminada (soft delete)
- Permite recuperación y sincronización futura

### Foreign Keys

- `beds.garden_id` → `gardens.id`
- `placed_plants.bed_id` → `beds.id`
- `placed_plants.plant_id` → `plants.id`
- `plot_actions.bed_id` → `beds.id`
- `plot_actions.plant_id` → `plants.id`

## Implementation Prompts (Progress)

### ✅ PROMPT 1: Cargo.toml Configuration
- [x] rusqlite 0.30 with bundled feature
- [x] directories 5.0
- [x] Build successful

### ✅ PROMPT 2: Storage Module Structure
- [x] `src/storage/mod.rs` - Re-exports
- [x] `src/storage/db.rs` - StorageProvider trait + StorageError
- [x] `src/storage/sqlite.rs` - SqliteStorage with init(), load_all(), save_all()
- [x] SQL schema embedded in init()
- [x] Compiles successfully

### ⏳ PROMPT 3: SQL Migrations Schema
- [ ] Create migrations.sql file (optional, currently embedded)
- [ ] Add migration versioning system
- [ ] Test schema creation on fresh DB

### ⏳ PROMPT 4: Init & Helper Functions
- [ ] Implement get_db_path() fully
- [ ] Implement open_connection() with error handling
- [ ] Add connection pooling (if needed)

### ⏳ PROMPT 5: CRUD - Load from SQLite
- [ ] Implement load queries for all 8 tables
- [ ] Map ResultSet → Rust structs
- [ ] Handle relationships (beds → plants, etc.)
- [ ] Test load_all()

### ⏳ PROMPT 6: CRUD - Save to SQLite
- [ ] Implement save queries (INSERT/UPDATE)
- [ ] Handle soft deletes
- [ ] Batch operations for performance
- [ ] Test save_all()

### ⏳ PROMPT 7: Demo Data Initialization
- [ ] Seed database with demo data:
  - 1 garden
  - 3 beds
  - 5 placed plants
  - 5 tasks
  - 2 journal entries
- [ ] Only if DB is empty

### ⏳ PROMPT 8: Main.rs Refactor
- [ ] Initialize SqliteStorage on startup
- [ ] Call storage.init() on first run
- [ ] Call storage.load_all() into GlobalSignals
- [ ] Remove old localStorage code

### ⏳ PROMPT 9: Integrate save_to_storage()
- [ ] Replace no-op in save_to_storage() with SQLite backend
- [ ] Auto-save on state changes
- [ ] Test persistence across app restarts

### ⏳ PROMPT 10: Verification & Testing
- [ ] Manual test: Create garden → close → reopen → data persists
- [ ] Manual test: Add plant to bed → save → reload
- [ ] Manual test: Soft delete → verify marked as deleted
- [ ] Verify DB file location: `~/.plantarium/data.db`
- [ ] Test with fresh DB (no demo data)
- [ ] Test with existing data

## Usage Example

```rust
use crate::storage::{SqliteStorage, StorageProvider};

fn main() {
    // Initialize storage
    let storage = SqliteStorage::new().expect("Failed to init storage");
    
    // Create tables if not exist
    storage.init().expect("Failed to initialize DB");
    
    // Load data into GlobalSignals
    let state = storage.load_all().expect("Failed to load data");
    
    // Populate GlobalSignals
    *GARDENS.write() = state.gardens;
    *BEDS.write() = state.beds;
    // ... etc
    
    // Launch Dioxus app
    dioxus::launch(App);
}

// In CRUD functions (state.rs)
pub fn save_to_storage() {
    let storage = SqliteStorage::new().unwrap();
    let state = AppState {
        gardens: GARDENS.read().clone(),
        beds: BEDS.read().clone(),
        // ... etc
    };
    storage.save_all(&state).expect("Failed to save");
}
```

## Future: Migration to PostgreSQL

When ready to migrate to cloud:

1. **Create PostgresStorage** implementing StorageProvider:
   ```rust
   pub struct PostgresStorage {
       pool: PgPool,
   }
   
   impl StorageProvider for PostgresStorage {
       fn init(&self) -> Result<(), StorageError> { /* create tables */ }
       fn load_all(&self) -> Result<AppState, StorageError> { /* load from PG */ }
       fn save_all(&self, state: &AppState) -> Result<(), StorageError> { /* save to PG */ }
   }
   ```

2. **Update main.rs**:
   ```rust
   // let storage = SqliteStorage::new()?;
   let storage = PostgresStorage::new("postgres://...")?;
   ```

3. **UI code unchanged** - StorageProvider abstraction handles the rest

## Performance Considerations

- **Sync I/O:** SQLite synchronous operations are fast enough for desktop (<10ms queries)
- **No connection pooling needed:** Single-threaded desktop app
- **Batch operations:** save_all() should use transactions
- **Indexing:** Primary keys on `id` columns provide sufficient indexing for MVP

## Troubleshooting

### DB file not created
- Check `~/.plantarium/` directory permissions
- Verify directories crate is working: `ProjectDirs::from("com", "plantarium", "plantarium")`

### Data not persisting
- Verify save_to_storage() is being called
- Check StorageError logs
- Use `sqlite3 ~/.plantarium/data.db` to inspect DB manually

### Migration errors
- Delete `~/.plantarium/data.db` and restart app (fresh start)
- Check SQL schema syntax in init()

---

**Status:** PROMPT 2 completed, ready for PROMPT 3-10 implementation  
**Last updated:** 2026-03-18
