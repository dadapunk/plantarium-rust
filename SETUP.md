# Migration Guide: From NestJS to Rust

This guide documents the migration process from the current NestJS/TypeScript stack to Rust.

## Prerequisites

### Required Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version    # Should show 1.75+
cargo --version

# Install Tauri CLI
cargo install tauri-cli

# Install additional tools
cargo install sea-orm-cli    # For migrations
cargo install cargo-watch    # For development
```

### Platform-Specific Requirements

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# For mobile development
brew install cocoapods
```

#### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libudev-dev \
    pkg-config
```

#### Windows
- Install Visual Studio Build Tools
- Install WebView2 Runtime

## Migration Phases

### Phase 1: Database Schema

#### 1.1 Export Current Schema

Export the existing SQLite schema:

```bash
# Use sqlite3 CLI
sqlite3 database/plantarium.schema ".schema"
```

#### 1.2 Define SeaORM Entities

Create entities in `backend/src/entities/`:

```rust
// backend/src/entities/plant.rs
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "plants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub species: Option<String>,
    pub family: Option<String>,
    pub leaf_shape: Option<String>,
    pub planting_date: Option<chrono::NaiveDate>,
    pub plot_id: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

#### 1.3 Run Migrations

```bash
cd backend
sea-orm-cli migrate -n plantarium
```

---

### Phase 2: Backend API (Axum)

#### 2.1 Project Setup

```bash
# Create new workspace
mkdir plantarium-rust
cd plantarium-rust
cargo init --name plantarium

# Add Cargo.toml with all dependencies
cat > Cargo.toml << 'EOF'
[workspace]
members = ["backend", "frontend"]

[package]
name = "plantarium"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.8"
sea-orm = { version = "0.13", features = ["sqlx-sqlite", "runtime-tokio-native-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1"
reqwest = { version = "0.12", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
dotenv = "0.15"
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace"] }
utoipa = { version = "4", features = ["axum", "swagger-ui"] }
EOF

mkdir backend
cd backend
cargo init
```

#### 2.2 Implement Handlers

Example plant handler:

```rust
// backend/src/handlers/plant.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::EntityTrait;

use crate::dto::{CreatePlantDto, UpdatePlantDto};
use crate::entities::plant;
use crate::error::AppError;
use crate::AppState;

pub async fn list_plants(
    State(state): State<AppState>,
) -> Result<Json<Vec<plant::Model>>, AppError> {
    let plants = plant::Entity::find()
        .all(&state.db)
        .await
        .map_err(AppError::Database)?;
    
    Ok(Json(plants))
}

pub async fn get_plant(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<plant::Model>, AppError> {
    let plant = plant::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Plant not found".into()))?;
    
    Ok(Json(plant))
}

pub async fn create_plant(
    State(state): State<AppState>,
    Json(dto): Json<CreatePlantDto>,
) -> Result<(StatusCode, Json<plant::Model>), AppError> {
    let active_model = plant::ActiveModel {
        name: sea_orm::ActiveValue::Set(dto.name),
        species: sea_orm::ActiveValue::Set(dto.species),
        family: sea_orm::ActiveValue::Set(dto.family),
        leaf_shape: sea_orm::ActiveValue::Set(dto.leaf_shape),
        planting_date: sea_orm::ActiveValue::Set(dto.planting_date),
        plot_id: sea_orm::ActiveValue::Set(dto.plot_id),
        ..Default::default()
    };
    
    let plant = active_model
        .insert(&state.db)
        .await
        .map_err(AppError::Database)?;
    
    Ok((StatusCode::CREATED, Json(plant)))
}
```

#### 2.3 Router Setup

```rust
// backend/src/router.rs
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{garden_note, plant, plot};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/plants", axum::routing::get(plant::list_plants))
        .route("/api/plants/:id", axum::routing::get(plant::get_plant))
        .route("/api/plants", axum::routing::post(plant::create_plant))
        .route("/api/plants/:id", axum::routing::put(plant::update_plant))
        .route("/api/plants/:id", axum::routing::delete(plant::delete_plant))
        .route("/api/plots", axum::routing::get(plot::list_plots))
        .route("/api/notes", axum::routing::get(garden_note::list_notes))
        .layer(cors)
        .with_state(state)
}
```

---

### Phase 3: Frontend (Tauri)

#### 3.1 Initialize Tauri Project

```bash
# Create Tauri app with React template
npm create tauri-app@latest plantarium-frontend -- --template react-ts
cd plantarium-frontend

# Add Tauri API
npm install @tauri-apps/api
```

#### 3.2 Configure Tauri

```json
// src-tauri/tauri.conf.json
{
  "productName": "Plantarium",
  "identifier": "com.plantarium.app",
  "version": "0.1.0",
  "build": {
    "devtools": true
  },
  "app": {
    "windows": [
      {
        "title": "Plantarium",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

#### 3.3 Mobile Configuration

Add mobile support:

```bash
# Install mobile dependencies
npm install @tauri-apps/api@latest

# Add Android platform
cargo tauri android init

# Add iOS platform (macOS only)
cargo tauri ios init
```

#### 3.4 IPC Commands

Define Rust commands in Tauri:

```rust
// frontend/src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Plantarium starting...");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            plantarium_api::get_plants,
            plantarium_api::create_plant,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### Phase 4: External API Integration

#### 4.1 Permapeople API Client

```rust
// backend/src/external/permapeople.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExternalPlant {
    pub id: String,
    pub name: String,
    pub scientific_name: Option<String>,
    pub family: Option<String>,
}

pub struct PermapeopleClient {
    client: Client,
    api_key: String,
}

impl PermapeopleClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn search_plants(&self, query: &str) -> Result<Vec<ExternalPlant>, AppError> {
        let url = format!(
            "https://permapeople.org/api/plants/search?q={}",
            query
        );
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| AppError::ExternalApi(e.to_string()))?;
        
        let plants: Vec<ExternalPlant> = response
            .json()
            .await
            .map_err(|e| AppError::ExternalApi(e.to_string()))?;
        
        Ok(plants)
    }
}
```

---

### Phase 5: Testing

#### 5.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plant_validation() {
        let dto = CreatePlantDto {
            name: "".to_string(), // Invalid: empty name
            species: None,
            family: None,
            leaf_shape: None,
            planting_date: None,
            plot_id: None,
        };
        
        assert!(dto.validate().is_err());
    }
}
```

#### 5.2 Integration Tests

```rust
// tests/api_test.rs
use axum::{
    body::Body,
    router::Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_list_plants() {
    let app = create_test_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/plants")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## Running the Application

### Development

```bash
# Backend only
cd backend
cargo run

# Frontend (Tauri)
cd frontend
npm run tauri dev

# Mobile
npm run tauri android dev   # Requires Android SDK
npm run tauri ios dev       # Requires macOS + Xcode
```

### Production Build

```bash
# Desktop
npm run tauri build

# Android
npm run tauri build -- --target aarch64-linux-android

# iOS (macOS only)
npm run tauri build -- --target x86_64-apple-darwin
```

---

## Environment Variables

Create `.env` file:

```env
# Database
DATABASE_URL=sqlite://plantarium.db

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=3002

# External APIs
PERMAPEOPLE_API_KEY=your_api_key_here
PERMAPEOPLE_KEY=your_key_here

# Logging
RUST_LOG=info
```

---

## Troubleshooting

### Compilation Errors

- Ensure Rust 1.75+ is installed: `rustup update`
- Clear build cache: `cargo clean`

### Database Issues

- Check SQLite file permissions
- Verify DATABASE_URL is correct
- Run migrations: `sea-orm-cli migrate run`

### Mobile Build Issues

- Android: Verify ANDROID_HOME is set
- iOS: Ensure Xcode is up to date
