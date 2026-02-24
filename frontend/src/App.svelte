<script lang="ts">
  import { location } from './lib/router';
  import Dashboard from './pages/Dashboard.svelte';
  import AreaDetail from './pages/AreaDetail.svelte';
  import Calendar from './pages/Calendar.svelte';
  import Tasks from './pages/Tasks.svelte';
  import Journal from './pages/Journal.svelte';
  import LayoutEditor from './pages/LayoutEditor.svelte';

  let currentPath = $state('/');

  $effect(() => {
    const unsub = location.subscribe(v => {
      currentPath = v;
    });
    return unsub;
  });
</script>

<nav class="navbar">
  <a href="#/">🌱 Plantarium</a>
  <div class="nav-links">
    <a href="#/">Áreas</a>
    <a href="#/calendar">Calendario</a>
    <a href="#/journal">Diario</a>
    <a href="#/tasks">Tareas</a>
  </div>
</nav>

<main>
  {#if currentPath === '/'}
    <Dashboard />
  {:else if currentPath.startsWith('/area/')}
    <AreaDetail areaId={currentPath.split('/')[2]} />
  {:else if currentPath.startsWith('/plot/')}
    <LayoutEditor plotId={currentPath.split('/')[2]} />
  {:else if currentPath === '/calendar'}
    <Calendar />
  {:else if currentPath === '/journal'}
    <Journal />
  {:else if currentPath === '/tasks'}
    <Tasks />
  {:else}
    <Dashboard />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background-color: #f5f5f5;
  }

  .navbar {
    background: #2d5a27;
    color: white;
    padding: 1rem 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .navbar a {
    color: white;
    text-decoration: none;
    margin-left: 1.5rem;
  }

  .navbar > a {
    font-size: 1.5rem;
    font-weight: bold;
    margin-left: 0;
  }

  .nav-links {
    display: flex;
  }

  main {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
</style>
