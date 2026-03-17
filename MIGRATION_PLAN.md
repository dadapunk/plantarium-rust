# Plan de Migración: Svelte → Dioxus

## Visión General

- **Stack actual**: Svelte 5 + TypeScript + Vite + Tauri 2
- **Stack objetivo**: Dioxus 0.7.3 (desktop + web)
- **Razón**: Eliminar Tauri, usar Dioxus para desktop app nativa
- **Versión Dioxus**: 0.7.3 (última estable - verificado)

---

## Estado Actual del Proyecto

### Entidades y Tipos de Datos

| Entidad | Descripción |
|---------|-------------|
| `Garden` | Jardines con configuración de espaciado (minBedDistance, bedSpacing) |
| `Bed` | Bancales con posición libre (x,y), redimensionables |
| `Plant` | Biblioteca de 10 plantas predefinidas |
| `PlacedPlant` | Plantas colocadas en beds, con estado de harvest |
| `PlotAction` | Historial de acciones (planted, sowed, harvested, removed) |
| `Task` | 5 tipos (sowing, watering, harvest, fertilizing, custom) |
| `CalendarEvent` | Eventos vinculados a plantas |
| `JournalEntry` | Notas con Markdown + sintaxis especial |

### Plantas Predefinidas

```typescript
Tomate, Lechuga, Zanahoria, Pimiento, Cebolla, Ajo, Papa, Judía, Maíz, Calabaza
```

---

## Funcionalidades Implementadas

### Módulos y Features

| Módulo | Funcionalidades |
|--------|----------------|
| **Dashboard** | Vista compacta/expandida, crear jardines, preview cards |
| **GardenDetail** | CRUD beds, duplicar, redimensionar, settings, histórico modal |
| **BedEditor** | Colocar plantas (click), harvest, remove, fecha configurable |
| **FreeCanvas** | Drag & drop beds, snap-to-grid, detección colisiones, auto-snap |
| **ExpandedGardenSection** | Stats (plantas, beds, %), escala automática |
| **Journal** | Markdown + @parcela / @planta, DOMPurify sanitization |
| **Calendar** | Vista mensual, eventos, journal indicators, filtros por planta |
| **Tasks** | CRUD, filtros estado/tipo, 5 categorías |
| **Store** | LocalStorage v2, migraciones, soft-delete, bed orders persistidos |

---

## Plan de Ejecución

### Fase 1: Configuración del Proyecto

**Tiempo estimado**: 1 hora

- [ ] 1.1 Instalar Dioxus CLI
  ```bash
  cargo install dioxus-cli
  ```

- [ ] 1.2 Crear proyecto base
  ```bash
  dx new plantarium
  # Seleccionar template deseado cuando pregunte
  ```

- [ ] 1.3 Configurar `Cargo.toml`
  ```toml
  [package]
  name = "plantarium"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  dioxus = { version = "0.7.3", features = ["router"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  uuid = { version = "1", features = ["v4", "js"] }
  chrono = { version = "0.4", features = ["serde"] }
  gloo-storage = "0.3"      # LocalStorage
  pulldown-cmark = "0.12"   # Markdown
  ammonia = "4"             # HTML Sanitization

  [features]
  default = ["web"]
  web = ["dioxus/web"]
  desktop = ["dioxus/desktop"]
  ```

- [ ] 1.4 Configurar `Dioxus.toml`
  ```toml
  [application]
  name = "plantarium"
  out_dir = "dist"
  asset_dir = "assets"

  [web.app]
  title = "Plantarium"

  [web.watcher]
  reload_html = true
  watch_path = ["src", "assets"]

  [web.resource]
  style = ["/assets/main.css"]
  script = []

  [bundle]
  identifier = "com.plantarium"
  publisher = "Plantarium"
  icon = ["assets/icon.png"]
  ```

- [ ] 1.5 Configurar perfil de release en `Cargo.toml`
  ```toml
  [profile.release]
  opt-level = "z"
  debug = false
  lto = true
  codegen-units = 1
  panic = "abort"
  incremental = false
  ```

- [ ] 1.6 Eliminar `frontend/` (Svelte) y `src-tauri/` completamente

---

### Fase 2: Core/Shared

**Tiempo estimado**: 2 horas

- [ ] 2.1 **Tipos Rust**: Definir structs con serde
  ```rust
  // src/store/types.rs
  use serde::{Deserialize, Serialize};

  #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
  pub struct Garden {
      pub id: String,
      pub name: String,
      pub min_bed_distance: Option<i32>,
      pub bed_spacing: Option<i32>,
      pub created_at: i64,
      pub updated_at: i64,
      pub deleted_at: Option<i64>,
  }

  #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
  pub struct Bed {
      pub id: String,
      pub garden_id: String,
      pub name: String,
      pub width: i32,
      pub height: i32,
      pub x: Option<i32>,
      pub y: Option<i32>,
      pub plants: Vec<PlacedPlant>,
      pub created_at: i64,
      pub updated_at: i64,
      pub deleted_at: Option<i64>,
  }

  // ... resto de tipos
  ```

- [ ] 2.2 **Store con Signals globales**
  ```rust
  // src/store/state.rs
  use dioxus::prelude::*;
  use gloo_storage::{LocalStorage, Storage};
  use super::types::*;

  const STORAGE_KEY: &str = "plantarium_data_v2";

  // Estado global con Signal::global
  pub static GARDENS: GlobalSignal<Vec<Garden>> = Signal::global(Vec::new);
  pub static BEDS: GlobalSignal<Vec<Bed>> = Signal::global(Vec::new);
  pub static PLANTS: GlobalSignal<Vec<Plant>> = Signal::global(default_plants);
  pub static TASKS: GlobalSignal<Vec<Task>> = Signal::global(Vec::new);
  pub static EVENTS: GlobalSignal<Vec<CalendarEvent>> = Signal::global(Vec::new);
  pub static JOURNAL: GlobalSignal<Vec<JournalEntry>> = Signal::global(Vec::new);
  pub static PLOT_ACTIONS: GlobalSignal<Vec<PlotAction>> = Signal::global(Vec::new);

  fn default_plants() -> Vec<Plant> {
      vec![
          Plant { id: "1".into(), name: "Tomate".into(), color: "#e74c3c".into(), icon: "🍅".into(), ..Default::default() },
          // ... resto de plantas
      ]
  }

  pub fn load_from_storage() {
      if let Ok(data) = LocalStorage::get(STORAGE_KEY) {
          let stored: AppState = data;
          *GARDENS.write() = stored.gardens;
          *BEDS.write() = stored.beds;
          // ... etc
      }
  }

  pub fn save_to_storage() {
      let state = AppState {
          gardens: GARDENS.read().clone(),
          beds: BEDS.read().clone(),
          // ... etc
      };
      let _ = LocalStorage::set(STORAGE_KEY, &state);
  }
  ```

- [ ] 2.3 **Hook personalizado para persistencia** (opcional, alternativa al anterior)
  ```rust
  // src/hooks/use_persistent.rs
  use dioxus::prelude::*;
  use gloo_storage::{LocalStorage, Storage};
  use serde::{de::DeserializeOwned, Serialize};

  pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static + Clone>(
      key: impl ToString,
      init: impl FnOnce() -> T,
  ) -> UsePersistent<T> {
      let state = use_signal(move || {
          let key = key.to_string();
          let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(init);
          StorageEntry { key, value }
      });
      UsePersistent { inner: state }
  }

  pub struct UsePersistent<T: 'static> {
      inner: Signal<StorageEntry<T>>,
  }

  struct StorageEntry<T> {
      key: String,
      value: T,
  }

  impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
      pub fn get(&self) -> T {
          self.inner.read().value.clone()
      }

      pub fn set(&mut self, value: T) {
          let mut inner = self.inner.write();
          let _ = LocalStorage::set(inner.key.as_str(), &value);
          inner.value = value;
      }
  }
  ```

---

### Fase 3: Router y Componentes Base

**Tiempo estimado**: 3 horas

- [ ] 3.1 **Router**: Definir enum Routable
  ```rust
  // src/router.rs
  use dioxus::prelude::*;
  use crate::pages::*;

  #[derive(Clone, Debug, PartialEq, Routable)]
  pub enum Route {
      #[route("/")]
      Dashboard {},

      #[route("/garden/:id")]
      GardenDetail { id: String },

      #[route("/bed/:id")]
      BedEditor { id: String },

      #[route("/calendar")]
      Calendar {},

      #[route("/journal")]
      Journal {},

      #[route("/tasks")]
      Tasks {},
  }
  ```
  
  > **Nota**: Cada variante del enum debe tener un componente con el mismo nombre.
  > Por ejemplo, `Dashboard` renderiza el componente `fn Dashboard() -> Element`.

- [ ] 3.2 **main.rs y App component con Router**
  ```rust
  // src/main.rs
  use dioxus::prelude::*;

  mod router;
  mod store;
  mod components;
  mod pages;

  use router::Route;

  fn main() {
      // Cargar datos de localStorage al iniciar
      store::load_from_storage();
      
      // Lanzar app (detecta automáticamente web/desktop)
      dioxus::launch(App);
  }

  #[component]
  fn App() -> Element {
      rsx! {
          document::Stylesheet { href: asset!("/assets/main.css") }
          Router::<Route> {}
      }
  }
  ```

- [ ] 3.3 **Navbar**: Componente navegación
  ```rust
  // src/components/navbar.rs
  use dioxus::prelude::*;
  use crate::router::Route;

  #[component]
  pub fn Navbar() -> Element {
      let mut lang = use_signal(|| "ES".to_string());

      rsx! {
          nav { class: "navbar",
              Link { to: Route::Dashboard, "🌱 Plantarium" }
              div { class: "nav-links",
                  Link { to: Route::Dashboard, "Jardines" }
                  Link { to: Route::Calendar, "Calendario" }
                  Link { to: Route::Journal, "Diario" }
                  Link { to: Route::Tasks, "Tareas" }
                  button {
                      class: "lang-toggle",
                      onclick: move |_| {
                          let new_lang = if lang() == "ES" { "EN" } else { "ES" };
                          lang.set(new_lang.to_string());
                      },
                      "{lang}"
                  }
              }
          }
      }
  }
  ```

- [ ] 3.4 **PlantLibrary**: Grid de plantas clickables
  ```rust
  // src/components/plant_library.rsx
  use dioxus::prelude::*;
  use crate::store::{PLANTS};

  #[component]
  pub fn PlantLibrary(on_select: EventHandler<String>) -> Element {
      let selected = use_signal(|| None::<String>);

      rsx! {
          div { class: "plant-library",
              h3 { "Biblioteca de Plantas" }
              div { class: "plants-grid",
                  for plant in PLANTS.read().iter() {
                      button {
                          class: if selected() == Some(plant.id.clone()) { "plant-btn selected" } else { "plant-btn" },
                          onclick: move |_| {
                              selected.set(Some(plant.id.clone()));
                              on_select.call(plant.id.clone());
                          },
                          span { class: "icon", "{plant.icon}" }
                          span { class: "name", "{plant.name}" }
                      }
                  }
              }
          }
      }
  }
  ```

- [ ] 3.5 **Modal**: Componente genérico
  ```rust
  // src/components/modal.rsx
  use dioxus::prelude::*;

  #[component]
  pub fn Modal(title: String, on_close: EventHandler<()>, children: Element) -> Element {
      rsx! {
          div {
              class: "modal-overlay",
              onclick: move |_| on_close.call(()),
              div {
                  class: "modal",
                  onclick: |evt| evt.stop_propagation(),
                  div { class: "modal-header",
                      h2 { "{title}" }
                      button {
                          class: "close-btn",
                          onclick: move |_| on_close.call(()),
                          "×"
                      }
                  }
                  div { class: "modal-body",
                      {children}
                  }
              }
          }
      }
  }
  ```

---

### Fase 4: Pages

**Tiempo estimado**: 4-6 horas

- [ ] 4.1 **Dashboard.rsx**
  ```rust
  // src/pages/dashboard.rs
  use dioxus::prelude::*;
  use crate::router::Route;
  use crate::store::{GARDENS};
  use crate::components::{Navbar, ExpandedGardenSection, GardenPreview};

  #[component]
  pub fn Dashboard() -> Element {
      let mut new_garden_name = use_signal(|| String::new());
      let mut view_mode = use_signal(|| "expanded".to_string());

      let add_garden = move |_| {
          let name = new_garden_name();
          if !name.trim().is_empty() {
              // Crear nuevo jardín y guardarlo
              new_garden_name.set(String::new());
          }
      };

      rsx! {
          Navbar {}
          div { class: "dashboard",
              header { class: "dashboard-header",
                  h1 { "Jardines" }
                  div { class: "view-toggle",
                      button {
                          class: if view_mode() == "compact" { "active" } else { "" },
                          onclick: move |_| view_mode.set("compact".into()),
                          "Vista Compacta"
                      }
                      button {
                          class: if view_mode() == "expanded" { "active" } else { "" },
                          onclick: move |_| view_mode.set("expanded".into()),
                          "Vista Expandida"
                      }
                  }
              }

              // Formulario añadir jardín
              div { class: "add-form",
                  input {
                      r#type: "text",
                      placeholder: "Nombre del jardín...",
                      value: "{new_garden_name}",
                      oninput: move |evt| new_garden_name.set(evt.value()),
                      onkeydown: move |evt| {
                          if evt.key() == Key::Enter {
                              add_garden(evt);
                          }
                      }
                  }
                  button { onclick: add_garden, "+ Añadir Jardín" }
              }

              // Contenido condicional
              if GARDENS.read().is_empty() {
                  div { class: "empty",
                      p { "No hay jardines todavía" }
                      p { "¡Crea tu primer jardín!" }
                  }
              } else if view_mode() == "expanded" {
                  for garden in GARDENS.read().iter() {
                      ExpandedGardenSection { garden_id: garden.id.clone() }
                  }
              } else {
                  div { class: "gardens-grid",
                      for garden in GARDENS.read().iter() {
                          GardenPreview { garden: garden.clone() }
                      }
                  }
              }
          }
      }
  }
  ```

- [ ] 4.2 **GardenDetail.rsx** - CRUD beds, duplicar, redimensionar, settings, histórico

- [ ] 4.3 **BedEditor.rsx** - Canvas clickable, place/harvest/remove plants

- [ ] 4.4 **Tasks.rsx** - CRUD tareas con filtros

- [ ] 4.5 **Calendar.rsx** - Vista mensual, eventos

- [ ] 4.6 **Journal.rsx** - Markdown + parsing especial

---

### Fase 5: Canvas & Drag-Drop

**Tiempo estimado**: 2-3 horas

- [ ] 5.1 **FreeCanvasBedLayout.rsx** - Implementar con eventos mouse
  ```rust
  // Dioxus soporta eventos de drag nativos: ondrag, ondrop, ondragover
  // O implementar con mouse events: onmousedown, onmousemove, onmouseup

  #[component]
  pub fn FreeCanvasBedLayout(beds: Vec<Bed>, scale: f64) -> Element {
      let mut dragging_bed = use_signal(|| None::<String>);
      let mut drag_offset = use_signal(|| (0.0, 0.0));
      let mut ghost_position = use_signal(|| None::<(f64, f64)>);
      let mut has_collision = use_signal(|| false);

      let on_mouse_down = move |evt: MouseEvent, bed_id: String| {
          dragging_bed.set(Some(bed_id));
          // calcular offset inicial
      };

      let on_mouse_move = move |evt: MouseEvent| {
          if let Some(_) = dragging_bed() {
              // actualizar ghost_position
              // verificar colisiones
          }
      };

      let on_mouse_up = move |_| {
          if let (Some(bed_id), Some(pos)) = (dragging_bed(), ghost_position()) {
              // guardar posición final
          }
          dragging_bed.set(None);
          ghost_position.set(None);
      };

      rsx! {
          div {
              class: "free-canvas",
              onmousemove: on_mouse_move,
              onmouseup: on_mouse_up,
              // ... renderizar beds
          }
      }
  }
  ```

- [ ] 5.2 **BedLarge.rsx** - Preview a escala con plantas

---

### Fase 6: CSS y Estilos

**Tiempo estimado**: 1-2 horas

- [ ] 6.1 **Opción A: TailwindCSS** (recomendado)
  ```bash
  # Instalar Tailwind CLI
  npm install -D @tailwindcss/cli

  # Crear input.css
  echo '@tailwind utilities;' > input.css

  # Ejecutar en watch mode
  npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
  ```

  ```rust
  // En app.rs
  fn App() -> Element {
      rsx! {
          document::Stylesheet { href: asset!("/assets/tailwind.css") }
          // ...
      }
  }
  ```

- [ ] 6.2 **Opción B: CSS puro** (más simple)
  ```css
  /* assets/main.css */
  .navbar {
      background: #2d5a27;
      color: white;
      padding: 1rem 2rem;
      display: flex;
      justify-content: space-between;
  }
  /* ... resto de estilos migrados de Svelte */
  ```

---

### Fase 7: Persistencia y Testing

**Tiempo estimado**: 1 hora

- [ ] 7.1 Verificar que datos de localStorage existentes cargan correctamente
- [ ] 7.2 Probar migración de datos (usuarios con datos previos)
- [ ] 7.3 Testing funcional de todas las features

---

### Fase 8: Build y Deploy

**Tiempo estimado**: 1 hora

- [ ] 8.1 Desarrollo local
  ```bash
  # Servir con hot-reload (detecta plataforma automáticamente)
  dx serve
  
  # Específico para web
  dx serve --web
  
  # Específico para desktop
  dx serve --desktop
  ```

- [ ] 8.2 Build para producción
  ```bash
  # Build optimizado desktop
  dx build --desktop --release
  
  # Bundle para web (genera archivos estáticos)
  dx bundle --web --release
  ```

- [ ] 8.3 Verificar outputs
  - Desktop: binario en `dist/` o `target/release/`
  - Web: archivos estáticos en `dist/` (HTML, WASM, JS, CSS)

---

## Estructura de Archivos Objetivo

```
plantarium/
├── Cargo.toml
├── Dioxus.toml
├── input.css              # Para Tailwind (si se usa)
├── src/
│   ├── main.rs            # Entry point
│   ├── app.rs             # App component + router setup
│   ├── router.rs          # Route enum
│   ├── components/
│   │   ├── mod.rs
│   │   ├── navbar.rsx
│   │   ├── plant_library.rsx
│   │   ├── bed_preview.rsx
│   │   ├── bed_large.rsx
│   │   ├── free_canvas.rsx
│   │   ├── modal.rsx
│   │   └── garden_preview.rsx
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── dashboard.rsx
│   │   ├── garden_detail.rsx
│   │   ├── bed_editor.rsx
│   │   ├── calendar.rsx
│   │   ├── journal.rsx
│   │   └── tasks.rsx
│   ├── store/
│   │   ├── mod.rs
│   │   ├── state.rs       # GlobalSignals
│   │   └── types.rs       # Structs Rust
│   └── hooks/
│       ├── mod.rs
│       └── use_persistent.rs
├── assets/
│   ├── main.css           # Estilos globales
│   ├── tailwind.css       # Generado por Tailwind
│   └── icon.png
└── dist/                  # Output del build
```

---

## Mapeo Svelte → Dioxus

| Svelte | Dioxus 0.7 |
|--------|------------|
| `$state` | `use_signal(\|\| valor_inicial)` |
| `$derived` | Computed en el rsx! o `use_memo` |
| `$effect` | `use_effect(move \|\| { ... })` |
| `$props()` | Argumentos de función con `#[component]` |
| `bind:value` | `value: "{signal}", oninput: move \|evt\| signal.set(evt.value())` |
| `onclick` | `onclick: move \|_\| { ... }` |
| `class:active={cond}` | `class: if cond { "active" } else { "" }` |
| `{#each items as item}` | `for item in items.iter() { ... }` |
| `{#if cond}` | `if cond { ... }` |
| `{:else}` | `} else { ... }` |
| Stores (writable) | `Signal::global(\|\| valor)` o `use_context_provider` |
| Stores (lectura) | `signal()` o `signal.read()` |
| Stores (escritura) | `signal.set(value)` o `*signal.write() = value` |
| `on:click` | `onclick` |
| `on:keydown` | `onkeydown` (usa `evt.key() == Key::Enter`) |

### Ejemplos de Conversión

**Svelte:**
```svelte
<script>
  let count = $state(0);
  let doubled = $derived(count * 2);
  
  $effect(() => {
    console.log(count);
  });
</script>

<button onclick={() => count++}>{count} (doubled: {doubled})</button>
```

**Dioxus:**
```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    let doubled = count() * 2;  // Se recalcula automáticamente
    
    use_effect(move || {
        log!("{}", count());
    });

    rsx! {
        button {
            onclick: move |_| count += 1,
            "{count} (doubled: {doubled})"
        }
    }
}
```

---

## Eventos Disponibles en Dioxus

| Categoría | Eventos |
|-----------|---------|
| **Mouse** | `onclick`, `ondblclick`, `onmousedown`, `onmouseup`, `onmousemove`, `onmouseover`, `onmouseout`, `onmouseenter`, `onmouseleave` |
| **Drag & Drop** | `ondrag`, `ondragend`, `ondragenter`, `ondragleave`, `ondragover`, `ondragstart`, `ondrop` |
| **Keyboard** | `onkeydown`, `onkeyup`, `onkeypress` |
| **Form** | `oninput`, `onchange`, `onsubmit`, `onfocus`, `onblur` |
| **UI** | `onscroll`, `onresize`, `onload` |

---

## Paleta de Colores (mantener)

| Color | Uso |
|-------|-----|
| `#2d5a27` | Verde principal (navbar, headers) |
| `#4a7c44` | Verde secundario (botones, acentos) |
| `#e8f5e9` | Verde claro (fondos, canvas) |
| `#c0392b` | Rojo (eliminar, danger) |
| `#f39c12` | Naranja (harvest, warnings) |
| `#666` | Gris (texto secundario) |
| `#f5f5f5` | Gris claro (fondos) |

---

## Dependencias Necesarias

```toml
[dependencies]
dioxus = { version = "0.7.3", features = ["router"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "js"] }
chrono = { version = "0.4", features = ["serde"] }
gloo-storage = "0.3"      # LocalStorage para web/desktop
pulldown-cmark = "0.12"   # Markdown parsing
ammonia = "4"             # HTML sanitization

[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
```

---

## Notas Técnicas

### State Management

```rust
// Signal local (componente)
let mut count = use_signal(|| 0);
count()              // leer valor (clona)
count.read()         // leer referencia (borrow)
count.set(5);        // escribir nuevo valor
*count.write() += 1; // modificar in-place
count += 1;          // shorthand para modificar

// Signal global (toda la app)
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);
COUNT()              // leer
*COUNT.write() += 1; // modificar

// Context provider (compartir entre componentes hijos)
// En el padre:
let state = use_context_provider(|| Signal::new(MyState::default()));

// En cualquier hijo:
let state = use_context::<Signal<MyState>>();
```

### Navegación Programática

```rust
use dioxus::prelude::*;

#[component]
fn MyComponent() -> Element {
    let nav = navigator();

    rsx! {
        button {
            onclick: move |_| {
                // Navegar a otra ruta
                nav.push(Route::GardenDetail { id: "123".into() });
            },
            "Ir a jardín"
        }
        button {
            onclick: move |_| nav.go_back(),
            "Volver"
        }
    }
}
```

### LocalStorage

```rust
use gloo_storage::{LocalStorage, Storage};

// Leer
let data: Result<MyData, _> = LocalStorage::get("key");

// Escribir
LocalStorage::set("key", &my_data)?;
```

### Markdown + Sanitization

```rust
use pulldown_cmark::{html, Parser};
use ammonia::clean;

fn parse_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    clean(&html_output)  // Sanitizar HTML
}
```

### use_effect (Side Effects)

```rust
use dioxus::prelude::*;

#[component]
fn MyComponent() -> Element {
    let mut count = use_signal(|| 0);

    // Se ejecuta cuando count cambia
    use_effect(move || {
        // count es una dependencia automática
        let current = count();
        log!("Count changed to: {current}");
        
        // Para leer sin suscribirse:
        // let val = *some_signal.peek();
    });

    rsx! {
        button { onclick: move |_| count += 1, "Increment" }
    }
}
```

### Renderizado Condicional

```rust
rsx! {
    // if/else
    if logged_in() {
        Dashboard {}
    } else {
        LoginScreen {}
    }
    
    // match
    match status() {
        Status::Loading => rsx! { "Cargando..." },
        Status::Error(e) => rsx! { "Error: {e}" },
        Status::Ready => rsx! { Content {} },
    }
    
    // Loops
    for item in items.iter() {
        div { key: "{item.id}", "{item.name}" }
    }
}
```

---

## Estimación Total

| Fase | Tiempo |
|------|--------|
| Configuración | 1h |
| Core/Shared | 2h |
| Router y Componentes Base | 3h |
| Pages | 4-6h |
| Canvas & Drag-Drop | 2-3h |
| CSS y Estilos | 1-2h |
| Persistencia y Testing | 1h |
| Build y Deploy | 1h |
| **Total** | **15-19 horas (2-3 días)** |

---

## Plan de Rollback

Si la migración falla o hay problemas críticos:

1. **Backup**: Antes de empezar, hacer backup de `frontend/` y `src-tauri/`
2. **Git branch**: Trabajar en branch `feature/dioxus-migration`
3. **Rollback**: Si falla, volver a `main` y restaurar Svelte

---

## Orden de Migración Recomendado

1. Crear proyecto Dioxus en paralelo (no borrar Svelte hasta confirmar que funciona)
2. Implementar tipos + store + persistencia
3. Implementar Dashboard básico
4. Implementar GardenDetail
5. Implementar BedEditor con canvas
6. Implementar resto de páginas
7. Implementar drag-drop
8. Probar con datos reales (localStorage)
9. Eliminar código Svelte solo cuando todo funcione

---

## Referencias

- [Dioxus Docs 0.7](https://dioxuslabs.com/learn/0.7/)
- [Dioxus Router](https://dioxuslabs.com/learn/0.7/essentials/router/)
- [Dioxus Signals](https://dioxuslabs.com/learn/0.7/migration/to_05/state)
- [Dioxus Events](https://dioxuslabs.com/learn/0.7/essentials/basics/event_handlers)
- [Dioxus TailwindCSS](https://dioxuslabs.com/learn/0.7/guides/utilities/tailwind)
- [gloo-storage](https://docs.rs/gloo-storage/)
- [pulldown-cmark](https://docs.rs/pulldown-cmark/)
- [ammonia](https://docs.rs/ammonia/)
