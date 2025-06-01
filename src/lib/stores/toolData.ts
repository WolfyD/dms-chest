// src/lib/stores/toolData.ts
import { writable } from 'svelte/store';

export const toolData = writable<{
    id?: string;
    description?: string;
    // other data...
}>({});