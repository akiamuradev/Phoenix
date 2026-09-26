# Synnoema Roadmap

This roadmap describes the shortest conservative path from an empty repository to the first usable Synnoema knowledge workspace. Early releases validate the data model and architecture before large integrations or automation.

Read [AGENTS.md](AGENTS.md) and the
[projection and extensibility architecture](docs/architecture/PROJECTION_AND_EXTENSIBILITY.md)
before implementation. Restore green main CI before beginning 0.0.2. Deliver each
milestone through a validated PR; tag it only after merge and green main CI.
The sections below describe intended scope, not a claim of completion.

## 0.0.1 — Foundation

**Goal:** create a repository whose architecture can survive the implementation of Synnoema.

- GPL-3.0-or-later project metadata, documentation, formatting, linting, and CI;
- Tauri 2 desktop shell with a React, TypeScript, and Vite frontend;
- independent Rust `synnoema-core` modules for model, storage, commands, and migrations;
- versioned SQLite `.synoema` creation, opening, validation, and safe close;
- a first explicit migration and core lifecycle tests.

Definition of done: Synnoema and its Rust core can create `example.synoema`, close it, reopen it, and verify its schema. No graph editor is required.

## 0.0.2 — Graph kernel

**Goal:** prove the fundamental object and relationship model.

Implement entities, graphs, placements, relation types, and edges with UUIDv7 identifiers. Relation types—including directedness, inverse labels, and user-defined types—are stored as data rather than a hard-coded enum. Add transactional creation, placement, movement, relation, and edge commands with persistence and rollback tests.

## 0.0.3 — Canvas kernel

**Goal:** display persisted Synnoema data visually for the first time.

Add a PixiJS renderer with pan, zoom, selection, nodes, edges, graph viewport, and persisted placement movement. React owns application chrome; PixiJS owns graph rendering. The architecture permits viewport culling from the beginning.

Use imperative PixiJS 8/WebGL outside React reconciliation. Start with a selected
projection, keeping domain objects separate from scene objects and visibility.

## 0.0.4 — Notes and source foundation

**Goal:** distinguish knowledge objects from external resources.

Add note and source entities, source capabilities, local-file identity, missing-source handling, metadata, and a provider-neutral anchor identity. A source with zero relationships is valid.

## 0.0.5 — PDF

**Goal:** implement the first rich source integration.

Add PDF.js viewing, local PDF identity, page and selected-region/text anchors, navigation to anchors, and relationships from anchors. Anchor recovery must not rely exclusively on absolute coordinates.

## 0.0.6 — YouTube

**Goal:** prove the source model with a fundamentally different provider.

Add URL recognition, source identity, embedded playback, timestamp and time-range anchors, and navigation. YouTube-specific logic remains outside the graph core.

## 0.0.7 — Multi-graph navigation

**Goal:** make multiple knowledge spaces usable rather than merely storable.

Add graph switching and overview, graph-to-graph and cross-graph relationships, bridge visualization, and navigation through bridges without flattening the workspace.

Collapsed cards may indicate external relationships; users choose when to reveal them.

## 0.0.8 — Semantic zoom and progressive projection

**Goal:** establish rendering architecture for very large workspaces.

Introduce graph-, cluster-, node-, source-, and anchor-level representations, level-of-detail switching, viewport culling, and basic performance benchmarks. Synnoema does not attempt to display an entire workspace simultaneously.

Solidify View/Projection state for visible entities/relationships, expansion,
collapse, hidden branches, viewport, focus, and suitable filters. Multiple views
share persistent identities. Exercise Physics → Electricity → Ohm's law → Mathematics
as progressive reveal, and benchmark large stored workspaces with small visible
projections. This is more than zooming a complete static scene.

## 0.0.9 — Search and navigation

**Goal:** make growing workspaces navigable.

Use SQLite FTS5 for notes, names, relation types, source metadata, and extracted PDF text. Add global search and navigation to entities, graphs, sources, and anchors. Semantic/vector search remains out of scope.

Search indexes are rebuildable. Results navigate to an appropriate projection
without revealing the entire workspace.

## 0.1.0 — First usable Synnoema

**Goal:** demonstrate the complete core idea.

A user can create and reopen a workspace, build multiple graphs, add notes and PDF/YouTube sources, create anchors and custom relation types, connect entities within and across graphs, and navigate those relationships visually without data loss.

Include collapse/progressive reveal, search, and persistence of supported view state.
The product must work without plugins. No plugin SDK/runtime, marketplace, cloud
sync, collaboration, office adapters, mobile clients, visual scripting, or AI graph
generation is required for this release. Preserve extension boundaries using only
abstractions with current callers.

## After 0.1.0

Real usage will determine the order of office-document and web sources, relation suggestions, semantic search, local AI, advanced queries, packages, import/export, synchronization, collaboration, plugin SDKs, and mobile clients.

## Permanent constraints

- `synnoema-core` remains independent from the desktop shell.
- The canvas is never authoritative storage.
- Custom relationship types remain part of the core format.
- Integrations remain adapters rather than graph-model assumptions.
- Workspace data remains portable and readable independently of its original UI.
- The canvas shows a user-controlled projection; knowledge and view state stay separate.
- Duplicate labels do not imply identity, and views do not duplicate knowledge.
- Extensions use validated core APIs, never direct SQLite mutation.
- Optional extensions do not own fundamental data; missing extensions degrade gracefully.
- Future opaque extension metadata uses stable namespaces and is preserved where safe.
