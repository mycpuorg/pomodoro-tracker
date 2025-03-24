import { writable } from 'svelte/store';

export const WORK_DURATION = 25 * 60; // 25 minutes in seconds
export const BREAK_DURATION = 5 * 60; // 5 minutes in seconds

export const TimerState = {
  IDLE: 'idle',
  RUNNING: 'running',
  PAUSED: 'paused',
  BREAK: 'break',
};

export const timerState = writable(TimerState.IDLE);
export const remainingTime = writable(WORK_DURATION);
export const currentTask = writable('');
export const currentCategory = writable('Work');
