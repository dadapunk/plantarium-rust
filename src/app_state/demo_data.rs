use crate::components::{GardenData, HarvestItem, MaintenanceTask};

#[allow(dead_code)]
pub fn get_demo_gardens_v2() -> Vec<GardenData> {
    vec![
        GardenData {
            id: "demo-1".to_string(),
            name: "South Terrace".to_string(),
            status: "Active".to_string(),
            species_count: 12,
            light_exposure: "High Light Exposure".to_string(),
            tags: vec!["Moist".to_string(), "Nutrient Rich".to_string()],
            image_url: "".to_string(),
        },
        GardenData {
            id: "demo-2".to_string(),
            name: "Kitchen Herbarium".to_string(),
            status: "Harvest Ready".to_string(),
            species_count: 8,
            light_exposure: "Moderate Light Exposure".to_string(),
            tags: vec!["Culinary".to_string()],
            image_url: "".to_string(),
        },
    ]
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
