# Plantarium 🌱

**A cross-platform desktop & mobile application for garden planning and management**

## About

Plantarium is a desktop and mobile application designed to help home gardeners and small-scale growers:

- Plan garden layouts visually
- Get personalized planting schedules
- Receive timely gardening reminders
- Implement companion planting and crop rotation strategies

Built with **Rust + Tauri 2.0**, Plantarium runs on Windows, macOS, Linux, iOS, and Android.

## Features

### 🌿 Garden Planning
- Drag-and-drop garden layout designer
- Visual representation of plants and spacing
- Multiple garden areas and beds management

### 📅 Smart Scheduling
- Location-based planting calendar
- Automatic task reminders (planting, fertilizing, harvesting)
- Weather-integrated suggestions (frost alerts)

### 🌱 Plant Management
- Integrated plant database (Permapeople API)
- Custom plant entries
- Companion planting guidance
- Crop rotation tracking

### 🔔 Notifications
- Desktop/Mobile alerts for gardening tasks
- Rotation warnings
- Weather alerts

## Technology Stack

| Component | Technology |
|-----------|------------|
| **Backend** | Rust + Axum |
| **ORM** | SeaORM |
| **Database** | SQLite |
| **Frontend** | Tauri 2.0 + React/Vue/Svelte |
| **APIs** | Permapeople, OpenWeather |

## Quick Start

### Prerequisites

- Rust 1.75+
- Node.js 18+
- Git

### Installation

```bash
# 1. Clone repository
git clone https://github.com/yourusername/plantarium.git
cd plantarium

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Install frontend dependencies
cd frontend && npm install

# 4. Run development
npm run tauri dev
```

### Environment Variables

Create `.env` file:

```env
PERMAPEOPLE_API_KEY=your_key
OPENWEATHER_API_KEY=your_key
DATABASE_URL=sqlite://plantarium.db
```

## Documentation

- [SPEC.md](SPEC.md) - Full specification
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical architecture
- [SETUP.md](SETUP.md) - Setup guide

## License

MIT
