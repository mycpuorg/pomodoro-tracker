<!-- <script>
  import { onDestroy } from 'svelte';
  import { 
    timerState, 
    remainingTime, 
    currentTask, 
    currentCategory,
    TimerState, 
    WORK_DURATION, 
    BREAK_DURATION 
  } from '../stores/timer';
  import { saveSession } from '../stores/sessions';
  
  let timer = null;
  let startTime = 0;
  
  const categories = ['Work', 'Study', 'Health', 'Personal', 'Other'];
  
  function startTimer() {
    if ($timerState === TimerState.IDLE || $timerState === TimerState.PAUSED) {
      if ($timerState === TimerState.IDLE) {
        startTime = Date.now();
      }
      
      $timerState = TimerState.RUNNING;
      
      timer = setInterval(() => {
        $remainingTime -= 1;
        
        if ($remainingTime <= 0) {
          handleTimerComplete();
        }
      }, 1000);
    }
  }
  
  function pauseTimer() {
    if ($timerState === TimerState.RUNNING) {
      $timerState = TimerState.PAUSED;
      clearInterval(timer);
      timer = null;
    }
  }
  
  function resetTimer() {
    clearInterval(timer);
    timer = null;
    $timerState = TimerState.IDLE;
    $remainingTime = WORK_DURATION;
    startTime = 0;
  }
  
  async function handleTimerComplete() {
    clearInterval(timer);
    timer = null;
    
    if ($timerState === TimerState.RUNNING) {
      // Save completed session
      if ($timerState !== TimerState.BREAK) {
        const endTime = Date.now();
        await saveSession({
          task_name: $currentTask,
          category: $currentCategory,
          start_time: Math.floor(startTime / 1000),
          end_time: Math.floor(endTime / 1000),
        });
      }
      
      // Switch between work and break
      if ($timerState === TimerState.BREAK) {
        $timerState = TimerState.IDLE;
        $remainingTime = WORK_DURATION;
        // Notify user of work session
        new Notification('Pomodoro Timer', { 
          body: 'Break is over. Time to work!' 
        });
      } else {
        $timerState = TimerState.BREAK;
        $remainingTime = BREAK_DURATION;
        // Notify user of break
        new Notification('Pomodoro Timer', { 
          body: 'Work session complete! Time for a break.' 
        });
        startTimer(); // Automatically start break timer
      }
    }
  }
  
  function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  
  onDestroy(() => {
    if (timer) clearInterval(timer);
  });
</script> -->


<script>
  import { onDestroy } from 'svelte';
  import { 
    timerState, 
    remainingTime, 
    currentTask, 
    currentCategory,
    TimerState, 
    WORK_DURATION, 
    BREAK_DURATION 
  } from '../stores/timer';
  import { saveSession } from '../stores/sessions';
  
  let timer = null;
  let startTime = 0;
  
  const categories = ['Work', 'Study', 'Health', 'Personal', 'Other'];
  
  function startTimer() {
    if ($timerState === TimerState.IDLE || $timerState === TimerState.PAUSED) {
      if ($timerState === TimerState.IDLE) {
        startTime = Date.now();
      }
      
      $timerState = TimerState.RUNNING;
      
      timer = setInterval(() => {
        $remainingTime -= 1;
        
        if ($remainingTime <= 0) {
          handleTimerComplete();
        }
      }, 1000);
    }
  }
  
  function pauseTimer() {
    if ($timerState === TimerState.RUNNING) {
      $timerState = TimerState.PAUSED;
      clearInterval(timer);
      timer = null;
    }
  }
  
  function resetTimer() {
    clearInterval(timer);
    timer = null;
    $timerState = TimerState.IDLE;
    $remainingTime = WORK_DURATION;
    startTime = 0;
  }
  
  async function handleTimerComplete() {
    clearInterval(timer);
    timer = null;
    
    // Check if we just finished a work session
    if ($timerState === TimerState.RUNNING && $remainingTime <= 0) {
      // Save completed work session
      if ($remainingTime <= 0 && $timerState !== TimerState.BREAK) {
        const endTime = Date.now();
        await saveSession({
          task_name: $currentTask,
          category: $currentCategory,
          start_time: Math.floor(startTime / 1000),
          end_time: Math.floor(endTime / 1000),
        });
        
        // Switch to break mode
        $timerState = TimerState.BREAK;
        $remainingTime = BREAK_DURATION;
        // Notify user of break
        new Notification('Pomodoro Timer', { 
          body: 'Work session complete! Time for a break.' 
        });
      } 
      // If we just finished a break session
      else if ($remainingTime <= 0 && $timerState === TimerState.BREAK) {
        $timerState = TimerState.IDLE;
        $remainingTime = WORK_DURATION;
        // Notify user of work session
        new Notification('Pomodoro Timer', { 
          body: 'Break is over. Time to work!' 
        });
      }
    }
  }
  
  function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  
  onDestroy(() => {
    if (timer) clearInterval(timer);
  });
</script>

<!-- Rest of the component stays the same -->

<div class="timer-container">
  <h1 class="timer-display">{formatTime($remainingTime)}</h1>
  
  <div class="timer-status">
    {#if $timerState === TimerState.BREAK}
      <p class="status-text break">Break Time</p>
    {:else}
      <p class="status-text work">
        {$timerState === TimerState.IDLE ? 'Ready' : 
         $timerState === TimerState.PAUSED ? 'Paused' : 'Working'}
      </p>
    {/if}
  </div>
  
  {#if $timerState !== TimerState.BREAK}
    <div class="task-inputs">
      <input 
        type="text" 
        bind:value={$currentTask} 
        placeholder="What are you working on?"
        disabled={$timerState === TimerState.RUNNING}
      />
      
      <select 
        bind:value={$currentCategory}
        disabled={$timerState === TimerState.RUNNING}
      >
        {#each categories as category}
          <option value={category}>{category}</option>
        {/each}
      </select>
    </div>
  {/if}
  
  <div class="timer-controls">
    {#if $timerState === TimerState.RUNNING}
      <button on:click={pauseTimer} class="btn pause">Pause</button>
    {:else}
      <button on:click={startTimer} class="btn start">
        {$timerState === TimerState.PAUSED ? 'Resume' : 'Start'}
      </button>
    {/if}
    <button on:click={resetTimer} class="btn reset">Reset</button>
  </div>
</div>

<style>
  .timer-container {
    max-width: 400px;
    margin: 0 auto;
    padding: 2rem;
    border-radius: 1rem;
    background-color: #f5f5f5;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }
  
  .timer-display {
    font-size: 4rem;
    text-align: center;
    margin: 0;
    font-weight: bold;
    font-family: monospace;
  }
  
  .timer-status {
    text-align: center;
    margin: 1rem 0 2rem;
  }
  
  .status-text {
    font-size: 1.5rem;
    font-weight: 500;
    margin: 0;
  }
  
  .work { color: #3498db; }
  .break { color: #2ecc71; }
  
  .task-inputs {
    margin-bottom: 2rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  input, select {
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 0.5rem;
    font-size: 1rem;
  }
  
  .timer-controls {
    display: flex;
    gap: 1rem;
  }
  
  .btn {
    flex: 1;
    padding: 0.75rem;
    border: none;
    border-radius: 0.5rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .start { background-color: #2ecc71; color: white; }
  .pause { background-color: #f39c12; color: white; }
  .reset { background-color: #e74c3c; color: white; }
  
  .btn:hover {
    opacity: 0.9;
  }
</style>
