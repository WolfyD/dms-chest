import { invoke } from '@tauri-apps/api/core';


/**
 * Get all Calendars that match the name
 * @param name 
 * @returns 
 */
export async function getCalendars(name: string): Promise<{ name: string, id: number }[]> {
  try {
    const results = await invoke<Array<{ name: string, id: number }>>('query_database', {
      query: "SELECT CAST(id AS INTEGER) as id, name FROM calendars WHERE deleted_at IS NULL AND name LIKE '%' || ? || '%'",
      params: [name]
    });

    console.log("Raw results:", results);
    const mapped = results.map(row => {
      console.log("Row:", row);
      return { name: row.name, id: row.id };
    });
    console.log("Mapped results:", mapped);
    return mapped;
  } catch (error) {
    console.error('Failed to query database:', error);
    throw error;
  }
}
  
  
