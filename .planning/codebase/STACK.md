---
title: Technology Stack
focus: tech
date: 2026-03-26
version: 1.0
---

# Technology Stack

**Analysis Date:** 2026-03-26

## Languages

**Primary:**
- Rust (edition 2021) - Core application logic and Dioxus components

## Runtime

**Environment:**
- Dioxus Web (wasm) - Browser-based UI
- Dioxus Desktop - Native desktop application

**Package Manager:**
- Cargo (Rust)

## Frameworks

**Core:**
- Dioxus 0.7.3 - UI framework with built-in router
  - Features: `router`
  - Entry point: `src/main.rs` → `App` component

**Database:**
- rusqlite 0.30 - SQLite bindings with bundled SQLite
  - Used in: `src/storage/sqlite.rs`

**Serialization:**
- serde 1.0 - Serialization framework
- serde_json 1.0 - JSON support

**Utilities:**
- uuid 1.0 - ID generation (v4, js compatibility)
- chrono 0.4 - Date/time handling with serde
- directories 5.0 - Platform-specific data directories

**Web-specific:**
- gloo-storage 0.3 - Browser localStorage (optional, web only)
- pulldown-cmark 0.12 - Markdown parsing
- ammonia 4.0 - HTML sanitization

## Configuration Files

**`Cargo.toml`:**
- Package: plantarium v0.1.0
- Edition: 2021
- Features:
  - `default` = `["desktop"]`
  - `web` = `["dioxus/web", "gloo-storage"]`
  - `desktop` = `["dioxus/desktop"]`

**`Dioxus.toml`:**
- App name: plantarium
- Output: dist/
- Asset dir: assets/
- Bundle ID: com.plantarium

## Build Profile

**Release optimizations:**
- opt-level: z (size optimization)
- lto: true (Link Time Optimization)
- codegen-units: 1
- panic: abort
- incremental: false

---

*Stack analysis: 2026-03-26*