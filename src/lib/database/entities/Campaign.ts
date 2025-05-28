import type { BaseEntityData } from './BaseEntity';
import { BaseEntity } from './BaseEntity';

export interface CampaignData extends BaseEntityData {
    name: string;
    description?: string;
    settings?: Record<string, any>;
}

export class Campaign extends BaseEntity {
    protected static tableName = 'campaigns';
    
    public name: string;
    public description?: string;
    public settings?: Record<string, any>;

    constructor(data: CampaignData) {
        super(data);
        this.name = data.name;
        this.description = data.description;
        this.settings = data.settings;
    }

    protected async update(): Promise<void> {
        return new Promise((resolve, reject) => {
            Campaign.db.run(
                `UPDATE ${Campaign.tableName} 
                SET name = ?, description = ?, settings = ?, updated_at = ?, deleted_at = ?
                WHERE id = ?`,
                [
                    this.name,
                    this.description,
                    JSON.stringify(this.settings),
                    this.updated_at,
                    this.deleted_at,
                    this.id
                ],
                (err: Error | null) => {
                    if (err) reject(err);
                    resolve();
                }
            );
        });
    }

    protected async insert(): Promise<void> {
        return new Promise((resolve, reject) => {
            Campaign.db.run(
                `INSERT INTO ${Campaign.tableName} 
                (name, description, settings, created_at, updated_at, deleted_at)
                VALUES (?, ?, ?, ?, ?, ?)`,
                [
                    this.name,
                    this.description,
                    JSON.stringify(this.settings),
                    this.created_at,
                    this.updated_at,
                    this.deleted_at
                ],
                function(this: { lastID: number }, err: Error | null) {
                    if (err) reject(err);
                    if (this.lastID) {
                        Campaign.prototype.id = this.lastID;
                    }
                    resolve();
                }
            );
        });
    }

    public static async findByName(name: string): Promise<Campaign | null> {
        return new Promise((resolve, reject) => {
            this.db.get(
                `SELECT * FROM ${this.tableName} WHERE name = ? AND deleted_at IS NULL`,
                [name],
                (err: Error | null, row: any) => {
                    if (err) reject(err);
                    resolve(row ? new Campaign(row) : null);
                }
            );
        });
    }
} 