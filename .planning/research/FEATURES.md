# Feature Landscape

**Domain:** Personal Garden Management Application
**Researched:** 2026-03-26

## Executive Summary

A complete garden management application must deliver persistent data storage, visual bed planning, plant tracking, and task scheduling. Without these table-stakes features, users will abandon the app within the first session. The market (GrowVeg, Gardenize, Leaftide, Planter) shows that successful apps combine spatial planning with temporal scheduling — users expect to design their beds visually AND receive timely reminders for planting, watering, and harvesting.

For Plantarium specifically, the current feature set is nearly complete but suffers from critical bugs that prevent production-readiness: SQLite persistence is not wired (data is lost between sessions), garden deletion is missing (incomplete CRUD), and journal editing is broken. These are blockers before any new features can be considered.

## Table Stakes

Features users expect. Missing = product feels incomplete or unusable.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Data Persistence** | Users lose trust when data disappears between sessions. This is the #1 issue reported. | High | CRITICAL BUG: SQLite exists but not wired to state |
| **Garden CRUD** | Create, view, edit, and DELETE gardens. Users cannot remove test/old gardens. | Medium | Delete is explicitly missing |
| **Visual Bed Editor** | Drag-and-drop plant placement is the core value proposition. GrowVeg built an entire business on this. | High | Already exists ✓ |
| **Plant Database** | Without plant info (spacing, sun needs, water requirements), the bed editor is just a drawing tool. | High | Currently has minimal plant data |
| **Task Management** | Gardeners need reminders for watering, fertilizing, pruning, harvesting. | Medium | Already exists ✓ |
| **Calendar View** | Shows sowing, watering, harvest events over time. Connects planning to scheduling. | Medium | Already exists ✓ |
| **Journal** | Users want to document observations, track growth, attach photos. | Low | Edit button is broken - CRITICAL BUG |

### Minimum Viable Set for Production

1. Fix SQLite persistence — app is unusable without data survival
2. Implement garden deletion — incomplete without it
3. Fix journal edit button — broken functionality embarrasses the product
4. Expand plant database — current minimal data limits utility

## Differentiators

Features that set product apart. Not expected, but valued. Competitive advantage comes from doing these better than others.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Companion Planting** | Warn users about bad plant combinations, suggest good ones. GrowVeg and Leaftide have this built-in. | Medium | Currently missing - could use plant database extension |
| **Crop Rotation** | Track what was planted where in previous seasons to prevent disease buildup. GrowVeg includes this. | High | Requires multi-season data tracking |
| **Climate-Aware Scheduling** | Use frost dates, soil temperature, and local climate instead of generic zone-based dates. Leaftide differentiates on this. | High | Requires user location + climate data |
| **Permanent Plant Tracking** | Track fruit trees, berry bushes, perennials across years (not just seasonal veg). Leaftide's key differentiator. | Medium | Currently focused on seasonal beds |
| **Harvest Logging** | Record yields over time, compare seasons, estimate ROI. Gardenize does this well. | Low | Currently only has "harvest" action, no logging |
| **Photo Journaling** | Attach photos to journal entries for visual documentation. Gardenize and Leaftide emphasize this. | Low | Missing in current journal |
| **Container Planning** | Plan pots and grow bags, not just in-ground beds. Leaftide includes this, Planter focuses on it. | Medium | Currently bed-focused |
| **Weather Integration** | Connect to weather data for smart watering reminders, frost warnings. | High | Out of scope per PROJECT.md (no cloud) |

## Anti-Features

Features to explicitly NOT build. These are explicitly out of scope or will distract from core value.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Cloud Sync** | Explicit out-of-scope in PROJECT.md. Local-first is the design decision. | Focus on making local storage rock-solid first |
| **Mobile Apps** | Desktop-first per PROJECT.md. Web target is secondary. | Ensure web build works, but prioritize desktop UX |
| **User Accounts/Auth** | Single-user local app per PROJECT.md. Adds no value for this use case. | Skip entirely |
| **AI Plant Identification** | Requires external APIs, cloud integration. Out of scope. | Let users manually add plants from database |
| **Third-Party Integrations** | Weather APIs, seed vendor integrations. Adds complexity without core value. | Focus on core planning features first |
| **Team/Share Gardens** | Single-user local app. Adds auth + permission complexity. | Not relevant for this product |

## Feature Dependencies

```
Data Persistence (FIX)
    ↓
Garden CRUD Complete
    ↓
Bed Editor + Plant DB → Companion Planting
    ↓
Task + Calendar → Climate-Aware Scheduling
    ↓
Multi-season data → Crop Rotation
```

**Critical Path:**
1. Fix persistence (SQLite wiring) — unblocks everything
2. Complete CRUD (garden delete) — enables real usage
3. Fix journal edit — makes journal usable
4. Expand plant DB — makes bed editor valuable
5. Add companion planting — differentiates from basic tools

## MVP Recommendation

**Prioritize for production-readiness:**
1. **FIX: SQLite persistence** — Without this, users lose all data on restart. This is a dealbreaker.
2. **FIX: Garden deletion** — Cannot remove gardens makes app feel broken.
3. **FIX: Journal edit** — Broken UI element erodes trust.
4. **Add: Basic plant database** — 50-100 common vegetables/herbs with spacing, sun, water needs.
5. **Add: Harvest logging** — Simple yield tracking per plant.

**Defer:**
- Companion planting guidance (needs plant DB expansion first)
- Crop rotation (needs multi-season data, rare for new users)
- Climate-aware scheduling (needs location integration)
- Weather integration (out of scope per design decisions)

## Sources

- Leaftide comparison of garden planning apps (2026): https://leaftide.com/learn/best-garden-planning-apps/
- BioGarden365 feature guide: https://www.biogarden365.com/en/best-garden-planner-software-how-does-the-biogarden365-app-help-%F0%9F%93%B1/
- ZipDo garden management software rankings (2026): https://zipdo.co/best/garden-management-software/
- AI Garden Planner feature guides: https://aigardenplanner.com/blog/post/the-ultimate-guide-to-garden-manager-software-features-benefits-and-popular-options-993
- Fairfax County Master Gardeners app recommendations (2026): https://fairfaxgardening.org/apps-for-2026/

**Confidence:** HIGH - Multiple independent sources across 2026 agree on core feature requirements. Market leaders (GrowVeg, Gardenize, Leaftide) validate the table-stakes findings.

**Research Mode:** Ecosystem (features, competitive landscape)