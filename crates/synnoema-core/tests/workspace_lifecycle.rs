use std::fs;

use synnoema_core::{
    Error,
    commands::{create_workspace, open_workspace},
    migrations::{
        CURRENT_SCHEMA_VERSION, SYNNOEMA_APPLICATION_ID, WORKSPACE_FORMAT_NAME,
        WORKSPACE_MIME_TYPE,
    },
    storage::Workspace,
};
use rusqlite::Connection;
use tempfile::tempdir;

#[test]
fn creates_closes_and_reopens_a_workspace() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("example.synoema");

    let created = create_workspace(&path).unwrap();
    assert_eq!(created.format, WORKSPACE_FORMAT_NAME);
    assert_eq!(created.schema_version, CURRENT_SCHEMA_VERSION);

    let reopened = open_workspace(&path).unwrap();
    assert_eq!(reopened, created);

    let connection = Connection::open(&path).unwrap();
    let application_id: u32 = connection
        .pragma_query_value(None, "application_id", |row| row.get(0))
        .unwrap();
    assert_eq!(application_id, SYNNOEMA_APPLICATION_ID);
    let migration: u32 = connection
        .query_row("SELECT version FROM schema_migrations", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(migration, CURRENT_SCHEMA_VERSION);
}

#[test]
fn exposes_the_canonical_workspace_mime_type() {
    assert_eq!(WORKSPACE_MIME_TYPE, "application/x-synnoema-workspace");
}

#[test]
fn refuses_to_create_the_obsolete_workspace_extension() {
    let directory = tempdir().unwrap();
    let old_path = directory.path().join("example.phx");

    assert!(matches!(
        Workspace::create(&old_path),
        Err(Error::InvalidWorkspaceExtension(path)) if path == old_path
    ));
    assert!(!old_path.exists());
}

#[test]
fn never_overwrites_an_existing_file() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("notes.synoema");
    fs::write(&path, "important data").unwrap();

    assert!(matches!(
        Workspace::create(&path),
        Err(Error::WorkspaceAlreadyExists(existing)) if existing == path
    ));
    assert_eq!(fs::read_to_string(path).unwrap(), "important data");
}

#[test]
fn rejects_non_synnoema_sqlite_files() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("other.synoema");
    Connection::open(&path).unwrap();

    assert!(matches!(
        Workspace::open(&path),
        Err(Error::InvalidWorkspace(_))
    ));
}

#[test]
fn rejects_a_workspace_with_the_wrong_application_identity() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("wrong-identity.synoema");
    create_workspace(&path).unwrap();
    let connection = Connection::open(&path).unwrap();
    connection.pragma_update(None, "application_id", 0).unwrap();
    drop(connection);

    assert!(matches!(
        Workspace::open(&path),
        Err(Error::InvalidWorkspace(_))
    ));
}

#[test]
fn rejects_a_workspace_with_the_wrong_format_marker() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("wrong-format.synoema");
    create_workspace(&path).unwrap();
    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE workspace_metadata SET value = 'Other Workspace' WHERE key = 'format'",
            [],
        )
        .unwrap();
    drop(connection);

    assert!(matches!(
        Workspace::open(&path),
        Err(Error::InvalidWorkspace(_))
    ));
}

#[test]
fn rejects_newer_workspace_schemas() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("future.synoema");
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
