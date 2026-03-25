# Quick Reference: Implementation Prompts 3-10

## Estado Actual
- **PROMPT 1:** ✅ Cargo.toml configurado
- **PROMPT 2:** ✅ Storage module structure
- **PROMPT 3-10:** ⏳ Listos para ejecutar

---

## Cómo Usar Este Documento

1. **Leer PROMPTS.md** - Documentación completa de cada prompt
2. **Ejecutar secuencialmente** - PROMPT 3 → 4 → ... → 10
3. **Verificar después de cada prompt** - `cargo check`
4. **Commit después de cada prompt** - `git commit -m "PROMPT X: description"`

---

## Quick Commands

```bash
# Verificar compilación
cargo check

# Ejecutar app (desktop)
cargo run

# Ejecutar app (web)
cargo run --features web

# Inspeccionar BD
sqlite3 ~/.plantarium/data.db

# Reset BD (para testing)
rm ~/.plantarium/data.db
```

---

## PROMPT 3: SQL Schema (Optional)

**Skip?** → Sí, el schema ya está embebido en PROMPT 2  
**Do it?** → Solo si deseas mejorar mantenibilidad

**Archivos:** `src/storage/migrations.sql` (crear)

---

## PROMPT 4: Init & Helpers

**Tiempo estimado:** 30 min  
**Complejidad:** Fácil

**Funciones a implementar:**
- `get_db_path()` ✅ (Ya existe)
- `open_connection()` ✅ (Ya existe)
- `db_exists()` (Nuevo)
- `reset_db()` (Debug only)

**Archivo:** `src/storage/sqlite.rs`

---

## PROMPT 5: CRUD - Load

**Tiempo estimado:** 1.5 horas  
**Complejidad:** Media

**Funciones a implementar:**
- `load_all()` (Main)
- `load_gardens()`
- `load_beds()`
- `load_plants()`
- `load_placed_plants()`
- `load_tasks()`
- `load_journal_entries()`
- `load_calendar_events()`
- `load_plot_actions()`
- `load_bed_orders()`

**Key points:**
- Usar transacciones (`tx.prepare()`, `query_map()`)
- Mapear `ResultSet` → Rust structs
- Agrupar placed_plants dentro de beds

**Archivo:** `src/storage/sqlite.rs`

---

## PROMPT 6: CRUD - Save

**Tiempo estimado:** 1 hora  
**Complejidad:** Media

**Funciones a implementar:**
- `save_all()` (Main)
- `save_gardens()`
- `save_beds()`
- `save_plants()`
- `save_placed_plants()`
- `save_tasks()`
- `save_journal_entries()`
- `save_calendar_events()`
- `save_plot_actions()`
- `save_bed_orders()`

**Key points:**
- Usar `INSERT OR REPLACE` (UPSERT)
- Serializar enums a JSON (TaskType, PlotActionType)
- Convertir booleanos a i32 (0/1)
- Limpiar bed_orders antes de insertar

**Archivo:** `src/storage/sqlite.rs`

---

## PROMPT 7: Demo Data

**Tiempo estimado:** 45 min  
**Complejidad:** Fácil

**Funciones a implementar:**
- `init_demo_data()` (Auto-exec si BD vacía)
- `init_default_plants()` (Siempre)

**Demo data:**
- 1 garden: "Mi Huerto Demo"
- 3 beds: "Bancal 1", "Bancal 2", "Bancal 3"
- 5 placed plants: Tomate, Lechuga, Zanahoria, Pimiento, Cebolla
- 5 tasks: Riego, Fertilizar, Cosecha, Siembra, Plagas
- 2 journal entries: Notas de huerto

**Archivo:** `src/storage/sqlite.rs`

---

## PROMPT 8: Main.rs Refactor

**Tiempo estimado:** 30 min  
**Complejidad:** Fácil

**Cambios:**
1. Inicializar `SqliteStorage::new()` en `main()`
2. Llamar `storage.init()`
3. Cargar datos con `storage.load_all()`
4. Populate GlobalSignals
5. Remover `app_state::load_from_storage()` de `App()`

**Archivos:** 
- `src/main.rs` (Modificar)
- `src/app_state/state.rs` (Hacer no-op)

---

## PROMPT 9: save_to_storage() Integration

**Tiempo estimado:** 20 min  
**Complejidad:** Muy fácil

**Cambios:**
- Implementar parte `#[cfg(not(target_arch = "wasm32"))]` con SQLite

**Archivo:** `src/app_state/state.rs`

---

## PROMPT 10: Manual Testing

**Tiempo estimado:** 1 hora  
**Complejidad:** Bajo (sin código)

**Pruebas:**
1. ✅ Iniciar app (primera vez)
2. ✅ Crear garden
3. ✅ Crear bed
4. ✅ Colocar planta
5. ✅ Crear tarea
6. ✅ Crear journal entry
7. ✅ Soft delete
8. ✅ Editar entidad
9. ✅ Harvest planta
10. ✅ Integridad BD

Ver detalles en **PROMPTS.md → PROMPT 10**

---

## Common Issues & Solutions

### Error: "Could not determine project directories"
- Problema: `ProjectDirs::from()` retorna None
- Solución: Verificar sistema operativo, usar fallback path

### Error: "rusqlite::Error: database is locked"
- Problema: Múltiples conexiones simultáneas
- Solución: OK para desktop (single-threaded), revisar si hay transacciones no commiteadas

### Error: "FOREIGN KEY constraint failed"
- Problema: Insertar bed con garden_id que no existe
- Solución: Verificar que gardens se inserten antes que beds

### Error: "thread 'main' panicked at 'Failed to init storage'"
- Problema: `SqliteStorage::new()` falló
- Solución: Verificar permisos en `~/.plantarium/`, disk space

### Data no persiste after close/reopen
- Problema: `save_to_storage()` no llamado
- Solución: Verificar que todas las CRUD functions llamen `save_to_storage()`
- O verificar que `#[cfg(not(target_arch = "wasm32"))]` está correctamente en `save_to_storage()`

---

## Debugging with sqlite3

```bash
# Abrir BD
sqlite3 ~/.plantarium/data.db

# Ver tablas
.tables

# Ver schema
.schema gardens

# Ver datos
SELECT * FROM gardens;
SELECT * FROM beds WHERE garden_id='...';

# Contar registros
SELECT COUNT(*) FROM gardens;
SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL;

# Verificar integridad
PRAGMA integrity_check;
PRAGMA foreign_keys=ON;
FOREIGN KEY constraint test...

# Exportar schema
.schema > schema.sql

# Backup
.backup backup.db

# Exit
.quit
```

---

## Git Workflow

```bash
# Después de cada PROMPT
git add -A
git commit -m "PROMPT X: description"

# Ejemplo:
git commit -m "PROMPT 4: implement init helpers and db_exists()"
git commit -m "PROMPT 5: implement load_all() with transaction"
git commit -m "PROMPT 6: implement save_all() with UPSERT"

# Al terminar PROMPT 10
git commit -m "PROMPT 10: verification complete - SQLite persistence working"
```

---

## Performance Notes

- **Sync I/O:** OK para desktop (<10ms queries)
- **Transactions:** Usadas para atomicidad
- **Foreign Keys:** Habilitadas en BD
- **Indexing:** PLK en `id` es suficiente para MVP
- **Batch operations:** `save_all()` usa transacción (múltiples INSERTs en 1 tx)

---

## Next Steps After PROMPT 10

Si todo funciona:
1. **Celebrar** 🎉
2. **Commit final** a main branch
3. **Plan futuro:**
   - PROMPT 11+: Web localStorage backend
   - PROMPT 12+: Permapeople API integration
   - PROMPT 13+: OpenWeather API integration

---

## Resources

- **Dioxus Docs:** https://dioxuslabs.com/learn/0.7/
- **rusqlite Docs:** https://docs.rs/rusqlite/
- **Rust Book:** https://doc.rust-lang.org/book/
- **Project Spec:** `SPEC.md`
- **Storage Architecture:** `STORAGE.md`
- **Full Prompts:** `PROMPTS.md`

---

**Good luck with implementation! 🌱**
