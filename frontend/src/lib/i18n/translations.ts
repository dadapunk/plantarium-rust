export type Locale = 'es' | 'en';

export interface Translations {
  // Navigation
  home: string;
  gardens: string;
  garden: string;
  beds: string;
  bed: string;
  plants: string;
  plant: string;
  calendar: string;
  journal: string;
  tasks: string;
  settings: string;
  
  // Actions
  add: string;
  edit: string;
  delete: string;
  save: string;
  cancel: string;
  confirm: string;
  
  // Garden
  addGarden: string;
  gardenName: string;
  noGardens: string;
  createFirstGarden: string;
  
  // Bed
  addBed: string;
  bedName: string;
  width: string;
  height: string;
  noBeds: string;
  createFirstBed: string;
  resize: string;
  duplicate: string;
  history: string;
  
  // Plants
  plantLibrary: string;
  selectPlant: string;
  placePlant: string;
  harvest: string;
  remove: string;
  harvested: string;
  
  // Dates
  date: string;
  actionDate: string;
  selectDate: string;
  
  // History
  plotHistory: string;
  noHistory: string;
  planted: string;
  sowed: string;
  removed: string;
  
  // Dashboard
  dashboard: string;
  quickAccess: string;
  upcomingTasks: string;
  latestJournalEntry: string;
  
  // Calendar
  dayView: string;
  weekView: string;
  monthView: string;
  yearView: string;
  
  // Journal
  addEntry: string;
  entryContent: string;
  noEntries: string;
  firstEntry: string;
  
  // Tasks
  addTask: string;
  taskTitle: string;
  taskType: string;
  completed: string;
  pending: string;
  all: string;
  
  // Task types
  sowing: string;
  watering: string;
  fertilizing: string;
  harvestTask: string;
  custom: string;
}

export const translations: Record<Locale, Translations> = {
  es: {
    // Navigation
    home: 'Inicio',
    gardens: 'Jardines',
    garden: 'Jardín',
    beds: 'Bancales',
    bed: 'Bancal',
    plants: 'Plantas',
    plant: 'Planta',
    calendar: 'Calendario',
    journal: 'Diario',
    tasks: 'Tareas',
    settings: 'Ajustes',
    
    // Actions
    add: 'Añadir',
    edit: 'Editar',
    delete: 'Eliminar',
    save: 'Guardar',
    cancel: 'Cancelar',
    confirm: 'Confirmar',
    
    // Garden
    addGarden: 'Añadir Jardín',
    gardenName: 'Nombre del jardín',
    noGardens: 'No hay jardines',
    createFirstGarden: '¡Crea tu primer jardín!',
    
    // Bed
    addBed: 'Añadir Bancal',
    bedName: 'Nombre del bancal',
    width: 'Ancho (cm)',
    height: 'Alto (cm)',
    noBeds: 'No hay bancales',
    createFirstBed: '¡Crea tu primer bancal!',
    resize: 'Redimensionar',
    duplicate: 'Duplicar',
    history: 'Histórico',
    
    // Plants
    plantLibrary: 'Biblioteca de Plantas',
    selectPlant: 'Selecciona una planta',
    placePlant: 'Haz clic en el bancal para colocarla',
    harvest: 'Cosechar',
    remove: 'Eliminar',
    harvested: 'Cosechado',
    
    // Dates
    date: 'Fecha',
    actionDate: 'Fecha de la acción',
    selectDate: 'Selecciona una fecha',
    
    // History
    plotHistory: 'Histórico del Bancal',
    noHistory: 'No hay acciones registradas',
    planted: 'Plantado',
    sowed: 'Sembrado',
    removed: 'Eliminado',
    
    // Dashboard
    dashboard: 'Panel',
    quickAccess: 'Acceso rápido',
    upcomingTasks: 'Próximas tareas',
    latestJournalEntry: 'Última nota del diario',
    
    // Calendar
    dayView: 'Día',
    weekView: 'Semana',
    monthView: 'Mes',
    yearView: 'Año',
    
    // Journal
    addEntry: 'Nueva Nota',
    entryContent: 'Contenido de la nota',
    noEntries: 'No hay notas en el diario',
    firstEntry: '¡Escribe tu primera entrada!',
    
    // Tasks
    addTask: 'Añadir Tarea',
    taskTitle: 'Título de la tarea',
    taskType: 'Tipo de tarea',
    completed: 'Completada',
    pending: 'Pendiente',
    all: 'Todas',
    
    // Task types
    sowing: 'Siembra',
    watering: 'Riego',
    fertilizing: 'Fertilización',
    harvestTask: 'Cosecha',
    custom: 'Personalizada',
  },
  en: {
    // Navigation
    home: 'Home',
    gardens: 'Gardens',
    garden: 'Garden',
    beds: 'Beds',
    bed: 'Bed',
    plants: 'Plants',
    plant: 'Plant',
    calendar: 'Calendar',
    journal: 'Journal',
    tasks: 'Tasks',
    settings: 'Settings',
    
    // Actions
    add: 'Add',
    edit: 'Edit',
    delete: 'Delete',
    save: 'Save',
    cancel: 'Cancel',
    confirm: 'Confirm',
    
    // Garden
    addGarden: 'Add Garden',
    gardenName: 'Garden name',
    noGardens: 'No gardens',
    createFirstGarden: 'Create your first garden!',
    
    // Bed
    addBed: 'Add Bed',
    bedName: 'Bed name',
    width: 'Width (cm)',
    height: 'Height (cm)',
    noBeds: 'No beds',
    createFirstBed: 'Create your first bed!',
    resize: 'Resize',
    duplicate: 'Duplicate',
    history: 'History',
    
    // Plants
    plantLibrary: 'Plant Library',
    selectPlant: 'Select a plant',
    placePlant: 'Click on the bed to place it',
    harvest: 'Harvest',
    remove: 'Remove',
    harvested: 'Harvested',
    
    // Dates
    date: 'Date',
    actionDate: 'Action date',
    selectDate: 'Select a date',
    
    // History
    plotHistory: 'Bed History',
    noHistory: 'No actions recorded',
    planted: 'Planted',
    sowed: 'Sowed',
    removed: 'Removed',
    
    // Dashboard
    dashboard: 'Dashboard',
    quickAccess: 'Quick access',
    upcomingTasks: 'Upcoming tasks',
    latestJournalEntry: 'Latest journal entry',
    
    // Calendar
    dayView: 'Day',
    weekView: 'Week',
    monthView: 'Month',
    yearView: 'Year',
    
    // Journal
    addEntry: 'New Entry',
    entryContent: 'Entry content',
    noEntries: 'No journal entries',
    firstEntry: 'Write your first entry!',
    
    // Tasks
    addTask: 'Add Task',
    taskTitle: 'Task title',
    taskType: 'Task type',
    completed: 'Completed',
    pending: 'Pending',
    all: 'All',
    
    // Task types
    sowing: 'Sowing',
    watering: 'Watering',
    fertilizing: 'Fertilizing',
    harvestTask: 'Harvest',
    custom: 'Custom',
  }
};
