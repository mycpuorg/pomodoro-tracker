<script>
  import { onMount } from 'svelte';
  import PomodoroTimer from './lib/components/PomodoroTimer.svelte';
  import DailyView from './lib/components/DailyView.svelte';
  import Analytics from './lib/components/Analytics.svelte';
  
  // Tab constants
  const TIMER_TAB = 'timer';
  const DAILY_TAB = 'daily';
  const ANALYTICS_TAB = 'analytics';
  
  let activeTab = TIMER_TAB;
  let isMobileMenuOpen = false;
  
  function switchTab(tab) {
    activeTab = tab;
    isMobileMenuOpen = false;
  }
  
  function toggleMobileMenu() {
    isMobileMenuOpen = !isMobileMenuOpen;
  }
</script>

<main>
  <header>
    <div class="header-content">
      <div class="logo-container">
        <img src="./assets/logo.png" alt="Pomodoro Tracker Logo" class="app-logo" />
        <h1>Pomodoro Tracker</h1>
      </div>
      
      <!-- Mobile menu button -->
      <button class="mobile-menu-button" on:click={toggleMobileMenu} aria-label="Menu">
        <svg viewBox="0 0 100 80" width="24" height="24">
          <rect width="100" height="15" rx="8"></rect>
          <rect y="30" width="100" height="15" rx="8"></rect>
          <rect y="60" width="100" height="15" rx="8"></rect>
        </svg>
      </button>
      
      <!-- Desktop navigation -->
      <nav class="desktop-nav">
        <button 
          class:active={activeTab === TIMER_TAB} 
          on:click={() => switchTab(TIMER_TAB)}
        >
          Timer
        </button>
        <button 
          class:active={activeTab === DAILY_TAB} 
          on:click={() => switchTab(DAILY_TAB)}
        >
          Daily View
        </button>
        <button 
          class:active={activeTab === ANALYTICS_TAB} 
          on:click={() => switchTab(ANALYTICS_TAB)}
        >
          Analytics
        </button>
      </nav>
    </div>
    
    <!-- Mobile navigation -->
    <nav class="mobile-nav" class:open={isMobileMenuOpen}>
      <button 
        class:active={activeTab === TIMER_TAB} 
        on:click={() => switchTab(TIMER_TAB)}
      >
        Timer
      </button>
      <button 
        class:active={activeTab === DAILY_TAB} 
        on:click={() => switchTab(DAILY_TAB)}
      >
        Daily View
      </button>
      <button 
        class:active={activeTab === ANALYTICS_TAB} 
        on:click={() => switchTab(ANALYTICS_TAB)}
      >
        Analytics
      </button>
    </nav>
  </header>
  
  <div class="content">
    {#if activeTab === TIMER_TAB}
      <PomodoroTimer />
    {:else if activeTab === DAILY_TAB}
      <DailyView />
    {:else if activeTab === ANALYTICS_TAB}
      <Analytics />
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 
      Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    background-color: #f9f9f9;
    color: #333;
  }
  
  :global(*) {
    box-sizing: border-box;
  }
  
  main {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  header {
    background-color: #ffffff;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
    position: sticky;
    top: 0;
    z-index: 10;
  }
  
  .header-content {
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .logo-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  
  .app-logo {
    height: 32px;
    width: 32px;
  }
  
  header h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #3498db;
  }
  
  .desktop-nav {
    display: flex;
    gap: 1rem;
  }
  
  nav button {
    background: none;
    border: none;
    padding: 0.5rem 1rem;
    cursor: pointer;
    font-size: 1rem;
    border-radius: 0.25rem;
    color: #555;
  }
  
  nav button.active {
    background-color: #3498db;
    color: white;
  }
  
  .mobile-menu-button {
    display: none;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
  }
  
  .mobile-menu-button svg rect {
    fill: #555;
  }
  
  .mobile-nav {
    display: none;
    flex-direction: column;
    width: 100%;
    max-height: 0;
    overflow: hidden;
    transition: max-height 0.3s ease-out;
  }
  
  .mobile-nav.open {
    max-height: 200px;
  }
  
  .mobile-nav button {
    width: 100%;
    text-align: left;
    padding: 1rem;
    border-top: 1px solid #eee;
  }
  
  .content {
    flex: 1;
    padding: 1.5rem 1rem;
  }
  
  /* Mobile optimizations */
  @media (max-width: 768px) {
    .desktop-nav {
      display: none;
    }
    
    .mobile-menu-button {
      display: block;
    }
    
    .mobile-nav {
      display: flex;
    }
    
    .content {
      padding: 1rem 0.5rem;
    }
    
    .app-logo {
      height: 24px;
      width: 24px;
    }
    
    header h1 {
      font-size: 1.2rem;
    }
  }
</style>




