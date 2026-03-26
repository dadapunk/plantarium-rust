---
title: Codebase Structure
focus: arch
date: 2026-03-26
version: 1.0.0
---

# Codebase Structure

**Analysis Date:** 2026-03-26

## Directory Layout

```
plantarium-rust/
├── Cargo.toml              # Rust package manifest + dependencies
├── Dioxus.toml            # Dioxus framework config
├── src/
│   ├── main.rs            # App entry point
│   ├── router.rs          # Route definitions
│   ├── app_state/         # Global state + data models
│   ├── components/        # Reusable UI components
│   ├── layouts/           # Layout wrappers
│   ├── pages/             # Page-level components
│   └── storage/           # Persistence layer
├── assets/                # Static assets (CSS, images, HTML mocks)
└── .planning/codebase/    # This analysis
```

## Directory Purposes

### src/
- **Purpose:** All Rust source code
- **Contains:** Dioxus components, state, router, storage modules

### src/main.rs
- **Purpose:** Application entry point
- **Key:** `dioxus::launch(App)` bootstraps the app
- **Responsibilities:**
  - Load CSS stylesheets (main.css, plantarium-theme.css, stitch-theme.css, tasks.css)
  - Call `app_state::load_from_storage()` to initialize data
  - Mount `Router::<Route>` component

### src/router.rs
- **Purpose:** Define all application routes
- **Key:** Uses `#[derive(Routable)]` on `Route` enum
- **Routes:**
  - `/` → Dashboard
  - `/garden/:id` → GardenDetail
  - `/bed/:id` → BedEditor
  - `/calendar` → Calendar
  - `/journal` → Journal
  - `/tasks` → Tasks
- **Layout:** `Layout` component wraps all routes via `#[layout(Layout)]`

### src/app_state/
- **Purpose:** Global state management and data models
- **Files:**
  - `mod.rs` - Module exports
  - `state.rs` - **CRITICAL** - Contains all global signals and CRUD functions
  - `demo_data.rs` - Hardcoded demo content for UI prototyping

**Global Signals (state.rs):**
| Signal | Type | Purpose |
|--------|------|---------|
| `GARDENS` | `GlobalSignal<Vec<Garden>>` | All gardens |
| `BEDS` | `GlobalSignal<Vec<Bed>>` | All beds |
| `PLANTS` | `GlobalSignal<Vec<Plant>>` | Plant catalog |
| `TASKS` | `GlobalSignal<Vec<Task>>` | Task list |
| `EVENTS` | `GlobalSignal<Vec<CalendarEvent>>` | Calendar events |
| `JOURNAL` | `GlobalSignal<Vec<JournalEntry>>` | Journal entries |
| `PLOT_ACTIONS` | `GlobalSignal<Vec<PlotAction>>` | Planting/harvest history |
| `BED_ORDERS` | `GlobalSignal<HashMap<String, Vec<String>>>` | Bed ordering |

**Key Functions (state.rs):**
- `load_from_storage()` / `save_to_storage()` - Persistence
- `create_garden()`, `create_bed()`, `create_task()` - Create operations
- `add_plant_to_bed()`, `harvest_plant()`, `remove_plant_from_bed()` - Plot operations
- `get_garden_beds()`, `get_bed_by_id()`, `get_plant_by_id()` - Query operations

### src/components/
- **Purpose:** Reusable UI building blocks
- **Organization:** Flat components at root + feature-specific subdirectories

**Root Components:**
- `Navbar` - Main navigation bar
- `Header` - Page header
- `GardenCard` - Garden display card
- `GardenCardV2` - V2 dashboard card (Stitch design)
- `AddGardenCard` - Add new garden button/card
- `StatCard` - Statistics display
- `QuickReminders` - Reminder widget
- `ProTip` - Tips/guidance component

**Sub-modules:**
- `tasks/` - Task-specific components
  - `task_card.rs`, `task_list_item.rs` - Task display
  - `task_modal.rs`, `NewTask` - Task creation/editing
  - `calendar_widget.rs` - Calendar mini-view
  - `tasks_header.rs` - Tasks page header
  - `featured_card.rs`, `fertilizer_alert.rs` - Special task cards
- `dashboard_v2/` - V2 dashboard components
  - `dashboard_header.rs` - Dashboard header
  - `garden_card_v2.rs` - Garden card V2
  - `maintenance_panel.rs` - Maintenance task panel
  - `recent_harvests.rs` - Harvest history widget

### src/pages/
- **Purpose:** Full-screen page components (one per route)
- **Files:**
  - `dashboard.rs` - Main dashboard (V2 design)
  - `garden_detail.rs` - Single garden view with bed list
  - `bed_editor.rs` - Visual bed/plot editor
  - `calendar.rs` - Calendar view
  - `journal.rs` - Journal entries
  - `tasks.rs` - Task management

### src/layouts/
- **Purpose:** Shell layouts that wrap page content
- **Files:**
  - `mod.rs` - Contains `Layout` component with `Navbar` + `Outlet::<Route>`

### src/storage/
- **Purpose:** Database/persistence abstraction
- **Files:**
  - `mod.rs` - Module exports (`StorageProvider`, `SqliteStorage`)
  - `db.rs` - Storage trait definitions
  - `sqlite.rs` - SQLite implementation (desktop only)

### assets/
- **Purpose:** Static assets
- **Files:**
  - `main.css` - Base styles
  - `plantarium-theme.css` - Theme styles
  - `stitch-theme.css` - Stitch design system
  - `tasks.css` - Tasks page styles
  - `ui-designs/` - HTML/CSS mockups for design reference

## Key File Locations

### Entry Points
- `src/main.rs`: Application bootstrap
- `src/router.rs`: Route definitions

### Configuration
- `Cargo.toml`: Dependencies, features (web/desktop)
- `Dioxus.toml`: Dioxus build config

### Core Logic
- `src/app_state/state.rs`: All state + CRUD operations
- `src/components/tasks/task_modal.rs`: Task creation modal

### Testing
- `TESTING_GUIDE.md`: Testing documentation

## Where to Add New Code

### New Feature (New Page)
1. Create page component in `src/pages/<feature>.rs`
2. Export in `src/pages/mod.rs`
3. Add route in `src/router.rs`

### New Component
1. Add to `src/components/` or create new subdirectory
2. Export in `src/components/mod.rs`
3. Import in consuming page

### New State/Data Model
1. Add struct to `src/app_state/state.rs`
2. Create global signal if shared across components
3. Add CRUD functions
4. Update `load_from_storage()` / `save_to_storage()`

### New Utility Function
- If UI-related: add to appropriate component file
- If business logic: add to `src/app_state/state.rs`
- If data transformation: create `src/utils/` module

## Special Directories

### assets/ui-designs/
- **Purpose:** HTML/CSS mockups for design reference
- **Contains:** 8 screen designs as HTML/CSS pairs
- **Generated:** Pre-existing mockups, not from code

### .planning/codebase/
- **Purpose:** GSD planning documents
- **Generated:** Yes (this file)
- **Committed:** Yes

---

*Structure analysis: 2026-03-26*
