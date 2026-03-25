use crate::app_state::{DemoGarden, DemoTask, GardenStatus, Stats};
use crate::components::{GardenData, HarvestItem, MaintenanceTask};
use uuid::Uuid;

pub fn get_demo_gardens() -> Vec<DemoGarden> {
    vec![
        DemoGarden {
            id: Uuid::new_v4(),
            name: "Front Yard".to_string(),
            bed_count: 3,
            plant_count: 12,
            status: GardenStatus::Active,
            last_activity: "March 10".to_string(),
        },
        DemoGarden {
            id: Uuid::new_v4(),
            name: "Backyard Vegetables".to_string(),
            bed_count: 5,
            plant_count: 45,
            status: GardenStatus::NeedsWater,
            last_activity: "March 14".to_string(),
        },
        DemoGarden {
            id: Uuid::new_v4(),
            name: "Herb Garden".to_string(),
            bed_count: 1,
            plant_count: 8,
            status: GardenStatus::Harvestable,
            last_activity: "March 12".to_string(),
        },
    ]
}

pub fn get_demo_stats() -> Stats {
    Stats {
        total_gardens: 3,
        total_plants: 65,
        upcoming_tasks: 4,
        recent_harvests: 2,
    }
}

pub fn get_demo_tasks() -> Vec<DemoTask> {
    vec![
        DemoTask {
            id: Uuid::new_v4(),
            title: "Water Backyard Tomatoes".to_string(),
            time: "09:00 AM".to_string(),
            completed: false,
        },
        DemoTask {
            id: Uuid::new_v4(),
            title: "Prune Herb Garden Mint".to_string(),
            time: "DONE".to_string(),
            completed: true,
        },
    ]
}

// ========================================
// Dashboard V2 Demo Data
// ========================================

pub fn get_demo_gardens_v2() -> Vec<GardenData> {
    vec![
        GardenData {
            name: "South Terrace".to_string(),
            status: "Active".to_string(),
            species_count: 12,
            light_exposure: "High Light Exposure".to_string(),
            tags: vec!["Moist".to_string(), "Nutrient Rich".to_string()],
            image_url: "".to_string(),
        },
        GardenData {
            name: "Kitchen Herbarium".to_string(),
            status: "Harvest Ready".to_string(),
            species_count: 8,
            light_exposure: "Moderate Light Exposure".to_string(),
            tags: vec!["Culinary".to_string()],
            image_url: "".to_string(),
        },
    ]
}

pub fn get_demo_harvests() -> Vec<HarvestItem> {
    vec![
        HarvestItem {
            name: "Sweet Basil".to_string(),
            icon: "🌿".to_string(),
            harvested_date: "2 days ago".to_string(),
            yield_amount: "150g yield".to_string(),
        },
        HarvestItem {
            name: "Wild Mint".to_string(),
            icon: "🌱".to_string(),
            harvested_date: "yesterday".to_string(),
            yield_amount: "45g yield".to_string(),
        },
    ]
}

pub fn get_demo_maintenance_tasks() -> Vec<MaintenanceTask> {
    vec![
        MaintenanceTask {
            title: "Water Fiddle Leaf Fig".to_string(),
            location: "Living Room".to_string(),
            due_date: "Due Today".to_string(),
            is_critical: true,
            task_type: "Critical Care".to_string(),
            icon: "💧".to_string(),
        },
        MaintenanceTask {
            title: "Prune Monstera".to_string(),
            location: "South Terrace".to_string(),
            due_date: "Due Tomorrow".to_string(),
            is_critical: false,
            task_type: "Maintenance".to_string(),
            icon: "✂️".to_string(),
        },
        MaintenanceTask {
            title: "Fertilize Roses".to_string(),
            location: "Garden Bed A".to_string(),
            due_date: "In 3 days".to_string(),
            is_critical: false,
            task_type: "Nutrition".to_string(),
            icon: "🌱".to_string(),
        },
        MaintenanceTask {
            title: "Check for Pests".to_string(),
            location: "Greenhouse".to_string(),
            due_date: "In 5 days".to_string(),
            is_critical: false,
            task_type: "Inspection".to_string(),
            icon: "🔍".to_string(),
        },
    ]
}
