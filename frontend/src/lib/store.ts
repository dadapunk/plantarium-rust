import { writable } from 'svelte/store';
import type { GardenArea, Plot, Plant, Task, CalendarEvent } from '../types';

const STORAGE_KEY = 'plantarium_data';

function createAppStore() {
  const defaultPlants: Plant[] = [
    { id: '1', name: 'Tomate', color: '#e74c3c', icon: '🍅' },
    { id: '2', name: 'Lechuga', color: '#27ae60', icon: '🥬' },
    { id: '3', name: 'Zanahoria', color: '#e67e22', icon: '🥕' },
    { id: '4', name: 'Pimiento', color: '#c0392b', icon: '🫑' },
    { id: '5', name: 'Cebolla', color: '#8e44ad', icon: '🧅' },
    { id: '6', name: 'Ajo', color: '#f1c40f', icon: '🧄' },
    { id: '7', name: 'Papa', color: '#d35400', icon: '🥔' },
    { id: '8', name: 'Judía', color: '#16a085', icon: '🫛' },
    { id: '9', name: 'Maíz', color: '#f39c12', icon: '🌽' },
    { id: '10', name: 'Calabaza', color: '#e67e22', icon: '🎃' },
  ];

  const loadFromStorage = () => {
    if (typeof localStorage === 'undefined') return null;
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      return JSON.parse(data);
    }
    return null;
  };

  const stored = loadFromStorage();

  const areas = writable<GardenArea[]>(stored?.areas || []);
  const plots = writable<Plot[]>(stored?.plots || []);
  const plants = writable<Plant[]>(stored?.plants || defaultPlants);
  const tasks = writable<Task[]>(stored?.tasks || []);
  const events = writable<CalendarEvent[]>(stored?.events || []);

  const save = () => {
    let areasVal: GardenArea[] = [];
    let plotsVal: Plot[] = [];
    let plantsVal: Plant[] = [];
    let tasksVal: Task[] = [];
    let eventsVal: CalendarEvent[] = [];

    areas.subscribe(v => areasVal = v)();
    plots.subscribe(v => plotsVal = v)();
    plants.subscribe(v => plantsVal = v)();
    tasks.subscribe(v => tasksVal = v)();
    events.subscribe(v => eventsVal = v)();

    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      areas: areasVal,
      plots: plotsVal,
      plants: plantsVal,
      tasks: tasksVal,
      events: eventsVal,
    }));
  };

  areas.subscribe(save);
  plots.subscribe(save);
  plants.subscribe(save);
  tasks.subscribe(save);
  events.subscribe(save);

  return {
    areas,
    plots,
    plants,
    tasks,
    events,
  };
}

export const store = createAppStore();
