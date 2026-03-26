<!-- GSD:project-start source:PROJECT.md -->
## Project

**Plantarium**

Desktop application for personal garden management - plan garden beds, organize plants, track tasks and maintain a botanical journal. Built with Dioxus + Rust + SQLite.

**Core Value:** A personal garden planner that helps users design beds visually, track planting schedules, manage gardening tasks, and document their garden's growth over time.

### Constraints

- **Tech Stack**: Dioxus 0.7.3 + Rust 2021 + rusqlite — existing, not changing
- **Platform**: Desktop-first with web as secondary target — current architecture supports both
- **Data**: Local SQLite storage — no cloud/sync planned
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Rust (edition 2021) - Core application logic and Dioxus components
## Runtime
- Dioxus Web (wasm) - Browser-based UI
- Dioxus Desktop - Native desktop application
- Cargo (Rust)
## Frameworks
- Dioxus 0.7.3 - UI framework with built-in router
- rusqlite 0.30 - SQLite bindings with bundled SQLite
- serde 1.0 - Serialization framework
- serde_json 1.0 - JSON support
- uuid 1.0 - ID generation (v4, js compatibility)
- chrono 0.4 - Date/time handling with serde
- directories 5.0 - Platform-specific data directories
- gloo-storage 0.3 - Browser localStorage (optional, web only)
- pulldown-cmark 0.12 - Markdown parsing
- ammonia 4.0 - HTML sanitization
## Configuration Files
- Package: plantarium v0.1.0
- Edition: 2021
- Features:
- App name: plantarium
- Output: dist/
- Asset dir: assets/
- Bundle ID: com.plantarium
## Build Profile
- opt-level: z (size optimization)
- lto: true (Link Time Optimization)
- codegen-units: 1
- panic: abort
- incremental: false
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

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
## Code Style
### Formatting
- **Tool:** rustfmt via `cargo fmt`
- **Config:** Default Rust formatting rules
- **Run:** `cargo fmt` before committing
### Linting
- **Tool:** clippy via `cargo clippy`
- **Run:** `cargo clippy` during development
### Import Organization
### Module Declaration
## Dioxus Patterns
### Component Definition
#[component]
### Global State
### Router
#[derive(Clone, Debug, PartialEq, Routable)]
## Data Models
### Syncable Entities
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
### Enums with Serde
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
## Error Handling
### Custom Error Types
#[derive(Debug)]
### Trait Implementations
## Comments & Documentation
### TODO Comments
### Function Documentation
## Commit Message Convention
## Project Structure
## Platform-Specific Code
#[cfg(target_arch = "wasm32")]
#[cfg(not(target_arch = "wasm32"))]
## Key Configuration
### Cargo.toml Features
- `desktop` (default): Desktop application
- `web`: Web/WASM application
### Build Commands
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Single-page application (SPA) built with Dioxus 0.7.3
- Client-side routing via `dioxus-router` with file-based route definitions
- Global reactive state via Dioxus `Signal::global` for cross-component data sharing
- LocalStorage persistence for web builds (WASM)
- Layered architecture: Pages → Components → State → Storage
## Layers
### Presentation Layer (Pages)
- **Location:** `src/pages/`
- **Contains:** Page-level components that define full-screen views
- **Depends on:** Components, State, Router
- **Used by:** Router via `Route` enum
- `Dashboard` (`src/pages/dashboard.rs`) - Main landing with garden overview
- `GardenDetail` (`src/pages/garden_detail.rs`) - Single garden with bed management
- `BedEditor` (`src/pages/bed_editor.rs`) - Visual bed/plot editor
- `Calendar` (`src/pages/calendar.rs`) - Event calendar view
- `Journal` (`src/pages/journal.rs`) - Botanical journal entries
- `Tasks` (`src/pages/tasks.rs`) - Task management
### UI Component Layer
- **Location:** `src/components/`
- **Contains:** Reusable UI components (buttons, cards, widgets)
- **Depends on:** State (optional), Router (for navigation)
- **Used by:** Pages, Layouts
- `tasks/` - Task-specific components (calendar, cards, modals)
- `dashboard_v2/` - Dashboard V2 components (cards, panels, headers)
- Root components: `Navbar`, `GardenCard`, `StatCard`, `Header`, etc.
### Layout Layer
- **Location:** `src/layouts/`
- **Contains:** Shell layouts that wrap page content
- **Depends on:** Components (Navbar), Router (Outlet)
- **Used by:** Router via `#[layout]` attribute
### State Management Layer
- **Location:** `src/app_state/`
- **Contains:** Global signals, data models, business logic functions
- **Depends on:** Dioxus signals, LocalStorage (web)
- **Used by:** Components, Pages
- `GARDENS: GlobalSignal<Vec<Garden>>`
- `BEDS: GlobalSignal<Vec<Bed>>`
- `PLANTS: GlobalSignal<Vec<Plant>>`
- `TASKS: GlobalSignal<Vec<Task>>`
- `EVENTS: GlobalSignal<Vec<CalendarEvent>>`
- `JOURNAL: GlobalSignal<Vec<JournalEntry>>`
- `PLOT_ACTIONS: GlobalSignal<Vec<PlotAction>>`
- `BED_ORDERS: GlobalSignal<HashMap<String, Vec<String>>>`
### Persistence Layer
- **Location:** `src/storage/`
- **Contains:** Database abstraction (SQLite for desktop, LocalStorage for web)
- **Depends on:** State layer (reads/writes state)
- **Used by:** State management via `load_from_storage()` and `save_to_storage()`
### Router
- **Location:** `src/router.rs`
- **Contains:** Route enum with all application routes
- **Uses:** Dioxus `Routable` derive macro
```
```
## Data Flow
### Application Initialization
### Navigation Flow
```
```
### State Mutation Flow
```
```
## Key Abstractions
### Data Models (in `src/app_state/state.rs`)
- `SyncableEntity` - Base struct with id, timestamps, soft-delete
- `Garden` - Collection of beds with settings
- `Bed` - Growing area containing placed plants
- `PlacedPlant` - Plant instance at coordinates within a bed
- `Plant` - Master plant catalog (name, color, icon)
- `Task` / `CalendarEvent` / `JournalEntry` - Activity tracking
- `PlotAction` - History of planting/harvesting events
### Demo Data (`src/app_state/demo_data.rs`)
- Provides hardcoded demo content for UI prototyping
- Functions: `get_demo_gardens_v2()`, `get_demo_harvests()`, `get_demo_maintenance_tasks()`
## Entry Points
- Location: `src/main.rs`
- Triggers: `dioxus::launch(App)`
- Responsibilities: Load stylesheets, initialize storage, mount router
- Location: `src/router.rs`
- Triggers: `Router::<Route>` in main App
- Responsibilities: Match URL to route, render layout + page
## Error Handling
- Storage operations use `ok()` / `if let Ok()` to handle missing data gracefully
- `println!` for debugging click handlers (e.g., garden click in dashboard)
- No centralized error boundary
## Cross-Cutting Concerns
- `main.css`, `plantarium-theme.css`, `stitch-theme.css`, `tasks.css`
- `#[cfg(target_arch = "wasm32")]` - Web/LocalStorage
- `#[cfg(not(target_arch = "wasm32"))]` - Desktop fallback
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
