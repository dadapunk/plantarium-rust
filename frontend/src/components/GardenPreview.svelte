<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { Garden, Bed, Plant } from '../types';

  interface Props {
    garden: Garden;
  }

  let { garden }: Props = $props();

  let beds = $state<Bed[]>([]);
  let plants = $state<Plant[]>([]);
  let stats = $state({ totalPlants: 0, bedsCount: 0, occupationPercent: 0 });
  let recentActivity = $state<{ action: string; plantName: string; date: string } | null>(null);
  let plantTypes = $state<Array<{ type: string; icon: string; count: number }>>([]);

  const PREVIEW_WIDTH = 280;
  const PREVIEW_HEIGHT = 130;
  const MAX_BED_DISPLAY = 400;

  $effect(() => {
    beds = store.getGardenBeds(garden.id);
    stats = store.getGardenStats(garden.id);
    plantTypes = store.getPlantTypeCounts(garden.id);
    
    const activity = store.getRecentGardenActivity(garden.id);
    if (activity) {
      const plant = store.getPlantById(activity.plantId);
      recentActivity = {
        action: activity.action,
        plantName: plant?.name || 'Planta',
        date: activity.date
      };
    } else {
      recentActivity = null;
    }

    plants = [];
    store.plants.subscribe(p => plants = p)();
  });

  function getScaleFactors() {
    if (beds.length === 0) return { scaleX: 1, scaleY: 1, offsetX: 0, offsetY: 0 };

    let maxWidth = 0;
    let maxHeight = 0;
    let minX = Infinity;
    let minY = Infinity;

    beds.forEach(bed => {
      const bedX = bed.x ?? 0;
      const bedY = bed.y ?? 0;
      const right = bedX + bed.width;
      const bottom = bedY + bed.height;
      maxWidth = Math.max(maxWidth, right);
      maxHeight = Math.max(maxHeight, bottom);
      minX = Math.min(minX, bedX);
      minY = Math.min(minY, bedY);
    });

    const bedWidth = maxWidth - minX || 200;
    const bedHeight = maxHeight - minY || 100;

    const scaleX = (PREVIEW_WIDTH - 20) / bedWidth;
    const scaleY = (PREVIEW_HEIGHT - 20) / bedHeight;
    const scale = Math.min(scaleX, scaleY, 1);

    return {
      scaleX: scale,
      scaleY: scale,
      offsetX: 10 - minX * scale,
      offsetY: 10 - minY * scale
    };
  }

  function getBedStyle(bed: Bed) {
    const { scaleX, scaleY, offsetX, offsetY } = getScaleFactors();
    const bedX = bed.x ?? 0;
    const bedY = bed.y ?? 0;
    const width = Math.max(bed.width * scaleX, 30);
    const height = Math.max(bed.height * scaleY, 20);
    return `left: ${bedX * scaleX + offsetX}px; top: ${bedY * scaleY + offsetY}px; width: ${width}px; height: ${height}px;`;
  }

  function getPlantPosition(bed: Bed, plant: { x: number; y: number }) {
    const { scaleX, scaleY, offsetX, offsetY } = getScaleFactors();
    const bedX = bed.x ?? 0;
    const bedY = bed.y ?? 0;
    const left = (bedX + plant.x) * scaleX + offsetX;
    const top = (bedY + plant.y) * scaleY + offsetY;
    return `left: ${left}px; top: ${top}px;`;
  }

  function getPlantIcon(plantId: string): string {
    const plant = plants.find(p => p.id === plantId);
    return plant?.icon || '🌱';
  }

  function getDaysAgo(dateStr: string): string {
    const date = new Date(dateStr);
    const now = new Date();
    const diff = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));
    if (diff === 0) return 'hoy';
    if (diff === 1) return 'ayer';
    if (diff < 7) return `hace ${diff} días`;
    if (diff < 30) return `hace ${Math.floor(diff / 7)} sem`;
    return `hace ${Math.floor(diff / 30)} mes`;
  }

  function getActionLabel(action: string): string {
    const labels: Record<string, string> = {
      planted: 'Plantado',
      sowed: 'Siembra',
      harvested: 'Cosechado',
      removed: 'Eliminado'
    };
    return labels[action] || action;
  }
</script>

<div class="garden-preview">
  <div class="visual-preview">
    {#if beds.length === 0}
      <div class="empty-preview">
        <span>Sin bancales</span>
      </div>
    {:else}
      <div class="beds-container">
        {#each beds as bed}
          <div class="bed-box" style={getBedStyle(bed)}>
            {#each bed.plants.slice(0, 20) as placedPlant}
              <span class="plant-icon" style={getPlantPosition(bed, placedPlant)}>
                {getPlantIcon(placedPlant.plantId)}
              </span>
            {/each}
            {#if bed.plants.length > 20}
              <span class="more-plants">+{bed.plants.length - 20}</span>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <div class="stats-row">
    <div class="stat">
      <span class="stat-value">{stats.totalPlants}</span>
      <span class="stat-label">plantas</span>
    </div>
    <div class="stat">
      <span class="stat-value">{stats.bedsCount}</span>
      <span class="stat-label">bancales</span>
    </div>
    {#if recentActivity}
      <div class="recent-activity">
        <span class="activity-action">{getActionLabel(recentActivity.action)}</span>
        <span class="activity-name">{recentActivity.plantName}</span>
        <span class="activity-time">{getDaysAgo(recentActivity.date)}</span>
      </div>
    {/if}
  </div>

  <div class="actions-row">
    <button class="view-btn" onclick={() => navigate(`/garden/${garden.id}`)}>
      Ver Bancales
    </button>
    <button class="delete-btn" onclick={() => {
      if (confirm(`¿Eliminar "${garden.name}" y todos sus bancales?`)) {
        store.gardens.update(g => g.filter(item => item.id !== garden.id));
        store.beds.update(b => b.filter(bed => bed.gardenId !== garden.id));
      }
    }}>
      Eliminar
    </button>
  </div>
</div>

<style>
  .garden-preview {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    overflow: hidden;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .garden-preview:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  }

  .visual-preview {
    height: 130px;
    background: linear-gradient(135deg, #e8f5e9 0%, #c8e6c9 100%);
    position: relative;
    overflow: hidden;
  }

  .empty-preview {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    font-size: 0.9rem;
  }

  .beds-container {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .bed-box {
    position: absolute;
    background: rgba(255,255,255,0.7);
    border: 2px dashed #4a7c44;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .plant-icon {
    position: absolute;
    font-size: 10px;
    transform: translate(-50%, -50%);
    pointer-events: none;
  }

  .more-plants {
    position: absolute;
    font-size: 8px;
    color: #666;
    background: rgba(255,255,255,0.8);
    padding: 1px 4px;
    border-radius: 4px;
  }

  .stats-row {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: #fafafa;
    border-bottom: 1px solid #eee;
  }

  .stat {
    display: flex;
    align-items: baseline;
    gap: 0.25rem;
  }

  .stat-value {
    font-size: 1.1rem;
    font-weight: 600;
    color: #2d5a27;
  }

  .stat-label {
    font-size: 0.75rem;
    color: #666;
  }

  .recent-activity {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.75rem;
    color: #666;
    justify-content: flex-end;
  }

  .activity-action {
    color: #4a7c44;
    font-weight: 500;
  }

  .activity-name {
    color: #333;
  }

  .activity-time {
    color: #999;
  }

  .actions-row {
    display: flex;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
  }

  .view-btn, .delete-btn {
    flex: 1;
    padding: 0.5rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
    transition: background 0.2s;
  }

  .view-btn {
    background: #4a7c44;
    color: white;
  }

  .view-btn:hover {
    background: #3d6b39;
  }

  .delete-btn {
    background: #f5f5f5;
    color: #c0392b;
    max-width: 80px;
  }

  .delete-btn:hover {
    background: #ffebee;
  }
</style>
