# Roadmap — Plantarium

Prioridades de implementación post-MVP, ordenadas por criticidad.
El MVP (Fases 1-6 del `MVP_PLAN.md`) está completo.

---

## Fase 0 — Deuda técnica crítica (antes de cualquier feature nueva)

Estos issues afectan seguridad o correctitud del código existente.

### 0.1 Corregir XSS en el Diario

**Archivo:** `frontend/src/pages/Journal.svelte:167`
**Problema:** `{@html parseMarkdown(content)}` renderiza HTML sin sanitizar sobre input del usuario.

```bash
npm install marked dompurify
npm install -D @types/dompurify
```

Reemplazar `parseMarkdown()` para usar `marked` + `DOMPurify.sanitize()`.

### 0.2 Corregir tauri.conf.json ✅

**Archivo:** `frontend/src-tauri/tauri.conf.json`
Completado: `productName`, `identifier` y `title` actualizados a "Plantarium" / "com.plantarium.app".

### 0.3 Preparar modelo de datos para sincronización

**Archivos:** `frontend/src/types/index.ts`
**Problema:** Solo `JournalEntry` tiene `updatedAt`. Ninguna entidad tiene `deletedAt`.
Sin estos campos, el cloud sync no puede resolver conflictos ni detectar borrados offline.

Añadir a **todas** las interfaces la base común:

```typescript
interface SyncableEntity {
  id: string;
  createdAt: number;
  updatedAt: number;
  deletedAt: number | null;
}
```

Y extender: `GardenArea`, `Plot`, `PlacedPlant`, `Plant`, `Task`, `CalendarEvent`, `JournalEntry`.

Actualizar `store.ts` para que cada `update()` y `create()` setee `updatedAt: Date.now()`
y los borrados usen `deletedAt: Date.now()` en lugar de filtrar el array.

> Hacerlo ahora, mientras los datos viven en localStorage y no hay usuarios con datos reales.
> Migrar el schema después con usuarios es costoso.

---

## Fase 1 — Backend Rust y persistencia real

Reemplazar `localStorage` por SQLite via Tauri commands. Sin esto no hay
integridad de datos, relaciones entre entidades, ni posibilidad de expandir features.

### 1.1 Configurar tauri-plugin-sql

```toml
# frontend/src-tauri/Cargo.toml
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
```

```bash
npm install @tauri-apps/plugin-sql
```

### 1.2 Crear migraciones SQLite

Crear `frontend/src-tauri/migrations/` con el schema definido en `ARCHITECTURE.md`.
Tablas: `garden_areas`, `plots`, `placed_plants`, `plants`, `tasks`, `calendar_events`, `journal_entries`.

### 1.3 Implementar Tauri commands

En `frontend/src-tauri/src/lib.rs`, implementar commands para cada entidad:

```
get_areas / create_area / update_area / delete_area
get_plots / create_plot / update_plot / delete_plot
get_journal / create_entry / update_entry / delete_entry
get_tasks / create_task / update_task / delete_task
get_events / create_event / delete_event
```

### 1.4 Migrar stores de localStorage a invoke()

Reemplazar las llamadas a `localStorage` en `store.ts` por `invoke('command_name', args)`.

---

## Fase 2 — Features pendientes de la SPEC (sin APIs externas)

Features que la SPEC define y que se pueden implementar sin Permapeople ni OpenWeather.

### 2.1 Dashboard completo

**Archivo:** `frontend/src/pages/Dashboard.svelte`
El dashboard actual solo muestra áreas. La SPEC (sección 6.1) define:

- Resumen de áreas con contador de parcelas
- Última nota del diario (fecha + preview del contenido)
- Próximas tareas (los siguientes 5-7 días)
- Mini calendario del mes actual con marcadores de eventos
- Acceso rápido a todas las secciones

### 2.2 Vistas del Calendario

**Archivo:** `frontend/src/pages/Calendar.svelte`
Actualmente solo existe vista mensual. Agregar:

- **Day view:** tareas y eventos del día con línea de tiempo
- **Week view:** grid 7 columnas con eventos por día
- **Year view:** 12 meses en miniatura con indicadores de densidad de eventos
- Selector de vista (Day / Week / Month / Year)

### 2.3 Vistas del Diario

**Archivo:** `frontend/src/pages/Journal.svelte`
Actualmente es una lista plana. Agregar:

- **Day view:** entradas de una fecha específica
- **Week view:** entradas de la semana seleccionada
- **Month view:** entradas del mes seleccionado
- **Year view:** entradas agrupadas por mes
- Selector de período con navegación anterior/siguiente

### 2.4 Crop Rotation Tracking (sin API)

- Guardar historial de qué planta estuvo en qué parcela por año
- Al colocar una planta en el editor, verificar si su familia ya estuvo en esa parcela los últimos N años
- Mostrar advertencia si hay conflicto de rotación
- Vista de historial por parcela

---

## Fase 3 — Integración de APIs externas

### 3.1 Permapeople API (base de datos de plantas)

Implementar como Tauri command en Rust para no exponer la API key al frontend.

Funcionalidades:
- Búsqueda de plantas por nombre
- Detalle de planta (familia, espaciado, compañeras, requisitos)
- Caché local en SQLite para evitar llamadas repetidas
- Fallback offline (mostrar datos cacheados)

Referencia: consultar la documentación oficial de Permapeople para los endpoints exactos
y el formato de autenticación antes de implementar.

### 3.2 OpenWeather API

Implementar como Tauri command en Rust.

Funcionalidades:
- Input de ubicación del usuario (ciudad o coordenadas)
- Fetch de condiciones actuales y pronóstico 7 días
- Detección de riesgo de helada
- Alertas cuando la temperatura baja del umbral configurado

Endpoints a usar: `api.openweathermap.org/data/2.5/forecast` con `units=metric`.

### 3.3 Schedules de siembra basados en clima

- Calcular ventanas de siembra (interior, trasplante, siembra directa)
- Combinar datos de Permapeople (requisitos de la planta) con OpenWeather (clima local)
- Mostrar sugerencias en el Calendario

---

## Fase 4 — Notificaciones y alertas

Requiere `tauri-plugin-notification`.

```toml
# frontend/src-tauri/Cargo.toml
tauri-plugin-notification = "2"
```

### 4.1 Recordatorios de tareas

- Notificación nativa del OS para tareas del día
- Configurable: horario de notificación, días de anticipación

### 4.2 Alertas de helada

- Notificación cuando OpenWeather detecta riesgo de helada en la ubicación del usuario

### 4.3 Alertas de rotación

- Notificación al abrir la app si hay plantas en parcelas con conflicto de rotación

---

## Fase 5 — Companion Planting

Depende de Fase 3.1 (Permapeople).

- Al seleccionar una planta en el editor, mostrar sidebar con compañeras beneficiosas y perjudiciales
- Resaltar visualmente en el canvas las plantas incompatibles ya colocadas
- Tooltips de compatibilidad al hacer hover sobre una planta

---

## Fase 6 — Distribución open source

El objetivo es publicar la app para que la comunidad la use y contribuya.

### 6.1 Licencia

Decidir la licencia antes de hacer el repo público:

- **MIT** — máxima libertad, cualquiera puede hacer un fork con cloud propio sin obligaciones
- **AGPLv3** — quien ofrezca el servicio como cloud debe publicar su código; protege el modelo de negocio

Recomendación: **MIT para la app** (Tauri + Svelte), **AGPLv3o propietario para el servidor cloud** (Axum).

### 6.2 Build y distribución

- Configurar GitHub Actions para builds automáticos en cada release (Windows, macOS, Linux)
- Publicar binarios en GitHub Releases
- Documentar el proceso de instalación por plataforma
- Instrucciones para que usuarios instalen en macOS sin Apple Developer certificate (clic derecho → Abrir)

### 6.3 Contribución de la comunidad

- `CONTRIBUTING.md` actualizado con guía de frontend (Svelte) y backend (Rust)
- Issues etiquetados con `good first issue` para onboarding
- Datos de plantas: definir cómo la comunidad puede contribuir catálogos por región

---

## Fase 7 — Cloud sync (monetización)

Requiere Fases 1-6 completas. Es el modelo de negocio: la app es gratuita y open source,
la sincronización entre dispositivos es de pago.

### 7.1 Backend Axum (servidor cloud)

Servidor independiente de la app Tauri, desplegado en infraestructura propia:

- Auth de usuarios (email/password o OAuth)
- Endpoints de sync: recibe cambios con `updated_at` mayor al último sync
- Resolución de conflictos: last-write-wins por `updated_at` como estrategia inicial
- PostgreSQL como base de datos del servidor (SQLite solo en el cliente)

### 7.2 Protocolo de sincronización

Flujo básico offline-first:

```
1. App abre → compara updated_at local vs timestamp del último sync
2. Envía al server: todos los registros con updated_at > último_sync
3. Recibe del server: todos los registros remotos con updated_at > último_sync
4. Merge local: si mismo id y updated_at remoto > local → actualizar local
5. Los registros con deleted_at != null se propagan como borrados
```

### 7.3 Modelo freemium

- **Gratis:** app completa, datos solo locales, sin sync
- **De pago:** sync entre desktop y móvil, backup en cloud, historial ilimitado
- **Self-hosted:** la comunidad puede montar su propio servidor (código open source o no, según licencia elegida en Fase 6.1)

---

## Resumen de prioridades

| Fase | Descripción | Dependencias |
|------|-------------|--------------|
| **0** | XSS fix + tauri.conf ✅ + modelo sync-ready | Ninguna — hacerlo ya |
| **1** | SQLite + Tauri commands | Fase 0 |
| **2** | Dashboard, Calendar views, Journal views, Crop rotation | Fase 1 |
| **3** | Permapeople API, OpenWeather API, Schedules | Fase 1 |
| **4** | Notificaciones | Fase 1, Fase 3 |
| **5** | Companion planting | Fase 3.1 |
| **6** | Open source: licencia, builds, distribución | Fase 2-5 estables |
| **7** | Cloud sync + monetización | Fase 6 |

---

## Fuera del alcance actual

- Analytics y tracking de cosecha por planta/parcela
- Integración con sensores IoT de jardín
- Features comunitarias (compartir layouts, reviews de plantas)
- Gestión de plagas y enfermedades
