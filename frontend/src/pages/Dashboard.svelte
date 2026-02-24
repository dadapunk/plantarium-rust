<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { GardenArea } from '../types';

  let newAreaName = $state('');
  let areas = $state<GardenArea[]>([]);

  $effect(() => {
    const unsub = store.areas.subscribe(v => areas = v);
    return unsub;
  });

  function addArea() {
    if (!newAreaName.trim()) return;
    
    const area: GardenArea = {
      id: crypto.randomUUID(),
      name: newAreaName.trim(),
      createdAt: Date.now(),
    };
    
    store.areas.update(a => [...a, area]);
    newAreaName = '';
  }

  function deleteArea(id: string) {
    store.areas.update(a => a.filter(area => area.id !== id));
    store.plots.update(p => p.filter(plot => plot.areaId !== id));
  }
</script>

<div class="dashboard">
  <h1>Mis Áreas de Jardín</h1>
  
  <div class="add-form">
    <input 
      type="text" 
      placeholder="Nombre del área..." 
      bind:value={newAreaName}
    />
    <button onclick={addArea}>+ Añadir Área</button>
  </div>

  {#if areas.length === 0}
    <div class="empty">
      <p>No hay áreas todavía. ¡Crea tu primera área de jardín!</p>
    </div>
  {:else}
    <div class="areas-grid">
      {#each areas as area}
        <div class="area-card">
          <h3>{area.name}</h3>
          <p>Creada: {new Date(area.createdAt).toLocaleDateString()}</p>
          <div class="actions">
            <button onclick={() => navigate(`/area/${area.id}`)}>Ver Parcelas</button>
            <button class="delete" onclick={() => deleteArea(area.id)}>Eliminar</button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dashboard h1 {
    color: #2d5a27;
    margin-bottom: 2rem;
  }

  .add-form {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .add-form input {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
  }

  .add-form button {
    padding: 0.75rem 1.5rem;
    background: #2d5a27;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    cursor: pointer;
  }

  .empty {
    text-align: center;
    padding: 3rem;
    color: #666;
  }

  .areas-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
  }

  .area-card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .area-card h3 {
    margin: 0 0 0.5rem 0;
    color: #2d5a27;
  }

  .area-card p {
    color: #666;
    margin-bottom: 1rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .actions button {
    flex: 1;
    padding: 0.5rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    background: #4a7c44;
    color: white;
  }

  .actions button.delete {
    background: #c0392b;
  }
</style>
