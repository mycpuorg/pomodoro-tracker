import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export const sessions = writable([]);

export async function fetchSessions(startTimestamp, endTimestamp) {
  try {
    const data = await invoke('get_sessions', { 
      startTimestamp, 
      endTimestamp 
    });
    sessions.set(data);
    return data;
  } catch (error) {
    console.error('Error fetching sessions:', error);
    return [];
  }
}

export async function saveSession(session) {
  try {
    const id = await invoke('save_pomodoro_session', {
      taskName: session.task_name,
      category: session.category,
      startTime: session.start_time,
      endTime: session.end_time,
    });
    
    const newSession = { ...session, id };
    sessions.update(s => [newSession, ...s]);
    return id;
  } catch (error) {
    console.error('Error saving session:', error);
    throw error;
  }
}
