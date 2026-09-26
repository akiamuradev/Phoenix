use std::path::Path;

use crate::{Result, model::WorkspaceMetadata, storage::Workspace};

/// Creates, validates, and safely closes a new workspace.
///
/// # Errors
///
/// Returns an error when the path already exists or the workspace cannot be
/// created, migrated, inspected, or closed.
pub fn create_workspace(path: impl AsRef<Path>) -> Result<WorkspaceMetadata> {
    let workspace = Workspace::create(path)?;
    let metadata = workspace.metadata()?;
    workspace.close()?;
    Ok(metadata)
}

/// Opens, validates, and safely closes an existing workspace.
///
/// # Errors
///
/// Returns an error when the path is missing, is not a Phoenix workspace, has
/// an unsupported schema, or cannot be inspected or closed.
pub fn open_workspace(path: impl AsRef<Path>) -> Result<WorkspaceMetadata> {
    let workspace = Workspace::open(path)?;
    let metadata = workspace.metadata()?;
    workspace.close()?;
    Ok(metadata)
}
