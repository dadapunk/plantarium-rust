<script lang="ts">
  import { store } from '../lib/store';
  import type { Bed, Plant } from '../types';
  import BedLarge from './BedLarge.svelte';

  interface Props {
    gardenId: string;
    beds: Bed[];
    scale: number;
  }

  let { gardenId, beds, scale }: Props = $props();

  let plants = $state<Plant[]>([]);
  let draggingId = $state<string | null>(null);
  let dropTargetId = $state<string | null>(null);
  let dragGhost = $state<{ bed: Bed; x: number; y: number } | null>(null);

  $effect(() => {
    const unsub = store.plants.subscribe(p => plants = p);
    return unsub;
  });

  const orderedBeds = $derived(beds);

  function handleMouseDown(e: MouseEvent, bed: Bed) {
    e.preventDefault();
    e.stopPropagation();
    draggingId = bed.id;
    dragGhost = { bed, x: e.clientX, y: e.clientY };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (dragGhost) {
      dragGhost = { ...dragGhost, x: e.clientX, y: e.clientY };
    }
  }

  function handleMouseEnter(bedId: string) {
    if (draggingId && draggingId !== bedId) {
      dropTargetId = bedId;
    }
  }

  function handleMouseLeave() {
    dropTargetId = null;
  }

  function handleMouseUp() {
    if (draggingId && dropTargetId) {
      reorderBeds(draggingId, dropTargetId);
    }
    
    draggingId = null;
    dropTargetId = null;
    dragGhost = null;
    
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  }

  function reorderBeds(fromId: string, toId: string) {
    const currentOrder = [...orderedBeds];
    const fromIndex = currentOrder.findIndex(b => b.id === fromId);
    const toIndex = currentOrder.findIndex(b => b.id === toId);
    
    if (fromIndex === -1 || toIndex === -1) return;
    
    const [removed] = currentOrder.splice(fromIndex, 1);
    currentOrder.splice(toIndex, 0, removed);
    
    const newOrder = currentOrder.map(b => b.id);
    store.saveBedOrder(gardenId, newOrder);
  }
</script>

<div class="draggable-bed-grid">
  {#each orderedBeds as bed (bed.id)}
    <div 
      class="bed-wrapper"
      class:dragging={draggingId === bed.id}
      class:drop-target={dropTargetId === bed.id}
      role="listitem"
      aria-label={`Bancal ${bed.name}`}
      onmouseenter={() => handleMouseEnter(bed.id)}
      onmouseleave={handleMouseLeave}
    >
      <div 
        class="drag-trigger"
        role="button"
        aria-label={`Arrastrar ${bed.name}`}
        tabindex="0"
        onmousedown={(e) => handleMouseDown(e, bed)}
      ></div>
      <BedLarge 
        {bed} 
        {scale} 
        {plants}
        dragging={draggingId === bed.id}
        dropTarget={dropTargetId === bed.id}
      />
    </div>
  {/each}
</div>

{#if dragGhost}
  <div 
    class="drag-ghost"
    style="left: {dragGhost.x}px; top: {dragGhost.y}px;"
  >
    <BedLarge 
      bed={dragGhost.bed} 
      {scale} 
      {plants}
    />
  </div>
{/if}

<style>
  .draggable-bed-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
    padding: 1rem 0;
    align-items: start;
  }

  .bed-wrapper {
    position: relative;
    transition: transform 0.2s, opacity 0.2s;
  }

  .bed-wrapper.dragging {
    opacity: 0.3;
  }

  .bed-wrapper.drop-target {
    transform: scale(1.02);
  }

  .drag-trigger {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    cursor: grab;
    z-index: 10;
  }

  .drag-trigger:active {
    cursor: grabbing;
  }

  .drag-ghost {
    position: fixed;
    pointer-events: none;
    z-index: 1000;
    transform: translate(-50%, -50%) rotate(3deg);
    box-shadow: 0 8px 25px rgba(0,0,0,0.3);
  }

  @media (min-width: 1200px) {
    .draggable-bed-grid {
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    }
  }

  @media (max-width: 768px) {
    .draggable-bed-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
