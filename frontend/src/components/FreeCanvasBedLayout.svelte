<script lang="ts">
  import { store } from '../lib/store';
  import type { Bed, Plant } from '../types';
  import BedLarge from './BedLarge.svelte';

  interface Props {
    beds: Bed[];
    scale: number;
    canvasWidth: number;
    canvasHeight: number;
    minBedDistance?: number;
  }

  let { beds, scale, canvasWidth, canvasHeight, minBedDistance = 55 }: Props = $props();

  let plants = $state<Plant[]>([]);
  let draggingBed = $state<Bed | null>(null);
  let dragOffset = $state({ x: 0, y: 0 });
  let ghostPosition = $state<{ x: number; y: number } | null>(null);
  let hasCollision = $state(false);
  let canvasEl: HTMLDivElement;

  const SNAP_GRID = 10;
  const MAX_SEARCH_DISTANCE = 300;

  $effect(() => {
    const unsub = store.plants.subscribe(p => plants = p);
    return unsub;
  });

  function snapToGrid(value: number): number {
    return Math.round(value / SNAP_GRID) * SNAP_GRID;
  }

  function checkCollision(bed: Bed, x: number, y: number, excludeBedId?: string): boolean {
    return beds.some(other => {
      if (other.id === excludeBedId) return false;
      
      const otherX = other.x ?? 0;
      const otherY = other.y ?? 0;
      
      return !(
        x + bed.width + minBedDistance < otherX ||
        x > otherX + other.width + minBedDistance ||
        y + bed.height + minBedDistance < otherY ||
        y > otherY + other.height + minBedDistance
      );
    });
  }

  function findNearestFreeSpace(bed: Bed, targetX: number, targetY: number): { x: number; y: number } | null {
    const STEP = 50;
    
    for (let distance = STEP; distance <= MAX_SEARCH_DISTANCE; distance += STEP) {
      for (let angle = 0; angle < 360; angle += 45) {
        const rad = (angle * Math.PI) / 180;
        const testX = snapToGrid(targetX + Math.cos(rad) * distance);
        const testY = snapToGrid(targetY + Math.sin(rad) * distance);
        
        if (!checkCollision(bed, testX, testY, bed.id)) {
          return { x: testX, y: testY };
        }
      }
    }
    
    return null;
  }

  function handleMouseDown(e: MouseEvent, bed: Bed) {
    e.preventDefault();
    e.stopPropagation();
    
    const rect = canvasEl.getBoundingClientRect();
    const bedX = bed.x ?? 0;
    const bedY = bed.y ?? 0;
    
    draggingBed = bed;
    dragOffset = {
      x: (e.clientX - rect.left) / scale - bedX,
      y: (e.clientY - rect.top) / scale - bedY
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!draggingBed) return;
    
    const rect = canvasEl.getBoundingClientRect();
    const rawX = (e.clientX - rect.left) / scale - dragOffset.x;
    const rawY = (e.clientY - rect.top) / scale - dragOffset.y;
    
    const snappedX = snapToGrid(Math.max(0, rawX));
    const snappedY = snapToGrid(Math.max(0, rawY));
    
    ghostPosition = { x: snappedX, y: snappedY };
    hasCollision = checkCollision(draggingBed, snappedX, snappedY, draggingBed.id);
  }

  function handleMouseUp() {
    if (!draggingBed || !ghostPosition) {
      cleanup();
      return;
    }
    
    let finalPosition = ghostPosition;
    
    if (hasCollision) {
      const freeSpace = findNearestFreeSpace(draggingBed, ghostPosition.x, ghostPosition.y);
      
      if (freeSpace) {
        finalPosition = freeSpace;
      } else {
        cleanup();
        return;
      }
    }
    
    store.updateBedPosition(draggingBed.id, finalPosition.x, finalPosition.y);
    
    cleanup();
  }

  function cleanup() {
    draggingBed = null;
    dragOffset = { x: 0, y: 0 };
    ghostPosition = null;
    hasCollision = false;
    
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  }
</script>

<div 
  class="free-canvas"
  bind:this={canvasEl}
  style="width: {canvasWidth * scale}px; height: {canvasHeight * scale}px;"
>
  {#each beds as bed (bed.id)}
    {@const bedX = bed.x ?? 0}
    {@const bedY = bed.y ?? 0}
    {@const isDragging = draggingBed?.id === bed.id}
    
    <div 
      class="bed-absolute"
      class:dragging={isDragging}
      style="
        left: {bedX * scale}px; 
        top: {bedY * scale}px;
        opacity: {isDragging ? 0.3 : 1};
      "
      role="listitem"
      aria-label={`Bancal ${bed.name}`}
    >
      <div 
        class="drag-handle"
        role="button"
        aria-label={`Arrastrar ${bed.name}`}
        tabindex="0"
        onmousedown={(e) => handleMouseDown(e, bed)}
      ></div>
      
      <BedLarge 
        {bed} 
        {scale} 
        {plants}
      />
    </div>
  {/each}
  
  {#if draggingBed && ghostPosition}
    <div 
      class="bed-ghost"
      class:collision={hasCollision}
      style="
        left: {ghostPosition.x * scale}px; 
        top: {ghostPosition.y * scale}px;
      "
    >
      <BedLarge 
        bed={draggingBed} 
        {scale} 
        {plants}
      />
    </div>
  {/if}
</div>

<style>
  .free-canvas {
    position: relative;
    background: #e8f5e9;
    border: 2px dashed #4a7c44;
    border-radius: 8px;
    min-height: 400px;
    margin: 1rem 0;
    overflow: visible;
  }

  .bed-absolute {
    position: absolute;
    transition: opacity 0.2s;
  }

  .drag-handle {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    cursor: grab;
    z-index: 10;
  }

  .drag-handle:active {
    cursor: grabbing;
  }

  .bed-ghost {
    position: absolute;
    pointer-events: none;
    opacity: 0.7;
    z-index: 100;
    border: 2px solid #4a7c44;
    border-radius: 8px;
  }

  .bed-ghost.collision {
    border-color: #e74c3c;
    box-shadow: 0 0 15px rgba(231, 76, 60, 0.6);
  }
</style>
