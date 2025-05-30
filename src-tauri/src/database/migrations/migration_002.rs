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
        ".to_string(),
        down_sql: "
            DROP TABLE IF EXISTS players;
            DROP TABLE IF EXISTS player_availability;
            DROP TABLE IF EXISTS characters;
            DROP TABLE IF EXISTS character_details;
        ".to_string(),
    }
} 