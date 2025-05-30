mod migration_001;
mod migration_002;
mod migration_003;

use super::Migration;

pub fn get_all_migrations() -> Vec<Migration> {
    vec![
        migration_001::get_migration(),
        migration_002::get_migration(),
        migration_003::get_migration(),
    ]
} 