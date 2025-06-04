import { invoke } from "@tauri-apps/api/core";

export type CampaignName = {
    name: string;
    id: number;
}

export async function getCampaignNames() : Promise<CampaignName[]> {
    try {
        const results = await invoke<Array<{ name: string, id: number }>>('query_database_no_params', {
          query: "SELECT id, name FROM campaigns WHERE deleted_at IS NULL",
        });
    
        const mapped = results.map(row => {
          return { name: row.name, id: row.id };
        });
        return mapped;
      } catch (error) {
        console.error('Failed to query database:', error);
        throw error;
      }
}
