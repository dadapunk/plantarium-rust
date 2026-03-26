# Pitfalls Research

**Domain:** SQLite persistence in Rust desktop apps (Dioxus + rusqlite)
**Researched:** 2026-03-26
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: Database Lock Contention

**What goes wrong:** "database is locked" errors during write operations, data loss when writes fail silently.

**Why it happens:** SQLite uses file-level locking with only one writer at a time. Default journal mode (DELETE) forces readers to wait for writers. Zero busy timeout means immediate failure on lock conflicts.

**How to avoid:**
```rust
// Enable WAL mode for better concurrency
conn.execute_batch("PRAGMA journal_mode=WAL;")?;
// Set busy timeout to wait for locks
conn.execute_batch("PRAGMA busy_timeout=5000;")?; // 5 seconds
```

**Warning signs:** "database is locked" errors in console, slow save operations, app freezes during persistence.

**Phase to address:** Phase 1 (SQLite Wiring) - must be configured at initialization.

---

### Pitfall 2: Connection Not Thread-Safe

**What goes wrong:** Race conditions or panics when accessing database from multiple threads.

**Why it happens:** rusqlite::Connection is not thread-safe by default. Multiple threads accessing the same connection causes undefined behavior.

**How to avoid:**
```rust
// Use Mutex for single connection
use std::sync::Mutex;
let db = Mutex::new(Connection::open("plantarium.db")?);

// Or use thread_local for connection-per-thread
thread_local! {
    static DB: Connection = Connection::open("plantarium.db").unwrap();
}
```

**Warning signs:** Random panics only on desktop builds, "connection closed" errors, inconsistent data between reads.

**Phase to address:** Phase 1 (SQLite Wiring) - ensure connection is properly managed.

---

### Pitfall 3: Database File Path Issues

**What goes wrong:** "No such file or directory" error on first run, app fails to start.

**Why it happens:** Parent directories don't exist when using relative paths. SQLite won't create parent directories automatically.

**How to avoid:**
```rust
use std::fs;
use dirs;

let db_dir = dirs::data_local_dir()
    .unwrap_or_else(|| std::path::PathBuf::from("."))
    .join("plantarium");

fs::create_dir_all(&db_dir).expect("Failed to create database directory");
let db_path = db_dir.join("plantarium.db");
let conn = Connection::open(&db_path)?;
```

**Warning signs:** First-run crashes, database file never created, works in dev but fails in release.

**Phase to address:** Phase 1 (SQLite Wiring) - must handle initialization before opening DB.

---

### Pitfall 4: Not Wiring Save Operations to State Mutations

**What goes wrong:** Data persists in memory but never written to database — all data lost on app restart.

**Why it happens:** Code has storage layer but doesn't call save functions after mutations. Current Plantarium bug: `save_to_storage()` is a no-op for non-wasm32 targets.

**How to avoid:**
```rust
fn add_garden(&mut self, garden: Garden) {
    self.gardens.push(garden);
    self.save_to_storage(); // Must be called after EVERY mutation
}
```

**Warning signs:** Data appears in UI but disappears on restart, web works (localStorage) but desktop doesn't.

**Phase to address:** Phase 1 (SQLite Wiring) - ensure all state mutations trigger persistence.

---

### Pitfall 5: Missing Error Handling for Database Failures

**What goes wrong:** App panics when database operations fail (disk full, permission denied, corruption).

**Why it happens:** Using `.expect()` or `.unwrap()` on database operations that can fail. Current code: `src/storage/sqlite.rs` line 169 uses `.expect()`.

**How to avoid:**
```rust
let conn = match Connection::open("db.db") {
    Ok(c) => c,
    Err(e) => {
        eprintln!("Failed to open database: {}", e);
        return Err(AppError::DatabaseInit(e));
    }
};
```

**Warning signs:** "unwrap" or "expect" in database code, no error propagation from storage layer.

**Phase to address:** Phase 1 (SQLite Wiring) - build robust error handling from the start.

---

### Pitfall 6: Soft-Delete Filter Inconsistency

**What goes wrong:** Some queries return deleted records, others don't — users see deleted items inconsistently.

**Why it happens:** No consistent enforcement of `WHERE deleted_at IS NULL` across all query functions.

**How to avoid:** Audit all query functions. Current issue: `get_bed_by_id()` returns deleted beds, `get_garden_beds()` filters correctly.

**Warning signs:** Deleted items appearing in lists, different filtering behavior across pages.

**Phase to address:** Phase 1 (SQLite Wiring) - audit all query functions for consistency.

---

### Pitfall 7: No Input Validation Before Database Inserts

**What goes wrong:** Invalid data stored in database — garbage data causes UI rendering issues.

**Why it happens:** Form inputs accepted directly without validation. Current issue: width/height can be negative or zero.

**How to avoid:**
```rust
fn validate_bed_dimensions(width: f64, height: f64) -> Result<(f64, f64), ValidationError> {
    if width <= 0.0 || width > 10000.0 {
        return Err(ValidationError::InvalidWidth);
    }
    if height <= 0.0 || height > 10000.0 {
        return Err(ValidationError::InvalidHeight);
    }
    Ok((width, height))
}
```

**Warning signs:** Parsing errors in logs, UI rendering issues with data values.

**Phase to address:** Phase 2 (Data Validation) - add after basic persistence works.

---

### Pitfall 8: Uninitialized Database Schema

**What goes wrong:** App crashes on first run because tables don't exist.

**Why it happens:** Schema creation code exists but isn't called on startup.

**How to avoid:**
```rust
fn init_database(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS gardens (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            deleted_at TEXT
        );
        CREATE TABLE IF NOT EXISTS beds (
            id TEXT PRIMARY KEY,
            garden_id TEXT NOT NULL,
            name TEXT NOT NULL,
            width REAL NOT NULL,
            height REAL NOT NULL,
            x REAL NOT NULL,
            y REAL NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            deleted_at TEXT,
            FOREIGN KEY (garden_id) REFERENCES gardens(id)
        );
        CREATE INDEX IF NOT EXISTS idx_beds_garden ON beds(garden_id);"
    )?;
    Ok(())
}
```

**Warning signs:** "no such table" errors, works in dev (old DB exists), fails in production.

**Phase to address:** Phase 1 (SQLite Wiring) - call on every app start.

---

### Pitfall 9: Unbounded Data Growth

**What goes wrong:** App slows down as data grows — no pagination, loads all data into memory.

**Why it happens:** No pagination, loads all data on startup, no cleanup of old data.

**How to avoid:**
```rust
fn get_garden_beds_paginated(conn: &Connection, garden_id: &str, page: usize, per_page: usize) -> Result<Vec<Bed>> {
    let offset = page * per_page;
    let mut stmt = conn.prepare(
        "SELECT * FROM beds WHERE garden_id = ? AND deleted_at IS NULL LIMIT ? OFFSET ?"
    )?;
    // ... pagination logic
}
```

**Warning signs:** Slow startup, high memory usage, no pagination controls in UI.

**Phase to address:** Phase 3 (Performance) - optimize after MVP features.

---

### Pitfall 10: No Transaction for Multi-Operation Changes

**What goes wrong:** Partial data saved if operation fails mid-way — inconsistent state.

**Why it happens:** Individual INSERT/UPDATE without transaction wrapping.

**How to avoid:**
```rust
fn delete_garden_with_beds(conn: &Connection, garden_id: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("UPDATE beds SET deleted_at = ? WHERE garden_id = ?", 
               params![chrono::Utc::now().to_rfc3339(), garden_id])?;
    tx.execute("UPDATE gardens SET deleted_at = ? WHERE id = ?", 
               params![chrono::Utc::now().to_rfc3339(), garden_id])?;
    tx.commit()?;
    Ok(())
}
```

**Warning signs:** Orphaned records, inconsistent state after errors.

**Phase to address:** Phase 1 (SQLite Wiring) - use transactions for multi-table operations.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Using `.expect()` for database errors | Shorter code | App crashes on minor errors | Never — use proper error handling |
| Skipping WAL mode | Simpler initial setup | Lock contention issues | Only for read-only apps |
| Single connection without mutex | No synchronization needed | Race conditions, panics | Only single-threaded apps |
| No input validation | Faster to implement | Garbage data in DB | Only during prototyping |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Dioxus Desktop + rusqlite | Using `#[server]` functions | Use native desktop code, not server functions |
| Web vs Desktop builds | Using wasm32-specific storage | Use conditional compilation `#[cfg(not(target_arch = "wasm32"))]` |
| File paths | Relative paths | Use `dirs` crate for platform-appropriate paths |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Load all data on startup | Slow startup with large datasets | Add pagination, lazy loading | At ~1000+ records |
| No WAL mode | "database locked" errors under load | Enable WAL + busy timeout | Concurrent writes > 2 |
| No indexes | Slow queries on large tables | Add indexes on foreign keys | At ~100+ rows |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| No encryption | Data readable if machine compromised | SQLite encryption (SQLCipher) for sensitive data |
| No input sanitization | SQL injection (if dynamic SQL) | Use parameterized queries only |
| World-readable DB file | Privacy leak | Set appropriate file permissions |

---

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| No error feedback on save failure | User doesn't know data wasn't saved | Show toast/notification on save errors |
| No undo for delete | Deleted data lost forever | Soft-delete with recovery period |
| No backup option | No way to recover from data loss | Implement export functionality |

---

## "Looks Done But Isn't" Checklist

- [ ] **SQLite persistence:** Database file created but no data written — verify data survives restart
- [ ] **Save triggers:** Save functions called but connection not initialized on desktop — verify desktop builds persist
- [ ] **WAL mode:** Pragma set but not verified — check WAL file created
- [ ] **Error handling:** Try/catch exists but catches all errors the same — verify meaningful error messages
- [ ] **Schema init:** Tables created but indexes missing — verify query performance
- [ ] **Soft-delete:** Delete function exists but filter inconsistent — verify deleted items never appear

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Lock contention | LOW | Enable WAL mode, set busy timeout, retry logic |
| Corrupted DB | MEDIUM | Restore from backup, re-enter data |
| Missing parent directory | LOW | Create directory, re-run app |
| Uninitialized schema | LOW | Call init, restart app |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Database locks (#1) | Phase 1 | Check WAL file exists, test concurrent writes |
| Thread safety (#2) | Phase 1 | Run multi-threaded tests, check for panics |
| Path issues (#3) | Phase 1 | Test first-run on fresh machine |
| Not wiring saves (#4) | Phase 1 | Restart app, verify data persists |
| Error handling (#5) | Phase 1 | Simulate disk full, permission errors |
| Soft-delete (#6) | Phase 1 | Query deleted items, verify filtered |
| Input validation (#7) | Phase 2 | Test negative values, overflow |
| Schema init (#8) | Phase 1 | Fresh install test |
| Unbounded growth (#9) | Phase 3 | Load test with large dataset |
| No transactions (#10) | Phase 1 | Test error mid-operation |

---

## Sources

- rusqlite documentation (https://docs.rs/rusqlite/latest/rusqlite/)
- SQLite pragma reference (https://sqlite.org/pragma.html)
- Matrix Rust SDK SQLite locked issue (#5362): https://github.com/matrix-org/matrix-rust-sdk/issues/5362
- Dioxus database guide (https://dioxuslabs.com/learn/0.7/tutorial/databases/)
- SQLite locking documentation (https://sqlite.org/lockingv3.html)
- "Battling with SQLite in a Concurrent Environment" (2025-12-19): https://www.drmhse.com/posts/battling-with-sqlite-in-a-concurrent-environment/
- "Fix: SQLite Database Is Locked Error" (2026-03-10): https://fixdevs.com/blog/sqlite-database-is-locked/

---

*Pitfalls research for: SQLite persistence in Rust desktop apps*
*Researched: 2026-03-26*