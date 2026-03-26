---
title: TESTING.md
focus: quality
date: 2026-03-26
version: 1.0
---

# Testing Patterns

**Analysis Date:** 2026-03-26

## Test Framework Status

**Current State:** No automated unit tests exist in the project.

The project relies on manual testing via `cargo run` for development verification. There are no dedicated test modules, test files, or automated test infrastructure in place.

---

## Manual Testing Approach

### Running the Application

```bash
# Development
cargo run

# Clean rebuild
rm -rf target
cargo clean
cargo build --no-default-features --features desktop

# Run with debug output
RUST_LOG=debug cargo run
```

### Build Verification

```bash
# Check compilation without building
cargo check

# Build development binary
cargo build

# Build release binary
cargo build --release
```

---

## Test Configuration

### Cargo.toml Test Settings

Per `CONTRIBUTING.md`:

```bash
# Run all tests
cargo test

# Run with coverage
cargo test --coverage
```

Note: These commands are documented but no actual test modules exist to execute.

---

## Manual Test Scenarios

### Application Startup

- [ ] App launches without errors
- [ ] Desktop window opens
- [ ] All routes are accessible

### Navigation

- [ ] Navbar renders with all links
- [ ] Routes work correctly:
  - Dashboard (/)
  - Garden Detail (/garden/:id)
  - Bed Editor (/bed/:id)
  - Calendar (/calendar)
  - Journal (/journal)
  - Tasks (/tasks)

### Component Verification

Refer to `TESTING_GUIDE.md` for detailed component verification checklist:

- Tasks page components render correctly
- Task modal opens/closes properly
- Form validation works
- Checkbox interactions provide visual feedback

### CSS Class Verification

- [ ] All CSS class names match component references
- [ ] Responsive layout adapts to different screen sizes

---

## Known Issues & Verification

### Historical Issues (Now Fixed)

| Issue | Status | Verification |
|-------|--------|--------------|
| "Failed to open URL" error | Fixed | Launch app, check no errors |
| CSS class naming mismatch | Fixed | Inspect elements in DevTools |
| App compilation failures | Fixed | `cargo build` succeeds |
| Compilation warnings | Fixed | `cargo build` shows no warnings |

---

## Test Data

### Demo Data

The project includes demo data in `src/app_state/demo_data.rs`:

- Demo gardens with bed counts and plant counts
- Sample tasks with various types and states
- Demo harvest data

### Default Plants

Pre-populated plant library (10 plants):
- Tomato, Lettuce, Carrot, Pepper, Onion, Garlic, Potato, Bean, Corn, Pumpkin

---

## Storage Testing

### LocalStorage (Web)

Testing persistence via browser LocalStorage:
- Open browser DevTools → Application → Local Storage
- Verify `plantarium_data_v2` key exists
- Verify `plantarium_bed_order` key exists

### SQLite (Desktop)

Database location: `~/.plantarium/data.db` (Linux/macOS)

- Verify database file is created
- Verify schema tables exist

---

## Testing Patterns to Implement

### Recommended Test Structure

If tests were to be added, the recommended structure would be:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syncable_entity_default() {
        let entity = SyncableEntity::default();
        assert!(!entity.id.is_empty());
        assert!(entity.created_at > 0);
    }
    
    #[test]
    fn test_create_garden() {
        let garden = create_garden("Test Garden");
        assert_eq!(garden.name, "Test Garden");
    }
}
```

### Test File Organization

- Co-located with implementation: `src/storage/sqlite.rs` → `src/storage/sqlite_test.rs`
- Or dedicated test directory: `tests/` for integration tests

### Key Areas Requiring Tests

1. **State Management** (`src/app_state/state.rs`)
   - CRUD operations for all entities
   - Storage load/save functions

2. **Storage Layer** (`src/storage/`)
   - `StorageProvider` trait implementations
   - SQLite initialization
   - Data migration

3. **Component Logic**
   - Data transformation functions
   - Event handlers

---

## CI/CD Considerations

### Pre-commit Checks

```bash
cargo fmt --check
cargo clippy
cargo test
cargo build
```

### Recommended Git Hooks

- Pre-commit: Format and lint checks
- Pre-push: Full test suite

---

## Documentation References

- `TESTING_GUIDE.md` - Detailed manual testing checklist
- `CONTRIBUTING.md` - Test command references
- `README.md` - Feature verification checklist

---

## Gaps & Recommendations

### Current Testing Gaps

1. **No automated unit tests** - All testing is manual
2. **No integration tests** - Database interactions untested
3. **No component tests** - Dioxus component rendering untested
4. **No E2E tests** - Full user flows not automated

### Recommended Improvements

1. Add unit tests for state management functions
2. Add integration tests for SQLite storage
3. Add snapshot tests for components
4. Consider Playwright for E2E (see `.PLAYWRIGHT_MCP.md`)

---

*Testing analysis: 2026-03-26*
