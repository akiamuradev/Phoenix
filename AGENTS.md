# Synnoema implementation rules

## Identity and stack

Synnoema, by akiamuradev, is licensed GPL-3.0-or-later. The canonical repository
is https://github.com/akiamuradev/Synnoema-Workspace. Synnoema Workspace uses
`.synoema`: a SQLite database, not a custom binary container. Preserve the stable
application ID, format metadata, explicit schema versions, and ordered migrations.
No legacy workspace compatibility is required; do not introduce it.

Use Rust for `synnoema-core`, Tauri 2 as the replaceable shell, and React with
TypeScript 5.9.x for application UI. Change the TypeScript baseline only in a
deliberate toolchain task. The large canvas direction is imperative PixiJS 8/WebGL,
outside React reconciliation; do not center the renderer on `@pixi/react`.

## Authority and integrity

- `synnoema-core` MUST NOT depend on Tauri. Keep coherent core modules together;
  extract crates only when real boundaries justify it.
- SQLite behind core APIs is canonical persistence. Frontend state and renderer
  state are not authoritative domain state. Replacing the UI must preserve data.
- All persistent mutations go through explicit core commands and validated
  transactions. The frontend and extensions MUST NOT directly mutate SQLite.
- Treat workspace input as untrusted. Verify SQLite validity, application ID,
  format metadata, and supported schema; never trust the filename alone.
- Preserve atomic operations, meaningful errors, deterministic ordered migrations,
  and rejection of unsupported future schemas. Avoid panics in normal file handling.
- Workspace contents are data, never executable scripts. Rebuildable FTS5, R*Tree,
  and in-memory indexes must not become sole owners of domain information.

## Knowledge and projections

The Canvas is a view, not the data model. Synnoema must never assume that a
workspace graph is rendered in full. The canvas presents a user-controlled
projection of workspace knowledge. Entities, graphs, and relationships may remain
collapsed and are progressively revealed through interaction. Knowledge state and
rendering/view state are separate concepts: existence, membership, and relationships
must not be confused with visibility, expansion, filters, focus, or viewport state.

Persistent UUIDv7 identities are independent of visible labels: duplicate titles
are valid. Placements and coordinates are separable from entity identity. Multiple
graphs retain independent contexts and support cross-graph bridges and graph
relationships. Multiple views may reference the same knowledge without duplicating it.
Use selective projection first, then viewport culling and level of detail. Never
solve scalability by forcing every object into the scene.

Progressive reveal is fundamental: Physics may reveal Mechanics/Electricity/Optics;
Electricity may reveal Voltage/Current/Resistance/Ohm's law; the user may then
reveal an external relationship to Mathematics. Two Physics cards may have distinct
identities. Do not freeze a complete View schema before its milestone.

Core concepts evolve around Workspace, Entity, Graph, Placement, Source, Anchor,
RelationType, Edge, and View/Projection. Custom relation types are persistent DATA,
never a closed enum; built-in defaults use the same model. Sources and anchors are
first-class objects. Provider-specific logic stays outside the graph core.

## Extensibility

Design for controlled extension APIs: extension → command API → core → validated
transaction → SQLite. Renderers, source adapters, layouts, commands, import/export,
themes, AI providers, context actions, reveal strategies, and domain visualizations
are possible capabilities, not commitments to implement them now.

Extensions MUST NOT own fundamental entities or storage. Optional extensions must
not be required to open fundamental workspace structure. Missing extensions degrade
gracefully while preserving identity, relationships, placements, metadata, sources,
and anchors. Preserve unknown opaque extension metadata where safe, using stable
namespaces such as `org.example.timeline`, not display names. Normal migrations
must not casually discard it. Do not execute metadata.

Prefer data-driven customization before executable plugins. Do not implement a
plugin ABI prematurely: preserve boundaries first. Through 0.1.0, avoid speculative
SDKs, marketplaces, downloaders, sandboxes, WASM/JS/native hosts, signing systems,
permission UI, scripting languages, or frameworks with no current caller.
See [the architecture rationale](docs/architecture/PROJECTION_AND_EXTENSIBILITY.md).

## Product and delivery discipline

“5 minutes to start, 5 years to reach the ceiling.” Normal users need no programming.
Keep UI calm and workspace-oriented: left navigation/resources, central canvas,
contextual editing/creation, and an optional future AI drawer. Avoid neon/cyberpunk,
unnecessary glassmorphism, a permanent inspector or terminal, and marketing flows.
Do not polish the temporary welcome screen or turn Synnoema into visual programming.

Before each milestone read this file, ROADMAP.md, relevant code, migrations, and
tests. Define the smallest coherent implementation and document deviations.
Resolve stale roadmap prose deliberately; never silently change architectural rules.
If a proposed change conflicts with these rules, stop and report the conflict.
Do not start 0.0.2 while main CI is red. Do not skip milestones or mix later features
into earlier milestones. Avoid major dependency upgrades unless necessary for the
current milestone or a justified compatibility/security repair.

Use a branch from synchronized main, Conventional Commits, a PR, and green CI
before merging. Delete merged remote/local branches and synchronize main. Never
force-push main, destroy history, or rewrite published tags. Tag completed numbered
milestones only after merge and green main CI. Keep Rust/Tauri/npm versions aligned.
The historical v0.0.1 tag is immutable. Do not commit build output or test databases.

Required checks: `npm ci`, `npm run format:check`, `npm run typecheck`,
`npm run lint`, `npm run build`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets --all-features -- -D warnings`, and
`cargo test --workspace --all-features`. Preserve the Linux/Windows/macOS CI matrix
and explicit Linux desktop dependencies. Inspect actual logs on failure; do not
remove checks or suppress legitimate failures to turn CI green.

Test real domain invariants, rollback, migrations, identity, and reopening. Report
each milestone's version, branch, architecture decisions, migrations/tests, local
and CI results, PR, merge commit, tag, limitations, and deferred work. Never claim
tests, GUI execution, merges, or releases that were not verified.
