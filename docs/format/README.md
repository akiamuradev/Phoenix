# Phoenix workspace format

A `.phx` workspace is a versioned SQLite database. The extension identifies intended use; the SQLite header and Phoenix application ID identify the actual file format.

## Foundation schema (version 1)

SQLite `PRAGMA application_id` is `0x50485801`. `PRAGMA user_version` is the authoritative integer schema version.

The initial migration creates:

- `workspace_metadata(key, value)` for stable format metadata;
- `schema_migrations(version, applied_at)` for an auditable migration history.

Schema version 1 stores `format = phoenix` and `schema_version = 1`. Opening a workspace checks its application ID, supported schema version, migration history, and format marker. Newer schemas are rejected instead of being modified by an older Phoenix build.

Workspace connections enable foreign-key enforcement and close automatically on drop. Public lifecycle commands also close explicitly so deferred SQLite errors can be reported.

The schema is intentionally tiny. Graph entities, relations, sources, anchors, and placements enter through later explicit migrations after their models are validated.
