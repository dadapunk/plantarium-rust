---
title: External Integrations
focus: tech
date: 2026-03-26
version: 1.0
---

# External Integrations

**Analysis Date:** 2026-03-26

## Data Storage

**SQLite (Local):**
- Provider: rusqlite 0.30 (bundled)
- Connection: Local file at `com.plantarium/plantarium/data.db`
- Client: Custom `SqliteStorage` implementation in `src/storage/sqlite.rs`
- Tables:
  - gardens
  - beds
  - placed_plants
  - plants
  - tasks
  - journal_entries
  - calendar_events
  - plot_actions
  - bed_orders

**Web Storage (Optional):**
- Provider: gloo-storage 0.3
- Used for: Browser localStorage (web build only)
- Feature flag: `web` feature

## Platform Paths

**Desktop:**
- Uses `directories` crate (v5.0) to determine platform-specific data directory
- Path resolution: `ProjectDirs::from("com", "plantarium", "plantarium")`

**Web:**
- Uses browser localStorage via gloo-storage

## Authentication & Identity

**None detected** - No external auth provider configured

## Monitoring & Observability

**None detected** - No error tracking or logging services integrated

## CI/CD & Deployment

**Hosting:**
- Dioxus web build (wasm)
- Dioxus desktop build (native)

**Build targets:**
- `cargo build --features web` - Web/WASM
- `cargo build --features desktop` - Desktop (default)

## Environment Configuration

**Required environment variables:** None detected

**Secrets location:** Not applicable (local-only app)

## Webhooks & Callbacks

**Incoming:** None

**Outgoing:** None

---

*Integration audit: 2026-03-26*