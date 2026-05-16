use crate::app_state::AppState;
use crate::storage::db::{StorageError, StorageProvider};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct SqliteStorage {
    db_path: PathBuf,
}

impl SqliteStorage {
    pub fn new() -> Result<Self, StorageError> {
        let db_path = Self::get_db_path()?;
        Ok(Self { db_path })
    }

    fn get_db_path() -> Result<PathBuf, StorageError> {
        let project_dirs =
            ProjectDirs::from("com", "plantarium", "plantarium").ok_or_else(|| {
                StorageError::Custom("Could not determine project directories".to_string())
            })?;

        let data_dir = project_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        Ok(data_dir.join("data.db"))
    }

    fn open_connection(&self) -> Result<Connection, StorageError> {
        Connection::open(&self.db_path).map_err(Into::into)
    }
}

impl StorageProvider for SqliteStorage {
    fn init(&self) -> Result<(), StorageError> {
        let conn = self.open_connection()?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS gardens (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                min_bed_distance INTEGER,
                bed_spacing INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS beds (
                id TEXT PRIMARY KEY,
                garden_id TEXT NOT NULL,
                name TEXT NOT NULL,
                width INTEGER NOT NULL,
                height INTEGER NOT NULL,
                x INTEGER,
                y INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER,
                FOREIGN KEY (garden_id) REFERENCES gardens(id)
            );

            CREATE TABLE IF NOT EXISTS placed_plants (
                id TEXT PRIMARY KEY,
                bed_id TEXT NOT NULL,
                plant_id TEXT NOT NULL,
                x REAL NOT NULL,
                y REAL NOT NULL,
                harvested_at INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER,
                FOREIGN KEY (bed_id) REFERENCES beds(id),
                FOREIGN KEY (plant_id) REFERENCES plants(id)
            );

            CREATE TABLE IF NOT EXISTS plants (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                color TEXT NOT NULL,
                icon TEXT NOT NULL,
                family TEXT,
                species TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                date TEXT NOT NULL,
                type TEXT NOT NULL,
                completed INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS journal_entries (
                id TEXT PRIMARY KEY,
                date TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS calendar_events (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                date TEXT NOT NULL,
                type TEXT NOT NULL,
                plant_id TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS plot_actions (
                id TEXT PRIMARY KEY,
                bed_id TEXT NOT NULL,
                plant_id TEXT NOT NULL,
                action TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                date TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                deleted_at INTEGER,
                FOREIGN KEY (bed_id) REFERENCES beds(id),
                FOREIGN KEY (plant_id) REFERENCES plants(id)
            );

            CREATE TABLE IF NOT EXISTS bed_orders (
                garden_id TEXT NOT NULL,
                bed_id TEXT NOT NULL,
                position INTEGER NOT NULL,
                PRIMARY KEY (garden_id, bed_id)
            );
            "#,
        )?;

        Ok(())
    }

    fn load_all(&self) -> Result<AppState, StorageError> {
        let _conn = self.open_connection()?;

        let mut state = AppState::default();

        // TODO: Implement load queries in PROMPT 5
        // For now, return empty state

        Ok(state)
    }

    fn save_all(&self, _state: &AppState) -> Result<(), StorageError> {
        let _conn = self.open_connection()?;

        // TODO: Implement save queries in PROMPT 6

        Ok(())
    }
}

impl Default for SqliteStorage {
    fn default() -> Self {
        Self::new().expect("Failed to initialize SQLite storage")
    }
}
