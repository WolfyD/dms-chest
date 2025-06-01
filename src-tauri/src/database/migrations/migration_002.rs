use super::super::Migration;

pub fn get_migration() -> Migration {
    Migration {
        version: 2,
        description: "Create important tables".to_string(),
        up_sql: "
            CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                nickname TEXT,
                email TEXT,
                contact_1_method TEXT,
                contact_1_value TEXT,
                contact_2_method TEXT,
                contact_2_value TEXT,
                contact_3_method TEXT,
                contact_3_value TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS player_availability (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER DEFAULT null,
                player_id INTEGER NOT NULL,
                day TEXT NOT NULL DEFAULT '',
                time_slot TEXT DEFAULT '',
                available BOOLEAN NOT NULL DEFAULT false,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (player_id) REFERENCES players(id)
            );

            CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                player_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (player_id) REFERENCES players(id)
            );

            CREATE TABLE IF NOT EXISTS character_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                character_id INTEGER NOT NULL,
                class TEXT NOT NULL,
                race TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (character_id) REFERENCES characters(id)
            );

            CREATE TABLE IF NOT EXISTS races (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                custom_to_world_id INTEGER DEFAULT null,
                custom_to_campaign_id INTEGER DEFAULT null,
                name TEXT NOT NULL,
                description TEXT,
                is_standard_race BOOLEAN NOT NULL DEFAULT false,
                is_subrace BOOLEAN NOT NULL DEFAULT false,
                parent_race_id INTEGER DEFAULT null,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (custom_to_campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (custom_to_world_id) REFERENCES worlds(id),
                FOREIGN KEY (parent_race_id) REFERENCES races(id)
            );
            

            CREATE TABLE IF NOT EXISTS race_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                race_id INTEGER NOT NULL,

                -- Mechanics
                speed INTEGER DEFAULT NULL, -- in feet or meters
                size_category TEXT DEFAULT 'Medium' CHECK(size_category IN ('Tiny', 'Small', 'Medium', 'Large', 'Huge', 'Gargantuan')),
                ability_score_bonuses TEXT DEFAULT '{}',
                languages TEXT DEFAULT '[]',
                vision_type TEXT DEFAULT 'Normal' CHECK(vision_type IN ('Normal', 'Darkvision', 'Blindsight', 'Truesight', 'None')),
                vision_range INTEGER DEFAULT NULL,
                can_be_pc_race BOOLEAN NOT NULL DEFAULT true,
                resistances TEXT DEFAULT '[]',
                immunities TEXT DEFAULT '[]',
                weaknesses TEXT DEFAULT '[]',
                innate_spellcasting TEXT DEFAULT '[]',
                natural_armor TEXT DEFAULT '[]',
                natural_attacks TEXT DEFAULT '[]',
                proficiencies TEXT DEFAULT '[]',
                type TEXT DEFAULT 'Humanoid' CHECK(type IN ('Humanoid', 'Aberration', 'Beast', 'Construct', 'Dragon', 'Elemental', 'Fey', 'Giant', 'Monstrosity', 'Ooze', 'Plant', 'Undead')),

                -- Flavor
                lifespan INTEGER DEFAULT NULL,
                average_height TEXT DEFAULT '',
                average_weight TEXT DEFAULT '',
                typical_alignment TEXT DEFAULT '' CHECK(typical_alignment IN ('Lawful Good', 'Neutral Good', 'Chaotic Good', 'Lawful Neutral', 'True Neutral', 'Chaotic Neutral', 'Lawful Evil', 'Neutral Evil', 'Chaotic Evil')),
                homeland_locations TEXT DEFAULT '[]',
                culture_notes TEXT DEFAULT '',
                society_notes TEXT DEFAULT '',
                religious_beliefs TEXT DEFAULT '',
                naming_conventions TEXT DEFAULT '',

                -- Additional
                favored_classes TEXT DEFAULT '[]',
                common_professions TEXT DEFAULT '[]',

                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (race_id) REFERENCES races(id)
            );

            CREATE TABLE IF NOT EXISTS race_traits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                race_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                trait_type TEXT DEFAULT 'feature' CHECK(trait_type IN ('feature', 'resistance', 'weakness', 'language', 'custom')),
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (race_id) REFERENCES races(id)
            );

            
            

            
            






        ".to_string(),
        down_sql: "
            DROP TABLE IF EXISTS players;
            DROP TABLE IF EXISTS player_availability;
            DROP TABLE IF EXISTS characters;
            DROP TABLE IF EXISTS character_details;
        ".to_string(),
    }
} 