import { Migration } from './Migration';

export class CreateCustomTables extends Migration {
    public version = 3;
    public description = 'Create custom tables';

    public async up(): Promise<void> {
        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS classes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS class_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                class_id INTEGER NOT NULL,
                
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        
        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS monsters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS monster_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                monster_id INTEGER NOT NULL,
                
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        
        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS item_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                item_id INTEGER NOT NULL,
                
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        
        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS spells (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                campaign_id INTEGER NOT NULL,
                world_id INTEGER NOT NULL,
                name TEXT,
                description TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);

        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS spell_details (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                spell_id INTEGER NOT NULL,
                
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP
            )
        `);
    }

    public async down(): Promise<void> {
        await this.executeQuery('DROP TABLE IF EXISTS classes');
        await this.executeQuery('DROP TABLE IF EXISTS class_details');
        await this.executeQuery('DROP TABLE IF EXISTS monsters');
        await this.executeQuery('DROP TABLE IF EXISTS monster_details');
        await this.executeQuery('DROP TABLE IF EXISTS items');
        await this.executeQuery('DROP TABLE IF EXISTS item_details');
        await this.executeQuery('DROP TABLE IF EXISTS spells');
        await this.executeQuery('DROP TABLE IF EXISTS spell_details');
    }
} 