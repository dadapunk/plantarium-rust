<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { Garden, Bed } from '../types';
  import FreeCanvasBedLayout from './FreeCanvasBedLayout.svelte';

  interface Props {
    gardenId: string;
    containerWidth: number;
  }

  let { gardenId, containerWidth }: Props = $props();

  let garden = $state<Garden | null>(null);
  let beds = $state<Bed[]>([]);
  let scale = $state(1);

  $effect(() => {
    const unsubGardens = store.gardens.subscribe(g => {
      garden = g.find(x => x.id === gardenId) || null;
    });
    const unsubOrders = store.bedOrders.subscribe(() => {
      beds = store.getOrderedBeds(gardenId);
    });
    const unsubBeds = store.beds.subscribe(_ => {
      beds = store.getOrderedBeds(gardenId);
    });
    beds = store.getOrderedBeds(gardenId);
    return () => {
      unsubGardens();
      unsubOrders();
      unsubBeds();
    };
  });

  const minBedDistance = $derived(garden?.minBedDistance ?? 55);

  $effect(() => {
    if (beds.length === 0) {
      scale = 1;
      return;
    }

    const dims = canvasDimensions;
    const MARGIN = 20;
    const availableWidth = containerWidth - (MARGIN * 2);
    
    scale = Math.min(1, availableWidth / dims.width);
  });

  const canvasDimensions = $derived.by(() => {
    if (beds.length === 0) {
      return { width: 800, height: 400 };
    }
    
    let maxX = 0;
    let maxY = 0;
    
    beds.forEach(bed => {
      const bedX = bed.x ?? 0;
      const bedY = bed.y ?? 0;
      maxX = Math.max(maxX, bedX + bed.width);
      maxY = Math.max(maxY, bedY + bed.height);
    });
    
    const PADDING = 100;
    return { 
      width: maxX + PADDING, 
      height: maxY + PADDING 
    };
  });

  const stats = $derived(garden ? store.getGardenStats(gardenId) : { totalPlants: 0, bedsCount: 0, occupationPercent: 0 });
</script>

<div class="garden-expanded-section">
  {#if garden}
  <header class="garden-header">
    <div class="header-info">
      <h2>{garden.name}</h2>
      <div class="garden-stats">
        <span class="stat">
          <span class="stat-value">{stats.totalPlants}</span>
          <span class="stat-label">plantas</span>
        </span>
        <span class="stat">
          <span class="stat-value">{stats.bedsCount}</span>
          <span class="stat-label">bancales</span>
        </span>
      </div>
    </div>
    <button class="manage-btn" onclick={() => navigate(`/garden/${gardenId}`)}>
      Gestionar
    </button>
  </header>

  <FreeCanvasBedLayout 
    {beds} 
    {scale}
    canvasWidth={canvasDimensions.width}
    canvasHeight={canvasDimensions.height}
    {minBedDistance}
  />

  <div 
    class="add-bed-placeholder"
    onclick={() => navigate(`/garden/${gardenId}`)}
    onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && navigate(`/garden/${gardenId}`)}
    role="button"
    tabindex="0"
  >
    <span>+ Añadir Bancal</span>
  </div>
  {/if}
</div>

<style>
  .garden-expanded-section {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    overflow: visible;
  }

  .garden-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #eee;
  }

  .header-info h2 {
    margin: 0;
    color: #2d5a27;
    font-size: 1.3rem;
  }

  .garden-stats {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .stat {
    display: flex;
    align-items: baseline;
    gap: 0.25rem;
  }

  .stat-value {
    font-weight: 600;
    color: #4a7c44;
  }

  .stat-label {
    font-size: 0.85rem;
    color: #666;
  }

  .manage-btn {
    padding: 0.5rem 1rem;
    background: #4a7c44;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background 0.2s;
  }

  .manage-btn:hover {
    background: #3d6b39;
  }

  .add-bed-placeholder {
    margin-top: 1rem;
    padding: 1rem;
    border: 2px dashed #ccc;
    border-radius: 8px;
    text-align: center;
    color: #999;
    cursor: pointer;
    transition: all 0.2s;
  }

  .add-bed-placeholder:hover {
    border-color: #4a7c44;
    color: #4a7c44;
    background: #f0f7f0;
  }
</style>
