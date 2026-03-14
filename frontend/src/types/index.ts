export interface SyncableEntity {
  id: string;
  createdAt: number;
  updatedAt: number;
  deletedAt: number | null;
}

export interface GardenArea extends SyncableEntity {
  name: string;
}

export interface PlacedPlant extends SyncableEntity {
  plantId: string;
  x: number;
  y: number;
  harvestedAt?: number;  // Timestamp de primera cosecha (para lógica no destructiva)
}

export interface Plot extends SyncableEntity {
  areaId: string;
  name: string;
  width: number;
  height: number;
  plants: PlacedPlant[];
}

export interface Plant extends SyncableEntity {
  name: string;
  color: string;
  icon: string;
  family?: string;
  species?: string;
}

export type PlotActionType = 'planted' | 'sowed' | 'harvested' | 'removed';

export interface PlotAction extends SyncableEntity {
  plotId: string;
  plantId: string;
  action: PlotActionType;
  quantity: number;
  date: string;  // Fecha de la acción (no timestamp)
}

export type TaskType = 'sowing' | 'watering' | 'harvest' | 'fertilizing' | 'custom';

export interface Task extends SyncableEntity {
  title: string;
  date: string;
  type: TaskType;
  completed: boolean;
}

export interface CalendarEvent extends SyncableEntity {
  title: string;
  date: string;
  type: TaskType;
  plantId?: string;
}

export interface JournalEntry extends SyncableEntity {
  date: string;
  content: string;
}

export interface AppState {
  areas: GardenArea[];
  plots: Plot[];
  plants: Plant[];
  tasks: Task[];
  events: CalendarEvent[];
  journal: JournalEntry[];
  plotActions: PlotAction[];
}
