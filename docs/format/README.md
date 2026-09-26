# Synnoema Workspace format

A `.synoema` workspace is a versioned SQLite database. The extension identifies intended use; the SQLite header and Synnoema application ID identify the actual file format.

## Foundation schema (version 1)

SQLite `PRAGMA application_id` is the stable 32-bit value `0x53594E4F` (`SYNO` in ASCII). `PRAGMA user_version` is the authoritative integer schema version.

The initial migration creates:

- `workspace_metadata(key, value)` for stable format metadata;
- `schema_migrations(version, applied_at)` for an auditable migration history.

Schema version 1 stores `format = Synnoema Workspace` and `schema_version = 1`. Opening a workspace checks its application ID, supported schema version, migration history, and format marker. Newer schemas are rejected instead of being modified by an older Synnoema build.

The canonical extension is `.synoema`, and the preferred MIME type is `application/x-synnoema-workspace`. The extension is not trusted as proof of identity: a renamed SQLite file is valid only when its application ID, format marker, and schema are all valid.

Workspace connections enable foreign-key enforcement and close automatically on drop. Public lifecycle commands also close explicitly so deferred SQLite errors can be reported.

The schema is intentionally tiny. Graph entities, relations, sources, anchors, and placements enter through later explicit migrations after their models are validated.

Future View/Projection data must remain separable from canonical knowledge:
visibility or collapse does not determine existence. Multiple views reference the
same persistent objects. Exact view storage is not fixed in schema 1.

Optional extensions must not become required to read fundamental workspace
structure. Future metadata should use stable namespaces, preserve unknown opaque
data where safe through normal migrations, and never execute workspace contents.
Extensions request mutations through core commands, not raw SQL. See the
[architectural rationale](../architecture/PROJECTION_AND_EXTENSIBILITY.md).
