import { writable, type Writable } from 'svelte/store';
import type { Garden, Bed, Plant, Task, CalendarEvent, JournalEntry, PlotAction, PlotActionType, PlacedPlant, SyncableEntity } from '../types';

const STORAGE_KEY = 'plantarium_data_v2';

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
  const migrateBedPositions = (beds: Bed[]): Bed[] => {
    const SPACING = 60;
    const SNAP_GRID = 10;
    const COLS = 2;
    
    const byGarden = new Map<string, { nextX: number, nextY: number, col: number }>();
    
    return beds.map(bed => {
      if (bed.x !== undefined && bed.y !== undefined) {
        return bed;
      }
      
      if (!byGarden.has(bed.gardenId)) {
        byGarden.set(bed.gardenId, { nextX: 50, nextY: 50, col: 0 });
      }
      
      const state = byGarden.get(bed.gardenId)!;
      const x = Math.round(state.nextX / SNAP_GRID) * SNAP_GRID;
      const y = Math.round(state.nextY / SNAP_GRID) * SNAP_GRID;
      
      state.col++;
      if (state.col >= COLS) {
        state.col = 0;
        state.nextX = 50;
        state.nextY += bed.height + SPACING;
      } else {
        state.nextX += bed.width + SPACING;
      }
      
      return { ...bed, x, y };
    });
  };

  const loadFromStorage = () => {
    if (typeof localStorage === 'undefined') return null;
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      const parsed = JSON.parse(data);
      return {
        gardens: migrateToSyncable(parsed.gardens || []),
        beds: migrateBedPositions(migrateToSyncable(parsed.beds || [])),
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

  const gardens = writable<Garden[]>(stored?.gardens || []);
  const beds = writable<Bed[]>(stored?.beds || []);
  const plants = writable<Plant[]>(stored?.plants || defaultPlants);
  const tasks = writable<Task[]>(stored?.tasks || []);
  const events = writable<CalendarEvent[]>(stored?.events || []);
  const journal = writable<JournalEntry[]>(stored?.journal || []);
  const plotActions = writable<PlotAction[]>(stored?.plotActions || []);

  const save = () => {
    let gardensVal: Garden[] = [];
    let bedsVal: Bed[] = [];
    let plantsVal: Plant[] = [];
    let tasksVal: Task[] = [];
    let eventsVal: CalendarEvent[] = [];
    let journalVal: JournalEntry[] = [];
    let plotActionsVal: PlotAction[] = [];

    gardens.subscribe(v => gardensVal = v)();
    beds.subscribe(v => bedsVal = v)();
    plants.subscribe(v => plantsVal = v)();
    tasks.subscribe(v => tasksVal = v)();
    events.subscribe(v => eventsVal = v)();
    journal.subscribe(v => journalVal = v)();
    plotActions.subscribe(v => plotActionsVal = v)();

    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      gardens: gardensVal,
      beds: bedsVal,
      plants: plantsVal,
      tasks: tasksVal,
      events: eventsVal,
      journal: journalVal,
      plotActions: plotActionsVal,
    }));
  };

  gardens.subscribe(save);
  beds.subscribe(save);
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

  function addPlotAction(bedId: string, plantId: string, action: PlotActionType, quantity: number, date: string): PlotAction {
    const newAction: PlotAction = {
      id: crypto.randomUUID(),
      bedId,
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

  function getPlotActionsByBed(bedId: string): PlotAction[] {
    let actions: PlotAction[] = [];
    plotActions.subscribe(v => actions = v)();
    return getActive(actions).filter(a => a.bedId === bedId).sort((a, b) => 
      new Date(b.date).getTime() - new Date(a.date).getTime()
    );
  }

  function harvestPlant(bedId: string, placedPlantId: string, date: string = new Date().toISOString().split('T')[0]): void {
    const placedPlant = getPlacedPlantById(bedId, placedPlantId);
    if (!placedPlant) return;
    
    const now = createTimestamp();
    beds.update(ps => ps.map(b => {
      if (b.id === bedId) {
        return {
          ...b,
          plants: b.plants.map(plant => 
            plant.id === placedPlantId 
              ? { ...plant, harvestedAt: plant.harvestedAt || now }
              : plant
          ),
          updatedAt: createTimestamp(),
        };
      }
      return b;
    }));

    addPlotAction(bedId, placedPlant.plantId, 'harvested', 1, date);
  }

  function getPlacedPlantById(bedId: string, placedPlantId: string): PlacedPlant | undefined {
    let bs: Bed[] = [];
    beds.subscribe(v => bs = v)();
    const bed = bs.find(b => b.id === bedId);
    return bed?.plants.find(p => p.id === placedPlantId);
  }

  function getPlantById(plantId: string): Plant | undefined {
    let ps: Plant[] = [];
    plants.subscribe(v => ps = v)();
    return ps.find(p => p.id === plantId);
  }

  function getGardenBeds(gardenId: string): Bed[] {
    let allBeds: Bed[] = [];
    beds.subscribe(v => allBeds = v)();
    return getActive(allBeds).filter(b => b.gardenId === gardenId);
  }

  function getGardenStats(gardenId: string): { totalPlants: number; bedsCount: number; occupationPercent: number } {
    const gardenBeds = getGardenBeds(gardenId);
    const bedsCount = gardenBeds.length;
    let totalPlants = 0;
    let totalArea = 0;
    let usedArea = 0;

    gardenBeds.forEach(bed => {
      totalArea += bed.width * bed.height;
      usedArea += bed.plants.length * 400; // ~20x20cm por planta
      totalPlants += bed.plants.length;
    });

    const occupationPercent = totalArea > 0 ? Math.round((usedArea / totalArea) * 100) : 0;
    
    return { totalPlants, bedsCount, occupationPercent };
  }

  function getRecentGardenActivity(gardenId: string): PlotAction | null {
    const gardenBeds = getGardenBeds(gardenId);
    let allActions: PlotAction[] = [];
    plotActions.subscribe(v => allActions = v)();
    
    const gardenActions = getActive(allActions)
      .filter(a => gardenBeds.some(bed => bed.id === a.bedId))
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    
    return gardenActions[0] || null;
  }

  function getPlantTypeCounts(gardenId: string): Array<{ type: string; icon: string; count: number }> {
    const gardenBeds = getGardenBeds(gardenId);
    let allPlants: Plant[] = [];
    plants.subscribe(v => allPlants = v)();

    const counts = new Map<string, number>();
    const icons = new Map<string, string>();

    gardenBeds.forEach(bed => {
      bed.plants.forEach(placed => {
        const plant = allPlants.find(p => p.id === placed.plantId);
        if (plant) {
          counts.set(plant.name, (counts.get(plant.name) || 0) + 1);
          icons.set(plant.name, plant.icon);
        }
      });
    });

    return Array.from(counts.entries()).map(([type, count]) => ({
      type,
      icon: icons.get(type) || '🌱',
      count
    }));
  }

  const BED_ORDER_KEY = 'plantarium_bed_order';

  function loadBedOrdersFromStorage(): Record<string, string[]> {
    if (typeof localStorage === 'undefined') return {};
    const data = localStorage.getItem(BED_ORDER_KEY);
    return data ? JSON.parse(data) : {};
  }

  const bedOrders = writable<Record<string, string[]>>(loadBedOrdersFromStorage());

  bedOrders.subscribe(orders => {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(BED_ORDER_KEY, JSON.stringify(orders));
    }
  });

  function saveBedOrder(gardenId: string, bedIds: string[]): void {
    bedOrders.update(orders => ({
      ...orders,
      [gardenId]: bedIds,
    }));
  }

  function getOrderedBeds(gardenId: string): Bed[] {
    const gardenBeds = getGardenBeds(gardenId);
    let orders: Record<string, string[]> = {};
    const unsub = bedOrders.subscribe(o => orders = o);
    unsub();
    
    const order = orders[gardenId];
    
    if (!order) {
      return gardenBeds.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
    }
    
    return gardenBeds.sort((a, b) => {
      const aIndex = order.indexOf(a.id);
      const bIndex = order.indexOf(b.id);
      if (aIndex === -1 && bIndex === -1) return 0;
      if (aIndex === -1) return 1;
      if (bIndex === -1) return -1;
      return aIndex - bIndex;
    });
  }

  function updateBedPosition(bedId: string, x: number, y: number): void {
    beds.update(allBeds => 
      allBeds.map(bed => 
        bed.id === bedId 
          ? { ...bed, x, y, updatedAt: Date.now() }
          : bed
      )
    );
  }

  return {
    gardens,
    beds,
    plants,
    tasks,
    events,
    journal,
    plotActions,
    bedOrders,
    create,
    updateEntity,
    softDelete,
    getActive,
    addPlotAction,
    getPlotActionsByBed,
    harvestPlant,
    getPlacedPlantById,
    getPlantById,
    getGardenBeds,
    getGardenStats,
    getRecentGardenActivity,
    getPlantTypeCounts,
    saveBedOrder,
    getOrderedBeds,
    updateBedPosition,
  };
}

export const store = createAppStore();
