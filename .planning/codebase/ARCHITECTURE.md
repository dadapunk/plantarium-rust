---
title: Architecture Analysis
focus: arch
date: 2026-03-26
version: 1.0.0
---

# Architecture

**Analysis Date:** 2026-03-26

## Pattern Overview

**Overall:** Dioxus SPA with Global Signal State Management

**Key Characteristics:**
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

Pages:
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

Sub-modules:
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

Global Signals (in `src/app_state/state.rs`):
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

Routes:
```
/              → Dashboard
/garden/:id    → GardenDetail
/bed/:id       → BedEditor
/calendar      → Calendar
/journal       → Journal
/tasks         → Tasks
```

## Data Flow

### Application Initialization
1. `main()` launches Dioxus app
2. `App` component calls `app_state::load_from_storage()`
3. LocalStorage data loaded into global signals
4. Router renders layout + outlet

### Navigation Flow
```
User Click → Link Component → Router → Route Match → Page Component
```

### State Mutation Flow
```
User Action → Component Handler → State Function → Global Signal Update → Automatic Re-render
```

Example: Adding a bed (`src/pages/garden_detail.rs`)
1. User fills form inputs → signals `new_bed_name`, `new_bed_width`, `new_bed_height`
2. Button click triggers `create_bed()` 
3. `create_bed()` creates `Bed`, writes to `BEDS` global signal
4. `save_to_storage()` persists to LocalStorage
5. UI automatically updates via signal reactivity

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

**Application Entry:**
- Location: `src/main.rs`
- Triggers: `dioxus::launch(App)`
- Responsibilities: Load stylesheets, initialize storage, mount router

**Router Entry:**
- Location: `src/router.rs`
- Triggers: `Router::<Route>` in main App
- Responsibilities: Match URL to route, render layout + page

## Error Handling

**Strategy:** Simple console logging for errors, silent failures for storage

**Patterns:**
- Storage operations use `ok()` / `if let Ok()` to handle missing data gracefully
- `println!` for debugging click handlers (e.g., garden click in dashboard)
- No centralized error boundary

## Cross-Cutting Concerns

**Styling:** External CSS files loaded via Dioxus `document::Stylesheet`
- `main.css`, `plantarium-theme.css`, `stitch-theme.css`, `tasks.css`

**Internationalization:** Hardcoded Spanish text in UI (e.g., "Añadir Bancal", "Calendario")

**Platform-Specific Code:**
- `#[cfg(target_arch = "wasm32")]` - Web/LocalStorage
- `#[cfg(not(target_arch = "wasm32"))]` - Desktop fallback

---

*Architecture analysis: 2026-03-26*
