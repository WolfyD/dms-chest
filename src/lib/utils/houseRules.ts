import { invoke } from '@tauri-apps/api/core';


/**
 * Get all Calendars that match the name
 * @param name 
 * @returns 
 */
export async function getHouseRules(name: string): Promise<{ name: string, id: number }[]> {
  try {
    const results = await invoke<Array<{ name: string, id: number }>>('query_database', {
      query: "SELECT CAST(id AS INTEGER) as id, name FROM house_rules WHERE deleted_at IS NULL AND name LIKE '%' || ? || '%'",
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
  
  /**
   * Check the number of house rules in the database
   * @returns { count: number }
   */
  export async function checkHouseRuleCount(): Promise<{ count: number }> {
    try {
      const results = await invoke<Array<{ count: number }>>('query_database_no_params', {
        query: "SELECT COUNT(*) as count FROM house_rules WHERE deleted_at IS NULL",
      });

      return { count: results[0].count };
    } catch (error) {
      console.error('Failed to query database:', error);
      throw error;
    }
  }