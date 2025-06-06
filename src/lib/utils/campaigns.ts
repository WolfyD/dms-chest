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

export async function createCampaign(campaignName: string, campaignDescription: string, worldId: number, campaignType: string, partySize: string, partyLevel: string, themes: string, tones: string, winConditions: string, sessionZeroNotes: string, playerAgreements: string, difficultyLevel: string, startingLocationId: number, calendarId: number, houseRulesId: number) : Promise<number> {
    try {
        const results = await invoke<Array<{ name: string, id: number }>>('query_database_params', {
          query: "INSERT INTO campaigns (name, description) VALUES (?, ?)",
          params: [campaignName, campaignDescription],
        });
        const campaignId = results[0].id;

        const detailsResults = await invoke<Array<{ id: number }>>('query_database_params', {
          query: "INSERT INTO campaign_details (campaign_id, world_id, type, party_size, party_level, themes, tones, win_conditions, session_zero_notes, player_agreements, difficulty_level, starting_location_id, calendar_id, house_rules_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
          params: [campaignId, worldId, campaignType, partySize, partyLevel, themes, tones, winConditions, sessionZeroNotes, playerAgreements, difficultyLevel, startingLocationId, calendarId, houseRulesId],
        });
        return campaignId;
    } catch (error) {
        console.error('Failed to create campaign:', error);
        throw error;
    }
}
