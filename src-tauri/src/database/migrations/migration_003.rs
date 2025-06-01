use super::super::Migration;

pub fn get_migration() -> Migration {
    Migration {
        version: 3,
        description: "Create custom tables".to_string(),
        up_sql: "
            CREATE TABLE IF NOT EXISTS classes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id)
            );

            CREATE TABLE IF NOT EXISTS class_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                class_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (class_id) REFERENCES classes(id)
            );

            CREATE TABLE IF NOT EXISTS monsters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id)
            );

            CREATE TABLE IF NOT EXISTS monster_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                monster_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (monster_id) REFERENCES monsters(id)
            );

            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id)
            );

            CREATE TABLE IF NOT EXISTS item_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                item_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (item_id) REFERENCES items(id)
            );

            CREATE TABLE IF NOT EXISTS spells (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
                FOREIGN KEY (world_id) REFERENCES worlds(id)
            );

            CREATE TABLE IF NOT EXISTS spell_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                spell_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (spell_id) REFERENCES spells(id)
            );



            -- Indexes --
            -- campaigns:
                CREATE INDEX IF NOT EXISTS idx_campaigns_name ON campaigns(name);

            -- campaign_details:
                CREATE INDEX IF NOT EXISTS idx_campaign_details_house_rules_id ON campaign_details(house_rules_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_calendar_id ON campaign_details(calendar_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_world_id ON campaign_details(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_campaign_id ON campaign_details(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_campaign_id ON campaign_details(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_world_id ON campaign_details(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_calendar_id ON campaign_details(calendar_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_house_rules_id ON campaign_details(house_rules_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_campaign_type ON campaign_details(campaign_type);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_starting_location_name ON campaign_details(starting_location_name);

            -- campaign_arcs:
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_world_id ON campaign_arcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_campaign_id ON campaign_arcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_campaign_id ON campaign_arcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_world_id ON campaign_arcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_name ON campaign_arcs(name);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_arc_type ON campaign_arcs(arc_type);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_status ON campaign_arcs(status);

            -- campaign_npcs:
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_npc_id ON campaign_npcs(npc_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_world_id ON campaign_npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_campaign_id ON campaign_npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_campaign_id ON campaign_npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_world_id ON campaign_npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_npc_id ON campaign_npcs(npc_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_nickname ON campaign_npcs(nickname);

            -- npcs:
                CREATE INDEX IF NOT EXISTS idx_npcs_class_id ON npcs(class_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_world_id ON npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_campaign_id ON npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_campaign_id ON npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_world_id ON npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_class_id ON npcs(class_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_name ON npcs(name);

            -- npc_details:
                CREATE INDEX IF NOT EXISTS idx_npc_details_npc_id ON npc_details(npc_id);
                CREATE INDEX IF NOT EXISTS idx_npc_details_npc_id ON npc_details(npc_id);

            -- campaign_factions:
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_faction_id ON campaign_factions(faction_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_world_id ON campaign_factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_campaign_id ON campaign_factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_campaign_id ON campaign_factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_world_id ON campaign_factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_faction_id ON campaign_factions(faction_id);

            -- factions:
                CREATE INDEX IF NOT EXISTS idx_factions_world_id ON factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_factions_campaign_id ON factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_factions_campaign_id ON factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_factions_world_id ON factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_factions_name ON factions(name);

            -- faction_details:
                CREATE INDEX IF NOT EXISTS idx_faction_details_faction_id ON faction_details(faction_id);
                CREATE INDEX IF NOT EXISTS idx_faction_details_faction_id ON faction_details(faction_id);
                CREATE INDEX IF NOT EXISTS idx_faction_details_faction_type ON faction_details(faction_type);

            -- worlds:
                CREATE INDEX IF NOT EXISTS idx_worlds_name ON worlds(name);

            -- world_details:
                CREATE INDEX IF NOT EXISTS idx_world_details_world_id ON world_details(world_id);
                CREATE INDEX IF NOT EXISTS idx_world_details_world_id ON world_details(world_id);

            -- locations:
                CREATE INDEX IF NOT EXISTS idx_locations_world_id ON locations(world_id);
                CREATE INDEX IF NOT EXISTS idx_locations_world_id ON locations(world_id);
                CREATE INDEX IF NOT EXISTS idx_locations_has_parent ON locations(has_parent);
                CREATE INDEX IF NOT EXISTS idx_locations_has_children ON locations(has_children);
                CREATE INDEX IF NOT EXISTS idx_locations_name ON locations(name);
                CREATE INDEX IF NOT EXISTS idx_locations_type ON locations(type);

            -- location_details:
                CREATE INDEX IF NOT EXISTS idx_location_details_map_id ON location_details(map_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_location_id ON location_details(location_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_location_id ON location_details(location_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_map_id ON location_details(map_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_has_map ON location_details(has_map);

            -- maps:
                CREATE INDEX IF NOT EXISTS idx_maps_name ON maps(name);

            -- map_details:
                CREATE INDEX IF NOT EXISTS idx_map_details_image_id ON map_details(image_id);
                CREATE INDEX IF NOT EXISTS idx_map_details_map_id ON map_details(map_id);
                CREATE INDEX IF NOT EXISTS idx_map_details_map_id ON map_details(map_id);

            -- calendars:
                CREATE INDEX IF NOT EXISTS idx_calendars_name ON calendars(name);

            -- calendar_details:
                CREATE INDEX IF NOT EXISTS idx_calendar_details_calendar_id ON calendar_details(calendar_id);
                CREATE INDEX IF NOT EXISTS idx_calendar_details_calendar_id ON calendar_details(calendar_id);

            -- house_rules:
                CREATE INDEX IF NOT EXISTS idx_house_rules_campaign_id ON house_rules(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_house_rules_campaign_id ON house_rules(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_house_rules_name ON house_rules(name);

            -- players:
                CREATE INDEX IF NOT EXISTS idx_players_name ON players(name);
                CREATE INDEX IF NOT EXISTS idx_players_nickname ON players(nickname);

            -- player_availability:
                CREATE INDEX IF NOT EXISTS idx_player_availability_player_id ON player_availability(player_id);
                CREATE INDEX IF NOT EXISTS idx_player_availability_campaign_id ON player_availability(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_player_availability_campaign_id ON player_availability(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_player_availability_player_id ON player_availability(player_id);

            -- characters:
                CREATE INDEX IF NOT EXISTS idx_characters_player_id ON characters(player_id);
                CREATE INDEX IF NOT EXISTS idx_characters_campaign_id ON characters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_characters_campaign_id ON characters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_characters_player_id ON characters(player_id);
                CREATE INDEX IF NOT EXISTS idx_characters_name ON characters(name);

            -- character_details:
                CREATE INDEX IF NOT EXISTS idx_character_details_character_id ON character_details(character_id);
                CREATE INDEX IF NOT EXISTS idx_character_details_character_id ON character_details(character_id);

            -- races:
                CREATE INDEX IF NOT EXISTS idx_races_parent_race_id ON races(parent_race_id);
                CREATE INDEX IF NOT EXISTS idx_races_custom_to_world_id ON races(custom_to_world_id);
                CREATE INDEX IF NOT EXISTS idx_races_custom_to_campaign_id ON races(custom_to_campaign_id);
                CREATE INDEX IF NOT EXISTS idx_races_name ON races(name);
                CREATE INDEX IF NOT EXISTS idx_races_is_standard_race ON races(is_standard_race);
                CREATE INDEX IF NOT EXISTS idx_races_is_subrace ON races(is_subrace);

            -- race_details:
                CREATE INDEX IF NOT EXISTS idx_race_details_race_id ON race_details(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_details_race_id ON race_details(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_details_size_category ON race_details(size_category);
                CREATE INDEX IF NOT EXISTS idx_race_details_vision_type ON race_details(vision_type);
                CREATE INDEX IF NOT EXISTS idx_race_details_type ON race_details(type);

            -- race_traits:
                CREATE INDEX IF NOT EXISTS idx_race_traits_race_id ON race_traits(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_traits_race_id ON race_traits(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_traits_name ON race_traits(name);
                CREATE INDEX IF NOT EXISTS idx_race_traits_trait_type ON race_traits(trait_type);

            -- classes:
                CREATE INDEX IF NOT EXISTS idx_classes_world_id ON classes(world_id);
                CREATE INDEX IF NOT EXISTS idx_classes_campaign_id ON classes(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_classes_campaign_id ON classes(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_classes_world_id ON classes(world_id);
                CREATE INDEX IF NOT EXISTS idx_classes_name ON classes(name);

            -- class_details:
                CREATE INDEX IF NOT EXISTS idx_class_details_class_id ON class_details(class_id);
                CREATE INDEX IF NOT EXISTS idx_class_details_class_id ON class_details(class_id);

            -- monsters:
                CREATE INDEX IF NOT EXISTS idx_monsters_world_id ON monsters(world_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_campaign_id ON monsters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_campaign_id ON monsters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_world_id ON monsters(world_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_name ON monsters(name);

            -- monster_details:
                CREATE INDEX IF NOT EXISTS idx_monster_details_monster_id ON monster_details(monster_id);
                CREATE INDEX IF NOT EXISTS idx_monster_details_monster_id ON monster_details(monster_id);

            -- items:
                CREATE INDEX IF NOT EXISTS idx_items_world_id ON items(world_id);
                CREATE INDEX IF NOT EXISTS idx_items_campaign_id ON items(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_items_campaign_id ON items(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_items_world_id ON items(world_id);
                CREATE INDEX IF NOT EXISTS idx_items_name ON items(name);

            -- item_details:
                CREATE INDEX IF NOT EXISTS idx_item_details_item_id ON item_details(item_id);
                CREATE INDEX IF NOT EXISTS idx_item_details_item_id ON item_details(item_id);

            -- spells:
                CREATE INDEX IF NOT EXISTS idx_spells_world_id ON spells(world_id);
                CREATE INDEX IF NOT EXISTS idx_spells_campaign_id ON spells(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_spells_campaign_id ON spells(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_spells_world_id ON spells(world_id);
                CREATE INDEX IF NOT EXISTS idx_spells_name ON spells(name);

            -- spell_details:
                CREATE INDEX IF NOT EXISTS idx_spell_details_spell_id ON spell_details(spell_id);
                CREATE INDEX IF NOT EXISTS idx_spell_details_spell_id ON spell_details(spell_id);


        ".to_string(),
        down_sql: "
            DROP TABLE IF EXISTS classes;
            DROP TABLE IF EXISTS class_details;
            DROP TABLE IF EXISTS monsters;
            DROP TABLE IF EXISTS monster_details;
            DROP TABLE IF EXISTS items;
            DROP TABLE IF EXISTS item_details;
            DROP TABLE IF EXISTS spells;
            DROP TABLE IF EXISTS spell_details;
        ".to_string(),
    }
} 