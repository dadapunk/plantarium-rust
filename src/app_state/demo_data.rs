use crate::app_state::{DemoGarden, DemoTask, GardenStatus, Stats};
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
