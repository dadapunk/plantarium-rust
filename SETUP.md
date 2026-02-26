# Setup Guide — Plantarium

Guía para levantar el entorno de desarrollo del proyecto tal como está hoy.

## Estado actual del proyecto

El MVP está completo. La app corre como una aplicación Tauri 2.0 + Svelte 5
con datos persistidos en `localStorage`. No hay backend separado activo.

## Prerrequisitos

### Herramientas base

| Herramienta | Versión mínima | Verificar |
|-------------|----------------|-----------|
| Rust | 1.75+ | `rustc --version` |
| Node.js | 18+ | `node --version` |
| npm | 9+ | `npm --version` |

### Instalar Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Instalar dependencias del sistema operativo

#### macOS
```bash
xcode-select --install
```

#### Linux (Debian/Ubuntu)
```bash
sudo apt update && sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  build-essential \
  libssl-dev \
  pkg-config
```

#### Windows
- Instalar [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Instalar [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)

---

## Instalación

```bash
# 1. Clonar el repo
git clone <url-del-repo>
cd plantarium-rust

# 2. Instalar dependencias del frontend
cd frontend
npm install
```

---

## Correr en desarrollo

```bash
cd frontend
npm run tauri dev
```

Esto levanta:
- Vite dev server en `http://localhost:1420`
- La ventana Tauri apuntando a ese servidor con hot-reload

---

## Build de producción

```bash
cd frontend
npm run tauri build
```

Los binarios se generan en `frontend/src-tauri/target/release/bundle/`.

### Mobile (requiere setup adicional)

```bash
# Android — requiere Android SDK + NDK
npm run tauri android dev

# iOS — solo macOS, requiere Xcode
npm run tauri ios dev
```

---

## Estructura del proyecto

```
plantarium-rust/
├── SPEC.md             # Especificación de producto
├── ARCHITECTURE.md     # Arquitectura técnica y decisiones
├── ROADMAP.md          # Prioridades de implementación
│
└── frontend/
    ├── package.json
    ├── vite.config.ts
    ├── src/                    # Svelte 5 frontend
    │   ├── App.svelte          # Punto de entrada + router
    │   ├── lib/
    │   │   ├── router.ts       # Navegación hash-based
    │   │   └── store.ts        # Estado global + localStorage
    │   ├── pages/              # Vistas principales
    │   └── types/index.ts      # Interfaces TypeScript
    └── src-tauri/              # Runtime Rust (Tauri)
        ├── Cargo.toml
        ├── tauri.conf.json
        └── src/
            ├── main.rs
            └── lib.rs          # Comandos Tauri (ampliar aquí)
```

---

## Configuración pendiente

### tauri.conf.json

El archivo actual tiene valores de plantilla que deben actualizarse:

```json
{
  "productName": "frontend",        // ← cambiar a "Plantarium"
  "identifier": "com.dada.frontend" // ← cambiar a "com.plantarium.app"
}
```

### Variables de entorno

Cuando se integren las APIs externas, crear `frontend/.env`:

```env
# Solo necesarios para integración con APIs externas (post-MVP)
PERMAPEOPLE_API_KEY=tu_key
OPENWEATHER_API_KEY=tu_key
```

Las keys no deben exponerse al frontend — se acceden desde comandos Tauri en Rust.

---

## Comandos útiles

```bash
# Frontend
npm run dev          # Solo Vite (sin Tauri, para iterar UI rápido)
npm run build        # Build del frontend
npm run tauri dev    # App Tauri completa con hot-reload
npm run tauri build  # Build de producción

# Rust
cargo fmt            # Formatear código Rust
cargo clippy         # Linter Rust
cargo test           # Tests Rust
```

---

## Troubleshooting

**La ventana Tauri no abre**
- Verificar que el puerto 1420 esté libre: `lsof -i :1420`
- Verificar que las dependencias del sistema estén instaladas

**Error de compilación Rust**
- Actualizar toolchain: `rustup update`
- Limpiar cache: `cargo clean` desde `frontend/src-tauri/`

**Hot-reload no funciona**
- Detener y relanzar `npm run tauri dev`
- Los cambios en `src-tauri/` requieren recompilación completa
