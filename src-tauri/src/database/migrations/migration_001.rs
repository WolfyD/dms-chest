use super::super::Migration;

pub fn get_migration() -> Migration {
    Migration {
        version: 1,
        description: "Create initial tables".to_string(),
        up_sql: "
            CREATE TABLE IF NOT EXISTS campaigns (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                settings TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS campaign_details (
                id INTEGER PRIMARY KEY,
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
                difficulty_level INTEGER DEFAULT 3 CHECK(difficulty_level >= 1 AND difficulty_level <= 7),
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id),
                FOREIGN KEY (calendar_id) REFERENCES calendars(id),
                FOREIGN KEY (house_rules_id) REFERENCES house_rules(id)
            );

            CREATE TABLE IF NOT EXISTS campaign_arcs (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                summary TEXT DEFAULT '',
                arc_type TEXT DEFAULT 'major' CHECK(arc_type IN ('epic', 'major', 'minor', 'personal')),
                status TEXT DEFAULT 'active' CHECK(status IN ('active', 'paused', 'resolved', 'abandoned')),
                start_session_id INTEGER DEFAULT NULL,
                end_session_id INTEGER DEFAULT NULL,
                milestones TEXT DEFAULT '[]',
                notes TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id)
            );

            CREATE TABLE IF NOT EXISTS campaign_npcs (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                nickname TEXT DEFAULT '',
                npc_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id),      
                FOREIGN KEY (npc_id) REFERENCES npcs(id)
            );

            CREATE TABLE IF NOT EXISTS npcs (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER default NULL,
                world_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                class_id INTEGER DEFAULT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id),
                FOREIGN KEY (class_id) REFERENCES classes(id)
            );

            CREATE TABLE IF NOT EXISTS npc_details (
                id INTEGER PRIMARY KEY,
                npc_id INTEGER NOT NULL,
                role TEXT DEFAULT 'neutral' CHECK(role IN ('ally', 'villain', 'shopkeeper', 'neutral', 'unknown', 'other')),
                importance TEXT DEFAULT 'minor' CHECK(importance IN ('recurring', 'major', 'minor', 'trivia')),
                first_appearance_session_id INTEGER,
                last_known_location TEXT DEFAULT '',
                relationship_to_party TEXT DEFAULT '',
                appearance_notes TEXT DEFAULT '[]',
                personality_notes TEXT DEFAULT '',
                motives TEXT DEFAULT '',
                secrets TEXT DEFAULT '[]',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (npc_id) REFERENCES npcs(id)
            );

            CREATE TABLE IF NOT EXISTS campaign_factions (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                faction_id INTEGER NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id),
                FOREIGN KEY (faction_id) REFERENCES factions(id)
            );

            CREATE TABLE IF NOT EXISTS factions (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,
                
                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id)
            );

            CREATE TABLE IF NOT EXISTS faction_details (
                id INTEGER PRIMARY KEY,
                faction_id INTEGER NOT NULL,
                faction_type TEXT DEFAULT 'neutral' CHECK(faction_type IN ('ally', 'enemy', 'hidden', 'neutral', 'unknown', 'other')),
                influence_level INTEGER DEFAULT 3 CHECK(influence_level >= 1 AND influence_level <= 5),
                known_members TEXT DEFAULT '[]',
                goals TEXT DEFAULT '',
                alignment TEXT DEFAULT 'Chaotic Neutral' CHECK(alignment IN ('Lawful Good', 'Neutral Good', 'Chaotic Good', 'Lawful Neutral', 'True Neutral', 'Chaotic Neutral', 'Lawful Evil', 'Neutral Evil', 'Chaotic Evil')),
                location TEXT DEFAULT '',
                history TEXT DEFAULT '',
                relationship_to_party TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,
                
                FOREIGN KEY (faction_id) REFERENCES factions(id)
            );

            CREATE TABLE IF NOT EXISTS worlds (
                id INTEGER PRIMARY KEY,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS world_details (
                id INTEGER PRIMARY KEY,
                world_id INTEGER NOT NULL,
                genre TEXT DEFAULT 'Fantasy' CHECK(genre IN ('Fantasy', 'Sci-Fi', 'Post-Apocalyptic', 'Modern', 'Historical', 'Other')),
                tone TEXT DEFAULT 'Mixed' CHECK(tone IN ('Heroic', 'Dark', 'Grimdark', 'Hopeful', 'Tragic', 'Comedic', 'Mixed', 'Other')),
                tech_level TEXT DEFAULT 'Medieval' CHECK(tech_level IN ('Stone Age', 'Bronze Age', 'Iron Age', 'Medieval', 'Renaissance', 'Industrial', 'Modern', 'Near Future', 'Far Future', 'Magitech', 'Other')),
                magic_level TEXT DEFAULT 'Low' CHECK(magic_level IN ('None', 'Low', 'Moderate', 'High', 'Wild', 'Divine Only', 'Unknown', 'Other')),
                dominant_species TEXT DEFAULT '[]',
                other_species TEXT DEFAULT '[]',
                religions TEXT DEFAULT '[]',
                pantheon TEXT DEFAULT '[]',
                notable_landmarks TEXT DEFAULT '[]',
                history TEXT DEFAULT '',
                planar_structure TEXT DEFAULT 'Material Plane',
                calendar_id INTEGER DEFAULT NULL,
                established_material TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (world_id) REFERENCES worlds(id),
                FOREIGN KEY (calendar_id) REFERENCES calendars(id)
            );

            CREATE TABLE IF NOT EXISTS locations (
                id INTEGER PRIMARY KEY,
                world_id INTEGER NOT NULL,
                parent_id INTEGER DEFAULT NULL,
                has_parent BOOLEAN DEFAULT true,
                has_children BOOLEAN DEFAULT true,
                name TEXT NOT NULL,
                description TEXT DEFAULT '',
                area_type_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,
                
                FOREIGN KEY (world_id) REFERENCES worlds(id),
                FOREIGN KEY (area_type_id) REFERENCES area_types(id)
            );

            CREATE TABLE IF NOT EXISTS location_details (
                id INTEGER PRIMARY KEY,
                location_id INTEGER NOT NULL,
                population INTEGER DEFAULT NULL,
                known_for TEXT DEFAULT '',
                terrain TEXT DEFAULT '',
                climate TEXT DEFAULT '',
                danger_level INTEGER DEFAULT 1 CHECK(danger_level >= 1 AND danger_level <= 5),
                notable_landmarks TEXT DEFAULT '[]',
                history TEXT DEFAULT '',
                major_events TEXT DEFAULT '[]',
                notes TEXT DEFAULT '',
                has_map BOOLEAN DEFAULT false,
                map_id INTEGER DEFAULT NULL,
                map_image_url TEXT DEFAULT '',
                map_location TEXT DEFAULT '{}' CHECK (json_valid(map_location)),
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (location_id) REFERENCES locations(id),
                FOREIGN KEY (map_id) REFERENCES maps(id)
            );

            CREATE TABLE IF NOT EXISTS maps (
                id INTEGER PRIMARY KEY,
                name TEXT DEFAULT '',
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS map_details (
                id INTEGER PRIMARY KEY,
                map_id INTEGER NOT NULL,
                image_id TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (map_id) REFERENCES maps(id),
                FOREIGN KEY (image_id) REFERENCES images(id)
            );

            CREATE TABLE IF NOT EXISTS calendars (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT DEFAULT '',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS calendar_details (
                id INTEGER PRIMARY KEY,
                calendar_id INTEGER NOT NULL,

                days_in_year INTEGER NOT NULL,
                months_in_year INTEGER NOT NULL,
                days_in_week INTEGER NOT NULL,
                weeks_in_month INTEGER NOT NULL,
                months TEXT NOT NULL DEFAULT '[]',
                days_of_week TEXT NOT NULL DEFAULT '[]',
                hours_in_day INTEGER NOT NULL,
                minutes_in_hour INTEGER NOT NULL,
                seconds_in_minute INTEGER NOT NULL,
                pre_epoch_prefix TEXT NOT NULL DEFAULT '',
                post_epoch_prefix TEXT NOT NULL DEFAULT '',
                epoch_term TEXT NOT NULL DEFAULT '',
                important_holidays TEXT NOT NULL DEFAULT '[]',
                notable_events TEXT NOT NULL DEFAULT '[]',
                major_astronomical_events TEXT NOT NULL DEFAULT '[]',
                major_astrological_events TEXT NOT NULL DEFAULT '[]',
                major_historical_events TEXT NOT NULL DEFAULT '[]',
                moon_phases TEXT NOT NULL DEFAULT '[]',
                moon_phases_in_month INTEGER NOT NULL,
                moon_phase_at_0 INTEGER NOT NULL,

                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (calendar_id) REFERENCES calendars(id)
            );

            CREATE TABLE IF NOT EXISTS house_rules (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT DEFAULT '',
                rules TEXT DEFAULT '[]',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id)
            );

            CREATE TABLE area_types (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                level INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                deleted_at DATETIME
            );
            

            
        ".to_string(),
        down_sql: "
            DROP TABLE IF EXISTS campaigns;
            DROP TABLE IF EXISTS campaign_details;
            DROP TABLE IF EXISTS campaign_arcs;
            DROP TABLE IF EXISTS campaign_npcs;
            DROP TABLE IF EXISTS npcs;
            DROP TABLE IF EXISTS npc_details;
            DROP TABLE IF EXISTS campaign_factions;
            DROP TABLE IF EXISTS factions;
            DROP TABLE IF EXISTS faction_details;
            DROP TABLE IF EXISTS worlds;
            DROP TABLE IF EXISTS world_details;
            DROP TABLE IF EXISTS locations;
            DROP TABLE IF EXISTS location_details;
            DROP TABLE IF EXISTS maps;
            DROP TABLE IF EXISTS map_details;
            DROP TABLE IF EXISTS calendars;
            DROP TABLE IF EXISTS calendar_details;
            DROP TABLE IF EXISTS house_rules;
            DROP TABLE IF EXISTS area_types;
        ".to_string(),
    }
} 