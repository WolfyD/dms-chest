use rusqlite::{Connection, Result as SqliteResult, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

mod migrations;

#[derive(Debug)]
pub enum DatabaseError {
    SqliteError(rusqlite::Error),
    LockError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::SqliteError(e) => write!(f, "Database error: {}", e),
            DatabaseError::LockError(e) => write!(f, "Lock error: {}", e),
        }
    }
}

impl Error for DatabaseError {}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        DatabaseError::SqliteError(err)
    }
}

pub struct DatabaseState(Mutex<Connection>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Migration {
    version: i32,
    description: String,
    up_sql: String,
    down_sql: String,
}

pub fn init_database() -> SqliteResult<Connection> {
    let db_path = PathBuf::from("dms-chest.db");
    let conn = Connection::open(db_path)?;

    // Create migrations table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            version INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    Ok(conn)
}

#[tauri::command]
pub async fn initialize_database(state: State<'_, DatabaseState>) -> Result<(), String> {
    let mut conn = state.0.lock()
        .map_err(|_| DatabaseError::LockError("Failed to acquire database lock".to_string()))
        .map_err(|e| e.to_string())?;
    
    // Run migrations
    let migrations = get_pending_migrations(&conn)
        .map_err(|e| e.to_string())?;
    
    for migration in migrations {
        apply_migration(&mut conn, &migration)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn get_pending_migrations(conn: &Connection) -> Result<Vec<Migration>, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT version FROM migrations ORDER BY version"
    )?;
    
    let applied_versions: Vec<i32> = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?
    .collect::<SqliteResult<Vec<i32>>>()?;

    // Get all available migrations
    let available_migrations = migrations::get_all_migrations();
    
    // Filter out already applied migrations
    let pending_migrations: Vec<Migration> = available_migrations
        .into_iter()
        .filter(|m| !applied_versions.contains(&m.version))
        .collect();

    Ok(pending_migrations)
}

fn apply_migration(conn: &mut Connection, migration: &Migration) -> Result<(), DatabaseError> {
    println!("Applying migration: {}", migration.description);

    let tx = conn.transaction()?;

    // Execute the migration using execute_batch for multiple statements
    tx.execute_batch(&migration.up_sql)?;

    // If the migration fails, log the error
    if let Err(e) = tx.commit() {
        println!("Migration failed: {}", e);
    } else {
        println!("Migration applied successfully: {}", migration.description);
    }

    let tx = conn.transaction()?;

    // Record the migration
    tx.execute(
        "INSERT INTO migrations (version, description) VALUES (?, ?)",
        params![migration.version, &migration.description],
    )?;

    // Commit the transaction
    tx.commit()?;

    Ok(())
}

pub fn get_database_state() -> DatabaseState {
    DatabaseState(Mutex::new(init_database().expect("Failed to initialize database")))
} 