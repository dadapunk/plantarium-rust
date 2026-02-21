# Plantarium Rust Architecture

## Overview

This document describes the technical architecture for the Plantarium application migrated to Rust.

## Technology Stack

| Component | Technology | Version |
|-----------|------------|---------|
| Backend Framework | Axum | 0.8.x |
| ORM | SeaORM | 0.13.x |
| Database | SQLite | 3.x |
| Frontend Framework | Tauri | 2.0 |
| HTTP Client | reqwest | 0.12.x |
| Async Runtime | Tokio | 1.x |
| API Documentation | utoipa | 4.x |

## Architecture Layers

```
┌─────────────────────────────────────────────┐
│              Frontend (Tauri 2.0)            │
│         React / Vue / Svelte / Dioxus       │
├─────────────────────────────────────────────┤
│              API Layer (Axum)                │
│         REST Endpoints + WebSocket           │
├─────────────────────────────────────────────┤
│           Service Layer                      │
│      Business Logic + Validation             │
├─────────────────────────────────────────────┤
│           Repository Layer (SeaORM)          │
│         Database Operations                  │
├─────────────────────────────────────────────┤
│           Database (SQLite)                 │
│              plantarium.db                   │
└─────────────────────────────────────────────┘
```

## Project Structure

```
plantarium/
├── Cargo.toml              # Workspace root
├── rust-toolchain.toml     # Rust version
├── .env.example            # Environment template
│
├── backend/               # Axum API server
│   ├── Cargo.toml
│   ├── src/
│   │         # Entry point   ├── main.rs
│   │   ├── lib.rs          # Library root
│   │   ├── config.rs       # Configuration
│   │   ├── error.rs        # Error handling
│   │   ├── db.rs           # Database setup
│   │   ├── entities/       # SeaORM entities
│   │   │   ├── plant.rs
│   │   │   ├── plot.rs
│   │   │   └── garden_note.rs
│   │   ├── dto/            # Data transfer objects
│   │   ├── handlers/       # HTTP handlers
│   │   ├── services/       # Business logic
│   │   └── router.rs       # Route definitions
│   └── migrations/         # Database migrations
│
├── frontend/              # Tauri application
│   ├── src/               # Frontend source
│   ├── src-tauri/         # Rust backend
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── main.rs
│   │   ├── tauri.conf.json
│   │   ├── icons/
│   │   └── capabilities/
│   ├── index.html
│   └── package.json
│
└── documentation/         # Project docs
```

## Database Schema

### Plants Table

```sql
CREATE TABLE plants (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    species VARCHAR(255),
    family VARCHAR(255),
    leaf_shape VARCHAR(100),
    planting_date DATE,
    plot_id INTEGER REFERENCES plots(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Plots Table

```sql
CREATE TABLE plots (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    length DECIMAL(10, 2),
    width DECIMAL(10, 2),
    location VARCHAR(255),
    area DECIMAL(10, 2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Garden Notes Table

```sql
CREATE TABLE garden_notes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    note TEXT,
    date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## API Endpoints

### Plants

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/plants | List all plants |
| GET | /api/plants/:id | Get plant by ID |
| POST | /api/plants | Create new plant |
| PUT | /api/plants/:id | Update plant |
| DELETE | /api/plants/:id | Delete plant |
| GET | /api/plants/external | Fetch from Permapeople API |

### Plots

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/plots | List all plots |
| GET | /api/plots/:id | Get plot by ID |
| POST | /api/plots | Create new plot |
| PUT | /api/plots/:id | Update plot |
| DELETE | /api/plots/:id | Delete plot |

### Garden Notes

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/notes | List all notes |
| GET | /api/notes/:id | Get note by ID |
| POST | /api/notes | Create new note |
| PUT | /api/notes/:id | Update note |
| DELETE | /api/notes/:id | Delete note |

## Configuration

Environment variables:

```env
DATABASE_URL=sqlite://plantarium.db
SERVER_HOST=0.0.0.0
SERVER_PORT=3002
PERMAPEOPLE_API_KEY=your_api_key
PERMAPEOPLE_KEY=your_key
RUST_LOG=info
```

## Error Handling

Custom error types using `thiserror`:

```rust
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("External API error: {0}")]
    ExternalApi(String),
}
```

## Logging

Using `tracing` for structured logging:

```rust
tracing::info!("Starting server on {}", addr);
tracing::error!("Database connection failed: {}", err);
```

## Testing

- Unit tests: Test individual functions and modules
- Integration tests: Test API endpoints
- Use `cargo test` to run all tests

## Security

- Input validation using validator crates
- CORS configuration for API
- Rate limiting (optional)
- SQL injection prevention via SeaORM parameterization

## Performance Considerations

- Async/await throughout the application
- Connection pooling for database
- Lazy loading for relationships
- Pagination for list endpoints
