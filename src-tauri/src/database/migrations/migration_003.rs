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