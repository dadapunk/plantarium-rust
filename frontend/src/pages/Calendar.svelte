<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { CalendarEvent, TaskType, Plant, JournalEntry } from '../types';

  let currentDate = $state(new Date());
  let events = $state<CalendarEvent[]>([]);
  let journal = $state<JournalEntry[]>([]);
  let plants = $state<Plant[]>([]);
  let showAddEvent = $state(false);
  let newEventTitle = $state('');
  let newEventType = $state<TaskType>('custom');
  let newEventDate = $state('');
  let newEventPlantId = $state<string>('');
  let plantFilter = $state<string>('all');

  $effect(() => {
    const unsubEvents = store.events.subscribe(e => events = e);
    const unsubJournal = store.journal.subscribe(j => journal = j);
    const unsubPlants = store.plots.subscribe(() => {
      store.plots.subscribe(p => {
        const allPlants: Plant[] = [];
        store.plants.subscribe(plt => {
          p.forEach(plot => {
            plot.plants.forEach(placed => {
              const plant = plt.find(x => x.id === placed.plantId);
              if (plant && !allPlants.find(x => x.id === plant.id)) {
                allPlants.push(plant);
              }
            });
          });
          plants = allPlants;
        })();
      });
    });
    return () => {
      unsubEvents();
      unsubJournal();
      unsubPlants();
    };
  });

  function getDaysInMonth(date: Date): Date[] {
    const year = date.getFullYear();
    const month = date.getMonth();
    const firstDay = new Date(year, month, 1);
    const lastDay = new Date(year, month + 1, 0);
    
    const days: Date[] = [];
    
    const startDayOfWeek = firstDay.getDay();
    for (let i = startDayOfWeek - 1; i >= 0; i--) {
      days.push(new Date(year, month, -i));
    }
    
    for (let i = 1; i <= lastDay.getDate(); i++) {
      days.push(new Date(year, month, i));
    }
    
    const endDayOfWeek = lastDay.getDay();
    for (let i = 1; i < 7 - endDayOfWeek; i++) {
      days.push(new Date(year, month + 1, i));
    }
    
    return days;
  }

  function getFilteredEvents(): CalendarEvent[] {
    if (plantFilter === 'all') return events;
    return events.filter(e => e.plantId === plantFilter);
  }

  function getEventsForDay(date: Date): CalendarEvent[] {
    const dateStr = date.toISOString().split('T')[0];
    return getFilteredEvents().filter(e => e.date === dateStr);
  }

  function getJournalForDay(date: Date): JournalEntry[] {
    const dateStr = date.toISOString().split('T')[0];
    return journal.filter(e => e.date === dateStr);
  }

  function prevMonth() {
    currentDate = new Date(currentDate.getFullYear(), currentDate.getMonth() - 1, 1);
  }

  function nextMonth() {
    currentDate = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 1);
  }

  function addEvent() {
    if (!newEventTitle.trim() || !newEventDate) return;
    
    const event: CalendarEvent = {
      id: crypto.randomUUID(),
      title: newEventTitle.trim(),
      date: newEventDate,
      type: newEventType,
      plantId: newEventPlantId || undefined,
    };
    
    store.events.update(e => [...e, event]);
    newEventTitle = '';
    newEventDate = '';
    newEventPlantId = '';
    showAddEvent = false;
  }

  function deleteEvent(id: string) {
    store.events.update(e => e.filter(ev => ev.id !== id));
  }

  function getPlantName(plantId: string | undefined): string {
    if (!plantId) return '';
    const plant = plants.find(p => p.id === plantId);
    return plant ? `${plant.icon} ${plant.name}` : '';
  }

  function isToday(date: Date): boolean {
    const today = new Date();
    return date.getDate() === today.getDate() &&
           date.getMonth() === today.getMonth() &&
           date.getFullYear() === today.getFullYear();
  }

  function isCurrentMonth(date: Date): boolean {
    return date.getMonth() === currentDate.getMonth();
  }

  const monthNames = ['Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio', 
                      'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre'];
  const dayNames = ['Dom', 'Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb'];
</script>

<div class="calendar-page">
  <div class="header">
    <h1>Calendario</h1>
    <div class="header-actions">
      <button class="journal-btn" onclick={() => navigate('/journal')}>📖 Diario</button>
      <button class="add-btn" onclick={() => showAddEvent = !showAddEvent}>
        {showAddEvent ? 'Cancelar' : '+ Añadir Evento'}
      </button>
    </div>
  </div>

  <div class="filters">
    <label>
      Filtrar por planta:
      <select bind:value={plantFilter}>
        <option value="all">Todas</option>
        {#each plants as plant}
          <option value={plant.id}>{plant.icon} {plant.name}</option>
        {/each}
      </select>
    </label>
  </div>

  {#if showAddEvent}
    <div class="add-form">
      <input type="text" placeholder="Título del evento" bind:value={newEventTitle} />
      <select bind:value={newEventType}>
        <option value="sowing">Siembra</option>
        <option value="watering">Riego</option>
        <option value="harvest">Cosecha</option>
        <option value="fertilizing">Fertilizar</option>
        <option value="custom">Personalizado</option>
      </select>
      <select bind:value={newEventPlantId}>
        <option value="">Sin planta</option>
        {#each plants as plant}
          <option value={plant.id}>{plant.icon} {plant.name}</option>
        {/each}
      </select>
      <input type="date" bind:value={newEventDate} />
      <button onclick={addEvent}>Guardar</button>
    </div>
  {/if}

  <div class="calendar">
    <div class="calendar-header">
      <button onclick={prevMonth}>&lt;</button>
      <h2>{monthNames[currentDate.getMonth()]} {currentDate.getFullYear()}</h2>
      <button onclick={nextMonth}>&gt;</button>
    </div>

    <div class="weekdays">
      {#each dayNames as day}
        <div class="weekday">{day}</div>
      {/each}
    </div>

    <div class="days">
      {#each getDaysInMonth(currentDate) as day}
        <div 
          class="day" 
          class:today={isToday(day)}
          class:other-month={!isCurrentMonth(day)}
        >
          <span class="day-number">{day.getDate()}</span>
          <div class="day-events">
            {#each getJournalForDay(day) as entry}
              <div 
                class="journal-indicator" 
                onclick={() => navigate('/journal')}
                title="Ver en el diario"
              >
                📖 Nota
              </div>
            {/each}
            {#each getEventsForDay(day) as event}
              <div class="event" class:event-sowing={event.type === 'sowing'}
                   class:event-watering={event.type === 'watering'}
                   class:event-harvest={event.type === 'harvest'}
                   class:event-fertilizing={event.type === 'fertilizing'}
                   class:event-custom={event.type === 'custom'}
                   onclick={() => deleteEvent(event.id)}
                   title={getPlantName(event.plantId)}
              >
                {event.title}
                {#if event.plantId}
                  <span class="event-plant">{getPlantName(event.plantId)}</span>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .calendar-page h1 {
    color: #2d5a27;
    margin: 0;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .header-actions {
    display: flex;
    gap: 1rem;
  }

  .journal-btn {
    padding: 0.75rem 1.5rem;
    background: #8e44ad;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .add-btn {
    padding: 0.75rem 1.5rem;
    background: #2d5a27;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .filters {
    margin-bottom: 1rem;
    padding: 1rem;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  }

  .filters label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #2d5a27;
    font-weight: 500;
  }

  .filters select {
    padding: 0.5rem;
    border: 2px solid #ddd;
    border-radius: 6px;
  }

  .add-form {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    padding: 1.5rem;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    flex-wrap: wrap;
  }

  .add-form input, .add-form select {
    padding: 0.75rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
  }

  .add-form button {
    padding: 0.75rem 1.5rem;
    background: #4a7c44;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .calendar {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    overflow: hidden;
  }

  .calendar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    background: #2d5a27;
    color: white;
  }

  .calendar-header button {
    background: rgba(255,255,255,0.2);
    border: none;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
  }

  .calendar-header h2 {
    margin: 0;
  }

  .weekdays {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    background: #e8f5e9;
    padding: 0.5rem 0;
  }

  .weekday {
    text-align: center;
    padding: 0.5rem;
    font-weight: bold;
    color: #2d5a27;
  }

  .days {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
  }

  .day {
    min-height: 100px;
    border: 1px solid #eee;
    padding: 0.5rem;
    background: white;
  }

  .day.today {
    background: #fffde7;
  }

  .day.other-month {
    background: #f5f5f5;
    color: #999;
  }

  .day-number {
    font-weight: bold;
    font-size: 0.9rem;
  }

  .day-events {
    margin-top: 0.25rem;
  }

  .journal-indicator {
    font-size: 0.65rem;
    padding: 0.25rem 0.5rem;
    background: #f3e5f5;
    color: #7b1fa2;
    border-radius: 4px;
    margin-bottom: 0.25rem;
    cursor: pointer;
    text-align: center;
  }

  .journal-indicator:hover {
    background: #e1bee7;
  }

  .event {
    font-size: 0.7rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    margin-bottom: 0.25rem;
    cursor: pointer;
  }

  .event-plant {
    display: block;
    font-size: 0.65rem;
    opacity: 0.8;
  }

  .event-sowing { background: #e8f5e9; color: #2e7d32; }
  .event-watering { background: #e3f2fd; color: #1565c0; }
  .event-harvest { background: #fff3e0; color: #ef6c00; }
  .event-fertilizing { background: #f3e5f5; color: #7b1fa2; }
  .event-custom { background: #eceff1; color: #546e7a; }
</style>
