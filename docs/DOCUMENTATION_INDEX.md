# 📚 Plantarium Documentation Index

Guía completa de la documentación del proyecto. Navegue según su necesidad.

---

## 🎯 Empezar Aquí

### Desarrolladores nuevos
1. **[README.md](./README.md)** - Visión general del proyecto
2. **[CONTRIBUTING.md](./CONTRIBUTING.md)** - Cómo configurar el entorno
3. **[SPEC.md](./SPEC.md)** - Qué hace la app (features)

### Implementadores (PROMPTs 3-10)
1. **[IMPLEMENTATION_GUIDE.md](./IMPLEMENTATION_GUIDE.md)** - Guía rápida (5 min)
2. **[PROMPTS.md](./PROMPTS.md)** - Detalles completos de cada prompt (1 hora)
3. **[STORAGE.md](./STORAGE.md)** - Arquitectura SQLite (referencia)

---

## 📖 Documentación Completa

| Archivo | Líneas | Tamaño | Propósito | Audiencia |
|---------|--------|--------|----------|-----------|
| **README.md** | 217 | 7.4K | Descripción proyecto, setup, roadmap | Todos |
| **SPEC.md** | 151 | 10K | Especificación funcional y técnica | Product, Design |
| **STORAGE.md** | 403 | 12K | Arquitectura persistencia SQLite | Backend developers |
| **PROMPTS.md** | 1353 | 39K | Detalles PROMPT 3-10 con código | Implementadores |
| **IMPLEMENTATION_GUIDE.md** | 301 | 6.4K | Quick reference PROMPTs | Implementadores |
| **CHANGELOG.md** | 105 | 3.1K | Historial cambios v0.1-0.2 | Todos |
| **CONTRIBUTING.md** | 55 | 1.2K | Guía contribución | Contributors |
| **DOCUMENTATION_INDEX.md** | - | - | Este archivo | Navegación |

**Total:** 2,585 líneas de documentación

---

## 🔍 Por Caso de Uso

### "Quiero entender qué es Plantarium"
→ **README.md** (2 min)

### "Quiero saber qué características tiene"
→ **SPEC.md** (10 min)

### "Quiero setear el ambiente de desarrollo"
→ **CONTRIBUTING.md** (5 min)

### "Quiero implementar PROMPT 3-10"
→ **IMPLEMENTATION_GUIDE.md** (5 min) + **PROMPTS.md** (1 hora)

### "Quiero entender la arquitectura de persistencia"
→ **STORAGE.md** (15 min)

### "Quiero ver el historial de cambios"
→ **CHANGELOG.md** (3 min)

### "Quiero debuggear problemas de BD"
→ **STORAGE.md → Troubleshooting** (5 min)

---

## 🏗️ Estructura del Proyecto

```
plantarium-rust/
├── 📄 README.md                 # Start here
├── 📄 SPEC.md                   # Features & requirements
├── 📄 STORAGE.md                # SQLite architecture
├── 📄 PROMPTS.md                # Implementation details
├── 📄 IMPLEMENTATION_GUIDE.md    # Quick reference
├── 📄 CHANGELOG.md              # Version history
├── 📄 CONTRIBUTING.md           # Development setup
├── 📄 DOCUMENTATION_INDEX.md     # Este archivo
│
├── Cargo.toml                   # Dependencies
├── src/
│   ├── main.rs                  # Entry point
│   ├── router.rs                # Routes
│   ├── app_state/               # GlobalSignals + CRUD
│   │   ├── mod.rs
│   │   └── state.rs
│   ├── storage/                 # SQLite layer
│   │   ├── mod.rs
│   │   ├── db.rs
│   │   └── sqlite.rs
│   ├── pages/                   # 6 Pages
│   │   ├── dashboard.rs
│   │   ├── garden_detail.rs
│   │   ├── bed_editor.rs
│   │   ├── calendar.rs
│   │   ├── tasks.rs
│   │   └── journal.rs
│   └── components/              # Reusable UI
│       └── mod.rs (Navbar)
│
├── assets/
│   └── main.css
│
└── target/                      # Build output
```

---

## 📊 Estado del Proyecto

| Fase | Estado | Documento |
|------|--------|-----------|
| **PROMPT 1** | ✅ Cargo.toml | README.md § Roadmap |
| **PROMPT 2** | ✅ Storage module | STORAGE.md § Implementation |
| **PROMPT 3-10** | ⏳ Pendiente | PROMPTS.md |

---

## 🚀 Quick Start

### Clone & Setup
```bash
git clone https://github.com/tu-usuario/plantarium-rust
cd plantarium-rust
cargo build
```

### Run Desktop App
```bash
cargo run
```

### Run Web App (Future)
```bash
cargo run --features web
```

### Inspect Database
```bash
sqlite3 ~/.plantarium/data.db
```

---

## 💡 Key Concepts

### Storage Architecture
- **Desktop:** SQLite sync (no async)
- **Web:** localStorage (future)
- **Cloud:** PostgreSQL (future)
- **Pattern:** StorageProvider trait (abstraction layer)

### Data Model
- **Gardens:** Contenedores (huertos)
- **Beds:** Bancales dentro de jardines
- **Plants:** Biblioteca de 10 plantas predefinidas
- **PlacedPlants:** Plantas colocadas en bancales
- **Tasks:** Actividades a realizar
- **JournalEntries:** Notas del huerto
- **CalendarEvents:** Eventos con fechas
- **PlotActions:** Historial de acciones

### Soft Delete Pattern
- `deleted_at = NULL` → Activo
- `deleted_at = timestamp` → Eliminado (recuperable)
- Permite sincronización futura

---

## 🔗 External References

- **Dioxus 0.7:** https://dioxuslabs.com/learn/0.7/
- **rusqlite:** https://docs.rs/rusqlite/
- **Rust Book:** https://doc.rust-lang.org/book/
- **Permapeople API:** https://permapeople.org/
- **OpenWeather API:** https://openweathermap.org/api

---

## 📝 Convenciones

### Commits
```
PROMPT X: description
PROMPT 4: implement init helpers and db_exists()
PROMPT 5: implement load_all() with transaction
```

### Code Style
- `cargo fmt` - Format code
- `cargo clippy` - Lint
- `cargo test` - Test

### Database
- Paths: `~/.plantarium/data.db`
- Foreign keys enabled
- Transactions for atomicity
- Soft deletes preferred

---

## ❓ FAQ

**P: ¿Debo implementar PROMPT 3?**  
R: No, schema ya está en PROMPT 2. Salta a PROMPT 4.

**P: ¿Debo usar async para storage?**  
R: No, sync rusqlite es suficiente para desktop MVP.

**P: ¿Cómo debuggeo la BD?**  
R: Usa `sqlite3 ~/.plantarium/data.db` - Ver STORAGE.md § Debugging

**P: ¿Qué pasa si la BD se corrompe?**  
R: `rm ~/.plantarium/data.db` - Se recreará con demo data.

**P: ¿Puedo usar async en el futuro?**  
R: Sí, la abstracción StorageProvider lo permite. Solo crea PostgresStorage.

---

## 📞 Support

- **Bugs:** Abrir GitHub issue
- **Questions:** Discussiones en GitHub
- **Feature Requests:** GitHub issues

---

**Última actualización:** 2026-03-18  
**Version:** 0.2.0 (Dioxus + SQLite)  
**Maintainer:** Plantarium Team
