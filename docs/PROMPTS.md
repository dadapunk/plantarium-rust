# Implementation Prompts 3-10: SQLite Persistence

Este documento contiene los prompts detallados para implementar persistencia SQLite en Plantarium.

**Estado:** PROMPT 2 completado ✅  
**Siguiente:** PROMPT 3

---

## PROMPT 3: SQL Migrations Schema (Optional Enhancement)

**Objetivo:** Crear archivo separado para SQL schema (mejora de organización, opcional)

**Contexto:**
- PROMPT 2 ya tiene el schema embebido en `SqliteStorage::init()`
- Este prompt es para mejorar mantenibilidad (schema en archivo separado)
- **Alternativa:** Saltar directamente a PROMPT 4

**Archivos a crear:**
```
src/storage/
└── migrations.sql  # SQL schema
```

**Instrucciones:**

1. Crear `src/storage/migrations.sql` con todas las tablas:
   ```sql
   -- Plantarium SQLite Schema v1
   -- Created for desktop SQLite storage (sync, no async)

   CREATE TABLE IF NOT EXISTS gardens (
       id TEXT PRIMARY KEY,
       name TEXT NOT NULL,
       min_bed_distance INTEGER,
       bed_spacing INTEGER,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER
   );

   CREATE TABLE IF NOT EXISTS beds (
       id TEXT PRIMARY KEY,
       garden_id TEXT NOT NULL,
       name TEXT NOT NULL,
       width INTEGER NOT NULL,
       height INTEGER NOT NULL,
       x INTEGER,
       y INTEGER,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER,
       FOREIGN KEY (garden_id) REFERENCES gardens(id)
   );

   CREATE TABLE IF NOT EXISTS plants (
       id TEXT PRIMARY KEY,
       name TEXT NOT NULL,
       color TEXT NOT NULL,
       icon TEXT NOT NULL,
       family TEXT,
       species TEXT,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER
   );

   CREATE TABLE IF NOT EXISTS placed_plants (
       id TEXT PRIMARY KEY,
       bed_id TEXT NOT NULL,
       plant_id TEXT NOT NULL,
       x REAL NOT NULL,
       y REAL NOT NULL,
       harvested_at INTEGER,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER,
       FOREIGN KEY (bed_id) REFERENCES beds(id),
       FOREIGN KEY (plant_id) REFERENCES plants(id)
   );

   CREATE TABLE IF NOT EXISTS tasks (
       id TEXT PRIMARY KEY,
       title TEXT NOT NULL,
       date TEXT NOT NULL,
       type TEXT NOT NULL,
       completed INTEGER NOT NULL,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER
   );

   CREATE TABLE IF NOT EXISTS journal_entries (
       id TEXT PRIMARY KEY,
       date TEXT NOT NULL,
       content TEXT NOT NULL,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER
   );

   CREATE TABLE IF NOT EXISTS calendar_events (
       id TEXT PRIMARY KEY,
       title TEXT NOT NULL,
       date TEXT NOT NULL,
       type TEXT NOT NULL,
       plant_id TEXT,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER
   );

   CREATE TABLE IF NOT EXISTS plot_actions (
       id TEXT PRIMARY KEY,
       bed_id TEXT NOT NULL,
       plant_id TEXT NOT NULL,
       action TEXT NOT NULL,
       quantity INTEGER NOT NULL,
       date TEXT NOT NULL,
       created_at INTEGER NOT NULL,
       updated_at INTEGER NOT NULL,
       deleted_at INTEGER,
       FOREIGN KEY (bed_id) REFERENCES beds(id),
       FOREIGN KEY (plant_id) REFERENCES plants(id)
   );

   CREATE TABLE IF NOT EXISTS bed_orders (
       garden_id TEXT NOT NULL,
       bed_id TEXT NOT NULL,
       position INTEGER NOT NULL,
       PRIMARY KEY (garden_id, bed_id)
   );
   ```

2. Verificar compilación: `cargo check`

**Verificación:**
- ✅ Archivo `src/storage/migrations.sql` existe
- ✅ SQL syntax válido
- ✅ Proyecto compila

**Nota:** Este paso es opcional. PROMPT 4 procede con o sin este archivo.

---

## PROMPT 4: Init & Helper Functions

**Objetivo:** Implementar funciones helper para gestionar DB

**Contexto:**
- `SqliteStorage` ya tiene `new()`, `get_db_path()`, `open_connection()`
- Este prompt completa y optimiza estas funciones
- Preparar para PROMPT 5-6 (queries de carga/guardado)

**Archivos a modificar:**
- `src/storage/sqlite.rs`

**Instrucciones:**

1. Verificar que `get_db_path()` crea directorio si no existe:
   ```rust
   fn get_db_path() -> Result<PathBuf, StorageError> {
       let project_dirs = ProjectDirs::from("com", "plantarium", "plantarium")
           .ok_or_else(|| StorageError::Custom("Could not determine project directories".to_string()))?;
       
       let data_dir = project_dirs.data_dir();
       std::fs::create_dir_all(data_dir)?;  // ← Crea si no existe
       
       Ok(data_dir.join("data.db"))
   }
   ```

2. Verificar `open_connection()`:
   ```rust
   fn open_connection(&self) -> Result<Connection, StorageError> {
       Connection::open(&self.db_path).map_err(Into::into)
   }
   ```

3. Implementar función helper para verificar si DB existe:
   ```rust
   fn db_exists(&self) -> bool {
       self.db_path.exists()
   }
   ```

4. Implementar función helper para reset de DB (debug):
   ```rust
   #[cfg(test)]
   pub fn reset_db(&self) -> Result<(), StorageError> {
       std::fs::remove_file(&self.db_path).ok();
       self.init()
   }
   ```

**Verificación:**
- ✅ `cargo check` compila sin errores
- ✅ `db_path` se crea automáticamente en `~/.plantarium/`
- ✅ `open_connection()` abre BD sin errores

---

## PROMPT 5: CRUD - Load from SQLite

**Objetivo:** Implementar `load_all()` - cargar datos desde SQLite

**Contexto:**
- PROMPT 5 es la parte crítica de lectura
- Debe mapear ResultSet → AppState (todas las 7 entidades)
- Usar transactions para integridad

**Archivos a modificar:**
- `src/storage/sqlite.rs`

**Instrucciones:**

Reemplazar el `load_all()` con TODO por implementación completa:

```rust
fn load_all(&self) -> Result<AppState, StorageError> {
    let conn = self.open_connection()?;
    
    // Iniciar transacción (read-only)
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)?;
    
    // Cargar gardens
    let gardens = Self::load_gardens(&tx)?;
    
    // Cargar beds
    let beds = Self::load_beds(&tx)?;
    
    // Cargar plants
    let plants = Self::load_plants(&tx)?;
    
    // Cargar placed_plants (dentro de beds)
    let beds = Self::load_placed_plants(&tx, beds)?;
    
    // Cargar tasks
    let tasks = Self::load_tasks(&tx)?;
    
    // Cargar journal entries
    let journal = Self::load_journal_entries(&tx)?;
    
    // Cargar calendar events
    let events = Self::load_calendar_events(&tx)?;
    
    // Cargar plot actions
    let plot_actions = Self::load_plot_actions(&tx)?;
    
    // Cargar bed orders
    let bed_orders = Self::load_bed_orders(&tx)?;
    
    tx.commit()?;
    
    Ok(AppState {
        gardens,
        beds,
        plants,
        tasks,
        events,
        journal,
        plot_actions,
        bed_orders,
    })
}
```

Implementar funciones helper de carga:

```rust
impl SqliteStorage {
    fn load_gardens(tx: &rusqlite::Transaction) -> Result<Vec<Garden>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, name, min_bed_distance, bed_spacing, created_at, updated_at, deleted_at 
             FROM gardens"
        )?;
        
        let gardens = stmt.query_map([], |row| {
            Ok(Garden {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                    deleted_at: row.get(6)?,
                },
                name: row.get(1)?,
                min_bed_distance: row.get(2)?,
                bed_spacing: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(gardens)
    }
    
    fn load_beds(tx: &rusqlite::Transaction) -> Result<Vec<Bed>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, garden_id, name, width, height, x, y, created_at, updated_at, deleted_at 
             FROM beds"
        )?;
        
        let beds = stmt.query_map([], |row| {
            Ok(Bed {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                    deleted_at: row.get(9)?,
                },
                garden_id: row.get(1)?,
                name: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
                x: row.get(5)?,
                y: row.get(6)?,
                plants: Vec::new(),  // Se populate en load_placed_plants
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(beds)
    }
    
    fn load_plants(tx: &rusqlite::Transaction) -> Result<Vec<Plant>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, name, color, icon, family, species, created_at, updated_at, deleted_at 
             FROM plants"
        )?;
        
        let plants = stmt.query_map([], |row| {
            Ok(Plant {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    deleted_at: row.get(8)?,
                },
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                family: row.get(4)?,
                species: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(plants)
    }
    
    fn load_placed_plants(tx: &rusqlite::Transaction, mut beds: Vec<Bed>) -> Result<Vec<Bed>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, bed_id, plant_id, x, y, harvested_at, created_at, updated_at, deleted_at 
             FROM placed_plants"
        )?;
        
        let placed_plants = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,  // bed_id
                PlacedPlant {
                    base: SyncableEntity {
                        id: row.get(0)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        deleted_at: row.get(8)?,
                    },
                    plant_id: row.get(2)?,
                    x: row.get(3)?,
                    y: row.get(4)?,
                    harvested_at: row.get(5)?,
                },
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        // Agrupar placed_plants por bed_id
        for bed in beds.iter_mut() {
            for (bed_id, placed_plant) in &placed_plants {
                if bed_id == &bed.base.id {
                    bed.plants.push(placed_plant.clone());
                }
            }
        }
        
        Ok(beds)
    }
    
    fn load_tasks(tx: &rusqlite::Transaction) -> Result<Vec<Task>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, title, date, type, completed, created_at, updated_at, deleted_at 
             FROM tasks"
        )?;
        
        let tasks = stmt.query_map([], |row| {
            let task_type_str: String = row.get(3)?;
            Ok(Task {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    deleted_at: row.get(7)?,
                },
                title: row.get(1)?,
                date: row.get(2)?,
                r#type: serde_json::from_str(&task_type_str)
                    .unwrap_or(TaskType::Custom),
                completed: row.get::<_, i32>(4)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(tasks)
    }
    
    fn load_journal_entries(tx: &rusqlite::Transaction) -> Result<Vec<JournalEntry>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, date, content, created_at, updated_at, deleted_at 
             FROM journal_entries"
        )?;
        
        let entries = stmt.query_map([], |row| {
            Ok(JournalEntry {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    deleted_at: row.get(5)?,
                },
                date: row.get(1)?,
                content: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(entries)
    }
    
    fn load_calendar_events(tx: &rusqlite::Transaction) -> Result<Vec<CalendarEvent>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, title, date, type, plant_id, created_at, updated_at, deleted_at 
             FROM calendar_events"
        )?;
        
        let events = stmt.query_map([], |row| {
            let event_type_str: String = row.get(3)?;
            Ok(CalendarEvent {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    deleted_at: row.get(7)?,
                },
                title: row.get(1)?,
                date: row.get(2)?,
                r#type: serde_json::from_str(&event_type_str)
                    .unwrap_or(TaskType::Custom),
                plant_id: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(events)
    }
    
    fn load_plot_actions(tx: &rusqlite::Transaction) -> Result<Vec<PlotAction>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT id, bed_id, plant_id, action, quantity, date, created_at, updated_at, deleted_at 
             FROM plot_actions"
        )?;
        
        let actions = stmt.query_map([], |row| {
            let action_str: String = row.get(3)?;
            Ok(PlotAction {
                base: SyncableEntity {
                    id: row.get(0)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    deleted_at: row.get(8)?,
                },
                bed_id: row.get(1)?,
                plant_id: row.get(2)?,
                action: serde_json::from_str(&action_str)
                    .unwrap_or(PlotActionType::Planted),
                quantity: row.get(4)?,
                date: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(actions)
    }
    
    fn load_bed_orders(tx: &rusqlite::Transaction) -> Result<HashMap<String, Vec<String>>, StorageError> {
        let mut stmt = tx.prepare(
            "SELECT garden_id, bed_id FROM bed_orders ORDER BY position ASC"
        )?;
        
        let mut orders: HashMap<String, Vec<String>> = HashMap::new();
        
        stmt.query_map([], |row| {
            let garden_id: String = row.get(0)?;
            let bed_id: String = row.get(1)?;
            
            orders.entry(garden_id).or_insert_with(Vec::new).push(bed_id);
            
            Ok(())
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(orders)
    }
}
```

**Verificación:**
- ✅ `cargo check` compila sin errores
- ✅ `load_all()` retorna `Result<AppState, StorageError>`
- ✅ Todas las 9 entidades se cargan correctamente
- ✅ Transacción de lectura funciona

---

## PROMPT 6: CRUD - Save to SQLite

**Objetivo:** Implementar `save_all()` - guardar todos los datos a SQLite

**Contexto:**
- PROMPT 6 es la parte crítica de escritura
- Debe insertar/actualizar/marcar_como_eliminado todas las entidades
- Usar transactions para atomicidad

**Archivos a modificar:**
- `src/storage/sqlite.rs`

**Instrucciones:**

Reemplazar el `save_all()` con TODO por implementación completa:

```rust
fn save_all(&self, state: &AppState) -> Result<(), StorageError> {
    let conn = self.open_connection()?;
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)?;
    
    // Limpiar tablas (excepto plants que es predefinida)
    // O mejor: usar UPSERT (INSERT ... ON CONFLICT)
    
    // Guardar gardens
    Self::save_gardens(&tx, &state.gardens)?;
    
    // Guardar beds
    Self::save_beds(&tx, &state.beds)?;
    
    // Guardar plants
    Self::save_plants(&tx, &state.plants)?;
    
    // Guardar placed_plants (desde beds.plants)
    Self::save_placed_plants(&tx, &state.beds)?;
    
    // Guardar tasks
    Self::save_tasks(&tx, &state.tasks)?;
    
    // Guardar journal entries
    Self::save_journal_entries(&tx, &state.journal)?;
    
    // Guardar calendar events
    Self::save_calendar_events(&tx, &state.events)?;
    
    // Guardar plot actions
    Self::save_plot_actions(&tx, &state.plot_actions)?;
    
    // Guardar bed orders
    Self::save_bed_orders(&tx, &state.bed_orders)?;
    
    tx.commit()?;
    
    Ok(())
}
```

Implementar funciones helper de guardado:

```rust
impl SqliteStorage {
    fn save_gardens(tx: &rusqlite::Transaction, gardens: &[Garden]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO gardens 
             (id, name, min_bed_distance, bed_spacing, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for garden in gardens {
            stmt.execute(rusqlite::params![
                &garden.base.id,
                &garden.name,
                garden.min_bed_distance,
                garden.bed_spacing,
                garden.base.created_at,
                garden.base.updated_at,
                garden.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_beds(tx: &rusqlite::Transaction, beds: &[Bed]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO beds 
             (id, garden_id, name, width, height, x, y, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for bed in beds {
            stmt.execute(rusqlite::params![
                &bed.base.id,
                &bed.garden_id,
                &bed.name,
                bed.width,
                bed.height,
                bed.x,
                bed.y,
                bed.base.created_at,
                bed.base.updated_at,
                bed.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_plants(tx: &rusqlite::Transaction, plants: &[Plant]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO plants 
             (id, name, color, icon, family, species, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for plant in plants {
            stmt.execute(rusqlite::params![
                &plant.base.id,
                &plant.name,
                &plant.color,
                &plant.icon,
                &plant.family,
                &plant.species,
                plant.base.created_at,
                plant.base.updated_at,
                plant.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_placed_plants(tx: &rusqlite::Transaction, beds: &[Bed]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO placed_plants 
             (id, bed_id, plant_id, x, y, harvested_at, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for bed in beds {
            for placed_plant in &bed.plants {
                stmt.execute(rusqlite::params![
                    &placed_plant.base.id,
                    &bed.base.id,
                    &placed_plant.plant_id,
                    placed_plant.x,
                    placed_plant.y,
                    placed_plant.harvested_at,
                    placed_plant.base.created_at,
                    placed_plant.base.updated_at,
                    placed_plant.base.deleted_at,
                ])?;
            }
        }
        
        Ok(())
    }
    
    fn save_tasks(tx: &rusqlite::Transaction, tasks: &[Task]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO tasks 
             (id, title, date, type, completed, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for task in tasks {
            let task_type_str = serde_json::to_string(&task.r#type).unwrap_or_default();
            stmt.execute(rusqlite::params![
                &task.base.id,
                &task.title,
                &task.date,
                task_type_str,
                if task.completed { 1 } else { 0 },
                task.base.created_at,
                task.base.updated_at,
                task.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_journal_entries(tx: &rusqlite::Transaction, entries: &[JournalEntry]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO journal_entries 
             (id, date, content, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?)"
        )?;
        
        for entry in entries {
            stmt.execute(rusqlite::params![
                &entry.base.id,
                &entry.date,
                &entry.content,
                entry.base.created_at,
                entry.base.updated_at,
                entry.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_calendar_events(tx: &rusqlite::Transaction, events: &[CalendarEvent]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO calendar_events 
             (id, title, date, type, plant_id, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for event in events {
            let event_type_str = serde_json::to_string(&event.r#type).unwrap_or_default();
            stmt.execute(rusqlite::params![
                &event.base.id,
                &event.title,
                &event.date,
                event_type_str,
                &event.plant_id,
                event.base.created_at,
                event.base.updated_at,
                event.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_plot_actions(tx: &rusqlite::Transaction, actions: &[PlotAction]) -> Result<(), StorageError> {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO plot_actions 
             (id, bed_id, plant_id, action, quantity, date, created_at, updated_at, deleted_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        for action in actions {
            let action_str = serde_json::to_string(&action.action).unwrap_or_default();
            stmt.execute(rusqlite::params![
                &action.base.id,
                &action.bed_id,
                &action.plant_id,
                action_str,
                action.quantity,
                &action.date,
                action.base.created_at,
                action.base.updated_at,
                action.base.deleted_at,
            ])?;
        }
        
        Ok(())
    }
    
    fn save_bed_orders(tx: &rusqlite::Transaction, orders: &HashMap<String, Vec<String>>) -> Result<(), StorageError> {
        // Primero limpiar tabla
        tx.execute("DELETE FROM bed_orders", [])?;
        
        let mut stmt = tx.prepare(
            "INSERT INTO bed_orders (garden_id, bed_id, position) VALUES (?, ?, ?)"
        )?;
        
        for (garden_id, bed_ids) in orders {
            for (position, bed_id) in bed_ids.iter().enumerate() {
                stmt.execute(rusqlite::params![
                    garden_id,
                    bed_id,
                    position as i32,
                ])?;
            }
        }
        
        Ok(())
    }
}
```

**Verificación:**
- ✅ `cargo check` compila sin errores
- ✅ `save_all()` retorna `Result<(), StorageError>`
- ✅ INSERT OR REPLACE funciona para todas las tablas
- ✅ Transactions se commitean correctamente

---

## PROMPT 7: Demo Data Initialization

**Objetivo:** Crear datos de demostración si BD está vacía

**Contexto:**
- Al abrir app por primera vez, crear demo data
- 1 garden, 3 beds, 5 placed plants, 5 tasks, 2 journal entries
- Solo si DB está vacía

**Archivos a modificar:**
- `src/storage/sqlite.rs`

**Instrucciones:**

Añadir función helper en `SqliteStorage`:

```rust
pub fn init_demo_data(&self) -> Result<(), StorageError> {
    let conn = self.open_connection()?;
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)?;
    
    // Verificar si hay datos
    let garden_count: i32 = tx.query_row(
        "SELECT COUNT(*) FROM gardens",
        [],
        |row| row.get(0)
    )?;
    
    if garden_count > 0 {
        return Ok(()); // Ya hay datos, no crear demo
    }
    
    let now = chrono::Utc::now().timestamp_millis();
    
    // 1. Crear 1 garden
    let garden_id = uuid::Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO gardens (id, name, min_bed_distance, bed_spacing, created_at, updated_at, deleted_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            &garden_id,
            "Mi Huerto Demo",
            55,
            60,
            now,
            now,
            None::<i64>,
        ],
    )?;
    
    // 2. Crear 3 beds
    let mut bed_ids = Vec::new();
    for i in 0..3 {
        let bed_id = uuid::Uuid::new_v4().to_string();
        let y_pos = 50 + (i as i32 * 120);
        tx.execute(
            "INSERT INTO beds (id, garden_id, name, width, height, x, y, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                &bed_id,
                &garden_id,
                format!("Bancal {}", i + 1),
                100,
                100,
                50,
                y_pos,
                now,
                now,
                None::<i64>,
            ],
        )?;
        bed_ids.push(bed_id);
    }
    
    // 3. Crear 5 placed plants (distribuir en beds)
    let plant_ids = vec!["1", "2", "3", "4", "5"]; // Tomate, Lechuga, Zanahoria, Pimiento, Cebolla
    for (i, plant_id) in plant_ids.iter().enumerate() {
        let bed_idx = i % bed_ids.len();
        let placed_plant_id = uuid::Uuid::new_v4().to_string();
        
        tx.execute(
            "INSERT INTO placed_plants (id, bed_id, plant_id, x, y, harvested_at, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                &placed_plant_id,
                &bed_ids[bed_idx],
                plant_id,
                20.0 + (i as f64 * 15.0),
                20.0,
                None::<i64>,
                now,
                now,
                None::<i64>,
            ],
        )?;
    }
    
    // 4. Crear 5 tasks
    let task_data = vec![
        ("Riego semanal", "2026-03-25", "watering"),
        ("Fertilizar", "2026-03-28", "fertilizing"),
        ("Cosecha tomates", "2026-04-15", "harvest"),
        ("Semilla de lechugas", "2026-03-20", "sowing"),
        ("Revisar plagas", "2026-03-22", "custom"),
    ];
    
    for (title, date, task_type) in task_data {
        let task_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO tasks (id, title, date, type, completed, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                &task_id,
                title,
                date,
                task_type,
                0,
                now,
                now,
                None::<i64>,
            ],
        )?;
    }
    
    // 5. Crear 2 journal entries
    let journal_data = vec![
        ("2026-03-18", "Plantado tomates y lechugas en el huerto. El clima es perfecto esta semana."),
        ("2026-03-17", "Preparado el terreno. Añadido compost. Listo para plantar."),
    ];
    
    for (date, content) in journal_data {
        let entry_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO journal_entries (id, date, content, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                &entry_id,
                date,
                content,
                now,
                now,
                None::<i64>,
            ],
        )?;
    }
    
    tx.commit()?;
    Ok(())
}
```

Añadir en `init()` para autoejecutar al crear BD:

```rust
fn init(&self) -> Result<(), StorageError> {
    let conn = self.open_connection()?;
    
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS gardens (...)
        ... [resto del schema]
        "#,
    )?;
    
    // Inicializar plantas predefinidas (siempre)
    Self::init_default_plants(&conn)?;
    
    drop(conn);
    
    // Inicializar demo data si BD está vacía
    self.init_demo_data()?;
    
    Ok(())
}

fn init_default_plants(conn: &rusqlite::Connection) -> Result<(), StorageError> {
    let plants = vec![
        ("1", "Tomate", "#e74c3c", "🍅"),
        ("2", "Lechuga", "#27ae60", "🥬"),
        ("3", "Zanahoria", "#e67e22", "🥕"),
        ("4", "Pimiento", "#c0392b", "🫑"),
        ("5", "Cebolla", "#8e44ad", "🧅"),
        ("6", "Ajo", "#f1c40f", "🧄"),
        ("7", "Papa", "#d35400", "🥔"),
        ("8", "Judía", "#16a085", "🫛"),
        ("9", "Maíz", "#f39c12", "🌽"),
        ("10", "Calabaza", "#e67e22", "🎃"),
    ];
    
    let now = 0i64; // Plantas predefinidas tienen timestamp 0
    
    for (id, name, color, icon) in plants {
        conn.execute(
            "INSERT OR IGNORE INTO plants (id, name, color, icon, family, species, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                id, name, color, icon,
                None::<String>, None::<String>,
                now, now, None::<i64>,
            ],
        )?;
    }
    
    Ok(())
}
```

**Verificación:**
- ✅ `cargo check` compila
- ✅ BD vacía → demo data se crea automáticamente
- ✅ BD con datos → demo data NO se crea
- ✅ Plantas predefinidas (10) siempre están presentes

---

## PROMPT 8: Main.rs Refactor - Load on Startup

**Objetivo:** Refactorizar `main.rs` para cargar datos del SQLite al iniciar

**Contexto:**
- Actualmente: `load_from_storage()` es no-op en desktop
- Después PROMPT 8: Cargar desde SQLite automáticamente
- Eliminar código antiguo de localStorage desktop

**Archivos a modificar:**
- `src/main.rs`
- `src/app_state/state.rs` (función `load_from_storage()`)

**Instrucciones:**

1. Actualizar `src/main.rs`:

```rust
use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::Router;

mod app_state;
mod components;
mod pages;
mod router;
mod storage;

use storage::{SqliteStorage, StorageProvider};
use app_state::{GARDENS, BEDS, PLANTS, TASKS, EVENTS, JOURNAL, PLOT_ACTIONS};

fn main() {
    // Inicializar almacenamiento SQLite
    let storage = match SqliteStorage::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Error inicializando storage: {}", e);
            std::process::exit(1);
        }
    };
    
    // Crear tablas si no existen (y cargar demo data si es primera vez)
    if let Err(e) = storage.init() {
        eprintln!("❌ Error inicializando BD: {}", e);
        std::process::exit(1);
    }
    
    // Cargar datos desde SQLite en los GlobalSignals
    match storage.load_all() {
        Ok(state) => {
            *GARDENS.write() = state.gardens;
            *BEDS.write() = state.beds;
            *PLANTS.write() = if state.plants.is_empty() {
                app_state::state::default_plants()
            } else {
                state.plants
            };
            *TASKS.write() = state.tasks;
            *EVENTS.write() = state.events;
            *JOURNAL.write() = state.journal;
            *PLOT_ACTIONS.write() = state.plot_actions;
        }
        Err(e) => {
            eprintln!("⚠️ Error cargando datos: {}. Usando valores por defecto.", e);
            *PLANTS.write() = app_state::state::default_plants();
        }
    }
    
    // Lanzar app
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

2. Actualizar `src/app_state/state.rs` - función `load_from_storage()`:

```rust
pub fn load_from_storage() {
    // Ahora es no-op - la carga ocurre en main.rs
    // (Se mantiene por compatibilidad con código existente)
    
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(data) = LocalStorage::get::<AppState>(STORAGE_KEY) {
            *GARDENS.write() = data.gardens;
            *BEDS.write() = data.beds;
            *PLANTS.write() = if data.plants.is_empty() {
                default_plants()
            } else {
                data.plants
            };
            *TASKS.write() = data.tasks;
            *EVENTS.write() = data.events;
            *JOURNAL.write() = data.journal;
            *PLOT_ACTIONS.write() = data.plot_actions;
        } else {
            *PLANTS.write() = default_plants();
        }

        if let Ok(orders) = LocalStorage::get::<HashMap<String, Vec<String>>>(BED_ORDERS_KEY) {
            *BED_ORDERS.write() = orders;
        }
    }
}
```

**Verificación:**
- ✅ `cargo check` compila
- ✅ `cargo run` inicia sin errores
- ✅ GlobalSignals se llenan desde SQLite
- ✅ Demo data se muestra en primera ejecución

---

## PROMPT 9: Integrate save_to_storage() with SQLite Backend

**Objetivo:** Hacer que `save_to_storage()` guarde en SQLite (no no-op)

**Contexto:**
- Actualmente: `save_to_storage()` en desktop es no-op
- Después PROMPT 9: Persiste todos los cambios a SQLite
- Cada CRUD operation llama a `save_to_storage()`

**Archivos a modificar:**
- `src/app_state/state.rs` (función `save_to_storage()`)

**Instrucciones:**

Reemplazar `save_to_storage()` en `src/app_state/state.rs`:

```rust
pub fn save_to_storage() {
    #[cfg(target_arch = "wasm32")]
    {
        // Web: usar localStorage
        let state = AppState {
            gardens: GARDENS.read().clone(),
            beds: BEDS.read().clone(),
            plants: PLANTS.read().clone(),
            tasks: TASKS.read().clone(),
            events: EVENTS.read().clone(),
            journal: JOURNAL.read().clone(),
            plot_actions: PLOT_ACTIONS.read().clone(),
            bed_orders: BED_ORDERS.read().clone(),
        };
        let _ = LocalStorage::set(STORAGE_KEY, &state);
        let _ = LocalStorage::set(BED_ORDERS_KEY, &*BED_ORDERS.read());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Desktop: usar SQLite
        if let Ok(storage) = crate::storage::SqliteStorage::new() {
            let state = AppState {
                gardens: GARDENS.read().clone(),
                beds: BEDS.read().clone(),
                plants: PLANTS.read().clone(),
                tasks: TASKS.read().clone(),
                events: EVENTS.read().clone(),
                journal: JOURNAL.read().clone(),
                plot_actions: PLOT_ACTIONS.read().clone(),
                bed_orders: BED_ORDERS.read().clone(),
            };
            
            if let Err(e) = storage.save_all(&state) {
                eprintln!("⚠️ Error guardando datos: {}", e);
            }
        }
    }
}
```

**Verificación:**
- ✅ `cargo check` compila
- ✅ `cargo run` inicia correctamente
- ✅ Crear/editar/eliminar entidades persiste en SQLite
- ✅ Cerrar app → datos en BD
- ✅ Reaabrir app → datos se cargan desde BD

---

## PROMPT 10: Verification & Manual Testing

**Objetivo:** Verificar que persistencia SQLite funciona end-to-end

**Contexto:**
- Todos los PROMPTs 1-9 completados
- Verificación manual de flujos completos
- Resolución de bugs si existen

**Checklist de Pruebas:**

### Prueba 1: Iniciar app (primera vez)
```
1. Eliminar ~/.plantarium/data.db (si existe)
2. cargo run
3. ✅ Verificar: App inicia con demo data
4. ✅ Verificar: 1 garden, 3 beds, 5 plantas colocadas visibles
5. ✅ Verificar: ~/.plantarium/data.db se crea
```

### Prueba 2: Crear nuevo garden
```
1. En Dashboard, ingresar "Mi Huerto" → clic "+Añadir Jardín"
2. ✅ Verificar: Garden se añade a la lista
3. Cerrar app (Ctrl+C)
4. cargo run
5. ✅ Verificar: "Mi Huerto" aparece en Dashboard
```

### Prueba 3: Crear bed
```
1. Hacer clic en un garden → GardenDetail
2. Ingresar nombre "Cama A" → clic añadir bed
3. ✅ Verificar: Bed aparece en canvas
4. Cerrar y reaabrir app
5. ✅ Verificar: "Cama A" sigue ahí con misma posición
```

### Prueba 4: Colocar planta
```
1. En BedEditor, seleccionar planta (ej: Tomate)
2. Hacer clic en canvas para colocar
3. ✅ Verificar: Planta aparece en posición
4. Cerrar y reaabrir
5. ✅ Verificar: Planta sigue en misma posición
```

### Prueba 5: Crear tarea
```
1. Ir a Tasks → "+Nueva Tarea"
2. Ingresar "Riego" → fecha 2026-03-30 → Clic crear
3. ✅ Verificar: Tarea aparece en lista
4. Cerrar y reaabrir
5. ✅ Verificar: Tarea persiste
```

### Prueba 6: Crear journal entry
```
1. Ir a Journal → "Nueva Entrada"
2. Ingresar fecha y contenido Markdown
3. Clic guardar
4. ✅ Verificar: Entrada aparece
5. Cerrar y reaabrir
6. ✅ Verificar: Entrada sigue visible
```

### Prueba 7: Soft delete
```
1. En Tasks, eliminar una tarea
2. ✅ Verificar: Tarea desaparece de UI
3. Abrir BD con sqlite3: sqlite3 ~/.plantarium/data.db
4. SELECT * FROM tasks WHERE id='...'; 
5. ✅ Verificar: deleted_at está populated (no NULL)
```

### Prueba 8: Editar entidad
```
1. En Journal, editar una entrada
2. Cambiar contenido
3. Clic guardar
4. ✅ Verificar: updated_at se actualiza en BD
5. Cerrar y reaabrir
6. ✅ Verificar: Cambio persiste
```

### Prueba 9: Harvest planta
```
1. En BedEditor, clic derecho en planta → "Cosecha"
2. ✅ Verificar: Planta cambia estado visual
3. Cerrar y reaabrir
4. ✅ Verificar: Estado de cosecha persiste
```

### Prueba 10: Base de datos integridad
```
1. Ejecutar múltiples operaciones (crear, editar, eliminar)
2. Cerrar app
3. sqlite3 ~/.plantarium/data.db
4. PRAGMA foreign_keys=ON;
5. SELECT COUNT(*) FROM gardens;
6. ✅ Verificar: Foreign keys válidas, conteos correctos
```

**Debugging si falla algo:**

```bash
# Ver estado actual BD
sqlite3 ~/.plantarium/data.db ".tables"
sqlite3 ~/.plantarium/data.db ".schema gardens"
sqlite3 ~/.plantarium/data.db "SELECT * FROM gardens;"

# Reset BD (para empezar de 0)
rm ~/.plantarium/data.db

# Ver logs de error
cargo run 2>&1 | grep -E "Error|❌|⚠️"
```

**Verificación Final:**
- ✅ Todas 10 pruebas pasan
- ✅ Sin panics o errores críticos
- ✅ DB persiste correctamente
- ✅ Demo data aparece en primera ejecución
- ✅ Soft deletes funcionan
- ✅ Timestamps (created_at, updated_at, deleted_at) se actualizan correctamente

---

## Resumen PROMPTs 3-10

| PROMPT | Objetivo | Archivos | Estado |
|--------|----------|----------|--------|
| 3 | SQL schema (opcional) | migrations.sql | ⏳ Pendiente |
| 4 | Helpers & init | sqlite.rs | ⏳ Pendiente |
| 5 | CRUD Load | sqlite.rs | ⏳ Pendiente |
| 6 | CRUD Save | sqlite.rs | ⏳ Pendiente |
| 7 | Demo data | sqlite.rs | ⏳ Pendiente |
| 8 | Main.rs refactor | main.rs, state.rs | ⏳ Pendiente |
| 9 | save_to_storage() | state.rs | ⏳ Pendiente |
| 10 | Verificación | Manual testing | ⏳ Pendiente |

**Total tiempo estimado:** 6-8 horas de implementación + testing

---

**Próximo paso:** Ejecutar PROMPT 3 o saltar directamente a PROMPT 4 (schema ya está en init())
