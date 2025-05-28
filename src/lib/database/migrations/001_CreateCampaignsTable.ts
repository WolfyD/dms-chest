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
    }

    public async down(): Promise<void> {
        await this.executeQuery('DROP TABLE IF EXISTS campaigns');
    }
} 