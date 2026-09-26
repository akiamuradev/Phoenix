# Progressive knowledge projection and open extensibility

This is a permanent architectural direction, not a claim that the foundation
already implements projections or extensions. Concrete schemas and APIs should
follow tested milestone requirements. The agent rules live in [AGENTS.md](../../AGENTS.md).

## Large knowledge space, small working context

A workspace may contain 100,000 entities, 250,000 relationships, and 500 independent
graphs while its canvas shows 20 entities, 25 relationships, and a few collapsed
graphs. These are illustrative sizes, not measured capacity claims. The user should
never need to see an entire workspace. The canvas is a user-controlled projection.

Consider four cards: Physics, Chemistry, Physics, Russian Language. They can denote
graphs, subjects, collections, entities, or other higher-level objects. Matching
titles do not imply matching persistent IDs. Expanding Physics may reveal Mechanics,
Thermodynamics, Electricity, and Optics. Expanding Electricity may reveal Voltage,
Current, Resistance, and Ohm's law. The user can then reveal an external relationship
from Ohm's law to Mathematics without merging both graphs or showing everything.

The same principle applies to expanding a cluster, source, relationship, bridge,
or saved view. Possible strategies include revealing neighbours, prerequisites,
contradictions, citations, evidence, related sources, and external relationships.
The vocabulary is open: reveal strategies select knowledge without changing the
fundamental domain model. Hiding or collapsing a branch never deletes that knowledge.

## Distinct responsibilities

| Responsibility  | Question                                  | Examples                                                                                                        |
| --------------- | ----------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Knowledge       | What exists and how is it related?        | IDs, entities, graphs, membership, edges, relation types, sources, anchors                                      |
| View/projection | What has the user chosen to see?          | selected objects/edges, expanded or collapsed structures, hidden branches, filters, focus, placements, viewport |
| Renderer        | How is that projection drawn efficiently? | scene objects, visible bounds, level of detail, GPU resources                                                   |

Placements may be persisted, but coordinates and presentation do not define entity
identity. Graph membership, entity existence, visibility, and expansion must not
be conflated. An entity may have several placements in different graphs/views.
Removing a view or renderer must not erase the referenced knowledge. Exact placement
and View schema ownership remains a future milestone decision, not a frozen contract.

Views such as Exam preparation, Physics ↔ Mathematics, Only my notes, Sources and
evidence, Historical development, Contradictions, and Current research select
different subsets of the same knowledge. Creating them does not duplicate entities.
Graphs retain separate contexts; bridges are relationships across those contexts,
not an instruction to flatten the workspace into one force-directed graph.

The intended read path is workspace/core queries → selected projection → render
model → canvas. Retrieve only the needed slice with bounded/paged queries as the
model develops. Selective projection is the primary usability and scalability
mechanism. Viewport culling then omits offscreen members of that projection; semantic
zoom changes their representation. None of these requires a complete scene in memory.
React owns chrome, not reconciliation of the large graph scene. Imperative PixiJS/
WebGL renders the relevant projection. FTS5, R*Tree, and in-memory indexes are
rebuildable infrastructure, not canonical owners of knowledge.

## Customization and extension boundaries

Synnoema aims to be simple by default and deeply extensible when needed:
“5 minutes to start, 5 years to reach the ceiling.” A normal user never needs to
program. Persistent custom relation types are the first example of customization
as data: explains, supports, contradicts, requires, historically influenced,
read before, or my counterargument all use an extensible model. Built-in defaults
must not become a closed enum. Templates, views, and presentation rules may later
apply the same principle without requiring executable code.

Possible future extensions include source adapters, renderers, layouts, commands,
importers/exporters, themes, AI providers, context actions, reveal strategies,
custom viewers, and domain visualizations. Zotero, arXiv, GitHub, Wikipedia, Spotify,
and custom APIs illustrate adapters. Flashcards, equations, chemical structures,
timelines, code, 3D objects, and argument maps illustrate renderers. Tree, radial,
timeline, force-directed, hierarchical, and argument layouts illustrate presentation
choices. These examples are not roadmap commitments or a reason to build a registry
with no current caller.

Persistent mutation follows extension → controlled Synnoema command API →
synnoema-core → validated transaction → SQLite. Direct extension SQL writes and
dependencies on undocumented tables are forbidden. The core owns invariants and
schema evolution; this boundary enables future undo/redo, history, recovery,
permissions, and isolation without committing to an implementation now.

A renderer interprets an entity; it does not own its identity or persistence.
The core supplies entities, metadata, capabilities, relationships, sources, and
anchors. Extensions supply presentation, actions, and interpretation. Provider
details remain in adapters outside graph core. Removing an extension must not
destroy objects, graph structure, sources, anchors, or relationships.

## Missing extensions and metadata preservation

A workspace requiring `org.example.equations` for enhanced display still opens
without it. Preserve entity IDs, relationships, placements, source identity, anchors,
and metadata. Show a fallback such as “Renderer unavailable — requires:
org.example.equations” for affected objects while normal navigation remains usable.
An optional renderer is not a prerequisite for reading the fundamental workspace.

Use stable namespaced identifiers rather than human-readable extension titles.
Identity, metadata ownership, renderer ownership, capabilities, and dependencies
must be distinguishable. For example, future metadata might conceptually contain:

```json
{
  "org.example.timeline": {
    "start": "1945",
    "end": "1991",
    "displayMode": "period"
  }
}
```

This is an example, not a schema or ABI. The core need not understand every field.
It should store, preserve, and return opaque data safely, including during normal
migrations and while an extension is absent. Structural validation and appropriate
size limits can protect integrity without interpreting arbitrary extension semantics.
Unknown metadata is data, never executable code. A future extension API needs explicit
ownership and authorization rules; a namespace alone is not a permission grant.

## Open format and deliberate sequencing

Synnoema Workspace is direct SQLite under `.synoema`, identified by its stable
application ID, format marker, and version, independent of the filename. It remains
local-first and usable without proprietary cloud services. Document the format so
independent tools such as synnoema-cli, synnoema-export, synnoema-repair, and
synnoema-viewer can exist without reverse engineering. These are possibilities,
not current binaries. Extensions enhance access to knowledge rather than own it.

Before 0.1.0, preserve seams exercised by actual callers. Do not implement a plugin
SDK/ABI, marketplace, package repository/downloader, signing, permission UI,
runtime sandbox, WASM/JavaScript/native host, scripting language, visual programming,
sync, collaboration, full View schema ahead of its milestone, or graph query language
as part of this architectural documentation. Later milestones choose mechanisms
from real requirements. The workspace remains data even if future mods execute code.

## Foundation audit and future review criteria

The current core is independent from Tauri; SQLite lives behind core storage and
command APIs. The frontend invokes the shell's commands. The foundation has no
graph scene, projection schema, extension runtime, or closed relation vocabulary,
so there is no existing implementation conflict with these principles. Its welcome
UI is temporary and does not validate the future navigation design.

Future changes should demonstrate that duplicate titles retain distinct identities,
multiple views share underlying entities, collapse does not delete knowledge,
bounded projections work over larger datasets, and replacing a renderer preserves
domain state. When extensions become real, exercise missing-renderer fallbacks and
unknown-metadata round trips/migrations. Do not add pretend extension implementations
or tests solely to claim these future behaviors already exist.
