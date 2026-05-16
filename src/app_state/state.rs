use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STORAGE_KEY: &str = "plantarium_data_v2";
const BED_ORDERS_KEY: &str = "plantarium_bed_order";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SyncableEntity {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Default for SyncableEntity {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Garden {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub name: String,
    pub min_bed_distance: Option<i32>,
    pub bed_spacing: Option<i32>,
}

impl Default for Garden {
    fn default() -> Self {
        Self {
            base: SyncableEntity::default(),
            name: String::new(),
            min_bed_distance: Some(55),
            bed_spacing: Some(60),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlacedPlant {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub plant_id: String,
    pub x: f64,
    pub y: f64,
    pub harvested_at: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Bed {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub garden_id: String,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub plants: Vec<PlacedPlant>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Plant {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub family: Option<String>,
    pub species: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlotActionType {
    Planted,
    Sowed,
    Harvested,
    Removed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlotAction {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub bed_id: String,
    pub plant_id: String,
    pub action: PlotActionType,
    pub quantity: i32,
    pub date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    Sowing,
    Watering,
    Harvest,
    Fertilizing,
    Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub title: String,
    pub date: String,
    pub r#type: TaskType,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CalendarEvent {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub title: String,
    pub date: String,
    pub r#type: TaskType,
    pub plant_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JournalEntry {
    #[serde(flatten)]
    pub base: SyncableEntity,
    pub date: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AppState {
    pub gardens: Vec<Garden>,
    pub beds: Vec<Bed>,
    pub plants: Vec<Plant>,
    pub tasks: Vec<Task>,
    pub events: Vec<CalendarEvent>,
    pub journal: Vec<JournalEntry>,
    pub plot_actions: Vec<PlotAction>,
    pub bed_orders: HashMap<String, Vec<String>>,
}

pub static GARDENS: GlobalSignal<Vec<Garden>> = Signal::global(Vec::new);
pub static BEDS: GlobalSignal<Vec<Bed>> = Signal::global(Vec::new);
pub static PLANTS: GlobalSignal<Vec<Plant>> = Signal::global(Vec::new);
pub static TASKS: GlobalSignal<Vec<Task>> = Signal::global(Vec::new);
pub static EVENTS: GlobalSignal<Vec<CalendarEvent>> = Signal::global(Vec::new);
pub static JOURNAL: GlobalSignal<Vec<JournalEntry>> = Signal::global(Vec::new);
pub static PLOT_ACTIONS: GlobalSignal<Vec<PlotAction>> = Signal::global(Vec::new);
pub static BED_ORDERS: GlobalSignal<HashMap<String, Vec<String>>> = Signal::global(HashMap::new);

fn default_plants() -> Vec<Plant> {
    vec![
        plant("1", "Tomate", "#e74c3c", "🍅"),
        plant("2", "Lechuga", "#27ae60", "🥬"),
        plant("3", "Zanahoria", "#e67e22", "🥕"),
        plant("4", "Pimiento", "#c0392b", "🫑"),
        plant("5", "Cebolla", "#8e44ad", "🧅"),
        plant("6", "Ajo", "#f1c40f", "🧄"),
        plant("7", "Papa", "#d35400", "🥔"),
        plant("8", "Judía", "#16a085", "🫛"),
        plant("9", "Maíz", "#f39c12", "🌽"),
        plant("10", "Calabaza", "#e67e22", "🎃"),
    ]
}

fn plant(id: &str, name: &str, color: &str, icon: &str) -> Plant {
    Plant {
        base: SyncableEntity {
            id: id.into(),
            created_at: 0,
            updated_at: 0,
            deleted_at: None,
        },
        name: name.into(),
        color: color.into(),
        icon: icon.into(),
        family: None,
        species: None,
    }
}

pub fn load_from_storage() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(data) = LocalStorage::get::<AppState>(STORAGE_KEY) {
            *GARDENS.write() = data.gardens;
            *BEDS.write() = data.beds;
            *PLANTS.write() = if data.plants.is_empty() {
                default_plants()
            } else {
                data.plants
            };
            *TASKS.write() = data.tasks;
            *EVENTS.write() = data.events;
            *JOURNAL.write() = data.journal;
            *PLOT_ACTIONS.write() = data.plot_actions;
        } else {
            *PLANTS.write() = default_plants();
        }

        if let Ok(orders) = LocalStorage::get::<HashMap<String, Vec<String>>>(BED_ORDERS_KEY) {
            *BED_ORDERS.write() = orders;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        *PLANTS.write() = default_plants();
    }
}

pub fn save_to_storage() {
    #[cfg(target_arch = "wasm32")]
    {
        let state = AppState {
            gardens: GARDENS.read().clone(),
            beds: BEDS.read().clone(),
            plants: PLANTS.read().clone(),
            tasks: TASKS.read().clone(),
            events: EVENTS.read().clone(),
            journal: JOURNAL.read().clone(),
            plot_actions: PLOT_ACTIONS.read().clone(),
            bed_orders: BED_ORDERS.read().clone(),
        };
        let _ = LocalStorage::set(STORAGE_KEY, &state);
        let _ = LocalStorage::set(BED_ORDERS_KEY, &*BED_ORDERS.read());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Desktop: no-op for now (data not persisted)
    }
}

pub fn create_garden(name: &str) -> Garden {
    let now = chrono::Utc::now().timestamp_millis();
    let garden = Garden {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        name: name.into(),
        min_bed_distance: Some(55),
        bed_spacing: Some(60),
    };
    GARDENS.write().push(garden.clone());
    save_to_storage();
    garden
}

pub fn create_bed(garden_id: &str, name: &str, width: i32, height: i32) -> Bed {
    let now = chrono::Utc::now().timestamp_millis();
    let beds = BEDS.read();
    let existing_beds: Vec<_> = beds.iter().filter(|b| b.garden_id == garden_id).collect();

    let spacing = 60;
    let (x, y) = if existing_beds.is_empty() {
        (50, 50)
    } else {
        let last_bed = existing_beds.last().unwrap();
        let last_y = last_bed.y.unwrap_or(0);
        (50, last_y + last_bed.height + spacing)
    };

    let bed = Bed {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        garden_id: garden_id.into(),
        name: name.into(),
        width,
        height,
        x: Some(x),
        y: Some(y),
        plants: Vec::new(),
    };
    drop(beds);
    BEDS.write().push(bed.clone());
    save_to_storage();
    bed
}

pub fn add_plant_to_bed(bed_id: &str, plant_id: &str, x: f64, y: f64, date: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let placed = PlacedPlant {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        plant_id: plant_id.into(),
        x,
        y,
        harvested_at: None,
    };

    let mut beds = BEDS.write();
    if let Some(bed) = beds.iter_mut().find(|b| b.base.id == bed_id) {
        bed.plants.push(placed);
        bed.base.updated_at = now;
    }
    drop(beds);

    let action = PlotAction {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        bed_id: bed_id.into(),
        plant_id: plant_id.into(),
        action: PlotActionType::Planted,
        quantity: 1,
        date: date.into(),
    };
    PLOT_ACTIONS.write().push(action);
    save_to_storage();
}

pub fn harvest_plant(bed_id: &str, placed_plant_id: &str, date: &str) {
    let now = chrono::Utc::now().timestamp_millis();

    let mut beds = BEDS.write();
    if let Some(bed) = beds.iter_mut().find(|b| b.base.id == bed_id) {
        if let Some(plant) = bed.plants.iter_mut().find(|p| p.base.id == placed_plant_id) {
            if plant.harvested_at.is_none() {
                plant.harvested_at = Some(now);
                let plant_id = plant.plant_id.clone();
                drop(beds);

                let action = PlotAction {
                    base: SyncableEntity {
                        id: uuid::Uuid::new_v4().to_string(),
                        created_at: now,
                        updated_at: now,
                        deleted_at: None,
                    },
                    bed_id: bed_id.into(),
                    plant_id,
                    action: PlotActionType::Harvested,
                    quantity: 1,
                    date: date.into(),
                };
                PLOT_ACTIONS.write().push(action);
                save_to_storage();
                return;
            }
        }
    }
}

pub fn remove_plant_from_bed(bed_id: &str, placed_plant_id: &str, date: &str) {
    let now = chrono::Utc::now().timestamp_millis();

    let mut beds = BEDS.write();
    if let Some(bed) = beds.iter_mut().find(|b| b.base.id == bed_id) {
        if let Some(idx) = bed.plants.iter().position(|p| p.base.id == placed_plant_id) {
            let plant_id = bed.plants[idx].plant_id.clone();
            bed.plants.remove(idx);
            bed.base.updated_at = now;
            drop(beds);

            let action = PlotAction {
                base: SyncableEntity {
                    id: uuid::Uuid::new_v4().to_string(),
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                },
                bed_id: bed_id.into(),
                plant_id,
                action: PlotActionType::Removed,
                quantity: 1,
                date: date.into(),
            };
            PLOT_ACTIONS.write().push(action);
            save_to_storage();
        }
    }
}

pub fn get_garden_beds(garden_id: &str) -> Vec<Bed> {
    BEDS.read()
        .iter()
        .filter(|b| b.garden_id == garden_id && b.base.deleted_at.is_none())
        .cloned()
        .collect()
}

pub fn get_bed_by_id(bed_id: &str) -> Option<Bed> {
    BEDS.read().iter().find(|b| b.base.id == bed_id).cloned()
}

pub fn get_plant_by_id(plant_id: &str) -> Option<Plant> {
    PLANTS
        .read()
        .iter()
        .find(|p| p.base.id == plant_id)
        .cloned()
}

pub fn get_plot_actions_by_bed(bed_id: &str) -> Vec<PlotAction> {
    PLOT_ACTIONS
        .read()
        .iter()
        .filter(|a| a.bed_id == bed_id && a.base.deleted_at.is_none())
        .cloned()
        .collect()
}

pub fn update_bed_position(bed_id: &str, x: i32, y: i32) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut beds = BEDS.write();
    if let Some(bed) = beds.iter_mut().find(|b| b.base.id == bed_id) {
        bed.x = Some(x);
        bed.y = Some(y);
        bed.base.updated_at = now;
    }
    save_to_storage();
}

pub fn delete_bed(bed_id: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut beds = BEDS.write();
    if let Some(bed) = beds.iter_mut().find(|b| b.base.id == bed_id) {
        bed.base.deleted_at = Some(now);
    }
    save_to_storage();
}

pub fn create_task(title: &str, date: &str, task_type: TaskType) -> Task {
    let now = chrono::Utc::now().timestamp_millis();
    let task = Task {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        title: title.into(),
        date: date.into(),
        r#type: task_type,
        completed: false,
    };
    TASKS.write().push(task.clone());
    save_to_storage();
    task
}

pub fn toggle_task(id: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut tasks = TASKS.write();
    if let Some(task) = tasks.iter_mut().find(|t| t.base.id == id) {
        task.completed = !task.completed;
        task.base.updated_at = now;
    }
    save_to_storage();
}

pub fn delete_task(id: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut tasks = TASKS.write();
    if let Some(task) = tasks.iter_mut().find(|t| t.base.id == id) {
        task.base.deleted_at = Some(now);
    }
    save_to_storage();
}

pub fn update_task(id: &str, title: &str, date: &str, task_type: TaskType) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut tasks = TASKS.write();
    if let Some(task) = tasks.iter_mut().find(|t| t.base.id == id) {
        task.title = title.into();
        task.date = date.into();
        task.r#type = task_type;
        task.base.updated_at = now;
    }
    save_to_storage();
}

pub fn get_tasks_by_type(task_type: TaskType) -> Vec<Task> {
    TASKS
        .read()
        .iter()
        .filter(|t| t.r#type == task_type && t.base.deleted_at.is_none())
        .cloned()
        .collect()
}

pub fn get_tasks_by_status(completed: bool) -> Vec<Task> {
    TASKS
        .read()
        .iter()
        .filter(|t| t.completed == completed && t.base.deleted_at.is_none())
        .cloned()
        .collect()
}

pub fn create_event(
    title: &str,
    date: &str,
    event_type: TaskType,
    plant_id: Option<String>,
) -> CalendarEvent {
    let now = chrono::Utc::now().timestamp_millis();
    let event = CalendarEvent {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        title: title.into(),
        date: date.into(),
        r#type: event_type,
        plant_id,
    };
    EVENTS.write().push(event.clone());
    save_to_storage();
    event
}

pub fn delete_event(id: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut events = EVENTS.write();
    if let Some(event) = events.iter_mut().find(|e| e.base.id == id) {
        event.base.deleted_at = Some(now);
    }
    save_to_storage();
}

pub fn update_event(
    id: &str,
    title: &str,
    date: &str,
    event_type: TaskType,
    plant_id: Option<String>,
) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut events = EVENTS.write();
    if let Some(event) = events.iter_mut().find(|e| e.base.id == id) {
        event.title = title.into();
        event.date = date.into();
        event.r#type = event_type;
        event.plant_id = plant_id;
        event.base.updated_at = now;
    }
    save_to_storage();
}

pub fn create_journal_entry(date: &str, content: &str) -> JournalEntry {
    let now = chrono::Utc::now().timestamp_millis();
    let entry = JournalEntry {
        base: SyncableEntity {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        },
        date: date.into(),
        content: content.into(),
    };
    JOURNAL.write().push(entry.clone());
    save_to_storage();
    entry
}

pub fn update_journal_entry(id: &str, content: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut journal = JOURNAL.write();
    if let Some(entry) = journal.iter_mut().find(|e| e.base.id == id) {
        entry.content = content.into();
        entry.base.updated_at = now;
    }
    save_to_storage();
}

pub fn delete_journal_entry(id: &str) {
    let now = chrono::Utc::now().timestamp_millis();
    let mut journal = JOURNAL.write();
    if let Some(entry) = journal.iter_mut().find(|e| e.base.id == id) {
        entry.base.deleted_at = Some(now);
    }
    save_to_storage();
}

// Dashboard-specific structures for Stitch design
#[derive(Clone, Debug, PartialEq)]
pub enum GardenStatus {
    Active,
    NeedsWater,
    Harvestable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DemoGarden {
    pub id: uuid::Uuid,
    pub name: String,
    pub bed_count: usize,
    pub plant_count: usize,
    pub status: GardenStatus,
    pub last_activity: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stats {
    pub total_gardens: usize,
    pub total_plants: usize,
    pub upcoming_tasks: usize,
    pub recent_harvests: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DemoTask {
    pub id: uuid::Uuid,
    pub title: String,
    pub time: String,
    pub completed: bool,
}
