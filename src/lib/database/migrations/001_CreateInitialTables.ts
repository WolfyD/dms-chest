import { Migration } from './Migration';

export class CreateCampaignsTable extends Migration {
    public version = 1;
    public description = 'Create campaigns table';

    public async up(): Promise<void> {
        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS campaigns (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                settings TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS campaign_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                campaign_type TEXT NOT NULL,
                party_level INTEGER NOT NULL,
                party_size INTEGER NOT NULL,
                themes TEXT DEFAULT '{}',
                tone TEXT DEFAULT '{}',
                starting_location_name TEXT DEFAULT '',
                starting_location_id INTEGER DEFAULT null,
                win_conditions TEXT DEFAULT '{}',
                session_zero_notes TEXT DEFAULT '',
                player_agreements TEXT DEFAULT '',
                calendar_id INTEGER DEFAULT 1,
                house_rules_id INTEGER DEFAULT null,
                difficulty_level TEXT DEFAULT 'medium' CHECK(difficulty_level IN ('easy', 'medium', 'hard', 'insane')),
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS campaign_arcs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                summary TEXT DEFAULT '',
                arc_type TEXT DEFAULT 'major' CHECK(arc_type IN ('epic', 'major', 'minor', 'personal')),
                status TEXT DEFAULT 'active' CHECK(status IN ('active', 'paused', 'resolved', 'abandoned')),
                start_session_id INTEGER DEFAULT NULL,
                end_session_id INTEGER DEFAULT NULL,
                milestones TEXT DEFAULT '[]' comment 'JSON array of milestone objects eg: [{"name": "Milestone 1", "description": "Description of Milestone 1", "completed": true, "started_in_session_id": 1, "completed_in_session_id": 2}]',
                notes TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS campaign_npcs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                nickname TEXT DEFAULT '',
                npc_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS npcs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER default NULL,
                world_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                class_id INTEGER DEFAULT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS npcs_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                npc_id INTEGER NOT NULL,
                role TEXT DEFAULT 'neutral' CHECK(role IN ('ally', 'villain', 'shopkeeper', 'neutral', 'unknown', 'other')),
                importance TEXT DEFAULT 'minor' CHECK(importance IN ('recurring', 'major', 'minor', 'trivia')),
                first_appearance_session_id INTEGER,
                last_known_location TEXT DEFAULT '',
                relationship_to_party TEXT DEFAULT '',
                appearance_notes TEXT DEFAULT '[]' comment 'JSON array of appearance notes eg: [{"session_id": 1, "notes": "NPC was seen in the tavern"}]',
                personality_notes TEXT DEFAULT '',
                motives TEXT DEFAULT '',
                secrets TEXT DEFAULT '[]' comment 'JSON array of secrets eg: [{"name": "Secret 1", "description": "Description of Secret 1", "revealed": true, "revealed_in_session_id": 1, "revealed_to_characters": [2, 4, 5]}]',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS campaign_factions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                faction_id INTEGER NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS factions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS faction_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                faction_id INTEGER NOT NULL,
                faction_type TEXT DEFAULT 'neutral' CHECK(faction_type IN ('ally', 'enemy', 'hidden', 'neutral', 'unknown', 'other')),
                influence_level INTEGER DEFAULT 3 CHECK(influence_level >= 1 AND influence_level <= 5),
                known_members TEXT DEFAULT '[]' comment 'JSON list of associated NPCs or leaders eg: [{"npc_id": 1, "role": "ally"}, {"npc_id": 0, name: "Phileas Fogg", "role": "ally"}]',
                goals TEXT DEFAULT '',
                alignment TEXT DEFAULT 'Chaotic Neutral' CHECK(alignment IN ('Lawful Good', 'Neutral Good', 'Chaotic Good', 'Lawful Neutral', 'True Neutral', 'Chaotic Neutral', 'Lawful Evil', 'Neutral Evil', 'Chaotic Evil')),
                location TEXT DEFAULT '',
                history TEXT DEFAULT '',
                relationship_to_party TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);



        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS worlds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS world_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                world_id INTEGER NOT NULL,
                
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);
    }

    public async down(): Promise<void> {
        await this.executeQuery('DROP TABLE IF EXISTS campaigns');
        await this.executeQuery('DROP TABLE IF EXISTS campaign_details');
        await this.executeQuery('DROP TABLE IF EXISTS campaign_arcs');
        await this.executeQuery('DROP TABLE IF EXISTS campaign_npcs');
        await this.executeQuery('DROP TABLE IF EXISTS campaign_factions');
        await this.executeQuery('DROP TABLE IF EXISTS worlds');
        await this.executeQuery('DROP TABLE IF EXISTS world_details');
    }
}