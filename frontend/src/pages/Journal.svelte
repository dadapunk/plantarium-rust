<script lang="ts">
  import { store } from '../lib/store';
  import { navigate } from '../lib/router';
  import type { JournalEntry, Plot, Plant, GardenArea } from '../types';

  let journal = $state<JournalEntry[]>([]);
  let plots = $state<Plot[]>([]);
  let plants = $state<Plant[]>([]);
  let areas = $state<GardenArea[]>([]);
  
  let showAddEntry = $state(false);
  let editingEntryId = $state<string | null>(null);
  let newDate = $state(new Date().toISOString().split('T')[0]);
  let newContent = $state('');

  $effect(() => {
    const unsubJournal = store.journal.subscribe(j => journal = j);
    const unsubPlots = store.plots.subscribe(p => plots = p);
    const unsubPlants = store.plants.subscribe(p => plants = p);
    const unsubAreas = store.areas.subscribe(a => areas = a);
    return () => {
      unsubJournal();
      unsubPlots();
      unsubPlants();
      unsubAreas();
    };
  });

  let sortedEntries = $derived(
    [...journal].sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
  );

  function addEntry() {
    if (!newContent.trim() || !newDate) return;
    
    const entry: JournalEntry = {
      id: crypto.randomUUID(),
      date: newDate,
      content: newContent.trim(),
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };
    
    store.journal.update(j => [...j, entry]);
    newContent = '';
    newDate = new Date().toISOString().split('T')[0];
    showAddEntry = false;
  }

  function updateEntry(id: string) {
    if (!newContent.trim()) return;
    store.journal.update(j => j.map(entry => 
      entry.id === id 
        ? { ...entry, content: newContent.trim(), updatedAt: Date.now() }
        : entry
    ));
    editingEntryId = null;
    newContent = '';
  }

  function deleteEntry(id: string) {
    store.journal.update(j => j.filter(entry => entry.id !== id));
  }

  function startEdit(entry: JournalEntry) {
    editingEntryId = entry.id;
    newContent = entry.content;
    newDate = entry.date;
  }

  function cancelEdit() {
    editingEntryId = null;
    showAddEntry = false;
    newContent = '';
    newDate = new Date().toISOString().split('T')[0];
  }

  function parseMarkdown(text: string): string {
    let result = text;
    
    result = result.replace(/^### (.+)$/gm, '<h3>$1</h3>');
    result = result.replace(/^## (.+)$/gm, '<h2>$1</h2>');
    result = result.replace(/^# (.+)$/gm, '<h1>$1</h1>');
    result = result.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
    result = result.replace(/\*(.+?)\*/g, '<em>$1</em>');
    result = result.replace(/\n/g, '<br>');
    
    result = result.replace(/\[\[parcela:(.+?)\]\]/g, (_match, name) => {
      const plot = plots.find(p => p.name.toLowerCase() === name.toLowerCase());
      if (plot) {
        return `<a href="#/area/${plot.areaId}" class="link-parcela">🏡 ${name}</a>`;
      }
      return `<span class="link-parcela">🏡 ${name}</span>`;
    });
    
    result = result.replace(/\[\[planta:(.+?)\]\]/g, (_match, name) => {
      const plant = plants.find(p => p.name.toLowerCase() === name.toLowerCase());
      if (plant) {
        return `<span class="link-planta">${plant.icon} ${name}</span>`;
      }
      return `<span class="link-planta">🌱 ${name}</span>`;
    });
    
    return result;
  }

  function getPlotName(plotId: string): string {
    const plot = plots.find(p => p.id === plotId);
    return plot ? plot.name : '';
  }

  function getAreaName(areaId: string): string {
    const area = areas.find(a => a.id === areaId);
    return area ? area.name : '';
  }
</script>

<div class="journal-page">
  <div class="header">
    <h1>Diario de la Huerta</h1>
    <button class="add-btn" onclick={() => showAddEntry = !showAddEntry}>
      {showAddEntry ? 'Cancelar' : '+ Nueva Nota'}
    </button>
  </div>

  {#if showAddEntry || editingEntryId}
    <div class="editor">
      <div class="editor-header">
        <input type="date" bind:value={newDate} />
        <div class="help-text">
          Usa [[parcela:nombre]] o [[planta:nombre]] para crear enlaces
        </div>
      </div>
      <textarea 
        bind:value={newContent} 
        placeholder="# Título&#10;&#10;Escribe tu nota en markdown...&#10;&#10;Ejemplo:&#10;Hoy observé los tomates en [[parcela:norte]]&#10;Las plantas de [[planta:tomate]] están creciendo bien."
        rows="10"
      ></textarea>
      <div class="editor-actions">
        {#if editingEntryId}
          <button onclick={() => updateEntry(editingEntryId)}>Guardar Cambios</button>
          <button class="cancel" onclick={cancelEdit}>Cancelar</button>
        {:else}
          <button onclick={addEntry}>Guardar Nota</button>
          <button class="cancel" onclick={cancelEdit}>Cancelar</button>
        {/if}
      </div>
    </div>
  {/if}

  <div class="entries-list">
    {#if sortedEntries.length === 0}
      <div class="empty">
        <p>No hay notas en el diario. ¡Escribe tu primera entrada!</p>
      </div>
    {:else}
      {#each sortedEntries as entry}
        <div class="entry-card">
          <div class="entry-header">
            <span class="entry-date">{new Date(entry.date).toLocaleDateString('es-ES', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}</span>
            <div class="entry-actions">
              <button onclick={() => startEdit(entry)}>Editar</button>
              <button class="delete" onclick={() => deleteEntry(entry.id)}>Eliminar</button>
            </div>
          </div>
          <div class="entry-content">
            {@html parseMarkdown(entry.content)}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .journal-page h1 {
    color: #2d5a27;
    margin: 0;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .add-btn {
    padding: 0.75rem 1.5rem;
    background: #2d5a27;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .editor {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
  }

  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .editor-header input {
    padding: 0.75rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
  }

  .help-text {
    font-size: 0.8rem;
    color: #666;
  }

  textarea {
    width: 100%;
    padding: 1rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
    font-family: monospace;
    resize: vertical;
    box-sizing: border-box;
  }

  .editor-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
  }

  .editor-actions button {
    padding: 0.75rem 1.5rem;
    background: #4a7c44;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }

  .editor-actions button.cancel {
    background: #666;
  }

  .entries-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .empty {
    text-align: center;
    padding: 3rem;
    color: #666;
  }

  .entry-card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .entry-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid #eee;
  }

  .entry-date {
    font-weight: bold;
    color: #2d5a27;
    text-transform: capitalize;
  }

  .entry-actions {
    display: flex;
    gap: 0.5rem;
  }

  .entry-actions button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    background: #4a7c44;
    color: white;
    font-size: 0.85rem;
  }

  .entry-actions button.delete {
    background: #c0392b;
  }

  .entry-content {
    line-height: 1.6;
    color: #333;
  }

  .entry-content :global(h1) {
    color: #2d5a27;
    margin: 0 0 1rem 0;
  }

  .entry-content :global(h2) {
    color: #4a7c44;
    margin: 1rem 0 0.5rem 0;
  }

  .entry-content :global(h3) {
    color: #666;
    margin: 1rem 0 0.5rem 0;
  }

  .entry-content :global(.link-parcela) {
    color: #2d5a27;
    font-weight: bold;
    text-decoration: none;
  }

  .entry-content :global(.link-planta) {
    background: #e8f5e9;
    padding: 0.125rem 0.5rem;
    border-radius: 4px;
  }
</style>
