# 🌱 Plantarium

Gestión de huerto personal - Desktop app construida con Rust puro

## Descripción

Plantarium es una aplicación de escritorio para planificar y gestionar huertos personales. Permite diseñar bancales, organizar plantas, registrar tareas y llevar un diario del jardín.

**Stack actual:** Dioxus 0.7.3 + SQLite + Rust puro  
**Arquitectura:** Desktop-first con storage abstraction layer (preparado para migrar a web/mobile)

## Características

- **Jardines** - Crea y organiza múltiples jardines
- **Bancales** - Diseña bancales con dimensiones personalizadas y posición libre
- **Editor Visual** - Coloca plantas en el bancal con click, marca cosechas
- **Calendario** - Registra eventos de siembra, riego, cosecha
- **Tareas** - Gestiona tareas pendientes con filtros por tipo y estado
- **Diario** - Notas con soporte Markdown y seguimiento del huerto
- **Persistencia SQLite** - Datos guardados localmente en `~/.plantarium/data.db`

## Tech Stack

| Tecnología | Versión | Uso |
|------------|---------|-----|
| **Dioxus** | 0.7.3 | Framework UI declarativo (React-like) |
| **Rust** | 1.75+ | Lenguaje principal |
| **rusqlite** | 0.30 | SQLite sincrónico (sin async) |
| **directories** | 5.0 | Paths cross-platform para datos |
| **serde** | 1.0 | Serialización/deserialización |
| **chrono** | 0.4 | Fechas y timestamps |
| **uuid** | 1.0 | IDs únicos |

## Arquitectura

```
┌─────────────────────────────────────────┐
│           Frontend (Dioxus)             │
│  ┌─────────┬─────────┬─────────┐       │
│  │ Pages   │ Comps   │ Router  │       │
│  └────┬────┴────┬────┴────┬────┘       │
│       │         │         │             │
│       └─────────┴─────────┘             │
│                 │                        │
│         AppState (GlobalSignals)        │
│                 │                        │
└─────────────────┼───────────────────────┘
                  │
┌─────────────────┼───────────────────────┐
│    Storage Abstraction Layer            │
│  ┌──────────────┴──────────────┐       │
│  │   StorageProvider trait     │       │
│  └───────┬─────────────┬───────┘       │
│          │             │                │
│  ┌───────┴──────┐ ┌───┴──────────┐    │
│  │ SqliteStorage│ │Future: Web   │    │
│  │ (desktop)    │ │localStorage  │    │
│  └──────────────┘ └──────────────┘    │
└─────────────────────────────────────────┘
```

**Principios:**
- Desktop-first (sin async runtime para storage)
- Storage abstraction permite migrar a PostgreSQL/cloud sin cambios en UI
- Sincrono por simplicidad (sqlite sync es suficiente para MVP)
- Preparado para web (localStorage) y mobile (mismo AppState)

## Instalación y Uso

### Requisitos

- Rust 1.75+
- Cargo

### Desarrollo

```bash
# Clonar repositorio
git clone https://github.com/tu-usuario/plantarium-rust
cd plantarium-rust

# Ejecutar en modo desarrollo (desktop)
cargo run

# Ejecutar en modo desarrollo (web)
cargo run --features web

# Verificar compilación
cargo check
cargo build
```

### Producción

```bash
# Build optimizado desktop
cargo build --release

# Build optimizado web
cargo build --features web --release
```

## Estructura del Proyecto

```
plantarium-rust/
├── Cargo.toml              # Dependencias y features
├── src/
│   ├── main.rs             # Entry point
│   ├── router.rs           # Definición de rutas
│   ├── app_state/
│   │   ├── mod.rs
│   │   └── state.rs        # GlobalSignals + CRUD functions
│   ├── storage/
│   │   ├── mod.rs          # Re-exports
│   │   ├── db.rs           # StorageProvider trait + StorageError
│   │   └── sqlite.rs       # SqliteStorage implementation
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── dashboard.rs    # Vista principal de jardines
│   │   ├── garden_detail.rs # Detalle de jardín + CRUD beds
│   │   ├── bed_editor.rs   # Editor visual de bancal
│   │   ├── calendar.rs     # Vista calendario
│   │   ├── tasks.rs        # Gestión de tareas
│   │   └── journal.rs      # Diario del huerto
│   └── components/
│       └── mod.rs          # Navbar
├── assets/
│   ├── main.css            # Estilos globales
│   └── ui-designs/
│       └── plantarium-screens/  # Diseños UI de Stitch
│           ├── *.html      # 8 pantallas HTML+CSS
│           └── index.html  # Navegador de pantallas
├── README.md               # Este archivo
├── CHANGELOG.md            # Historial de cambios
├── SPEC.md                 # Especificación funcional
├── STORAGE.md              # Documentación de persistencia
└── CONTRIBUTING.md         # Guía de contribución
```

## Diseño UI

Los diseños de interfaz están disponibles en `assets/ui-designs/plantarium-screens/`:

### Pantallas Disponibles (8 total)

| # | Pantalla | Tema | Resolución |
|---|----------|------|------------|
| 1 | Tareas del Jardín | Oscuro | 2560x2488 |
| 2 | Editor de Bancales | Claro | 2560x2048 |
| 3 | Dashboard | Claro | 2560x2640 |
| 4 | Editor de Bancales | Oscuro | 2560x2048 |
| 5 | Dashboard | Oscuro | 2560x2514 |
| 6 | Botanical Journal | Claro | 2560x4904 |
| 7 | Diario | Oscuro | 2560x2838 |
| 8 | Tasks + Calendar | Claro | 2560x2668 |

### Design System

**"Botanical Editorial"** - Sistema de diseño basado en:
- **Colores primarios**: `#37602C`, `#4F7942` (verdes naturales)
- **Color secundario**: `#9F402D` (terracota)
- **Tipografía**: Noto Serif (títulos) + Manrope (cuerpo)
- **Redondeado**: 8px
- **Modos**: Light y Dark themes

Para explorar los diseños:
```bash
# Abrir navegador de pantallas
open assets/ui-designs/plantarium-screens/index.html
```

## Datos

### Ubicación

- **Desktop:** `~/.plantarium/data.db` (Linux/macOS) o `%APPDATA%/plantarium/data.db` (Windows)
- **Web:** localStorage (futuro, no implementado aún)

### Esquema SQLite

8 tablas principales:
- `gardens` - Jardines
- `beds` - Bancales (con FK a gardens)
- `plants` - Biblioteca de plantas
- `placed_plants` - Plantas colocadas (con FK a beds y plants)
- `tasks` - Tareas pendientes
- `journal_entries` - Diario del huerto
- `calendar_events` - Eventos del calendario
- `plot_actions` - Historial de acciones (planted, harvested, etc.)
- `bed_orders` - Orden de bancales por jardín

Ver detalles en [STORAGE.md](./STORAGE.md)

### Plantas Predefinidas

10 plantas disponibles por defecto:
- 🍅 Tomate
- 🥬 Lechuga
- 🥕 Zanahoria
- 🫑 Pimiento
- 🧅 Cebolla
- 🧄 Ajo
- 🥔 Papa
- 🫛 Judía
- 🌽 Maíz
- 🎃 Calabaza

## Roadmap

### ✅ Completado

- [x] Migración Svelte + Tauri → Dioxus 0.7.3
- [x] 6 páginas funcionales (Dashboard, GardenDetail, BedEditor, Calendar, Tasks, Journal)
- [x] Componentes refactorizados
- [x] Desktop app compilando y ejecutando
- [x] Cargo.toml configurado (rusqlite + directories)
- [x] Storage module structure (PROMPT 2)

### 🟡 En Progreso

- [ ] PROMPT 3: SQL migrations schema
- [ ] PROMPT 4: Init & helper functions
- [ ] PROMPT 5: CRUD - Load from SQLite
- [ ] PROMPT 6: CRUD - Save to SQLite
- [ ] PROMPT 7: Demo data initialization
- [ ] PROMPT 8: Main.rs refactor - load on startup
- [ ] PROMPT 9: Integrate save_to_storage() with SQLite backend
- [ ] PROMPT 10: Verification & manual testing

### ⏳ Futuro

- [ ] Integración Permapeople API (base de datos de plantas)
- [ ] Integración OpenWeather API (clima)
- [ ] Rotación de cultivos automática
- [ ] Plantas companion (compatibilidad)
- [ ] Exportar/importar datos
- [ ] Soporte web (localStorage fallback)
- [ ] Soporte mobile (iOS/Android)

## Contribuir

Ver [CONTRIBUTING.md](./CONTRIBUTING.md)

## Licencia

MIT

## Referencias

- [Dioxus 0.7 Docs](https://dioxuslabs.com/learn/0.7/)
- [rusqlite](https://docs.rs/rusqlite/)
- [directories crate](https://docs.rs/directories/)
