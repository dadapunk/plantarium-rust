import { writable, type Writable } from 'svelte/store';
import type { GardenArea, Plot, Plant, Task, CalendarEvent, JournalEntry, PlotAction, PlotActionType, PlacedPlant, SyncableEntity } from '../types';

const STORAGE_KEY = 'plantarium_data';

function createTimestamp(): number {
  return Date.now();
}

function ensureSyncable<T extends SyncableEntity>(entity: T): T {
  return {
    ...entity,
    createdAt: entity.createdAt || createTimestamp(),
    updatedAt: entity.updatedAt || createTimestamp(),
    deletedAt: entity.deletedAt ?? null,
  };
}

function migrateToSyncable<T extends SyncableEntity>(entities: T[]): T[] {
  return entities.map(entity => ensureSyncable(entity));
}

const defaultPlants: Plant[] = [
  { id: '1', name: 'Tomate', color: '#e74c3c', icon: '🍅', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '2', name: 'Lechuga', color: '#27ae60', icon: '🥬', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '3', name: 'Zanahoria', color: '#e67e22', icon: '🥕', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '4', name: 'Pimiento', color: '#c0392b', icon: '🫑', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '5', name: 'Cebolla', color: '#8e44ad', icon: '🧅', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '6', name: 'Ajo', color: '#f1c40f', icon: '🧄', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '7', name: 'Papa', color: '#d35400', icon: '🥔', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '8', name: 'Judía', color: '#16a085', icon: '🫛', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '9', name: 'Maíz', color: '#f39c12', icon: '🌽', createdAt: 0, updatedAt: 0, deletedAt: null },
  { id: '10', name: 'Calabaza', color: '#e67e22', icon: '🎃', createdAt: 0, updatedAt: 0, deletedAt: null },
];

function createAppStore() {
  const loadFromStorage = () => {
    if (typeof localStorage === 'undefined') return null;
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      const parsed = JSON.parse(data);
      return {
        areas: migrateToSyncable(parsed.areas || []),
        plots: migrateToSyncable(parsed.plots || []),
        plants: migrateToSyncable(parsed.plants || defaultPlants),
        tasks: migrateToSyncable(parsed.tasks || []),
        events: migrateToSyncable(parsed.events || []),
        journal: migrateToSyncable(parsed.journal || []),
        plotActions: migrateToSyncable(parsed.plotActions || []),
      };
    }
    return null;
  };

  const stored = loadFromStorage();

  const areas = writable<GardenArea[]>(stored?.areas || []);
  const plots = writable<Plot[]>(stored?.plots || []);
  const plants = writable<Plant[]>(stored?.plants || defaultPlants);
  const tasks = writable<Task[]>(stored?.tasks || []);
  const events = writable<CalendarEvent[]>(stored?.events || []);
  const journal = writable<JournalEntry[]>(stored?.journal || []);
  const plotActions = writable<PlotAction[]>(stored?.plotActions || []);

  const save = () => {
    let areasVal: GardenArea[] = [];
    let plotsVal: Plot[] = [];
    let plantsVal: Plant[] = [];
    let tasksVal: Task[] = [];
    let eventsVal: CalendarEvent[] = [];
    let journalVal: JournalEntry[] = [];
    let plotActionsVal: PlotAction[] = [];

    areas.subscribe(v => areasVal = v)();
    plots.subscribe(v => plotsVal = v)();
    plants.subscribe(v => plantsVal = v)();
    tasks.subscribe(v => tasksVal = v)();
    events.subscribe(v => eventsVal = v)();
    journal.subscribe(v => journalVal = v)();
    plotActions.subscribe(v => plotActionsVal = v)();

    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      areas: areasVal,
      plots: plotsVal,
      plants: plantsVal,
      tasks: tasksVal,
      events: eventsVal,
      journal: journalVal,
      plotActions: plotActionsVal,
    }));
  };

  areas.subscribe(save);
  plots.subscribe(save);
  plants.subscribe(save);
  tasks.subscribe(save);
  events.subscribe(save);
  journal.subscribe(save);
  plotActions.subscribe(save);

  function create<T extends SyncableEntity>(store: Writable<T[]>, data: Omit<T, 'id' | 'createdAt' | 'updatedAt' | 'deletedAt'>): T {
    const newEntity = {
      ...data,
      id: crypto.randomUUID(),
      createdAt: createTimestamp(),
      updatedAt: createTimestamp(),
      deletedAt: null,
    } as T;
    store.update(entities => [...entities, newEntity]);
    return newEntity;
  }

  function updateEntity<T extends SyncableEntity>(store: Writable<T[]>, id: string, data: Partial<Omit<T, 'id' | 'createdAt' | 'deletedAt'>>): void {
    store.update(entities => 
      entities.map(entity => 
        entity.id === id 
          ? { ...entity, ...data, updatedAt: createTimestamp() }
          : entity
      )
    );
  }

  function softDelete<T extends SyncableEntity>(store: Writable<T[]>, id: string): void {
    store.update(entities =>
      entities.map(entity =>
        entity.id === id
          ? { ...entity, deletedAt: createTimestamp(), updatedAt: createTimestamp() }
          : entity
      )
    );
  }

  function getActive<T extends SyncableEntity>(entities: T[]): T[] {
    return entities.filter(e => e.deletedAt === null);
  }

  function addPlotAction(plotId: string, plantId: string, action: PlotActionType, quantity: number, date: string): PlotAction {
    const newAction: PlotAction = {
      id: crypto.randomUUID(),
      plotId,
      plantId,
      action,
      quantity,
      date,
      createdAt: createTimestamp(),
      updatedAt: createTimestamp(),
      deletedAt: null,
    };
    plotActions.update(actions => [...actions, newAction]);
    return newAction;
  }

  function getPlotActionsByPlot(plotId: string): PlotAction[] {
    let actions: PlotAction[] = [];
    plotActions.subscribe(v => actions = v)();
    return getActive(actions).filter(a => a.plotId === plotId).sort((a, b) => 
      new Date(b.date).getTime() - new Date(a.date).getTime()
    );
  }

  function harvestPlant(plotId: string, placedPlantId: string, date: string = new Date().toISOString().split('T')[0]): void {
    const placedPlant = getPlacedPlantById(plotId, placedPlantId);
    if (!placedPlant) return;
    
    const now = createTimestamp();
    plots.update(ps => ps.map(p => {
      if (p.id === plotId) {
        return {
          ...p,
          plants: p.plants.map(plant => 
            plant.id === placedPlantId 
              ? { ...plant, harvestedAt: plant.harvestedAt || now }
              : plant
          ),
          updatedAt: createTimestamp(),
        };
      }
      return p;
    }));

    addPlotAction(plotId, placedPlant.plantId, 'harvested', 1, date);
  }

  function getPlacedPlantById(plotId: string, placedPlantId: string): PlacedPlant | undefined {
    let ps: Plot[] = [];
    plots.subscribe(v => ps = v)();
    const plot = ps.find(p => p.id === plotId);
    return plot?.plants.find(p => p.id === placedPlantId);
  }

  function getPlantById(plantId: string): Plant | undefined {
    let ps: Plant[] = [];
    plants.subscribe(v => ps = v)();
    return ps.find(p => p.id === plantId);
  }

  return {
    areas,
    plots,
    plants,
    tasks,
    events,
    journal,
    plotActions,
    create,
    updateEntity,
    softDelete,
    getActive,
    addPlotAction,
    getPlotActionsByPlot,
    harvestPlant,
    getPlacedPlantById,
    getPlantById,
  };
}

export const store = createAppStore();
