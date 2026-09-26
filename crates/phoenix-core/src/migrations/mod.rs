use rusqlite::{Connection, Transaction};

use crate::Result;

pub const CURRENT_SCHEMA_VERSION: u32 = 1;
pub const PHOENIX_APPLICATION_ID: u32 = 0x5048_5801;

struct Migration {
    version: u32,
    apply: fn(&Transaction<'_>) -> rusqlite::Result<()>,
}

const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    apply: migration_1,
}];

pub(crate) fn apply_all(connection: &mut Connection) -> Result<()> {
    let current = schema_version(connection)?;
    if current > CURRENT_SCHEMA_VERSION {
        return Err(crate::Error::UnsupportedSchema {
            found: current,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }

    for migration in MIGRATIONS.iter().filter(|item| item.version > current) {
        let transaction = connection.transaction()?;
        (migration.apply)(&transaction)?;
        transaction.pragma_update(None, "user_version", migration.version)?;
        transaction.commit()?;
    }

    Ok(())
}

pub(crate) fn schema_version(connection: &Connection) -> Result<u32> {
    Ok(connection.pragma_query_value(None, "user_version", |row| row.get(0))?)
}

fn migration_1(transaction: &Transaction<'_>) -> rusqlite::Result<()> {
    transaction.execute_batch(
        "CREATE TABLE workspace_metadata (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL
        ) STRICT;
        CREATE TABLE schema_migrations (
            version INTEGER PRIMARY KEY NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        ) STRICT;
        INSERT INTO workspace_metadata (key, value) VALUES
            ('format', 'phoenix'),
            ('schema_version', '1');
        INSERT INTO schema_migrations (version) VALUES (1);",
    )
}
