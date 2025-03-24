import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export interface PomodoroSession {
  id?: number;
  task_name: string;
  category: string;
  start_time: number;
  end_time: number;
}

export const sessions = writable<PomodoroSession[]>([]);

export async function fetchSessions(startTimestamp: number, endTimestamp: number) {
  try {
    const data = await invoke<PomodoroSession[]>('get_sessions', { 
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

export async function saveSession(session: PomodoroSession) {
  try {
    const id = await invoke<number>('save_pomodoro_session', {
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
