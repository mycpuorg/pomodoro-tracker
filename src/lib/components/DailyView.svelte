<script>
  import { onMount } from 'svelte';
  import { fetchSessions } from '../stores/sessions';

  let selectedDate = new Date();
  let sessions = [];
  
  $: formattedDate = selectedDate.toLocaleDateString('en-US', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
  
  $: loadSessions(selectedDate);
  
  async function loadSessions(date) {
    const startOfDay = new Date(date);
    startOfDay.setHours(0, 0, 0, 0);
    
    const endOfDay = new Date(date);
    endOfDay.setHours(23, 59, 59, 999);
    
    sessions = await fetchSessions(
      Math.floor(startOfDay.getTime() / 1000),
      Math.floor(endOfDay.getTime() / 1000)
    );
  }
  
  function changeDate(offset) {
    const newDate = new Date(selectedDate);
    newDate.setDate(selectedDate.getDate() + offset);
    selectedDate = newDate;
  }
  
  function formatTime(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit'
    });
  }
  
  function getDuration(start, end) {
    const durationMinutes = Math.floor((end - start) / 60);
    return `${durationMinutes} min`;
  }
  
  function getCategoryColor(category) {
    const colors = {
      'Work': '#3498db',
      'Study': '#9b59b6',
      'Health': '#2ecc71',
      'Personal': '#e67e22',
      'Other': '#95a5a6'
    };
    
    return colors[category] || colors['Other'];
  }
</script>

<!-- Rest of the component stays the same -->

<div class="daily-view">
  <div class="date-selector">
    <button on:click={() => changeDate(-1)} aria-label="Previous day">&lt;</button>
    <h2>{formattedDate}</h2>
    <button on:click={() => changeDate(1)} aria-label="Next day">&gt;</button>
  </div>
  
  <div class="sessions-list">
    {#if sessions.length === 0}
      <p class="no-sessions">No Pomodoro sessions recorded for this day.</p>
    {:else}
      {#each sessions as session}
        <div class="session-card" style="border-left-color: {getCategoryColor(session.category)}">
          <div class="session-header">
            <h3>{session.task_name}</h3>
            <span class="category-tag" style="background-color: {getCategoryColor(session.category)}">
              {session.category}
            </span>
          </div>
          <div class="session-details">
            <span>{formatTime(session.start_time)} - {formatTime(session.end_time)}</span>
            <span class="duration">{getDuration(session.start_time, session.end_time)}</span>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .daily-view {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
  }
  
  .date-selector {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }
  
  .date-selector button {
    background: none;
    border: 1px solid #ddd;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    cursor: pointer;
    font-size: 1.2rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .date-selector h2 {
    flex: 1;
    text-align: center;
    margin: 0;
    font-size: clamp(1rem, 5vw, 1.5rem);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  
  .sessions-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .no-sessions {
    text-align: center;
    color: #777;
    font-style: italic;
    padding: 2rem 1rem;
  }
  
  .session-card {
    background-color: white;
    border-radius: 0.5rem;
    padding: 1rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
    border-left: 4px solid #ddd;
  }
  
  .session-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .session-header h3 {
    margin: 0;
    font-size: 1.1rem;
    word-break: break-word;
  }
  
  .category-tag {
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.8rem;
    color: white;
    white-space: nowrap;
  }
  
  .session-details {
    display: flex;
    justify-content: space-between;
    color: #777;
    font-size: 0.9rem;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .duration {
    font-weight: 500;
  }
  
  /* Mobile optimizations */
  @media (max-width: 480px) {
    .date-selector button {
      width: 36px;
      height: 36px;
      font-size: 1rem;
    }
    
    .session-header {
      flex-direction: column;
      align-items: flex-start;
    }
    
    .session-details {
      flex-direction: column;
    }
  }
</style>
