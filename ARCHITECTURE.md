# Plantarium Rust Architecture

## Overview

This document describes the technical architecture for the Plantarium application built with Rust + Tauri 2.0 + Svelte 5.

## Technology Stack

### Current (MVP)

| Component | Technology | Version | Notes |
|-----------|------------|---------|-------|
| Desktop/Mobile Framework | Tauri | 2.0 | ✅ Activo |
| Frontend Framework | Svelte | 5.0 | ✅ Activo — usa Runes API |
| Language (frontend) | TypeScript | 5.8 | ✅ Activo |
| Build Tool | Vite | 6.0 | ✅ Activo |
| Persistence | localStorage | — | ⚠️ Temporal, solo MVP |
| Backend Rust | — | — | ❌ Solo `greet` de plantilla |

### Planned (Post-MVP)

| Component | Technology | Version | Notes |
|-----------|------------|---------|-------|
| Persistence | SQLite via tauri-plugin-sql | — | Reemplaza localStorage |
| Notificaciones | tauri-plugin-notification | — | Para alertas y recordatorios |
| HTTP externo | fetch nativo / tauri-plugin-http | — | APIs Permapeople y OpenWeather |
| Markdown | marked + DOMPurify | — | Reemplaza parser casero (ver seguridad) |
| Fechas | date-fns o dayjs | — | Para cálculos de calendario y schedules |

> **Nota sobre Axum:** Para la app local (Tauri), la comunicación frontend-backend usa
> **Tauri commands** (`#[tauri::command]`), no un servidor HTTP interno.
> Sin embargo, Axum sí tiene su lugar en este proyecto: es el backend del **servicio cloud**
> (sincronización entre dispositivos, modelo de monetización). El servidor Axum vive
> separado de la app Tauri, desplegado en infraestructura propia.

## Architecture

### Actual (MVP)

```
┌──────────────────────────────────────────┐
│           Frontend (Svelte 5)            │
│   App.svelte + pages/ + store.ts         │
├──────────────────────────────────────────┤
│           localStorage                   │
│   JSON serializado bajo 'plantarium_data'│
└──────────────────────────────────────────┘
     ↕ Tauri WebView (sin comandos activos)
┌──────────────────────────────────────────┐
│        Tauri Runtime (Rust)              │
│   src-tauri/src/lib.rs — solo `greet`   │
└──────────────────────────────────────────┘
```

### Target (Post-MVP — local)

```
┌──────────────────────────────────────────┐
│           Frontend (Svelte 5)            │
│   Llama a Tauri commands via invoke()    │
├──────────────────────────────────────────┤
│        Tauri Commands (Rust)             │
│   #[tauri::command] en src-tauri/src/   │
│   Lógica de negocio + validación        │
├──────────────────────────────────────────┤
│        tauri-plugin-sql (SQLite)         │
│         plantarium.db                    │
└──────────────────────────────────────────┘
```

### Visión long-term (cloud sync)

```
┌─────────────────┐     ┌─────────────────┐
│  App Desktop    │     │   App Móvil     │
│  Tauri + Svelte │     │  Tauri + Svelte │
│  SQLite local   │     │  SQLite local   │
└────────┬────────┘     └────────┬────────┘
         │    sync (Axum API)    │
         └──────────┬────────────┘
              ┌─────▼──────┐
              │ Cloud Axum │  ← servicio de pago
              │ + SQLite / │
              │ PostgreSQL  │
              └────────────┘
```

La app funciona 100% offline. La sincronización es opcional y de pago.
Los datos locales siempre son la fuente de verdad — el cloud es una réplica.

## Project Structure

```
plantarium-rust/
├── SPEC.md                 # Especificación de producto
├── ARCHITECTURE.md         # Este archivo
├── MVP_PLAN.md             # Plan del MVP (completado)
│
└── frontend/               # Aplicación Tauri
    ├── package.json
    ├── vite.config.ts
    ├── svelte.config.js
    ├── index.html
    │
    ├── src/                # Frontend Svelte
    │   ├── main.ts
    │   ├── App.svelte      # Router hash-based + navbar
    │   ├── lib/
    │   │   ├── router.ts   # Router custom (hash)
    │   │   └── store.ts    # Stores Svelte + auto-save localStorage
    │   ├── pages/
    │   │   ├── Dashboard.svelte      # Lista de áreas
    │   │   ├── AreaDetail.svelte     # Parcelas de un área
    │   │   ├── LayoutEditor.svelte   # Editor click-to-place
    │   │   ├── Calendar.svelte       # Vista mensual
    │   │   ├── Journal.svelte        # Diario con markdown
    │   │   └── Tasks.svelte          # Lista de tareas
    │   └── types/
    │       └── index.ts    # Interfaces TypeScript
    │
    └── src-tauri/          # Backend Rust (Tauri)
        ├── Cargo.toml
        ├── tauri.conf.json
        └── src/
            ├── main.rs     # Entry point
            └── lib.rs      # Tauri commands (solo `greet` por ahora)
```

## Data Model

### Tipos actuales (TypeScript — `types/index.ts`)

> ⚠️ **Problema de sincronización:** Para que el cloud sync funcione correctamente,
> todas las entidades necesitan `updatedAt` y `deletedAt`. Actualmente solo
> `JournalEntry` tiene `updatedAt`. Hay que añadirlos **antes** de migrar a SQLite.

```typescript
// Estado actual — incompleto para sync
interface GardenArea { id, name, createdAt }                         // ❌ falta updatedAt, deletedAt
interface Plot       { id, areaId, name, width, height, plants[] }   // ❌ falta updatedAt, deletedAt
interface PlacedPlant{ id, plantId, x, y }                           // ❌ falta updatedAt, deletedAt
interface Plant      { id, name, color, icon }                        // ❌ falta updatedAt, deletedAt
interface Task       { id, title, date, type, completed }             // ❌ falta updatedAt, deletedAt
interface CalendarEvent { id, title, date, type, plantId? }          // ❌ falta updatedAt, deletedAt
interface JournalEntry  { id, date, content, createdAt, updatedAt }  // ❌ falta deletedAt

// Target — listo para sync
interface SyncableEntity {
  id: string;          // UUID v4 — globalmente único entre dispositivos
  createdAt: number;   // timestamp ms UTC
  updatedAt: number;   // timestamp ms UTC — actualizar en cada mutación
  deletedAt: number | null; // null = activo, timestamp = borrado (soft delete)
}

interface GardenArea extends SyncableEntity { name }
interface Plot       extends SyncableEntity { areaId, name, width, height }
interface PlacedPlant extends SyncableEntity { plotId, plantId, x, y }
interface Plant      extends SyncableEntity { name, color, icon, family?, species? }
interface Task       extends SyncableEntity { title, date, type, completed }
interface CalendarEvent extends SyncableEntity { title, date, type, plantId? }
interface JournalEntry  extends SyncableEntity { date, content }
```

**Por qué soft deletes:** Si un usuario borra algo en móvil estando offline y luego
sincroniza, el servidor necesita saber que fue un borrado intencional — no que
el registro simplemente "no llegó". Sin `deletedAt`, el sync restauraría
los registros borrados en el próximo ciclo.

### Database Schema (Target — SQLite)

Todas las tablas incluyen `updated_at` y `deleted_at` para soportar sincronización.
Las queries de lectura deben filtrar siempre por `deleted_at IS NULL`.

```sql
CREATE TABLE garden_areas (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER             -- NULL = activo
);

CREATE TABLE plots (
    id          TEXT PRIMARY KEY,
    area_id     TEXT NOT NULL REFERENCES garden_areas(id),
    name        TEXT NOT NULL,
    width       REAL NOT NULL,
    height      REAL NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER
);

CREATE TABLE placed_plants (
    id          TEXT PRIMARY KEY,
    plot_id     TEXT NOT NULL REFERENCES plots(id),
    plant_id    TEXT NOT NULL,
    x           REAL NOT NULL,
    y           REAL NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER
);

CREATE TABLE plants (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    color       TEXT NOT NULL,
    icon        TEXT NOT NULL,
    family      TEXT,
    species     TEXT,
    notes       TEXT,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER
);

CREATE TABLE tasks (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL,
    date        TEXT NOT NULL,
    type        TEXT NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER
);

CREATE TABLE calendar_events (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL,
    date        TEXT NOT NULL,
    type        TEXT NOT NULL,
    plant_id    TEXT,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER
);

CREATE TABLE journal_entries (
    id          TEXT PRIMARY KEY,
    date        TEXT NOT NULL,
    content     TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER
);

-- Índices para sync eficiente: buscar registros modificados desde última sync
CREATE INDEX idx_areas_updated      ON garden_areas(updated_at);
CREATE INDEX idx_plots_updated      ON plots(updated_at);
CREATE INDEX idx_plants_updated     ON plants(updated_at);
CREATE INDEX idx_tasks_updated      ON tasks(updated_at);
CREATE INDEX idx_events_updated     ON calendar_events(updated_at);
CREATE INDEX idx_journal_updated    ON journal_entries(updated_at);
```

> **ON DELETE CASCADE eliminado intencionalmente.** Con soft deletes, las filas
> nunca se borran físicamente. Un `plot` borrado mantiene sus `placed_plants`
> con `deleted_at` propio para que el sync pueda reconstruir el historial en otros
> dispositivos.

## Tauri Commands (Target)

En lugar de endpoints HTTP, la comunicación frontend-backend usa Tauri commands:

```rust
// src-tauri/src/lib.rs
#[tauri::command]
async fn get_areas(db: State<'_, DbPool>) -> Result<Vec<GardenArea>, String> { ... }

#[tauri::command]
async fn create_area(db: State<'_, DbPool>, name: String) -> Result<GardenArea, String> { ... }

#[tauri::command]
async fn get_plots(db: State<'_, DbPool>, area_id: String) -> Result<Vec<Plot>, String> { ... }

#[tauri::command]
async fn fetch_plant_external(query: String) -> Result<Vec<PlantData>, String> { ... }
```

Llamados desde el frontend:
```typescript
import { invoke } from '@tauri-apps/api/core';
const areas = await invoke<GardenArea[]>('get_areas');
```

## Tauri Plugins Requeridos

| Plugin | Propósito | Prioridad |
|--------|-----------|-----------|
| `tauri-plugin-sql` | SQLite local | Alta — reemplaza localStorage |
| `tauri-plugin-notification` | Recordatorios y alertas | Alta — requerido por SPEC |
| `tauri-plugin-http` | Llamadas a Permapeople y OpenWeather | Media |
| `tauri-plugin-store` | Settings persistentes del usuario | Baja |

## Security

### XSS en el Diario (CRÍTICO)

`Journal.svelte` usa `{@html parseMarkdown(content)}` con un parser de regex casero
sobre contenido de usuario sin sanitizar. Esto permite XSS arbitrario.

**Solución:**
```bash
npm install marked dompurify
npm install -D @types/dompurify
```

```typescript
import { marked } from 'marked';
import DOMPurify from 'dompurify';

function parseMarkdown(text: string): string {
  return DOMPurify.sanitize(marked.parse(text) as string);
}
```

### API Keys

No embeber las keys de Permapeople y OpenWeather en el código frontend.
Almacenarlas en el proceso Tauri (Rust) y exponerlas solo via commands.

### SQLite

Usar `tauri-plugin-sql` con sentencias parametrizadas para prevenir SQL injection.

## Known Gaps vs SPEC

| Requisito SPEC | Estado | Tarea pendiente |
|----------------|--------|-----------------|
| SQLite persistencia | ❌ | Migrar de localStorage a tauri-plugin-sql |
| Notificaciones nativas | ❌ | Integrar tauri-plugin-notification |
| API Permapeople | ❌ | Tauri command que llama la API externa |
| API OpenWeather | ❌ | Tauri command con ubicación del usuario |
| Calendario Day/Week/Year views | ❌ | Implementar en Calendar.svelte |
| Diario vistas por período | ❌ | Implementar en Journal.svelte |
| Companion planting UI | ❌ | Requiere datos de Permapeople |
| Crop rotation tracking | ❌ | Requiere historial en DB |
| Dashboard (hub central) | ❌ | Dashboard actual solo muestra áreas |
| Markdown seguro | ⚠️ XSS | Reemplazar parser casero con marked + DOMPurify |

## Performance Considerations

- Svelte 5 compila a JS vanilla — sin virtual DOM, bundle pequeño
- Tauri usa el WebView nativo del OS — no bundlea Chromium
- SQLite con WAL mode para escrituras concurrentes
- Cachear respuestas de Permapeople API localmente en SQLite
- Operaciones de DB en Tauri commands (hilo async Tokio), nunca bloqueando el UI thread
