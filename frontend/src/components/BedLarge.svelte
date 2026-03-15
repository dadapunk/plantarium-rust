<script lang="ts">
  import { navigate } from '../lib/router';
  import type { Bed, Plant } from '../types';

  interface Props {
    bed: Bed;
    scale: number;
    plants: Plant[];
    dragging?: boolean;
    dropTarget?: boolean;
  }

  let { bed, scale, plants, dragging = false, dropTarget = false }: Props = $props();

  function handleClick() {
    if (!dragging) {
      navigate(`/bed/${bed.id}`);
    }
  }

  function getPlantStyle(plant: { x: number; y: number }) {
    const x = plant.x * scale;
    const y = plant.y * scale;
    return `left: ${x}px; top: ${y}px;`;
  }

  function getPlantIcon(plantId: string): string {
    const plant = plants.find(p => p.id === plantId);
    return plant?.icon || '🌱';
  }
</script>

<div 
  class="bed-large"
  class:dragging
  class:drop-target={dropTarget}
  style="width: {bed.width * scale}px; height: {bed.height * scale}px;"
  onclick={handleClick}
  onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleClick()}
  role="button"
  tabindex="0"
>
  <div class="bed-name">{bed.name}</div>
  <div class="plants-visual">
    {#each bed.plants.slice(0, 50) as plant}
      <span class="plant-emoji" style={getPlantStyle(plant)}>
        {getPlantIcon(plant.plantId)}
      </span>
    {/each}
    {#if bed.plants.length > 50}
      <span class="more-indicator">+{bed.plants.length - 50}</span>
    {/if}
  </div>
  {#if bed.plants.length === 0}
    <div class="empty-bed">Sin plantas</div>
  {/if}
  <div class="drag-handle">⋮⋮</div>
</div>

<style>
  .bed-large {
    background: #f5f5f5;
    border: 2px dashed #4a7c44;
    border-radius: 8px;
    position: relative;
    cursor: pointer;
    transition: border-color 0.2s, box-shadow 0.2s;
    overflow: hidden;
    min-width: 120px;
    min-height: 60px;
    user-select: none;
  }

  .bed-large:hover {
    border-color: #2d5a27;
    box-shadow: 0 4px 12px rgba(45, 90, 39, 0.2);
  }

  .bed-large.dragging {
    opacity: 0.4;
    border-color: #999;
  }

  .bed-large.drop-target {
    border-color: #2d5a27;
    border-style: solid;
    background: #e8f5e9;
    box-shadow: 0 0 0 3px rgba(45, 90, 39, 0.3);
  }

  .bed-name {
    position: absolute;
    top: 4px;
    left: 8px;
    font-size: 11px;
    font-weight: 500;
    color: #2d5a27;
    background: rgba(255, 255, 255, 0.9);
    padding: 2px 6px;
    border-radius: 4px;
    z-index: 10;
    pointer-events: none;
  }

  .plants-visual {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  .plant-emoji {
    position: absolute;
    font-size: 16px;
    transform: translate(-50%, -50%);
    filter: drop-shadow(0 1px 1px rgba(0,0,0,0.3));
    pointer-events: none;
  }

  .empty-bed {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #aaa;
    font-size: 0.8rem;
  }

  .drag-handle {
    position: absolute;
    bottom: 2px;
    right: 4px;
    font-size: 10px;
    color: #999;
    opacity: 0.5;
    pointer-events: none;
  }

  .bed-large:hover .drag-handle {
    opacity: 1;
  }

  .more-indicator {
    position: absolute;
    bottom: 4px;
    left: 50%;
    transform: translateX(-50%);
    font-size: 10px;
    color: #666;
    background: rgba(255,255,255,0.8);
    padding: 1px 4px;
    border-radius: 4px;
  }
</style>
