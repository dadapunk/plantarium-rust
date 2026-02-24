export interface GardenArea {
  id: string;
  name: string;
  createdAt: number;
}

export interface PlacedPlant {
  id: string;
  plantId: string;
  x: number;
  y: number;
}

export interface Plot {
  id: string;
  areaId: string;
  name: string;
  width: number;
  height: number;
  plants: PlacedPlant[];
}

export interface Plant {
  id: string;
  name: string;
  color: string;
  icon: string;
}

export type TaskType = 'sowing' | 'watering' | 'harvest' | 'fertilizing' | 'custom';

export interface Task {
  id: string;
  title: string;
  date: string;
  type: TaskType;
  completed: boolean;
}

export interface CalendarEvent {
  id: string;
  title: string;
  date: string;
  type: TaskType;
  plantId?: string;
}

export interface JournalEntry {
  id: string;
  date: string;
  content: string;
  createdAt: number;
  updatedAt: number;
}

export interface AppState {
  areas: GardenArea[];
  plots: Plot[];
  plants: Plant[];
  tasks: Task[];
  events: CalendarEvent[];
  journal: JournalEntry[];
}
