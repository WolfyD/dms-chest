import { invoke } from '@tauri-apps/api/core';

let isInitialized = false;

export async function initializeDatabase(): Promise<void> {
    if (isInitialized) return;

    try {
        await invoke('initialize_database');
        isInitialized = true;
        console.log('Database initialized successfully');
    } catch (error) {
        console.error('Failed to initialize database:', error);
        throw error;
    }
}

export async function closeDatabase(): Promise<void> {
    if (!isInitialized) return;
    isInitialized = false;
    console.log('Database closed successfully');
} 