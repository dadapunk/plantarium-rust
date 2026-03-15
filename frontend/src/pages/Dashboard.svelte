<script lang="ts">
  import { store } from '../lib/store';
  import type { Garden } from '../types';
  import GardenPreview from '../components/GardenPreview.svelte';
  import ExpandedGardenSection from '../components/ExpandedGardenSection.svelte';

  let newGardenName = $state('');
  let gardens = $state<Garden[]>([]);
  let viewMode = $state<'compact' | 'expanded'>('expanded');
  let containerWidth = $state(800);

  $effect(() => {
    const unsub = store.gardens.subscribe(v => gardens = v);
    return unsub;
  });

  function addGarden() {
    if (!newGardenName.trim()) return;
    
    const garden: Garden = {
      id: crypto.randomUUID(),
      name: newGardenName.trim(),
      createdAt: Date.now(),
      updatedAt: Date.now(),
      deletedAt: null,
    };
    
    store.gardens.update(g => [...g, garden]);
    newGardenName = '';
  }
</script>

<div class="dashboard">
  <header class="dashboard-header">
    <h1>Jardines</h1>
    <div class="view-toggle">
      <button 
        class:active={viewMode === 'compact'}
        onclick={() => viewMode = 'compact'}
      >
        Vista Compacta
      </button>
      <button 
        class:active={viewMode === 'expanded'}
        onclick={() => viewMode = 'expanded'}
      >
        Vista Expandida
      </button>
    </div>
  </header>
  
  <div class="add-form">
    <input 
      type="text" 
      placeholder="Nombre del jardín..." 
      bind:value={newGardenName}
      onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && addGarden()}
    />
    <button onclick={addGarden}>+ Añadir Jardín</button>
  </div>

  {#if gardens.length === 0}
    <div class="empty">
      <p>No hay jardines todavía</p>
      <p>¡Crea tu primer jardín!</p>
    </div>
  {:else if viewMode === 'expanded'}
    <div class="expanded-view" bind:clientWidth={containerWidth}>
      {#each gardens as garden}
        <ExpandedGardenSection gardenId={garden.id} {containerWidth} />
      {/each}
    </div>
  {:else}
    <div class="gardens-grid">
      {#each gardens as garden}
        <GardenPreview {garden} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .dashboard {
    max-width: 1400px;
    margin: 0 auto;
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .dashboard-header h1 {
    color: #2d5a27;
    margin: 0;
  }

  .view-toggle {
    display: flex;
    gap: 0.5rem;
    background: #f0f0f0;
    padding: 4px;
    border-radius: 8px;
  }

  .view-toggle button {
    padding: 0.5rem 1rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 6px;
    font-size: 0.9rem;
    color: #666;
    transition: all 0.2s;
  }

  .view-toggle button:hover {
    color: #2d5a27;
  }

  .view-toggle button.active {
    background: white;
    color: #2d5a27;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
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

  .gardens-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .expanded-view {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
