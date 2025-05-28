import { Database, RunResult } from 'sqlite3';

export interface BaseEntityData {
    id?: number;
    created_at?: string;
    updated_at?: string;
    deleted_at?: string | null;
}

export abstract class BaseEntity {
    protected static tableName: string;
    protected static db: Database;

    public id?: number;
    public created_at?: string;
    public updated_at?: string;
    public deleted_at?: string | null;

    constructor(data: BaseEntityData = {}) {
        this.id = data.id;
        this.created_at = data.created_at;
        this.updated_at = data.updated_at;
        this.deleted_at = data.deleted_at;
    }

    public static setDatabase(db: Database) {
        this.db = db;
    }

    public static async findById<T extends BaseEntity>(this: new (data: any) => T, id: number): Promise<T | null> {
        return new Promise((resolve, reject) => {
            this.db.get(
                `SELECT * FROM ${this.tableName} WHERE id = ? AND deleted_at IS NULL`,
                [id],
                (err: Error | null, row: any) => {
                    if (err) reject(err);
                    resolve(row ? new this(row) : null);
                }
            );
        });
    }

    public static async findAll<T extends BaseEntity>(this: new (data: any) => T): Promise<T[]> {
        return new Promise((resolve, reject) => {
            this.db.all(
                `SELECT * FROM ${this.tableName} WHERE deleted_at IS NULL`,
                (err: Error | null, rows: any[]) => {
                    if (err) reject(err);
                    resolve(rows.map(row => new this(row)));
                }
            );
        });
    }

    public async save(): Promise<void> {
        const now = new Date().toISOString();
        if (this.id) {
            // Update
            this.updated_at = now;
            await this.update();
        } else {
            // Insert
            this.created_at = now;
            this.updated_at = now;
            await this.insert();
        }
    }

    public async softDelete(): Promise<void> {
        if (!this.id) throw new Error('Cannot delete unsaved entity');
        this.deleted_at = new Date().toISOString();
        await this.update();
    }

    protected abstract update(): Promise<void>;
    protected abstract insert(): Promise<void>;
} 