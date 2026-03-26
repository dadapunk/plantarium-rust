---
title: Plantarium Requirements
date: 2026-03-26
version: 1.0
---

# Requirements

## v1 Requirements

### Persistence

- [ ] **PERSIST-01**: Data persists between app sessions (SQLite wired to load/save)
- [ ] **PERSIST-02**: WAL mode enabled for concurrent access
- [ ] **PERSIST-03**: Graceful error handling on storage failures (no .expect() crashes)

### Garden Management

- [ ] **GARDEN-01**: User can create gardens with name and description
- [ ] **GARDEN-02**: User can view list of all gardens on dashboard
- [ ] **GARDEN-03**: User can edit garden name and description
- [ ] **GARDEN-04**: User can delete gardens

### Bed Management

- [ ] **BED-01**: User can create beds within a garden with dimensions
- [ ] **BED-02**: User can position beds freely on garden canvas
- [ ] **BED-03**: User can edit bed dimensions and position
- [ ] **BED-04**: User can delete beds

### Visual Editor

- [ ] **EDITOR-01**: User can place plants on bed grid via click
- [ ] **EDITOR-02**: User can move placed plants within bed
- [ ] **EDITOR-03**: User can mark plants as harvested
- [ ] **EDITOR-04**: Bed displays plant positions correctly

### Task Management

- [ ] **TASK-01**: User can create tasks with title, type, due date
- [ ] **TASK-02**: User can filter tasks by type (watering, fertilizing, harvesting, general)
- [ ] **TASK-03**: User can filter tasks by status (pending, done)
- [ ] **TASK-04**: User can mark tasks as complete
- [ ] **TASK-05**: User can delete tasks

### Calendar

- [ ] **CAL-01**: User can view calendar with events
- [ ] **CAL-02**: User can create calendar events (sowing, watering, harvest)
- [ ] **CAL-03**: User can edit event details
- [ ] **CAL-04**: User can delete events

### Journal

- [ ] **JOURNAL-01**: User can create journal entries with Markdown support
- [ ] **JOURNAL-02**: User can view list of journal entries
- [ ] **JOURNAL-03**: User can edit existing journal entries (FIX broken button)
- [ ] **JOURNAL-04**: User can delete journal entries

### Dashboard

- [ ] **DASH-01**: Dashboard shows overview of all gardens
- [ ] **DASH-02**: Dashboard displays quick stats (total gardens, beds, tasks)
- [ ] **DASH-03**: Dashboard shows recent harvests

## v2 Requirements (Deferred)

- [ ] **PLANT-01**: Expand plant database to 50-100 plants with spacing/sun/water data
- [ ] **HARVEST-01**: Harvest logging with yield tracking
- [ ] **COMPANION-01**: Companion planting guidance
- [ ] **ROTATION-01**: Crop rotation suggestions

## Out of Scope

| Exclusion | Reason |
|-----------|--------|
| Cloud sync | Local-first app, single user |
| User authentication | Single user local app |
| Mobile apps | Desktop-first, web as secondary |
| Photo journaling | Journal text-only sufficient for v1 |
| AI plant identification | Out of scope, adds complexity |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PERSIST-01 | Phase 1 | Pending |
| PERSIST-02 | Phase 1 | Pending |
| PERSIST-03 | Phase 1 | Pending |
| GARDEN-01 | Phase 1 | Pending |
| GARDEN-02 | Phase 1 | Pending |
| GARDEN-03 | Phase 1 | Pending |
| GARDEN-04 | Phase 1 | Pending |
| BED-01 | Phase 1 | Pending |
| BED-02 | Phase 1 | Pending |
| BED-03 | Phase 1 | Pending |
| BED-04 | Phase 1 | Pending |
| JOURNAL-03 | Phase 1 | Pending |
| TASK-01 | Phase 2 | Pending |
| TASK-02 | Phase 2 | Pending |
| TASK-03 | Phase 2 | Pending |
| TASK-04 | Phase 2 | Pending |
| TASK-05 | Phase 2 | Pending |
| CAL-01 | Phase 2 | Pending |
| CAL-02 | Phase 2 | Pending |
| CAL-03 | Phase 2 | Pending |
| CAL-04 | Phase 2 | Pending |
| DASH-01 | Phase 3 | Pending |
| DASH-02 | Phase 3 | Pending |
| DASH-03 | Phase 3 | Pending |
| EDITOR-01 | Phase 3 | Pending |
| EDITOR-02 | Phase 3 | Pending |
| EDITOR-03 | Phase 3 | Pending |
| EDITOR-04 | Phase 3 | Pending |
| JOURNAL-01 | Phase 1 | Pending |
| JOURNAL-02 | Phase 1 | Pending |
| JOURNAL-03 | Phase 1 | Pending |
| JOURNAL-04 | Phase 1 | Pending |

---

*Last updated: 2026-03-26 after research synthesis*