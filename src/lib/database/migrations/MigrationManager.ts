import { Database } from 'sqlite3';
import { Migration } from './Migration';

export class MigrationManager {
    private db: Database;
    private migrations: Migration[] = [];

    constructor(db: Database) {
        this.db = db;
        Migration.setDatabase(db);
    }

    public registerMigration(migration: Migration) {
        this.migrations.push(migration);
    }

    public async initialize(): Promise<void> {
        await this.createMigrationsTable();
    }

    private async createMigrationsTable(): Promise<void> {
        await this.executeQuery(`
            CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        `);
    }

    public async migrate(): Promise<void> {
        const appliedMigrations = await this.getAppliedMigrations();
        const pendingMigrations = this.migrations
            .filter(m => !appliedMigrations.includes(m.version))
            .sort((a, b) => a.version - b.version);

        for (const migration of pendingMigrations) {
            await this.applyMigration(migration);
        }
    }

    public async rollback(steps: number = 1): Promise<void> {
        const appliedMigrations = await this.getAppliedMigrations();
        const migrationsToRollback = this.migrations
            .filter(m => appliedMigrations.includes(m.version))
            .sort((a, b) => b.version - a.version)
            .slice(0, steps);

        for (const migration of migrationsToRollback) {
            await this.rollbackMigration(migration);
        }
    }

    private async getAppliedMigrations(): Promise<number[]> {
        return new Promise((resolve, reject) => {
            this.db.all('SELECT version FROM migrations ORDER BY version', (err: Error | null, rows: any[]) => {
                if (err) reject(err);
                resolve(rows.map(row => row.version));
            });
        });
    }

    private async applyMigration(migration: Migration): Promise<void> {
        await migration.up();
        await this.executeQuery(
            'INSERT INTO migrations (version, description) VALUES (?, ?)',
            [migration.version, migration.description]
        );
    }

    private async rollbackMigration(migration: Migration): Promise<void> {
        await migration.down();
        await this.executeQuery(
            'DELETE FROM migrations WHERE version = ?',
            [migration.version]
        );
    }

    private async executeQuery(query: string, params: any[] = []): Promise<void> {
        return new Promise((resolve, reject) => {
            this.db.run(query, params, (err: Error | null) => {
                if (err) reject(err);
                resolve();
            });
        });
    }
} 