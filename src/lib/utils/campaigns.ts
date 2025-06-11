import { invoke } from "@tauri-apps/api/core";
import { type CampaignName } from '$lib/types';

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

export async function createCampaign(
    campaignName:         string, 
    campaignDescription:  string, 
    worldId:              number, 
    campaignType:         string, 
    partySize:            string, 
    partyLevel:           string, 
    themes:               string, 
    tones:                string, 
    winConditions:        string, 
    sessionZeroNotes:     string, 
    playerAgreements:     string, 
    difficultyLevel:      number, 
    startingLocationId:   number | null, 
    calendarId:           number | null, 
    houseRulesId:         number | null
  ) : Promise<number> {
    try {
        // Start transaction
        await invoke('query_database', {
            query: "BEGIN TRANSACTION",
            params: []
        });

        try {
            // Check for existing campaign
            const existingCampaign = await invoke<Array<{ name: string, id: number }>>('query_database', {
                query: "SELECT name, id FROM campaigns WHERE name = ? AND deleted_at IS NULL",
                params: [campaignName]
            });

            if (existingCampaign.length > 0) {
                // Rollback transaction
                await invoke('query_database', {
                    query: "ROLLBACK",
                    params: []
                });
                console.log("Campaign already exists");
                return -1;
            }

            // Create new campaign
            const results = await invoke<Array<{ name: string, id: number }>>('query_database', {
                query: "INSERT INTO campaigns (name, description, created_at, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                params: [campaignName, campaignDescription]
            });
            
            console.log("Campaign created");

            // Get the new campaign ID
            const q_result = await invoke<Array<{ id: number }>>('query_database', {
                query: "SELECT CAST(id AS INTEGER) as id FROM campaigns WHERE name = ? AND description = ?",
                params: [campaignName, campaignDescription]
            });

            const campaignId = q_result[0].id;
            console.log("Campaign ID: " + campaignId);

            // Create campaign details
            const detailsResults = await invoke<Array<{ id: number }>>('query_database', {
                query: "INSERT INTO campaign_details (campaign_id, world_id, campaign_type, party_size, party_level, themes, tone, win_conditions, session_zero_notes, player_agreements, difficulty_level, starting_location_id, calendar_id, house_rules_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                params: [
                    campaignId,          // number
                    worldId,            // number
                    campaignType,       // string
                    partySize,          // string
                    partyLevel,         // string
                    themes,             // string
                    tones,              // string
                    winConditions,      // string
                    sessionZeroNotes,   // string
                    playerAgreements,   // string
                    difficultyLevel,    // number
                    startingLocationId, // number | null
                    calendarId,         // number | null
                    houseRulesId        // number | null
                ]
            });

            console.log("Campaign details created", detailsResults);

            // Commit transaction
            await invoke('query_database', {
                query: "COMMIT",
                params: []
            });

            return campaignId;
        } catch (error) {
            // Rollback transaction on error
            await invoke('query_database', {
                query: "ROLLBACK",
                params: []
            });
            throw error;
        }
    } catch (error) {
        console.error('Failed to create campaign:', error);
        throw error;
    }
}
