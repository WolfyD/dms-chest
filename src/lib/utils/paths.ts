import { appDataDir, join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

/**
 * Gets the path to the extracted JSON file
 * @returns The full path to the JSON file
 */
export async function getWordNetJsonPath(): Promise<string> {
    const appDir = await appDataDir();
    return await join(appDir, 'wn.json');
}

/**
 * Reads the contents of the world names JSON file
 * @returns The parsed JSON contents
 */
export async function readWordNetJson(): Promise<any> {
    const jsonPath = await getWordNetJsonPath();
    const contents = await invoke('read_file', { path: jsonPath });
    return JSON.parse(contents as string);
} 