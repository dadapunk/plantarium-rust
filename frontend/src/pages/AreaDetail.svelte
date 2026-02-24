<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { Plot, GardenArea } from '../types';

  interface Props {
    areaId: string;
  }

  let { areaId }: Props = $props();

  let area = $state<GardenArea | null>(null);
  let plots = $state<Plot[]>([]);
  let newPlotName = $state('');
  let newPlotWidth = $state(200);
  let newPlotHeight = $state(100);

  $effect(() => {
    const unsubAreas = store.areas.subscribe(a => {
      area = a.find(x => x.id === areaId) || null;
    });
    const unsubPlots = store.plots.subscribe(p => {
      plots = p.filter(x => x.areaId === areaId);
    });
    return () => {
      unsubAreas();
      unsubPlots();
    };
  });

  function addPlot() {
    if (!newPlotName.trim()) return;
    
    const plot: Plot = {
      id: crypto.randomUUID(),
      areaId,
      name: newPlotName.trim(),
      width: newPlotWidth,
      height: newPlotHeight,
      plants: [],
    };
    
    store.plots.update(p => [...p, plot]);
    newPlotName = '';
  }

  function deletePlot(id: string) {
    store.plots.update(p => p.filter(plot => plot.id !== id));
  }
</script>

<div class="area-detail">
  {#if area}
    <div class="header">
      <button class="back" onclick={() => navigate('/')}>&larr; Volver</button>
      <h1>{area.name}</h1>
    </div>
  {:else}
    <div class="header">
      <button class="back" onclick={() => navigate('/')}>&larr; Volver</button>
      <h1>Área no encontrada</h1>
    </div>
  {/if}

  <div class="add-form">
    <input 
      type="text" 
      placeholder="Nombre de la parcela..." 
      bind:value={newPlotName}
    />
    <input 
      type="number" 
      placeholder="Ancho (cm)" 
      bind:value={newPlotWidth}
      min="50"
      max="1000"
    />
    <input 
      type="number" 
      placeholder="Alto (cm)" 
      bind:value={newPlotHeight}
      min="50"
      max="1000"
    />
    <button onclick={addPlot}>+ Añadir Parcela</button>
  </div>

  {#if plots.length === 0}
    <div class="empty">
      <p>No hay parcelas en esta área. ¡Crea tu primera parcela!</p>
    </div>
  {:else}
    <div class="plots-grid">
      {#each plots as plot}
        <div class="plot-card">
          <div class="plot-preview" style="width: {plot.width}px; height: {plot.height}px;">
            {#each plot.plants as plant}
              <div 
                class="plant-marker" 
                style="left: {plant.x}px; top: {plant.y}px;"
                title={plant.plantId}
              ></div>
            {/each}
          </div>
          <h3>{plot.name}</h3>
          <p>{plot.width} x {plot.height} cm | {plot.plants.length} plantas</p>
          <div class="actions">
            <button onclick={() => navigate(`/plot/${plot.id}`)}>Editar Layout</button>
            <button class="delete" onclick={() => deletePlot(plot.id)}>Eliminar</button>
          </div>
        </div>
      {/each}
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

  .plots-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .plot-card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .plot-preview {
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

  .plot-card h3 {
    margin: 0 0 0.5rem 0;
    color: #2d5a27;
  }

  .plot-card p {
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
