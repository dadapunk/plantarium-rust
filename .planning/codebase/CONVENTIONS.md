---
title: CONVENTIONS.md
focus: quality
date: 2026-03-26
version: 1.0
---

# Coding Conventions

**Analysis Date:** 2026-03-26

## Naming Patterns

### Files

- **Rust source files:** snake_case (e.g., `task_card.rs`, `sqlite.rs`, `mod.rs`)
- **CSS files:** snake_case (e.g., `main.css`, `tasks.css`, `plantarium-theme.css`)
- **Configuration files:** kebab-case (e.g., `Dioxus.toml`)

### Modules

- **Directories:** snake_case (e.g., `app_state/`, `components/tasks/`, `storage/`)
- **Module names:** snake_case matching filename

### Components (Dioxus)

- **Component functions:** PascalCase (e.g., `TaskCard`, `Navbar`, `Layout`)
- **Props/structs:** PascalCase (e.g., `TaskCardData`, `GardenStatus`)

### Types

- **Structs:** PascalCase (e.g., `Garden`, `Bed`, `Task`)
- **Enums:** PascalCase with variant variants (e.g., `TaskType`, `PlotActionType`)
- **Traits:** PascalCase (e.g., `StorageProvider`)

### Variables & Functions

- **Functions:** snake_case (e.g., `create_garden`, `load_from_storage`, `save_to_storage`)
- **Private functions:** prefix with underscore where appropriate
- **Constants:** SCREAMING_SNAKE_CASE for configuration (e.g., `STORAGE_KEY`, `BED_ORDERS_KEY`)

### CSS Classes

- **Classes:** kebab-case (e.g., `task-card`, `task-card-header`, `task-list-item`)
- **Component-scoped:** Use consistent prefixes (e.g., `task-*` for task components)

---

## Code Style

### Formatting

- **Tool:** rustfmt via `cargo fmt`
- **Config:** Default Rust formatting rules
- **Run:** `cargo fmt` before committing

### Linting

- **Tool:** clippy via `cargo clippy`
- **Run:** `cargo clippy` during development

### Import Organization

Standard Rust import order:
1. `use` imports from standard library
2. `use` imports from external crates
3. `use` imports from local modules (`crate::`)

```rust
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod app_state;
mod components;
mod router;
```

### Module Declaration

Use `mod` for private modules, `pub mod` for public:

```rust
mod app_state;
mod components;
pub mod dashboard_v2;
pub mod tasks;
```

---

## Dioxus Patterns

### Component Definition

```rust
#[component]
pub fn TaskCard(task: TaskCardData, on_toggle: EventHandler<()>) -> Element {
    rsx! {
        div { class: "task-card",
            // component implementation
        }
    }
}
```

### Global State

Use `Signal::global` for global signals:

```rust
pub static GARDENS: GlobalSignal<Vec<Garden>> = Signal::global(Vec::new);
pub static BEDS: GlobalSignal<Vec<Bed>> = Signal::global(Vec::new);
```

### Router

Use `#[derive(Routable)]` with `#[layout(Layout)]`:

```rust
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Dashboard {},
    
    #[route("/tasks")]
    Tasks {},
}
```

---

## Data Models

### Syncable Entities

Use `#[serde(flatten)]` with a base struct:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SyncableEntity {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Garden {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub name: String,
    pub min_bed_distance: Option<i32>,
    pub bed_spacing: Option<i32>,
}
```

### Enums with Serde

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    Sowing,
    Watering,
    Harvest,
    Fertilizing,
    Custom,
}
```

---

## Error Handling

### Custom Error Types

```rust
#[derive(Debug)]
pub enum StorageError {
    DatabaseError(String),
    IoError(io::Error),
    Custom(String),
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            StorageError::IoError(err) => write!(f, "IO error: {}", err),
            StorageError::Custom(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}
```

### Trait Implementations

```rust
pub trait StorageProvider {
    fn load_all(&self) -> Result<AppState, StorageError>;
    fn save_all(&self, state: &AppState) -> Result<(), StorageError>;
    fn init(&self) -> Result<(), StorageError>;
}
```

---

## Comments & Documentation

### TODO Comments

Document planned implementation with TODO markers:

```rust
// TODO: Implement load queries in PROMPT 5
// TODO: Implement save queries in PROMPT 6
```

### Function Documentation

Document public functions:

```rust
/// Load application state from storage
/// For web: reads from LocalStorage
/// For desktop: uses SQLite (not yet implemented)
pub fn load_from_storage() { ... }
```

---

## Commit Message Convention

Follow conventional commits:

```
feat: add plant search functionality
fix: resolve database connection timeout
docs: update API documentation
refactor: simplify storage trait
```

---

## Project Structure

```
plantarium-rust/
├── src/
│   ├── main.rs              # Entry point
│   ├── router.rs            # Route definitions
│   ├── layouts/             # Layout components
│   ├── pages/               # Page components
│   ├── components/         # Reusable UI components
│   │   ├── mod.rs          # Main exports
│   │   ├── dashboard_v2/   # Dashboard components
│   │   └── tasks/           # Tasks components
│   ├── app_state/          # Global state management
│   └── storage/            # Storage abstraction
│       ├── db.rs           # Trait + error types
│       └── sqlite.rs       # SQLite implementation
└── assets/                 # CSS and UI designs
```

---

## Platform-Specific Code

Use conditional compilation for platform differences:

```rust
#[cfg(target_arch = "wasm32")]
{
    if let Ok(data) = LocalStorage::get::<AppState>(STORAGE_KEY) { ... }
}

#[cfg(not(target_arch = "wasm32"))]
{
    *PLANTS.write() = default_plants();
}
```

---

## Key Configuration

### Cargo.toml Features

- `desktop` (default): Desktop application
- `web`: Web/WASM application

### Build Commands

```bash
cargo fmt              # Format code
cargo clippy           # Lint code
cargo build            # Build development
cargo build --release  # Build production
```

---

*Convention analysis: 2026-03-26*
