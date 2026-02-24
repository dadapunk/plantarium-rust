<script lang="ts">
  import { store } from '../lib/store';
  import type { Task, TaskType } from '../types';

  let tasks = $state<Task[]>([]);
  let filter = $state<'all' | 'pending' | 'completed'>('all');
  let typeFilter = $state<TaskType | 'all'>('all');
  let showAddTask = $state(false);
  let newTaskTitle = $state('');
  let newTaskType = $state<TaskType>('custom');
  let newTaskDate = $state('');

  $effect(() => {
    const unsub = store.tasks.subscribe(t => tasks = t);
    return unsub;
  });

  let filteredTasks = $derived(() => {
    let result = [...tasks];
    
    if (filter === 'pending') {
      result = result.filter(t => !t.completed);
    } else if (filter === 'completed') {
      result = result.filter(t => t.completed);
    }
    
    if (typeFilter !== 'all') {
      result = result.filter(t => t.type === typeFilter);
    }
    
    return result.sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime());
  });

  function addTask() {
    if (!newTaskTitle.trim() || !newTaskDate) return;
    
    const task: Task = {
      id: crypto.randomUUID(),
      title: newTaskTitle.trim(),
      date: newTaskDate,
      type: newTaskType,
      completed: false,
    };
    
    store.tasks.update(t => [...t, task]);
    newTaskTitle = '';
    newTaskDate = '';
    showAddTask = false;
  }

  function toggleComplete(id: string) {
    store.tasks.update(t => t.map(task => 
      task.id === id ? { ...task, completed: !task.completed } : task
    ));
  }

  function deleteTask(id: string) {
    store.tasks.update(t => t.filter(task => task.id !== id));
  }

  function getTypeLabel(type: TaskType): string {
    const labels: Record<TaskType, string> = {
      sowing: 'Siembra',
      watering: 'Riego',
      harvest: 'Cosecha',
      fertilizing: 'Fertilizar',
      custom: 'Personalizado',
    };
    return labels[type];
  }
</script>

<div class="tasks-page">
  <div class="header">
    <h1>Tareas</h1>
    <button class="add-btn" onclick={() => showAddTask = !showAddTask}>
      {showAddTask ? 'Cancelar' : '+ Añadir Tarea'}
    </button>
  </div>

  {#if showAddTask}
    <div class="add-form">
      <input type="text" placeholder="Título de la tarea" bind:value={newTaskTitle} />
      <select bind:value={newTaskType}>
        <option value="sowing">Siembra</option>
        <option value="watering">Riego</option>
        <option value="harvest">Cosecha</option>
        <option value="fertilizing">Fertilizar</option>
        <option value="custom">Personalizado</option>
      </select>
      <input type="date" bind:value={newTaskDate} />
      <button onclick={addTask}>Guardar</button>
    </div>
  {/if}

  <div class="filters">
    <div class="filter-group">
      <label>Estado:</label>
      <select bind:value={filter}>
        <option value="all">Todos</option>
        <option value="pending">Pendientes</option>
        <option value="completed">Completadas</option>
      </select>
    </div>
    <div class="filter-group">
      <label>Tipo:</label>
      <select bind:value={typeFilter}>
        <option value="all">Todos</option>
        <option value="sowing">Siembra</option>
        <option value="watering">Riego</option>
        <option value="harvest">Cosecha</option>
        <option value="fertilizing">Fertilizar</option>
        <option value="custom">Personalizado</option>
      </select>
    </div>
  </div>

  <div class="tasks-list">
    {#if filteredTasks().length === 0}
      <div class="empty">
        <p>No hay tareas</p>
      </div>
    {:else}
      {#each filteredTasks() as task}
        <div class="task-card" class:completed={task.completed}>
          <button 
            class="checkbox" 
            class:checked={task.completed}
            onclick={() => toggleComplete(task.id)}
          >
            {task.completed ? '✓' : ''}
          </button>
          <div class="task-content">
            <span class="task-title">{task.title}</span>
            <span class="task-meta">
              <span class="task-type type-{task.type}">{getTypeLabel(task.type)}</span>
              <span class="task-date">{new Date(task.date).toLocaleDateString()}</span>
            </span>
          </div>
          <button class="delete-btn" onclick={() => deleteTask(task.id)}>×</button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .tasks-page h1 {
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

  .filters {
    display: flex;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .filter-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .filter-group label {
    font-weight: bold;
    color: #2d5a27;
  }

  .filter-group select {
    padding: 0.5rem;
    border: 2px solid #ddd;
    border-radius: 6px;
  }

  .tasks-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .empty {
    text-align: center;
    padding: 3rem;
    color: #666;
  }

  .task-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  }

  .task-card.completed {
    opacity: 0.6;
  }

  .task-card.completed .task-title {
    text-decoration: line-through;
  }

  .checkbox {
    width: 24px;
    height: 24px;
    border: 2px solid #ddd;
    border-radius: 50%;
    background: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
  }

  .checkbox.checked {
    background: #4a7c44;
    border-color: #4a7c44;
    color: white;
  }

  .task-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .task-title {
    font-size: 1rem;
  }

  .task-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.8rem;
    color: #666;
  }

  .task-type {
    padding: 0.125rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
  }

  .type-sowing { background: #e8f5e9; color: #2e7d32; }
  .type-watering { background: #e3f2fd; color: #1565c0; }
  .type-harvest { background: #fff3e0; color: #ef6c00; }
  .type-fertilizing { background: #f3e5f5; color: #7b1fa2; }
  .type-custom { background: #eceff1; color: #546e7a; }

  .delete-btn {
    width: 32px;
    height: 32px;
    border: none;
    background: #ffebee;
    color: #c62828;
    border-radius: 50%;
    cursor: pointer;
    font-size: 1.25rem;
  }
</style>
