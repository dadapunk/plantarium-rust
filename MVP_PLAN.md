# Plan MVP: Plantarium

**Alcance:** Layout visual + Calendario + Tareas  
**Stack:** Svelte + Tauri 2.0 + localStorage  
**Sin backend, sin APIs externas**

---

## Fases de Implementación

### Fase 1: Proyecto Base (0.5 días)

- [ ] Inicializar proyecto Tauri 2.0 + Svelte
- [ ] Configurar estructura de componentes

### Fase 2: Datos Locales (0.5 días)

- [ ] Definir interfaces TypeScript:
  - `GardenArea` (id, name)
  - `Plot` (id, areaId, name, width, height, plants[])
  - `Plant` (id, name, x, y, color)
  - `Task` (id, title, date, type, completed)
  - `CalendarEvent` (id, title, date, type)
- [ ] Crear store Svelte con localStorage
  - `load()` - cargar desde localStorage
  - `save()` - guardar en localStorage

### Fase 3: Gestión de Parcelas (1 día)

- [ ] **UI: Lista de áreas** - crear/editar/eliminar áreas de jardín
- [ ] **UI: Lista de parcelas** - crear/editar/eliminar parcelas por área
- [ ] **Rutas:**
  - `/` - Dashboard con lista de áreas
  - `/area/:id` - Detalle de área con parcelas

### Fase 4: Editor Visual (2 días)

- [ ] **Canvas interactivo** - área de diseño con coordenadas
- [ ] **Plantas predefinidas** - biblioteca de plantas (mock data)
- [ ] **Drag-and-drop** - arrastrar plantas desde biblioteca a parcelas
- [ ] **Posicionar** - mover plantas dentro de la parcela
- [ ] **Persistir** - guardar posiciones en localStorage

### Fase 5: Calendario (1 día)

- [ ] **Vista mensual** - grid de días del mes
- [ ] **Navegación** - mes anterior/siguiente
- [ ] **Eventos** - mostrar eventos en cada día
- [ ] **Añadir evento** - crear evento desde día del calendario

### Fase 6: Tareas (1 día)

- [ ] **Lista de tareas** - vista de tareas pendientes
- [ ] **Tipos:** siembra, riego, cosecha, fertilizar, personalizado
- [ ] **Completar** - marcar tarea como hecha
- [ ] **Filtrar** - por tipo o estado

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

## Entregable

- App de escritorio funcional (Windows/macOS/Linux)
- Crear/editar/eliminar áreas de jardín
- Crear/editar/eliminar parcelas
- Editor visual drag-and-drop de plantas
- Calendario con eventos
- Tareas con estados
- Todo persistido en localStorage
- Sin backend, sin APIs externas
