<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { Plot, Plant, PlacedPlant } from '../types';

  interface Props {
    plotId: string;
  }

  let { plotId }: Props = $props();

  let plot = $state<Plot | null>(null);
  let plants = $state<Plant[]>([]);
  let selectedPlantId = $state<string | null>(null);
  let dragging = $state(false);
  let canvasEl: HTMLDivElement;
  let actionDate = $state(new Date().toISOString().split('T')[0]);
  let showDatePicker = $state(false);

  $effect(() => {
    const unsubPlots = store.plots.subscribe(p => {
      plot = p.find(x => x.id === plotId) || null;
    });
    const unsubPlants = store.plants.subscribe(p => {
      plants = p;
    });
    return () => {
      unsubPlots();
      unsubPlants();
    };
  });

  function handleCanvasClick(e: MouseEvent) {
    if (!selectedPlantId || !canvasEl || !plot) return;
    
    const rect = canvasEl.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const placedPlant: PlacedPlant = {
      id: crypto.randomUUID(),
      plantId: selectedPlantId,
      x,
      y,
    };

    const updatedPlot = {
      ...plot,
      plants: [...plot.plants, placedPlant],
    };

    store.plots.update(p => p.map(pl => pl.id === plotId ? updatedPlot : pl));
    
    store.addPlotAction(plotId, selectedPlantId, 'planted', 1, actionDate);
  }

  function harvestPlant(placedPlantId: string) {
    store.harvestPlant(plotId, placedPlantId, actionDate);
  }

  function removePlant(plantId: string) {
    if (!plot) return;
    
    const placedPlant = plot.plants.find(p => p.id === plantId);
    if (placedPlant) {
      store.addPlotAction(plotId, placedPlant.plantId, 'removed', 1, actionDate);
    }
    
    const updatedPlot = {
      ...plot,
      plants: plot.plants.filter(p => p.id !== plantId),
    };
    store.plots.update(p => p.map(pl => pl.id === plotId ? updatedPlot : pl));
  }

  function getPlantInfo(plantId: string): Plant | undefined {
    return plants.find(p => p.id === plantId);
  }

  function isHarvested(placedPlant: PlacedPlant): boolean {
    return !!placedPlant.harvestedAt;
  }
</script>

<div class="layout-editor">
  <div class="header">
    <button class="back" onclick={() => navigate(`/area/${plot?.areaId}`)}>&larr; Volver</button>
    <h1>{plot?.name || 'Parcela'}</h1>
  </div>

  <div class="date-selector">
    <label for="action-date">Fecha de la acción:</label>
    <input 
      type="date" 
      id="action-date"
      bind:value={actionDate}
    />
    <span class="hint">Usa esta fecha al plantar, cosechar o eliminar plantas</span>
  </div>

  <div class="editor-container">
    <div class="plant-library">
      <h3>Biblioteca de Plantas</h3>
      <div class="plants-grid">
        {#each plants as plant}
          <button 
            class="plant-btn" 
            class:selected={selectedPlantId === plant.id}
            onclick={() => selectedPlantId = selectedPlantId === plant.id ? null : plant.id}
          >
            <span class="icon">{plant.icon}</span>
            <span class="name">{plant.name}</span>
          </button>
        {/each}
      </div>
      <p class="hint">Selecciona una planta y haz click en la parcela para colocarla</p>
    </div>

    <div class="canvas-container">
      <div 
        class="canvas"
        bind:this={canvasEl}
        onclick={handleCanvasClick}
        style="width: {plot?.width || 200}px; height: {plot?.height || 100}px;"
      >
        {#if plot}
          {#each plot.plants as placedPlant}
            {@const plantInfo = getPlantInfo(placedPlant.plantId)}
            {#if plantInfo}
              <div 
                class="placed-plant"
                class:harvested={isHarvested(placedPlant)}
                style="left: {placedPlant.x}px; top: {placedPlant.y}px; background: {plantInfo.color};"
                title={plantInfo.name}
              >
                <span class="plant-icon">{plantInfo.icon}</span>
                <div class="plant-actions">
                  {#if isHarvested(placedPlant)}
                    <span class="harvested-badge">🧺</span>
                  {:else}
                    <button class="harvest-btn" onclick={(e) => { e.stopPropagation(); harvestPlant(placedPlant.id); }} title="Cosechar">🧺</button>
                  {/if}
                  <button class="remove-btn" onclick={(e) => { e.stopPropagation(); removePlant(placedPlant.id); }} title="Eliminar">✕</button>
                </div>
              </div>
            {/if}
          {/each}
        {/if}
      </div>
    </div>
  </div>
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

  .date-selector {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
    padding: 0.75rem;
    background: #e8f5e9;
    border-radius: 8px;
  }

  .date-selector label {
    font-weight: bold;
    color: #2d5a27;
  }

  .date-selector input {
    padding: 0.5rem;
    border: 2px solid #ddd;
    border-radius: 6px;
    font-size: 1rem;
  }

  .date-selector .hint {
    font-size: 0.8rem;
    color: #666;
  }

  .editor-container {
    display: flex;
    gap: 2rem;
  }

  .plant-library {
    width: 250px;
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .plant-library h3 {
    margin: 0 0 1rem 0;
    color: #2d5a27;
  }

  .plants-grid {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .plant-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    background: white;
    cursor: pointer;
    transition: all 0.2s;
  }

  .plant-btn:hover {
    border-color: #4a7c44;
  }

  .plant-btn.selected {
    border-color: #2d5a27;
    background: #e8f5e9;
  }

  .icon {
    font-size: 1.5rem;
  }

  .name {
    font-size: 0.9rem;
  }

  .hint {
    margin-top: 1rem;
    font-size: 0.8rem;
    color: #666;
  }

  .canvas-container {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .canvas {
    background: #e8f5e9;
    border: 3px dashed #4a7c44;
    border-radius: 8px;
    position: relative;
    cursor: crosshair;
  }

  .placed-plant {
    position: absolute;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    cursor: pointer;
    transition: transform 0.2s;
  }

  .placed-plant:hover {
    transform: translate(-50%, -50%) scale(1.2);
  }

  .placed-plant.harvested {
    opacity: 0.6;
    border: 2px solid #f39c12;
  }

  .plant-icon {
    font-size: 1.2rem;
  }

  .plant-actions {
    position: absolute;
    top: -8px;
    left: 50%;
    transform: translateX(-50%);
    display: none;
    gap: 2px;
  }

  .placed-plant:hover .plant-actions {
    display: flex;
  }

  .harvest-btn, .remove-btn {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    font-size: 0.7rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .harvest-btn {
    background: #f39c12;
  }

  .remove-btn {
    background: #c0392b;
    color: white;
  }

  .harvested-badge {
    position: absolute;
    top: -10px;
    right: -10px;
    font-size: 0.8rem;
  }
</style>
