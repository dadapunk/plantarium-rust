export interface SyncableEntity {
  id: string;
  createdAt: number;
  updatedAt: number;
  deletedAt: number | null;
}

export interface Garden extends SyncableEntity {
  name: string;
  minBedDistance?: number;
  bedSpacing?: number;
}

export interface PlacedPlant extends SyncableEntity {
  plantId: string;
  x: number;
  y: number;
  harvestedAt?: number;
}

export interface Bed extends SyncableEntity {
  gardenId: string;
  name: string;
  width: number;
  height: number;
  x?: number;
  y?: number;
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
  bedId: string;
  plantId: string;
  action: PlotActionType;
  quantity: number;
  date: string;
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
  gardens: Garden[];
  beds: Bed[];
  plants: Plant[];
  tasks: Task[];
  events: CalendarEvent[];
  journal: JournalEntry[];
  plotActions: PlotAction[];
}
