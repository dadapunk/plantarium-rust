# Plan MVP: Plantarium

**Alcance:** Layout visual + Calendario + Tareas  
**Stack:** Svelte + Tauri 2.0 + localStorage  
**Sin backend, sin APIs externas**

---

## Fases de Implementación

### Fase 1: Proyecto Base (0.5 días) - COMPLETO

- [x] Proyecto Tauri 2.0 + Svelte
- [x] Configuración inicial del frontend

### Fase 2: Datos Locales (0.5 días) - COMPLETO

- [x] Definir interfaces TypeScript:
  - `GardenArea` (id, name)
  - `Plot` (id, areaId, name, width, height, plants[])
  - `Plant` (id, name, x, y, color)
  - `Task` (id, title, date, type, completed)
  - `CalendarEvent` (id, title, date, type)
- [x] Crear store Svelte con localStorage
  - `load()` - cargar desde localStorage
  - `save()` - guardar en localStorage

### Fase 3: Gestión de Parcelas (1 día) - COMPLETO

- [x] **UI: Lista de áreas** - crear/editar/eliminar áreas de jardín
- [x] **UI: Lista de parcelas** - crear/editar/eliminar parcelas por área
- [x] **Resize parcelas** - возможность изменять размер después de crear
- [x] **Duplicar parcelas** - duplicar una parcela existente
- [x] **Rutas:**
  - `/` - Dashboard con lista de áreas
  - `/area/:id` - Detalle de área con parcelas

### Fase 4: Editor Visual (2 días) - COMPLETO

- [x] **Canvas interactivo** - área de diseño con coordenadas
- [x] **Plantas predefinidas** - biblioteca de plantas (mock data)
- [x] **Drag-and-drop** - arrastrar plantas desde biblioteca a parcelas
- [x] **Posicionar** - mover plantas dentro de la parcela
- [x] **Persistir** - guardar posiciones en localStorage

### Fase 5: Calendario (1 día) - COMPLETO

- [x] **Vista mensual** - grid de días del mes
- [x] **Navegación** - mes anterior/siguiente
- [x] **Eventos** - mostrar eventos en cada día
- [x] **Añadir evento** - crear evento desde día del calendario
- [x] **Filtro por planta** - filtrar eventos por planta específica

### Fase 6: Tareas (1 día) - COMPLETO

- [x] **Lista de tareas** - vista de tareas pendientes
- [x] **Tipos:** siembra, riego, cosecha, fertilizar, personalizado
- [x] **Completar** - marcar tarea como hecha
- [x] **Filtrar** - por tipo o estado

---

## Estructura de Datos

```typescript
interface GardenArea {
  id: string;
  name: string;
  createdAt: number;
}

interface Plot {
  id: string;
  areaId: string;
  name: string;
  width: number;
  height: number;
  plants: PlacedPlant[];
}

interface PlacedPlant {
  id: string;
  plantId: string;
  x: number;
  y: number;
}

interface Plant {
  id: string;
  name: string;
  color: string;
  icon: string;
}

interface Task {
  id: string;
  title: string;
  date: string; // ISO date
  type: 'sowing' | 'watering' | 'harvest' | 'fertilizing' | 'custom';
  completed: boolean;
}
```

---

## Estimación Total

**~5-6 días**

---

## Decisiones de Implementación (vs SPEC.md)

- **Storage**: localStorage (no SQLite) - Temporal para MVP sin backend
- **Editor**: Click-to-place (no drag-and-drop completo)
- **Features fuera del MVP**: Notificaciones, Companion Planting, Crop Rotation, Weather API, Permapeople API

---

## Issues Identificados Post-MVP

Hallazgos del análisis del stack que deben resolverse antes de continuar con features nuevas:

### Crítico
- **XSS en el Diario**: `Journal.svelte` usa `{@html parseMarkdown()}` sin sanitizar el contenido del usuario. Cualquier `<script>` en una nota se ejecuta. Requiere `marked` + `DOMPurify`.

### Alto
- **localStorage → SQLite**: El store actual serializa todo el estado en un único JSON. No escala con volumen de datos, no tiene integridad referencial y no es lo que exige la SPEC. Migrar a `tauri-plugin-sql`.
- **Backend Rust vacío**: `src-tauri/src/lib.rs` solo tiene el `greet` de plantilla. Las features de la SPEC (notificaciones, APIs externas, DB) requieren Tauri commands reales.

### Medio
- **Sin librería de fechas**: Los cálculos de calendario, schedules de siembra y rotación plurianual necesitan `date-fns` o `dayjs`. Hacerlo con `new Date()` nativo es frágil.
- **Router custom limitado**: El hash router actual no soporta rutas anidadas con tipado. Evaluar `svelte-routing` o migrar a SvelteKit para la versión completa.

### Referencia
Ver `ARCHITECTURE.md` para el detalle completo, el schema SQLite target y los Tauri plugins requeridos.

---

## Entregable

- App de escritorio funcional (Windows/macOS/Linux)
- Crear/editar/eliminar áreas de jardín
- Crear/editar/eliminar parcelas
- **Resize de parcelas** - ajustar tamaño después de crear
- **Duplicar parcelas** - clonar parcelas existentes
- Editor visual de plantas (click-to-place)
- Calendario con eventos
- **Filtro por planta** en calendario
- Tareas con estados
- Todo persistido en localStorage
- Sin backend, sin APIs externas
