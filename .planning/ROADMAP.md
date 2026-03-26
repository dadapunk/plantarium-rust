# Plantarium Roadmap

## Phases

- [ ] **Phase 1: Persistence & Core CRUD** - Wire SQLite to state, implement garden deletion, fix journal edit
- [ ] **Phase 2: Tasks & Calendar** - Full task/calendar CRUD with persistence
- [ ] **Phase 3: Dashboard & Plant Database** - Validate dashboard, expand plant data

## Phase Details

### Phase 1: Persistence & Core CRUD

**Goal:** Fix critical data loss bug and complete missing CRUD operations

**Depends on:** Nothing (first phase)

**Requirements:** PERSIST-01, PERSIST-02, PERSIST-03, GARDEN-01, GARDEN-02, GARDEN-03, GARDEN-04, BED-01, BED-02, BED-03, BED-04, JOURNAL-03

**Success Criteria** (what must be TRUE):
  1. App loads saved data from SQLite on startup - no data loss between sessions
  2. All changes (create, edit, delete) persist to SQLite immediately
  3. User can delete gardens with confirmation
  4. User can edit existing journal entries (fix broken button)
  5. WAL mode enabled - no "database locked" errors during normal use
  6. Database errors show user-friendly messages, not crashes

**Plans**: TBD

---

### Phase 2: Tasks & Calendar

**Goal:** Full task and calendar functionality with persistence

**Depends on:** Phase 1

**Requirements:** TASK-01, TASK-02, TASK-03, TASK-04, TASK-05, CAL-01, CAL-02, CAL-03, CAL-04

**Success Criteria** (what must be TRUE):
  1. User can create, edit, and delete tasks that persist between sessions
  2. User can filter tasks by type (watering, fertilizing, harvesting, general)
  3. User can filter tasks by status (pending, done)
  4. User can mark tasks complete and see them in filtered views
  5. User can create, edit, and delete calendar events (sowing, watering, harvest)
  6. Calendar displays all events correctly

**Plans**: TBD

---

### Phase 3: Dashboard & Plant Database

**Goal:** Validate dashboard with real data, expand plant database

**Depends on:** Phase 2

**Requirements:** DASH-01, DASH-02, DASH-03, EDITOR-01, EDITOR-02, EDITOR-03, EDITOR-04

**Success Criteria** (what must be TRUE):
  1. Dashboard shows accurate stats from persisted data (not hardcoded)
  2. User can place plants on bed grid via click
  3. User can move placed plants within bed
  4. User can mark plants as harvested
  5. Bed displays plant positions correctly after restart

**Plans**: TBD

---

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Persistence & Core CRUD | 0/1 | Not started | - |
| 2. Tasks & Calendar | 0/1 | Not started | - |
| 3. Dashboard & Plant Database | 0/1 | Not started | - |
