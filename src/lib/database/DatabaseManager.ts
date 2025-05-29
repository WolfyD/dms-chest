import { Database } from 'sqlite3';
import { MigrationManager } from './migrations/MigrationManager';
import { BaseEntity } from './entities/BaseEntity';
import { CreateCampaignsTable } from './migrations/001_CreateInitialTables';
import { CreateImportantTables } from './migrations/002_CreateImportantTables';
import { CreateCustomTables } from './migrations/003_CreateCustomTables';

export class DatabaseManager {
    private static instance: DatabaseManager;
    private db: Database;
    private migrationManager: MigrationManager;

    private constructor() {
        this.db = new Database('dms-chest.db');
        this.migrationManager = new MigrationManager(this.db);
        this.initializeEntities();
        this.registerMigrations();
    }

    public static getInstance(): DatabaseManager {
        if (!DatabaseManager.instance) {
            DatabaseManager.instance = new DatabaseManager();
        }
        return DatabaseManager.instance;
    }

    private initializeEntities(): void {
        BaseEntity.setDatabase(this.db);
    }

    private registerMigrations(): void {
        // Register all migrations here
        this.migrationManager.registerMigration(new CreateCampaignsTable());
        this.migrationManager.registerMigration(new CreateImportantTables());
        this.migrationManager.registerMigration(new CreateCustomTables());
    }

    public async initialize(): Promise<void> {
        await this.migrationManager.initialize();
        await this.migrationManager.migrate();
    }

    public getDatabase(): Database {
        return this.db;
    }

    public async close(): Promise<void> {
        return new Promise((resolve, reject) => {
            this.db.close((err: Error | null) => {
                if (err) reject(err);
                resolve();
            });
        });
    }
} 