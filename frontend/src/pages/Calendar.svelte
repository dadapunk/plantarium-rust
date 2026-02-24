<script lang="ts">
  import { store } from '../lib/store';
  import type { CalendarEvent, TaskType } from '../types';

  let currentDate = $state(new Date());
  let events = $state<CalendarEvent[]>([]);
  let showAddEvent = $state(false);
  let newEventTitle = $state('');
  let newEventType = $state<TaskType>('custom');
  let newEventDate = $state('');

  $effect(() => {
    const unsub = store.events.subscribe(e => events = e);
    return unsub;
  });

  function getDaysInMonth(date: Date): Date[] {
    const year = date.getFullYear();
    const month = date.getMonth();
    const firstDay = new Date(year, month, 1);
    const lastDay = new Date(year, month + 1, 0);
    
    const days: Date[] = [];
    
    // Add days from previous month to fill the week
    const startDayOfWeek = firstDay.getDay();
    for (let i = startDayOfWeek - 1; i >= 0; i--) {
      days.push(new Date(year, month, -i));
    }
    
    // Add days of current month
    for (let i = 1; i <= lastDay.getDate(); i++) {
      days.push(new Date(year, month, i));
    }
    
    // Add days from next month to fill the week
    const endDayOfWeek = lastDay.getDay();
    for (let i = 1; i < 7 - endDayOfWeek; i++) {
      days.push(new Date(year, month + 1, i));
    }
    
    return days;
  }

  function getEventsForDay(date: Date): CalendarEvent[] {
    const dateStr = date.toISOString().split('T')[0];
    return events.filter(e => e.date === dateStr);
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
    };
    
    store.events.update(e => [...e, event]);
    newEventTitle = '';
    newEventDate = '';
    showAddEvent = false;
  }

  function deleteEvent(id: string) {
    store.events.update(e => e.filter(ev => ev.id !== id));
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
    <button class="add-btn" onclick={() => showAddEvent = !showAddEvent}>
      {showAddEvent ? 'Cancelar' : '+ Añadir Evento'}
    </button>
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
            {#each getEventsForDay(day) as event}
              <div class="event" class:event-sowing={event.type === 'sowing'}
                   class:event-watering={event.type === 'watering'}
                   class:event-harvest={event.type === 'harvest'}
                   class:event-fertilizing={event.type === 'fertilizing'}
                   class:event-custom={event.type === 'custom'}
                   onclick={() => deleteEvent(event.id)}
              >
                {event.title}
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
    margin-bottom: 2rem;
  }

  .add-btn {
    padding: 0.75rem 1.5rem;
    background: #2d5a27;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .add-form {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    padding: 1.5rem;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
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

  .event {
    font-size: 0.7rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    margin-bottom: 0.25rem;
    cursor: pointer;
  }

  .event-sowing { background: #e8f5e9; color: #2e7d32; }
  .event-watering { background: #e3f2fd; color: #1565c0; }
  .event-harvest { background: #fff3e0; color: #ef6c00; }
  .event-fertilizing { background: #f3e5f5; color: #7b1fa2; }
  .event-custom { background: #eceff1; color: #546e7a; }
</style>
