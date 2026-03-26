---
title: Plantarium
project: plantarium-rust
date: 2026-03-26
version: 1.0
---

# Plantarium

## What This Is

Desktop application for personal garden management - plan garden beds, organize plants, track tasks and maintain a botanical journal. Built with Dioxus + Rust + SQLite.

## Core Value

A personal garden planner that helps users design beds visually, track planting schedules, manage gardening tasks, and document their garden's growth over time.

## Requirements

### Validated

- ✓ Gardens - Create and organize multiple gardens — existing
- ✓ Garden detail view with bed management — existing
- ✓ Visual bed editor with free-position plant placement — existing
- ✓ Task management with filtering by type and status — existing
- ✓ Calendar for tracking sowing, watering, harvest events — existing
- ✓ Journal with Markdown support — existing
- ✓ Dashboard with overview statistics — existing

### Active

- [ ] Fix SQLite persistence (data not saved between sessions)
- [ ] Implement garden deletion functionality
- [ ] Fix journal edit button

### Out of Scope

- Cloud sync — local-first only
- Mobile apps — desktop-first, web as secondary
- User accounts/authentication — single user local app

## Context

**Current state (from codebase map):**
- Dioxus 0.7.3 SPA with global signal state
- Router-based navigation (6 main routes)
- Storage abstraction layer exists (SQLite + localStorage)
- Critical bug: SQLite persistence not wired up (data lost on restart)
- Demo data hardcoded in state (no way to clear)

**User feedback themes:**
- Need persistent data storage
- Want ability to delete gardens
- Journal editing broken

## Constraints

- **Tech Stack**: Dioxus 0.7.3 + Rust 2021 + rusqlite — existing, not changing
- **Platform**: Desktop-first with web as secondary target — current architecture supports both
- **Data**: Local SQLite storage — no cloud/sync planned

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Dioxus framework | React-like, cross-platform capable | ✓ Good |
| SQLite storage | Local persistence, cross-platform | ⚠️ Needs wiring |
| SPA architecture | Single page app with client-side routing | ✓ Good |

---

*Last updated: 2026-03-26 after GSD initialization (brownfield)*

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state