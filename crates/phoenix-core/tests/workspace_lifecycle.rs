use std::fs;

use phoenix_core::{
    Error,
    commands::{create_workspace, open_workspace},
    migrations::CURRENT_SCHEMA_VERSION,
    storage::Workspace,
};
use rusqlite::Connection;
use tempfile::tempdir;

#[test]
fn creates_closes_and_reopens_a_workspace() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("example.phx");

    let created = create_workspace(&path).unwrap();
    assert_eq!(created.format, "phoenix");
    assert_eq!(created.schema_version, CURRENT_SCHEMA_VERSION);

    let reopened = open_workspace(&path).unwrap();
    assert_eq!(reopened, created);

    let connection = Connection::open(&path).unwrap();
    let migration: u32 = connection
        .query_row("SELECT version FROM schema_migrations", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(migration, CURRENT_SCHEMA_VERSION);
}

#[test]
fn never_overwrites_an_existing_file() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("notes.phx");
    fs::write(&path, "important data").unwrap();

    assert!(matches!(
        Workspace::create(&path),
        Err(Error::WorkspaceAlreadyExists(existing)) if existing == path
    ));
    assert_eq!(fs::read_to_string(path).unwrap(), "important data");
}

#[test]
fn rejects_non_phoenix_sqlite_files() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("other.phx");
    Connection::open(&path).unwrap();

    assert!(matches!(
        Workspace::open(&path),
        Err(Error::InvalidWorkspace(_))
    ));
}

#[test]
fn rejects_newer_workspace_schemas() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("future.phx");
    create_workspace(&path).unwrap();
    let connection = Connection::open(&path).unwrap();
    connection.pragma_update(None, "user_version", 2).unwrap();
    drop(connection);

    assert!(matches!(
        Workspace::open(&path),
        Err(Error::UnsupportedSchema {
            found: 2,
            supported: 1
        })
    ));
}
