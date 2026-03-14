# Roadmap — Plantarium

Prioridades de implementación post-MVP, ordenadas por criticidad.
El MVP (Fases 1-6 del `MVP_PLAN.md`) está completo.

---

## Fase 0 — Deuda técnica crítica (antes de cualquier feature nueva)

Estos issues afectan seguridad o correctitud del código existente.

### 0.1 Corregir XSS en el Diario ✅

**Archivo:** `frontend/src/pages/Journal.svelte:167`
**Estado:** Completado - `marked` + `DOMPurify.sanitize()` implementados.

### 0.2 Corregir tauri.conf.json ✅

**Archivo:** `frontend/src-tauri/tauri.conf.json`
Completado: `productName`, `identifier` y `title` actualizados a "Plantarium" / "com.plantarium.app".

### 0.3 Preparar modelo de datos para sincronización ✅

**Archivos:** `frontend/src/types/index.ts`, `frontend/src/lib/store.ts`
**Estado:** Completado - Todas las entidades ahora extienden `SyncableEntity` con `updatedAt` y `deletedAt`. Store migrado automáticamente.

---

## Fase 1 — Features del NotebookLM (Mejoras del MVP)

Basado en la documentación del notebook, estas son las funcionalidades prioritarias para mejorar la experiencia de usuario.

### 1.1 Sistema de Ingreso de Cultivos

**Descripción:** Interfaz para que el usuario pueda seleccionar una planta, definir la cantidad y asociarla a un bancal específico con fecha.

**Implementación actual:** LayoutEditor tiene click-to-place básico.
**Mejora necesaria:** Formulario dedicado con:
- Selector de planta (desde la biblioteca)
- Input de cantidad
- Selector de bancal/parcela
- Selector de fecha (hoy por defecto, o fecha personalizada)
- Acción: plantar/sembrar/cosechar

**Archivo probable:** Nuevo componente `AddCropModal.svelte` o mejorar `LayoutEditor.svelte`

### 1.2 Histórico Cronológico por Bancal

**Descripción:** Registro histórico que guarda qué se ha plantado, sembrado o cosechado en cada bancal por fecha. Permite consultar estados pasados.

**Implementación:**
- Crear tabla/log de acciones por parcela
- Cada acción guardada: { plotId, plantId, actionType, date, quantity }
- Tipos de acción: 'planted', 'sowed', 'harvested'
- Vista de historial por parcela en `AreaDetail.svelte` o página dedicada

**Tipos de dato a agregar:**
```typescript
interface PlotAction {
  id: string;
  plotId: string;
  plantId: string;
  action: 'planted' | 'sowed' | 'harvested';
  quantity: number;
  date: string;
  createdAt: number;
  updatedAt: number;
  deletedAt: number | null;
}
```

### 1.3 Lógica de Cosecha No Destructiva

**Descripción:** Para plantas de producción escalonada (tomates, pimientos, etc.), al marcar una "cosecha" NO se elimina la planta. Los frutos verdes siguen madurando.

**Implementación:**
- Al cosechar, NO hacer soft-delete de la planta colocada
- Crear registro en `PlotAction` con action='harvested'
- La planta permanece en el canvas pero con estado "cosechada" (visual diferente)
- Campo `harvestedAt` en `PlacedPlant` para marcar fecha de primera cosecha

**Cambios en UI:**
- Añadir botón "Cosechar" en plantas colocdas
- Diferenciar visualmente plantas cosechadas (ej: opacidad, color muted)

### 1.4 Selector de Fechas

**Descripción:** El usuario puede guardar acciones (plantar, sembrar, cosechar) con fecha personalizada, no solo "hoy".

**Implementación:**
- En todos los formularios de ingreso, añadir campo de fecha
- Por defecto = fecha actual
- Permitir fecha pasada o futura
- Necesario para registrar siembras que se hicieron antes de usar la app

### 1.5 Disparadores Manuales (User-driven Actions)

**Descripción:** Ninguna acción es automática. Todas las transiciones de estado del huerto deben ser explícitas:
- El usuario decide cuándo plantar
- El usuario decide cuándo cosechar (no auto-remove)
- El usuario decide cuándo eliminar una planta

**Implementación:** Revisar que el flujo actual sea 100% manual. Añadir confirmaciones antes de acciones destructivas.

---

## Fase 2 — Backend Rust y persistencia real

Reemplazar `localStorage` por SQLite via Tauri commands. Sin esto no hay
integridad de datos, relaciones entre entidades, ni posibilidad de expandir features.

### 2.1 Configurar tauri-plugin-sql

```toml
# frontend/src-tauri/Cargo.toml
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
```

```bash
npm install @tauri-apps/plugin-sql
```

### 2.2 Crear migraciones SQLite

Crear `frontend/src-tauri/migrations/` con el schema definido en `ARCHITECTURE.md`.
Tablas: `garden_areas`, `plots`, `placed_plants`, `plants`, `tasks`, `calendar_events`, `journal_entries`.

### 2.3 Implementar Tauri commands

En `frontend/src-tauri/src/lib.rs`, implementar commands para cada entidad:

```
get_areas / create_area / update_area / delete_area
get_plots / create_plot / update_plot / delete_plot
get_journal / create_entry / update_entry / delete_entry
get_tasks / create_task / update_task / delete_task
get_events / create_event / delete_event
```

### 2.4 Migrar stores de localStorage a invoke()

Reemplazar las llamadas a `localStorage` en `store.ts` por `invoke('command_name', args)`.

---

## Fase 3 — Features pendientes de la SPEC (sin APIs externas)

Features que la SPEC define y que se pueden implementar sin Permapeople ni OpenWeather.

### 3.1 Dashboard completo

**Archivo:** `frontend/src/pages/Dashboard.svelte`
El dashboard actual solo muestra áreas. La SPEC (sección 6.1) define:

- Resumen de áreas con contador de parcelas
- Última nota del diario (fecha + preview del contenido)
- Próximas tareas (los siguientes 5-7 días)
- Mini calendario del mes actual con marcadores de eventos
- Acceso rápido a todas las secciones

### 3.2 Vistas del Calendario

**Archivo:** `frontend/src/pages/Calendar.svelte`
Actualmente solo existe vista mensual. Agregar:

- **Day view:** tareas y eventos del día con línea de tiempo
- **Week view:** grid 7 columnas con eventos por día
- **Year view:** 12 meses en miniatura con indicadores de densidad de eventos
- Selector de vista (Day / Week / Month / Year)

### 3.3 Vistas del Diario

**Archivo:** `frontend/src/pages/Journal.svelte`
Actualmente es una lista plana. Agregar:

- **Day view:** entradas de una fecha específica
- **Week view:** entradas de la semana seleccionada
- **Month view:** entradas del mes seleccionado
- **Year view:** entradas agrupadas por mes
- Selector de período con navegación anterior/siguiente

### 3.4 Crop Rotation Tracking (sin API)

- Guardar historial de qué planta estuvo en qué parcela por año
- Al colocar una planta en el editor, verificar si su familia ya estuvo en esa parcela los últimos N años
- Mostrar advertencia si hay conflicto de rotación
- Vista de historial por parcela

---

## Fase 4 — Integración de APIs externas

### 4.1 Permapeople API (base de datos de plantas)

Implementar como Tauri command en Rust para no exponer la API key al frontend.

Funcionalidades:
- Búsqueda de plantas por nombre
- Detalle de planta (familia, espaciado, compañeras, requisitos)
- Caché local en SQLite para evitar llamadas repetidas
- Fallback offline (mostrar datos cacheados)

Referencia: consultar la documentación oficial de Permapeople para los endpoints exactos
y el formato de autenticación antes de implementar.

### 4.2 OpenWeather API

Implementar como Tauri command en Rust.

Funcionalidades:
- Input de ubicación del usuario (ciudad o coordenadas)
- Fetch de condiciones actuales y pronóstico 7 días
- Detección de riesgo de helada
- Alertas cuando la temperatura baja del umbral configurado

Endpoints a usar: `api.openweathermap.org/data/2.5/forecast` con `units=metric`.

### 4.3 Schedules de siembra basados en clima

- Calcular ventanas de siembra (interior, trasplante, siembra directa)
- Combinar datos de Permapeople (requisitos de la planta) con OpenWeather (clima local)
- Mostrar sugerencias en el Calendario

---

## Fase 5 — Notificaciones y alertas

Requiere `tauri-plugin-notification`.

```toml
# frontend/src-tauri/Cargo.toml
tauri-plugin-notification = "2"
```

### 5.1 Recordatorios de tareas

- Notificación nativa del OS para tareas del día
- Configurable: horario de notificación, días de anticipación

### 5.2 Alertas de helada

- Notificación cuando OpenWeather detecta riesgo de helada en la ubicación del usuario

### 5.3 Alertas de rotación

- Notificación al abrir la app si hay plantas en parcelas con conflicto de rotación

---

## Fase 6 — Companion Planting

Depende de Fase 4.1 (Permapeople).

- Al seleccionar una planta en el editor, mostrar sidebar con compañeras beneficiosas y perjudiciales
- Resaltar visualmente en el canvas las plantas incompatibles ya colocadas
- Tooltips de compatibilidad al hacer hover sobre una planta

---

## Fase 7 — Distribución open source

El objetivo es publicar la app para que la comunidad la use y contribuya.

### 7.1 Licencia

Decidir la licencia antes de hacer el repo público:

- **MIT** — máxima libertad, cualquiera puede hacer un fork con cloud propio sin obligaciones
- **AGPLv3** — quien ofrezca el servicio como cloud debe publicar su código; protege el modelo de negocio

Recomendación: **MIT para la app** (Tauri + Svelte), **AGPLv3o propietario para el servidor cloud** (Axum).

### 7.2 Build y distribución

- Configurar GitHub Actions para builds automáticos en cada release (Windows, macOS, Linux)
- Publicar binarios en GitHub Releases
- Documentar el proceso de instalación por plataforma
- Instrucciones para que usuarios instalen en macOS sin Apple Developer certificate (clic derecho → Abrir)

### 7.3 Contribución de la comunidad

- `CONTRIBUTING.md` actualizado con guía de frontend (Svelte) y backend (Rust)
- Issues etiquetados con `good first issue` para onboarding
- Datos de plantas: definir cómo la comunidad puede contribuir catálogos por región

---

## Fase 8 — Cloud sync (monetización)

Requiere Fases 1-7 completas. Es el modelo de negocio: la app es gratuita y open source,
la sincronización entre dispositivos es de pago.

### 8.1 Backend Axum (servidor cloud)

Servidor independiente de la app Tauri, desplegado en infraestructura propia:

- Auth de usuarios (email/password o OAuth)
- Endpoints de sync: recibe cambios con `updated_at` mayor al último sync
- Resolución de conflictos: last-write-wins por `updated_at` como estrategia inicial
- PostgreSQL como base de datos del servidor (SQLite solo en el cliente)

### 8.2 Protocolo de sincronización

Flujo básico offline-first:

```
1. App abre → compara updated_at local vs timestamp del último sync
2. Envía al server: todos los registros con updated_at > último_sync
3. Recibe del server: todos los registros remotos con updated_at > último_sync
4. Merge local: si mismo id y updated_at remoto > local → actualizar local
5. Los registros con deleted_at != null se propagan como borrados
```

### 8.3 Modelo freemium

- **Gratis:** app completa, datos solo locales, sin sync
- **De pago:** sync entre desktop y móvil, backup en cloud, historial ilimitado
- **Self-hosted:** la comunidad puede montar su propio servidor (código open source o no, según licencia elegida en Fase 7.1)

---

## Resumen de prioridades

| Fase | Descripción | Dependencias |
|------|-------------|--------------|
| **0** | XSS fix + tauri.conf ✅ + modelo sync-ready | Ninguna — hacerlo ya |
| **1** | Features del Notebook (Ingreso cultivos, Histórico, Cosecha no destructiva, Selector fechas) | Fase 0 |
| **2** | SQLite + Tauri commands | Fase 0, Fase 1 |
| **3** | Dashboard, Calendar views, Journal views, Crop rotation | Fase 2 |
| **4** | Permapeople API, OpenWeather API, Schedules | Fase 2 |
| **5** | Notificaciones | Fase 2, Fase 4 |
| **6** | Companion planting | Fase 4 |
| **7** | Open source: licencia, builds, distribución | Fase 3-6 estables |
| **8** | Cloud sync + monetización | Fase 7 |

---

## Fuera del alcance actual

- Analytics y tracking de cosecha por planta/parcela
- Integración con sensores IoT de jardín
- Features comunitarias (compartir layouts, reviews de plantas)
- Gestión de plagas y enfermedades
