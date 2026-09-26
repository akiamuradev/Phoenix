//! UI-independent Phoenix domain and persistence core.

pub mod commands;
pub mod migrations;
pub mod model;
pub mod storage;

use std::path::PathBuf;

/// Errors produced by Phoenix workspace operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("workspace already exists: {0}")]
    WorkspaceAlreadyExists(PathBuf),
    #[error("workspace does not exist: {0}")]
    WorkspaceNotFound(PathBuf),
    #[error("path is not a Phoenix workspace: {0}")]
    InvalidWorkspace(PathBuf),
    #[error("workspace schema {found} is newer than supported schema {supported}")]
    UnsupportedSchema { found: u32, supported: u32 },
    #[error("workspace schema is incomplete: expected {expected}, found {found}")]
    IncompleteSchema { expected: u32, found: u32 },
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
