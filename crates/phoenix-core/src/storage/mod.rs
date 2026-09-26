use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags};

use crate::{
    Error, Result,
    migrations::{CURRENT_SCHEMA_VERSION, PHOENIX_APPLICATION_ID, apply_all, schema_version},
    model::WorkspaceMetadata,
};

/// An open Phoenix workspace. Its `SQLite` connection closes on drop.
pub struct Workspace {
    connection: Connection,
    path: PathBuf,
}

impl Workspace {
    /// Creates a new workspace without overwriting an existing path.
    ///
    /// # Errors
    ///
    /// Returns an error when the path exists or `SQLite` cannot initialize and
    /// migrate the workspace.
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            return Err(Error::WorkspaceAlreadyExists(path.to_owned()));
        }

        let mut connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        configure(&connection)?;
        connection.pragma_update(None, "application_id", PHOENIX_APPLICATION_ID)?;
        apply_all(&mut connection)?;

        Ok(Self {
            connection,
            path: path.to_owned(),
        })
    }

    /// Opens and validates an existing Phoenix workspace.
    ///
    /// # Errors
    ///
    /// Returns an error when the path is missing, is not a Phoenix workspace,
    /// uses an unsupported schema, or cannot be opened by `SQLite`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(Error::WorkspaceNotFound(path.to_owned()));
        }

        let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        configure(&connection)?;
        validate(&connection, path)?;

        Ok(Self {
            connection,
            path: path.to_owned(),
        })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the persisted workspace format metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when required metadata is missing, malformed, or cannot
    /// be read.
    pub fn metadata(&self) -> Result<WorkspaceMetadata> {
        let format = metadata_value(&self.connection, "format")?;
        let stored_version = metadata_value(&self.connection, "schema_version")?
            .parse::<u32>()
            .map_err(|_| Error::InvalidWorkspace(self.path.clone()))?;

        Ok(WorkspaceMetadata {
            format,
            schema_version: stored_version,
        })
    }

    /// Explicitly closes the workspace and reports deferred `SQLite` errors.
    ///
    /// # Errors
    ///
    /// Returns any error reported while closing the `SQLite` connection.
    pub fn close(self) -> Result<()> {
        self.connection.close().map_err(|(_, error)| error.into())
    }
}

fn configure(connection: &Connection) -> Result<()> {
    connection.pragma_update(None, "foreign_keys", true)?;
    connection.busy_timeout(std::time::Duration::from_secs(5))?;
    Ok(())
}

fn validate(connection: &Connection, path: &Path) -> Result<()> {
    let application_id: u32 =
        connection.pragma_query_value(None, "application_id", |row| row.get(0))?;
    if application_id != PHOENIX_APPLICATION_ID {
        return Err(Error::InvalidWorkspace(path.to_owned()));
    }

    let version = schema_version(connection)?;
    if version > CURRENT_SCHEMA_VERSION {
        return Err(Error::UnsupportedSchema {
            found: version,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }
    if version < CURRENT_SCHEMA_VERSION {
        return Err(Error::IncompleteSchema {
            expected: CURRENT_SCHEMA_VERSION,
            found: version,
        });
    }

    let migration_count: u32 = connection.query_row(
        "SELECT COUNT(*) FROM schema_migrations WHERE version <= ?1",
        [CURRENT_SCHEMA_VERSION],
        |row| row.get(0),
    )?;
    if migration_count != CURRENT_SCHEMA_VERSION {
        return Err(Error::IncompleteSchema {
            expected: CURRENT_SCHEMA_VERSION,
            found: migration_count,
        });
    }

    let format = metadata_value(connection, "format")?;
    if format != "phoenix" {
        return Err(Error::InvalidWorkspace(path.to_owned()));
    }

    Ok(())
}

fn metadata_value(connection: &Connection, key: &str) -> Result<String> {
    Ok(connection.query_row(
        "SELECT value FROM workspace_metadata WHERE key = ?1",
        [key],
        |row| row.get(0),
    )?)
}
