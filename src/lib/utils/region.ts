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
  
  export async function getLocationParents(id: number): Promise<{ name: string, id: number, type: string }[]> {
  
    console.log("getLocationParents", id);
  
    try {
      const results = await invoke<Array<{ name: string, id: number, type: string }>>('query_database', {
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
          SELECT name, id, type FROM location_chain order by id asc;`,
        params: [id.toString()]
      });
  
      console.log("results2", results);
  
      return results.map(row => ({ name: row.name, id: row.id, type: row.type }));
    } catch (error) {
      console.error('Failed to query database:', error);
      throw error;
    }
  }
  
  // Debounce utility function
  export function debounce<T extends (...args: any[]) => any>(
    func: T,
    wait: number
  ): (...args: Parameters<T>) => Promise<ReturnType<T>> {
    let timeout: NodeJS.Timeout;
    let lastPromise: Promise<ReturnType<T>> | null = null;
  
    return function executedFunction(...args: Parameters<T>): Promise<ReturnType<T>> {
      return new Promise((resolve) => {
        clearTimeout(timeout);
        
        timeout = setTimeout(async () => {
          try {
            const result = await func(...args);
            resolve(result);
          } catch (error) {
            console.error('Debounced function error:', error);
            throw error;
          }
        }, wait);
      });
    };
  }
  
  // Autocomplete functionality
  export interface AutocompleteState {
    suggestions: { name: string, id: number }[];
    showSuggestions: boolean;
    selectedIndex: number;
    selected?: { name: string, id: number };
  }
  
  export function createAutocomplete(
    searchFn: (text: string) => Promise<{ name: string, id: number }[]>,
    debounceTime: number = 300
  ) {
    let state: AutocompleteState = {
      suggestions: [],
      showSuggestions: false,
      selectedIndex: -1
    };
  
    const debouncedSearch = debounce(async (text: string) => {
      if (text.length >= 3) {
        state.suggestions = await searchFn(text);
        state.showSuggestions = true;
      } else {
        state.suggestions = [];
        state.showSuggestions = false;
      }
    }, debounceTime);
  
    return {
      handleInput: async (text: string) => {
        await debouncedSearch(text);
        return state;
      },
      handleKeydown: (event: KeyboardEvent): AutocompleteState => {
        if (!state.showSuggestions) return state;
  
        switch (event.key) {
          case 'ArrowDown':
            event.preventDefault();
            state.selectedIndex = Math.min(state.selectedIndex + 1, state.suggestions.length - 1);
            break;
          case 'ArrowUp':
            event.preventDefault();
            state.selectedIndex = Math.max(state.selectedIndex - 1, -1);
            break;
          case 'Enter':
            event.preventDefault();
            if (state.selectedIndex >= 0) {
              const selected = state.suggestions[state.selectedIndex];
              state.suggestions = [];
              state.showSuggestions = false;
              state.selectedIndex = -1;
              return { ...state, selected };
            }
            break;
          case 'Escape':
            state.showSuggestions = false;
            state.selectedIndex = -1;
            break;
        }
        return state;
      },
      selectSuggestion: (suggestion: { name: string, id: number }): AutocompleteState => {
        state.suggestions = [];
        state.showSuggestions = false;
        state.selectedIndex = -1;
        return { ...state, selected: suggestion };
      },
      getState: () => state
    };
  }
  
  