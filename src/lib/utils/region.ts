import { invoke } from '@tauri-apps/api/core';
/*
type Level = {
    key: number;
    level: string;
  };
  
  type GenerationOptions = {
    startKey: number;
    depth: number;
    skipLevels?: string[];
    levels: Level[];
  };
  
  type LocationNode = {
    level: string;
    count: number;
    names: string[];
    children?: Record<string, LocationNode[]>;
  };
  
  const DEFAULT_COUNTS = {
    // You can customize this later
    5: 10,  // Country → Provinces
    6: 30,  // Province → Cities
    7: 50,  // City → Districts
    8: 100  // District → Streets
  };
  
  function getChildrenKeys(startKey: number, levels: Level[], depth: number, skipLevels: string[]): Level[] {
    const sortedLevels = levels.sort((a, b) => a.key - b.key);
    const startIndex = sortedLevels.findIndex(l => l.key === startKey);
    return sortedLevels
      .slice(startIndex + 1, startIndex + 1 + depth)
      .filter(level => !skipLevels.includes(level.level));
  }
  
  function generateMockName(level: string, index: number): string {
    return `${level}_${index + 1}`;
  }
  
  function generateLocationTree(
    options: GenerationOptions,
    parentName = 'Root',
    levelKeyOverride?: number
  ): LocationNode {
    const { startKey, depth, skipLevels = [], levels } = options;
    const currentKey = levelKeyOverride ?? startKey;
    const currentLevel = levels.find(l => l.key === currentKey);
  
    if (!currentLevel) throw new Error(`Invalid start level key: ${currentKey}`);
  
    const childrenLevels = getChildrenKeys(currentKey, levels, depth, skipLevels);
    const nextLevel = childrenLevels[0];
  
    // Default to a reasonable count per level key
    const count = DEFAULT_COUNTS[currentKey] ?? 5;
  
    const names = Array.from({ length: count }, (_, i) => generateMockName(currentLevel.level, i));
    const node: LocationNode = {
      level: currentLevel.level,
      count,
      names
    };
  
    if (nextLevel) {
      node.children = {};
      for (const name of names) {
        node.children[name] = Array.from({ length: DEFAULT_COUNTS[nextLevel.key] ?? 3 }, (_, i) =>
          generateLocationTree(
            {
              ...options,
              startKey: nextLevel.key,
              depth: depth - 1,
              skipLevels
            },
            name,
            nextLevel.key
          )
        );
      }
    }
  
    return node;
  }
  */

  /**
   * Get all locations that match the name
   * @param name 
   * @returns 
   */
  export async function getLocations(name: string): Promise<{ name: string, id: number }[]> {
    try {
      const results = await invoke<Array<{ name: string, id: number }>>('query_database', {
        query: "SELECT CAST(id AS INTEGER) as id, name FROM locations WHERE deleted_at IS NULL AND name LIKE '%' || ? || '%'",
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
  
  export async function getLocationParents(id: number): Promise<{ name: string, id: number, type: string, parent_id: number, has_parent: boolean, has_children: boolean }[]> {
  
    console.log("getLocationParents", id);
  
    try {
      const results = await invoke<Array<{ name: string, id: number, type: string, parent_id: number, has_parent: boolean, has_children: boolean }>>('query_database', {
        query: `WITH RECURSIVE location_chain AS (
                SELECT
                  id,
                  name,
                  parent_id,
                  parent_id IS NOT NULL AS has_parent,
                  0 AS level,
                  *
                FROM locations
                WHERE id = ?

                UNION ALL

                SELECT
                  l.id,
                  l.name,
                  l.parent_id,
                  l.parent_id IS NOT NULL AS has_parent,
                  lc.level + 1,
                  l.*
                FROM locations l
                JOIN location_chain lc ON l.id = lc.parent_id
              )
              SELECT 
                name, 
                id, 
                (SELECT name FROM area_types WHERE id = area_type_id) as type,
                parent_id,
                has_parent,
                has_children
              FROM 
                location_chain 
              ORDER BY 
                id ASC;`,
            params: [id.toString()]
      });
  
      console.log("results2", results);
  
      return results.map(row => ({ name: row.name, id: row.id, type: row.type, parent_id: row.parent_id, has_parent: row.has_parent, has_children: row.has_children }));
    } catch (error) {
      console.error('Failed to query database:', error);
      throw error;
    }
  }

  export async function getLocationChildren(id: number): Promise<{ name: string, id: number, type: string, parent_id: number, has_parent: boolean, has_children: boolean }[]> {
  
    console.log("getLocationChildren", id);
  
    try {
      const results = await invoke<Array<{ name: string, id: number, type: string, parent_id: number, has_parent: boolean, has_children: boolean }>>('query_database', {
        query: `WITH RECURSIVE location_chain AS (
                SELECT
                  id,
                  name,
                  parent_id,
                  parent_id IS NOT NULL AS has_parent,
                  0 AS level,
                  *
                FROM locations
                WHERE id = ?

                UNION ALL

                SELECT
                  l.id,
                  l.name,
                  l.parent_id,
                  l.parent_id IS NOT NULL AS has_parent,
                  lc.level + 1,
                  l.*
                FROM locations l
                JOIN location_chain lc ON l.parent_id = lc.id
              )
              SELECT 
                name, 
                id, 
                (SELECT name FROM area_types WHERE id = area_type_id) as type,
                parent_id,
                has_parent,
                has_children
              FROM 
                location_chain 
              ORDER BY 
                id ASC;`,
            params: [id.toString()]
      });
  
      console.log("results2", results);
  
      return results.map(row => ({ name: row.name, id: row.id, type: row.type, parent_id: row.parent_id, has_parent: row.has_parent, has_children: row.has_children }));
    } catch (error) {
      console.error('Failed to query database:', error);
      throw error;
    }
  }
  
