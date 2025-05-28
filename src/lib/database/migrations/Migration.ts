import { Database } from 'sqlite3';

export abstract class Migration {
    protected static db: Database;
    public abstract version: number;
    public abstract description: string;

    public static setDatabase(db: Database) {
        this.db = db;
    }

    public abstract up(): Promise<void>;
    public abstract down(): Promise<void>;

    protected async executeQuery(query: string, params: any[] = []): Promise<void> {
        return new Promise((resolve, reject) => {
            Migration.db.run(query, params, (err) => {
                if (err) reject(err);
                resolve();
            });
        });
    }
} 