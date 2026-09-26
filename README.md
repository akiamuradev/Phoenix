# Synnoema

**Synnoema is an open-source local-first environment for building, exploring and connecting large spaces of knowledge.**

Synnoema treats documents, media, notes, concepts, and graphs not as isolated files, but as addressable objects that can be connected into multiple independent knowledge graphs. A Synnoema Workspace is stored as a portable `.synoema` file.

> Sources contain information.<br>
> Synnoema stores how that information relates.

## Status

**Version: 0.0.1 — foundation**

Synnoema is at the architectural foundation stage. The file format, domain model, and internal boundaries are being established before the visual graph editor. Expect breaking changes until the format reaches a stable specification.

## Why Synnoema?

Knowledge rarely exists in one format. One subject may involve books, PDFs, lectures, local media, office documents, web pages, notes, concepts, and arguments. File systems store these separately; note applications mostly connect notes; whiteboards mostly connect visual cards.

Synnoema instead makes relationships between information first-class data. A PDF paragraph can connect to a video timestamp, a concept in one graph can connect to a concept in another, and users can create their own relationship types.

## Core concepts

- **Workspace:** a portable `.synoema` SQLite file containing a knowledge space.
- **Graph:** an independent knowledge context and a first-class object.
- **Entity:** a persistent object, independent of its canvas representation.
- **Source:** an external or local information resource whose identity Synnoema stores.
- **Anchor:** an addressable fragment of a source, such as pages, text, or a time range.
- **Relation type:** extensible data describing the semantic meaning of a relationship.
- **Edge:** a persisted relationship between two entities.
- **Placement:** the visual presence and position of an entity in a graph.
- **Bridge:** a relationship crossing graph boundaries.

Sources may exist with no relationships. The same entity may have different placements in multiple graphs without duplication. Cross-graph relationships preserve different contexts for the same idea rather than flattening everything into one canvas.

## Architecture

Synnoema is designed around **large knowledge spaces, small visible projections**.
The user chooses what to reveal: a Physics card may unfold into Electricity, then
Ohm's law, then a relationship to Mathematics. Matching card titles do not imply
matching identities. Multiple views can explore the same knowledge without copying it.

“5 minutes to start, 5 years to reach the ceiling.” Data-driven customization comes
first; future extensions can add adapters, renderers, commands, layouts, and reveal
strategies through controlled core APIs. Missing optional extensions must preserve
readable workspace structure. Projection and extension behavior is planned, not
implemented in the foundation. See the
[architecture rationale](docs/architecture/PROJECTION_AND_EXTENSIBILITY.md) and
[implementation rules](AGENTS.md).

```text
React + TypeScript application UI
             │
             │ Tauri IPC
             ▼
 Synnoema Core / synnoema-core
            Rust
             │
             ▼
       SQLite (*.synoema)
```

The canvas is a view, not the database. `synnoema-core` owns authoritative mutations, validation, transactions, storage, and migrations without depending on Tauri. The desktop shell is replaceable infrastructure.

The long-term canvas renderer will use PixiJS/WebGL independently of React. Source providers such as PDF and YouTube remain adapters and do not leak into the graph model.

## Repository

```text
apps/desktop/                 React frontend and Tauri 2 shell
crates/synnoema-core/          UI-independent Rust core
docs/format/                  Workspace format documentation
```

## Development

Prerequisites:

- Rust 1.85 or newer;
- Node.js 22 or newer and npm;
- the [Tauri 2 system dependencies](https://v2.tauri.app/start/prerequisites/) for your platform.

```bash
npm ci
cargo test --workspace
npm run typecheck
npm run lint
npm run build
npm run dev
```

The UI can create and reopen a workspace path. The same lifecycle is covered directly through `synnoema-core` integration tests, with no graphical application required.

## Design principles

1. The canvas is a view, not the database.
2. Sources, anchors, graphs, and relationships are first-class objects.
3. Relationship types are extensible data, not a closed enum.
4. Different graphs preserve different contexts.
5. External integrations do not leak into the domain model.
6. The desktop shell remains replaceable.
7. Complexity appears only when the user needs it.
8. A `.synoema` file survives replacement of the UI implementation.

## License

Synnoema is free and open-source software licensed under the [GNU General Public License v3.0 or later](LICENSE) (`GPL-3.0-or-later`).

Copyright (C) 2026 akiamuradev.

See [ROADMAP.md](ROADMAP.md) for planned milestones and [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution workflow.
