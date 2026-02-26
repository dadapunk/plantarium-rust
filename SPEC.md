# Specification: Plantarium

## 1. Introduction & Goals

**Purpose**: Plantarium is a cross-platform desktop application designed to help home gardeners and small-scale growers plan, manage, and optimize their crop cultivation efficiently.

**Problem Solved**: It addresses the complexity of garden planning by simplifying layout design, providing timely, location-aware reminders for crucial tasks (planting, fertilizing, harvesting), and integrating best practices like companion planting and crop rotation to improve garden health and yield.

**High-Level Goals**:
- Offer an intuitive graphical interface for designing and visualizing garden layouts (beds, plants).
- Deliver personalized planting schedules based on local climate and real-time weather data.
- Automate notifications for key gardening activities.
- Guide users in implementing beneficial planting strategies (companion planting, crop rotation).
- Ensure a seamless user experience across major desktop operating systems (Windows, macOS, Linux).

## 2. Target Audience

**Primary**: Home gardeners (ranging from beginners needing guidance to intermediate users seeking better organization).

**Secondary**: Allotment holders, community garden members, small-scale hobby farmers.

Users seeking a digital alternative or supplement to paper-based garden planning methods.

## 3. Functional Requirements (Elaborated Features)

### 3.1 Garden Layout & Visualization:
- Users can define one or more distinct garden areas (e.g., "Backyard", "Front Patch").
- Within each area, users can create, name, resize, and position garden beds using a visual editor (using Tauri/WebView canvas capabilities). Support for basic shapes (rectangles, squares) is essential; polygons are a potential enhancement.
- Users can drag and drop specific plants (from the database or custom entries) onto the beds in the layout view.
- The layout should visually represent the plants, potentially showing approximate spacing or icons.
- Ability to save, load, and potentially duplicate garden layouts.
- **(Adding non-plant elements like paths is not planned for the initial version).
- **Plot/Parcela Detail View**: When viewing a plot (parcela), the UI must display a visual representation showing:
  - The graphical layout with sub-sections (bancales/hbeds) within the plot.
  - Visual placement of plants within each sub-section with their icons.
  - Ability to edit the layout directly from this view.

### 3.2 Plant Database Integration & Management:
- Seamless integration with the Permapeople API for searching and retrieving detailed plant information (e.g., planting times, spacing needs, light/water requirements, companion planting data, family).
- Robust search and filtering capabilities within the plant database.
- Users can add plants from the API results to a personal "My Plants" library for quick access.
- Crucial: Ability for users to add custom plants with key details (name, family, planting notes, etc.) if not found in the API.
- The application will implement caching for frequently accessed plant data from the Permapeople API to improve performance and handle potential API downtime.

### 3.3 Climate-Adapted Planting Schedules & Calendar:
- Calculates suggested planting windows (sow indoors, transplant, direct sow) based on:
  - User's specified location (e.g., postal code, city).
  - General climate data associated with the location.
  - Specific plant requirements (from Permapeople API or custom entry).
- (Optional Enhancement) Fine-tuning based on short-term OpenWeather API forecasts (e.g., delaying planting if frost is imminent).
- Provides an integrated calendar view displaying scheduled tasks: planting, fertilizing, watering reminders, projected harvest windows, user-added custom tasks.
- **Calendar Views**: The calendar must support multiple view modes:
  - **Day View**: Shows detailed schedule for a specific day with hourly breakdown.
  - **Week View**: Displays a weekly overview with tasks per day.
  - **Month View**: Traditional monthly calendar with task/event indicators.
  - **Year View**: Annual overview showing task density and patterns across months.
- Ability to filter the calendar by task type or plant.

### 3.4 Journal (Diary):
- Users can create, edit, and delete journal entries with date and markdown content.
- Entries support links to plots (parcelas) and plants using @parcela:name and @planta:name syntax.
- **Journal Views**: The journal must support filtering by time period:
  - **Day View**: Shows entries for a specific date.
  - **Week View**: Displays entries from the selected week.
  - **Month View**: Shows all entries from the selected month.
  - **Year View**: Overview of entries grouped by month for the selected year.
- Entries are sorted by date (newest first by default).

### 3.5 Notifications & Alerts System:
- Task Reminders: Timely alerts for planting, transplanting, fertilizing, and potentially user-defined tasks based on the schedule.
- Crop Rotation Warnings: If a user attempts to place a plant in a bed, the system checks the planting history (see 3.6) and warns if it violates rotation principles (e.g., planting brassicas where legumes were last season, if that's undesirable based on rules).
- Companion Planting Feedback: (Optional) Subtle alerts or indicators if incompatible plants are placed adjacent in the layout.
- Weather Alerts (Configurable): Notifications for significant weather events affecting the garden, like first/last frost warnings based on OpenWeather data.
- Delivery: Configurable notification methods (in-app messages, platform-specific notifications via Tauri's notification system).

### 3.5 Companion Planting Guidance:
- When a plant is selected or being placed in the layout, the UI should provide clear information about its beneficial and detrimental companions.
- This could be a sidebar list, visual highlighting of compatible/incompatible plants already in the layout, or tooltips.
- Data sourced primarily from the Permapeople API, potentially supplemented by an internal ruleset.

### 3.6 Crop Rotation Tracking:
- The application must automatically log which plant (and importantly, its family) was grown in which bed for each growing season/year.
- This history is used by the Crop Rotation Warnings feature (3.4).
- Users should be able to view the planting history for a specific bed.
- The number of years of history stored for rotation purposes will be configurable by the user (e.g., defaulting to 3 or 4 years).

### 3.7 Weather Integration (OpenWeather API):
- Requires user to input location or grant location access.
- Fetches current conditions and forecasts (temperature, precipitation, frost probability).
- Used primarily for frost warnings and potentially adjusting planting date suggestions or generating watering advice.

## 4. Non-Functional Requirements

- **Platform Compatibility**: Must run natively on Windows, macOS, Linux, iOS, and Android via Tauri 2.0.
- **Performance**: Smooth and responsive UI. Asynchronous operations for API calls and background tasks. Efficient data handling using Rust's memory safety.
- **Usability**: High priority on an intuitive, visually clear, and easy-to-navigate interface. Minimal learning curve for the target audience.
- **Reliability**: Stable operation with minimal crashes. Graceful handling of errors (e.g., network down, API unavailable) with informative messages to the user. Robust data saving.
- **Data Persistence**: User data (layouts, plant lists, settings, history) must be stored locally and reliably using SQLite. Ensure data integrity and provide backup/restore options if feasible.
- **Offline Functionality**: Core features (viewing/editing layouts, viewing cached data/schedules) must work without an internet connection. Features requiring live data (weather forecast, new plant searches) should be disabled gracefully when offline.
- **Security**: Protect user data. Securely store API keys (OpenWeather, Permapeople) – avoid embedding directly in code. Ensure secure connection when accessing external APIs and secure access to the local SQLite database.
- **Binary Size**: Target <15MB for desktop, <10MB for mobile (compared to 100MB+ for Electron).

## 5. Technical Specifications

### Current Stack (Rust Migration)

| Component | Technology | Version |
|-----------|------------|---------|
| **Backend Framework** | Axum | 0.8.x |
| **ORM** | SeaORM | 0.13.x |
| **Database** | SQLite | 3.x |
| **Desktop/Mobile Framework** | Tauri | 2.0 |
| **HTTP Client** | reqwest | 0.12.x |
| **Async Runtime** | Tokio | 1.x |
| **API Documentation** | utoipa | 4.x |

### Previous Stack (Legacy)
- ~~Flutter~~ → Tauri 2.0
- ~~NestJS~~ → Axum
- ~~TypeORM~~ → SeaORM
- ~~Node.js~~ → Rust

### External APIs
- Permapeople API (Plant data)
- OpenWeather API (Weather)

### Local Data Storage
- SQLite via SeaORM

### Code Management
- Git / Version Control System

## 6. UI/UX Considerations

- **Visual Design**: Clean, uncluttered aesthetic. Consider themes (light/dark) or subtle garden-inspired visuals. Prioritize clarity and readability.
- **Interaction**: Intuitive drag-and-drop for layout. Clear forms for data entry. Consistent navigation patterns.
- **Feedback**: Provide visual feedback for actions (saving, loading, API calls). Use loading indicators.
- **Accessibility**: Adhere to basic accessibility guidelines (sufficient color contrast, keyboard navigability, screen reader support where appropriate).

### 6.1 Dashboard (Home)

The home page serves as a central hub displaying a summary of all application sections:

- **Resumen de Áreas**: Card showing total areas and quick access to them.
- **Última Nota del Diario**: Display the most recent journal entry with title and date.
- **Próximas Tareas**: List of pending tasks sorted by date (next 5-7 days).
- **Calendario**: Mini calendar view showing the current month with markers for tasks/events.
- **Acceso Rápido**: Navigation to all main sections (Áreas, Diario, Tareas, Calendario).

## 7. Future Considerations (Potential Roadmap)

- ~~Mobile companion app~~ - Now included via Tauri 2.0!
- Cloud sync option for multi-device access.
- Advanced reporting/analytics (e.g., yield tracking per bed/plant).
- Integration with smart garden sensors.
- Community features (sharing layouts, tips, plant reviews).
- Pest and disease logging/management features.