import { Migration } from './Migration';

export class CreateImportantTables extends Migration {
    public version = 2;
    public description = 'Create important tables';

    public async up(): Promise<void> {
        await this.executeQuery(`
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
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS player_availability (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER DEFAULT null,
                player_id INTEGER NOT NULL,
                day TEXT NOT NULL DEFAULT '',
                time_slot TEXT DEFAULT '',
                available BOOLEAN NOT NULL DEFAULT false,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                player_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS character_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                character_id INTEGER NOT NULL,
                class TEXT NOT NULL,
                race TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);
    }

    public async down(): Promise<void> {
        await this.executeQuery('DROP TABLE IF EXISTS players');
        await this.executeQuery('DROP TABLE IF EXISTS player_availability');
        await this.executeQuery('DROP TABLE IF EXISTS characters');
        await this.executeQuery('DROP TABLE IF EXISTS character_details');
    }
} 