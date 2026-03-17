<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { Bed, Garden, PlotAction, Plant } from '../types';

  interface Props {
    gardenId: string;
  }

  let { gardenId }: Props = $props();

  let garden = $state<Garden | null>(null);
  let beds = $state<Bed[]>([]);
  let newBedName = $state('');
  let newBedWidth = $state(200);
  let newBedHeight = $state(100);
  let editingBedId = $state<string | null>(null);
  let editWidth = $state(200);
  let editHeight = $state(100);
  let showHistory = $state<string | null>(null);
  let bedHistory = $state<PlotAction[]>([]);
  let bedPlantsMap = $state<Record<string, Plant>>({});
  let gardenSettings = $state({ minBedDistance: 55, bedSpacing: 60 });

  const PREVIEW_SCALE = 0.35;

  $effect(() => {
    const unsubGardens = store.gardens.subscribe(g => {
      garden = g.find(x => x.id === gardenId) || null;
    });
    const unsubBeds = store.beds.subscribe(b => {
      beds = b.filter(x => x.gardenId === gardenId);
    });
    
    // Load garden settings once
    const allGardens: Garden[] = [];
    store.gardens.subscribe(v => allGardens.push(...v))();
    const currentGarden = allGardens.find(x => x.id === gardenId);
    if (currentGarden) {
      gardenSettings = {
        minBedDistance: currentGarden.minBedDistance ?? 55,
        bedSpacing: currentGarden.bedSpacing ?? 60
      };
    }
    
    return () => {
      unsubGardens();
      unsubBeds();
    };
  });

  function updateMinBedDistance(value: number) {
    gardenSettings = { ...gardenSettings, minBedDistance: value };
    store.gardens.update(gardens => 
      gardens.map(g => g.id === gardenId ? { ...g, minBedDistance: value, updatedAt: Date.now() } : g)
    );
  }

  function updateBedSpacing(value: number) {
    gardenSettings = { ...gardenSettings, bedSpacing: value };
    store.gardens.update(gardens => 
      gardens.map(g => g.id === gardenId ? { ...g, bedSpacing: value, updatedAt: Date.now() } : g)
    );
  }

  function addBed() {
    if (!newBedName.trim()) return;
    
    const existingBeds = store.getGardenBeds(gardenId);
    const SNAP_GRID = 10;
    const spacing = gardenSettings.bedSpacing;
    
    let x = 50;
    let y = 50;
    
    if (existingBeds.length > 0) {
      const bedsWithCoords = existingBeds.filter(b => b.y !== undefined);
      
      if (bedsWithCoords.length > 0) {
        const lastBed = bedsWithCoords.reduce((max, bed) => 
          (bed.y ?? 0) > (max.y ?? 0) ? bed : max
        );
        
        x = lastBed.x ?? 50;
        y = (lastBed.y ?? 0) + lastBed.height + spacing;
      }
    }
    
    x = Math.round(x / SNAP_GRID) * SNAP_GRID;
    y = Math.round(y / SNAP_GRID) * SNAP_GRID;
    
    const bed: Bed = {
      id: crypto.randomUUID(),
      gardenId,
      name: newBedName.trim(),
      width: newBedWidth,
      height: newBedHeight,
      x,
      y,
      plants: [],
      createdAt: Date.now(),
      updatedAt: Date.now(),
      deletedAt: null,
    };
    
    store.beds.update(b => [...b, bed]);
    newBedName = '';
  }

  function deleteBed(id: string) {
    store.beds.update(b => b.filter(bed => bed.id !== id));
  }

  function duplicateBed(bed: Bed) {
    const existingBeds = store.getGardenBeds(gardenId);
    const row = Math.floor(existingBeds.length / 2);
    const col = existingBeds.length % 2;
    const x = col * 220;
    const y = row * 120;
    
    const newBed: Bed = {
      id: crypto.randomUUID(),
      gardenId,
      name: `${bed.name} (copia)`,
      width: bed.width,
      height: bed.height,
      x,
      y,
      plants: bed.plants.map(plant => ({
        ...plant,
        id: crypto.randomUUID(),
      })),
      createdAt: Date.now(),
      updatedAt: Date.now(),
      deletedAt: null,
    };
    store.beds.update(b => [...b, newBed]);
  }

  function startEditSize(bed: Bed) {
    editingBedId = bed.id;
    editWidth = bed.width;
    editHeight = bed.height;
  }

  function saveEditSize() {
    if (!editingBedId) return;
    store.beds.update(b => b.map(bed => 
      bed.id === editingBedId 
        ? { ...bed, width: editWidth, height: editHeight, updatedAt: Date.now() }
        : bed
    ));
    editingBedId = null;
  }

  function cancelEditSize() {
    editingBedId = null;
  }

  function openHistory(bedId: string) {
    bedHistory = store.getPlotActionsByBed(bedId);
    let plantsList: Plant[] = [];
    store.plants.subscribe(p => plantsList = p)();
    bedPlantsMap = {};
    plantsList.forEach(p => {
      bedPlantsMap[p.id] = p;
    });
    showHistory = bedId;
  }

  function closeHistory() {
    showHistory = null;
    bedHistory = [];
  }

  function getActionIcon(action: string): string {
    switch (action) {
      case 'planted': return '🌱';
      case 'sowed': return '🌿';
      case 'harvested': return '🧺';
      case 'removed': return '🗑️';
      default: return '📋';
    }
  }

  function getActionLabel(action: string): string {
    switch (action) {
      case 'planted': return 'Plantado';
      case 'sowed': return 'Sembrado';
      case 'harvested': return 'Cosechado';
      case 'removed': return 'Eliminado';
      default: return action;
    }
  }
</script>

<div class="garden-detail">
  {#if garden}
    <div class="header">
      <button class="back" onclick={() => navigate('/')}>&larr; Inicio</button>
      <h1>{garden.name}</h1>
    </div>
    
    <div class="garden-settings">
      <div class="setting-group">
        <label for="bed-distance">Distancia mínima entre bancales:</label>
        <input 
          type="number" 
          id="bed-distance"
          value={gardenSettings.minBedDistance}
          min="10"
          max="200"
          onchange={(e) => updateMinBedDistance(parseInt(e.currentTarget.value))}
        />
        <span>cm</span>
      </div>
      
      <div class="setting-group">
        <label for="bed-spacing">Espaciado al crear nuevos:</label>
        <input 
          type="number" 
          id="bed-spacing"
          value={gardenSettings.bedSpacing}
          min="10"
          max="200"
          onchange={(e) => updateBedSpacing(parseInt(e.currentTarget.value))}
        />
        <span>cm</span>
      </div>
    </div>
  {:else}
    <div class="header">
      <button class="back" onclick={() => navigate('/')}>&larr; Inicio</button>
      <h1>Jardín no encontrado</h1>
    </div>
  {/if}

  <div class="add-form">
    <input 
      type="text" 
      placeholder="Nombre del bancal" 
      bind:value={newBedName}
    />
    <input 
      type="number" 
      placeholder="Ancho (cm)" 
      bind:value={newBedWidth}
      min="50"
      max="1000"
    />
    <input 
      type="number" 
      placeholder="Alto (cm)" 
      bind:value={newBedHeight}
      min="50"
      max="1000"
    />
    <button onclick={addBed}>+ Añadir Bancal</button>
  </div>

  {#if beds.length === 0}
    <div class="empty">
      <p>No hay bancales</p>
      <p>¡Crea tu primer bancal!</p>
    </div>
  {:else}
    <div class="beds-grid">
      {#each beds as bed}
        <div class="bed-card">
          {#if editingBedId === bed.id}
            <div class="bed-preview" style="width: {editWidth * PREVIEW_SCALE}px; height: {editHeight * PREVIEW_SCALE}px;">
              {#each bed.plants as plant}
                <div 
                  class="plant-marker" 
                  style="left: {plant.x * PREVIEW_SCALE}px; top: {plant.y * PREVIEW_SCALE}px;"
                ></div>
              {/each}
            </div>
            <div class="size-edit">
              <input 
                type="number" 
                bind:value={editWidth}
                min="50"
                max="1000"
              /> x
              <input 
                type="number" 
                bind:value={editHeight}
                min="50"
                max="1000"
              />
              <button onclick={saveEditSize}>✓</button>
              <button onclick={cancelEditSize}>✕</button>
            </div>
          {:else}
            <div class="bed-preview" style="width: {bed.width * PREVIEW_SCALE}px; height: {bed.height * PREVIEW_SCALE}px;">
              {#each bed.plants as plant}
                <div 
                  class="plant-marker" 
                  style="left: {plant.x * PREVIEW_SCALE}px; top: {plant.y * PREVIEW_SCALE}px;"
                ></div>
              {/each}
            </div>
          {/if}
          
          <h3>{bed.name}</h3>
          <p>{bed.width} x {bed.height} cm | {bed.plants.length} plantas</p>
          
          {#if editingBedId !== bed.id}
            <div class="actions">
              <button onclick={() => navigate(`/bed/${bed.id}`)}>Editar</button>
              <button onclick={() => startEditSize(bed)}>Redimensionar</button>
              <button onclick={() => duplicateBed(bed)}>Duplicar</button>
              <button onclick={() => openHistory(bed.id)}>Histórico</button>
              <button class="delete" onclick={() => deleteBed(bed.id)}>Eliminar</button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  {#if showHistory}
    <div class="modal-overlay" onclick={closeHistory}>
      <div class="modal" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h2>Histórico del Bancal</h2>
          <button class="close-btn" onclick={closeHistory}>&times;</button>
        </div>
        <div class="modal-body">
          {#if bedHistory.length === 0}
            <p class="empty-history">No hay acciones registradas</p>
          {:else}
            <ul class="history-list">
              {#each bedHistory as action}
                <li class="history-item">
                  <span class="action-icon">{getActionIcon(action.action)}</span>
                  <div class="action-details">
                    <span class="action-type">{getActionLabel(action.action)}</span>
                    <span class="plant-name">{bedPlantsMap[action.plantId]?.name || 'Planta desconocida'}</span>
                    <span class="action-date">{new Date(action.date).toLocaleDateString('es-ES')}</span>
                  </div>
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .back {
    padding: 0.5rem 1rem;
    background: #666;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
  }

  h1 {
    color: #2d5a27;
    margin: 0;
  }

  .add-form {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    flex-wrap: wrap;
  }

  .add-form input {
    padding: 0.75rem 1rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
    min-width: 150px;
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

  .beds-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .bed-card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .bed-preview {
    background: #e8f5e9;
    border: 2px dashed #4a7c44;
    border-radius: 8px;
    margin-bottom: 1rem;
    position: relative;
    overflow: hidden;
  }

  .plant-marker {
    position: absolute;
    width: 12px;
    height: 12px;
    background: #2d5a27;
    border-radius: 50%;
    transform: translate(-50%, -50%);
  }

  .size-edit {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .size-edit input {
    width: 70px;
    padding: 0.5rem;
    border: 2px solid #ddd;
    border-radius: 6px;
  }

  .size-edit button {
    padding: 0.5rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    background: #4a7c44;
    color: white;
  }

  .bed-card h3 {
    margin: 0 0 0.5rem 0;
    color: #2d5a27;
  }

  .bed-card p {
    color: #666;
    margin-bottom: 1rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .actions button {
    padding: 0.5rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    background: #4a7c44;
    color: white;
    font-size: 0.85rem;
  }

  .actions button.delete {
    background: #c0392b;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow-y: auto;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .modal-header h2 {
    margin: 0;
    color: #2d5a27;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #666;
  }

  .empty-history {
    text-align: center;
    color: #666;
    padding: 2rem;
  }

  .history-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .history-item {
    display: flex;
    gap: 1rem;
    padding: 0.75rem;
    border-bottom: 1px solid #eee;
  }

  .action-icon {
    font-size: 1.5rem;
  }

  .action-details {
    display: flex;
    flex-direction: column;
  }

  .action-type {
    font-weight: bold;
    color: #2d5a27;
  }

  .plant-name {
    color: #666;
  }

  .action-date {
    font-size: 0.85rem;
    color: #999;
  }

  .garden-settings {
    display: flex;
    gap: 2rem;
    padding: 1rem;
    background: #f5f5f5;
    border-radius: 8px;
    margin-bottom: 1rem;
    flex-wrap: wrap;
  }

  .setting-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .setting-group label {
    font-size: 0.9rem;
    color: #666;
  }

  .setting-group input {
    width: 70px;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .setting-group span {
    font-size: 0.85rem;
    color: #999;
  }
</style>
