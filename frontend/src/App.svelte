<script lang="ts">
  import { location } from './lib/router';
  import { locale, toggleLocale } from './lib/i18n';
  import Dashboard from './pages/Dashboard.svelte';
  import GardenDetail from './pages/GardenDetail.svelte';
  import BedEditor from './pages/BedEditor.svelte';
  import Calendar from './pages/Calendar.svelte';
  import Tasks from './pages/Tasks.svelte';
  import Journal from './pages/Journal.svelte';

  let currentPath = $state('/');
  let currentLang = $state('ES');

  $effect(() => {
    const unsubLocation = location.subscribe(v => {
      currentPath = v;
    });
    const unsubLocale = locale.subscribe(v => {
      currentLang = v.toUpperCase();
    });
    return () => {
      unsubLocation();
      unsubLocale();
    };
  });
</script>

<nav class="navbar">
  <a href="#/">🌱 Plantarium</a>
  <div class="nav-links">
    <a href="#/">Jardines</a>
    <a href="#/calendar">Calendario</a>
    <a href="#/journal">Diario</a>
    <a href="#/tasks">Tareas</a>
    <button class="lang-toggle" onclick={() => toggleLocale()}>{currentLang}</button>
  </div>
</nav>

<main>
  {#if currentPath === '/'}
    <Dashboard />
  {:else if currentPath.startsWith('/garden/')}
    <GardenDetail gardenId={currentPath.split('/')[2]} />
  {:else if currentPath.startsWith('/bed/')}
    <BedEditor bedId={currentPath.split('/')[2]} />
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
    align-items: center;
  }

  .lang-toggle {
    background: rgba(255,255,255,0.2);
    border: none;
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    margin-left: 1.5rem;
    font-size: 0.8rem;
  }

  .lang-toggle:hover {
    background: rgba(255,255,255,0.3);
  }

  main {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
</style>
