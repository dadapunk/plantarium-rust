# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-03-18

### Added

- **Migración completa a Dioxus 0.7.3**
  - Eliminado Svelte + Tauri (2.5GB de código removido)
  - App Rust pura con Dioxus como framework UI
  - Desktop-first architecture

- **6 páginas funcionales**
  - Dashboard: Vista principal de jardines (compact/expanded view)
  - GardenDetail: CRUD de bancales, duplicar, redimensionar, settings
  - BedEditor: Canvas interactivo, colocar plantas, harvest, remove
  - Calendar: Vista mensual, eventos, filtros por planta
  - Tasks: CRUD tareas, filtros estado/tipo, 5 categorías
  - Journal: Markdown support, entries CRUD

- **Componentes refactorizados**
  - TaskItem, JournalEntry, PlantButton, BedCanvas, PlacedPlantItem
  - Navbar con navegación
  - Router con 6 rutas definidas

- **SQLite Persistence Layer**
  - Cargo.toml: rusqlite 0.30 + directories 5.0 añadidos
  - Storage module structure creado (PROMPT 2 completado)
  - StorageProvider trait + StorageError enum
  - SqliteStorage con init(), load_all(), save_all()
  - SQL schema con 8 tablas definidas
  - DB path: `~/.plantarium/data.db`

- **AppState con GlobalSignals**
  - 7 GlobalSignals: GARDENS, BEDS, PLANTS, TASKS, EVENTS, JOURNAL, PLOT_ACTIONS
  - CRUD functions para todas las entidades
  - Soft-delete con timestamps (deleted_at)
  - Default plants (10 plantas predefinidas)

- **Documentación**
  - README.md reescrito con arquitectura actual
  - STORAGE.md creado para documentar persistencia
  - CHANGELOG.md actualizado

### Changed

- **Stack tecnológico**
  - Svelte 5 + TypeScript → Rust puro
  - Tauri 2 → Dioxus desktop
  - localStorage → SQLite (desktop-first)

- **Arquitectura de persistencia**
  - Web-first (localStorage) → Desktop-first (SQLite)
  - Async storage → Sync storage (sin Tokio/sqlx)
  - Sin abstracción → StorageProvider trait (preparado para cloud)

- **Build system**
  - Vite → Cargo
  - Tauri bundler → Dioxus CLI
  - npm scripts → cargo commands

### Removed

- `frontend/` directory (2.5GB Svelte + TypeScript code)
- Tauri dependencies
- gloo-storage como storage principal (desktop)
- Async runtime (Tokio) para storage

### Technical Details

- **Tamaño binario objetivo:** <15MB desktop, <10MB mobile
- **Runtime:** Sin async para storage (sync rusqlite)
- **Compatibilidad:** Desktop (actual), Web (futuro), Mobile (futuro)
- **Migración a cloud:** StorageProvider trait permite cambiar a PostgreSQL sin tocar UI

---

## [0.1.0] - Historical Reference

### Stack Original

- **Frontend:** Svelte 5 + TypeScript + Vite
- **Desktop:** Tauri 2.0
- **Backend:** NestJS + TypeORM (removido)
- **Features:** Mismas funcionalidades pero con stack diferente

### Notas

- Código original en `frontend/` (eliminado en 0.2.0)
- Migración completada exitosamente a Rust puro
- Ver MIGRATION_PLAN.md para detalles del proceso (histórico)

---

## Format

This changelog follows [Keep a Changelog](https://keepachangelog.com/) format.

Types of changes:
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for removed features
- `Fixed` for bug fixes
- `Security` for vulnerability fixes
