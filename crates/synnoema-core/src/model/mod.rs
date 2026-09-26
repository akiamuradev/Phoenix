/// Stable workspace metadata stored in every `.synoema` file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceMetadata {
    pub format: String,
    pub schema_version: u32,
}
