# Database Migrations Guide

## Overview
This project uses a migration system to manage database schema changes. Migrations are versioned and tracked to ensure database consistency across different environments and deployments.

## Migration Structure
Each migration is a class that extends the `Migration` abstract class and implements:
- `version`: A unique number identifying the migration
- `description`: A brief description of what the migration does
- `up()`: Method to apply the migration
- `down()`: Method to rollback the migration

## Creating a New Migration

1. Create a new file in the `migrations` directory with the format: `XXX_Description.ts`
   ```typescript
   import { Migration } from './Migration';

   export class CreateNewTable extends Migration {
       public version = 4; // Increment from the last migration
       public description = 'Create new table for feature X';

       public async up(): Promise<void> {
           await this.executeQuery(`
               CREATE TABLE IF NOT EXISTS new_table (
                   id INTEGER PRIMARY KEY AUTOINCREMENT,
                   name TEXT NOT NULL,
                   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
               )
           `);
       }

       public async down(): Promise<void> {
           await this.executeQuery('DROP TABLE IF EXISTS new_table');
       }
   }
   ```

2. Register the migration in `DatabaseManager.ts`:
   ```typescript
   private registerMigrations(): void {
       // ... existing migrations ...
       this.migrationManager.registerMigration(new CreateNewTable());
   }
   ```

## Migration Commands

### Initialize Database
The database is automatically initialized when the application starts. This creates the migrations table if it doesn't exist.

### Apply Migrations
Migrations are automatically applied when the application starts. The system will:
1. Check which migrations have been applied
2. Apply any pending migrations in order
3. Record the applied migrations in the migrations table

### Rollback Migrations
To rollback migrations, use the `rollback` method in `MigrationManager`:
```typescript
await migrationManager.rollback(1); // Rollback 1 migration
```

## Best Practices

1. **Versioning**
   - Always increment version numbers sequentially
   - Never reuse version numbers
   - Use clear, descriptive names for migration files

2. **Migration Content**
   - Keep migrations focused and atomic
   - Include both `up()` and `down()` methods
   - Use `IF NOT EXISTS` for table creation
   - Use `IF EXISTS` for table deletion
   - Always include proper error handling

3. **Testing**
   - Test migrations in development before deploying
   - Verify both `up()` and `down()` methods
   - Check data integrity after migrations

4. **Documentation**
   - Document complex migrations with comments
   - Update this README when adding new migration patterns
   - Keep track of migration dependencies

## Common Issues

1. **Version Conflicts**
   - If you get a version conflict, check the migrations table
   - Ensure no duplicate version numbers exist
   - Verify migration order in `DatabaseManager.ts`

2. **Failed Migrations**
   - Check the error message for specific issues
   - Verify SQL syntax
   - Ensure all required tables exist
   - Check for foreign key constraints

3. **Rollback Issues**
   - Ensure `down()` methods are properly implemented
   - Check for dependencies before rolling back
   - Verify data integrity after rollback

## Migration Table Structure

The migrations table tracks applied migrations:
```sql
CREATE TABLE migrations (
    version INTEGER PRIMARY KEY,
    description TEXT NOT NULL,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
```

## Related Files
- `Migration.ts`: Base migration class
- `MigrationManager.ts`: Manages migration execution
- `DatabaseManager.ts`: Registers and initializes migrations 