use super::super::Migration;

pub fn get_migration() -> Migration {
    Migration {
        version: 3,
        description: "Create custom tables".to_string(),
        up_sql: "
            CREATE TABLE IF NOT EXISTS classes (
                id INTEGER PRIMARY KEY,
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
                id INTEGER PRIMARY KEY,
                class_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (class_id) REFERENCES classes(id)
            );

            CREATE TABLE IF NOT EXISTS monsters (
                id INTEGER PRIMARY KEY,
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
                id INTEGER PRIMARY KEY,
                monster_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (monster_id) REFERENCES monsters(id)
            );

            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
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
                id INTEGER PRIMARY KEY,
                item_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (item_id) REFERENCES items(id)
            );

            CREATE TABLE IF NOT EXISTS spells (
                id INTEGER PRIMARY KEY,
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
                id INTEGER PRIMARY KEY,
                spell_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (spell_id) REFERENCES spells(id)
            );

            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY,
                tag_name TEXT NOT NULL UNIQUE,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS tag_relationships (
                id INTEGER PRIMARY KEY,
                tag_id INTEGER NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,

                FOREIGN KEY (tag_id) REFERENCES tags(id),
                FOREIGN KEY (entity_id) REFERENCES campaigns(id),
                FOREIGN KEY (entity_id) REFERENCES campaign_arcs(id),
                FOREIGN KEY (entity_id) REFERENCES campaign_npcs(id),
                FOREIGN KEY (entity_id) REFERENCES npcs(id),
                FOREIGN KEY (entity_id) REFERENCES factions(id),
                FOREIGN KEY (entity_id) REFERENCES campaign_factions(id),
                FOREIGN KEY (entity_id) REFERENCES worlds(id),
                FOREIGN KEY (entity_id) REFERENCES locations(id),
                FOREIGN KEY (entity_id) REFERENCES maps(id),
                FOREIGN KEY (entity_id) REFERENCES calendars(id),
                FOREIGN KEY (entity_id) REFERENCES house_rules(id),
                FOREIGN KEY (entity_id) REFERENCES players(id),
                FOREIGN KEY (entity_id) REFERENCES player_availability(id),
                FOREIGN KEY (entity_id) REFERENCES characters(id),
                FOREIGN KEY (entity_id) REFERENCES races(id),
                FOREIGN KEY (entity_id) REFERENCES race_traits(id),
                FOREIGN KEY (entity_id) REFERENCES classes(id),
                FOREIGN KEY (entity_id) REFERENCES monsters(id),
                FOREIGN KEY (entity_id) REFERENCES items(id),
                FOREIGN KEY (entity_id) REFERENCES spells(id)
            );




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
            DROP TABLE IF EXISTS tags;
            DROP TABLE IF EXISTS tag_relationships;
        ".to_string(),
    }
} 