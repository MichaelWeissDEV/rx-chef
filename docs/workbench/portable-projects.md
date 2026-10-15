# Portable projects and recovery

A portable definition is different from a local library and from execution data.
Use exports to move reviewed definitions between browser, desktop and CLI.
Rebind data in the destination client and execute explicitly.

## Formats and compatibility

| Format | Contains | Does not contain |
|---|---|---|
| `.rxgraph.json`, version 1 | One graph, ports, arguments, edges, presentation positions | Input bindings, result bytes, connection token |
| `.rxproject.json`, version 1 | Graphs, recipes, selected definitions, theme, external file references, bounded run metadata | Bundled external files, execution payloads, credentials |
| Legacy CLI project JSON/YAML | Pipeline, variables and an explicit data source | Workbench session state |

Legacy `project init` creates a `.rxchef` Store marker. It does not create a
portable workbench project. Legacy project `run` can intentionally resolve its
configured file input; portable `input_files` references never cause file reads.
See [legacy project commands](../cli/project.md) and [storage](../concepts/storage.md).

A minimal portable project is:

<!-- rxchef-example: portable-project -->
```json
{
  "version": 1,
  "name": "Empty portable project",
  "graphs": [],
  "recipes": [],
  "settings": {"theme": "dark"},
  "input_files": [],
  "history": []
}
```

Validate exported projects before distributing them:

```sh
rxchef project validate session.rxproject.json
rxchef project inspect session.rxproject.json
```

Unknown fields and unsupported versions are rejected. Do not remove a version
field or silently reinterpret a newer document as version 1. Preserve the source
and use a compatible client or an explicit reviewed migration.

## Validation boundaries

| Boundary | Version-1 limit / rule |
|---|---|
| Serialized project | At most 16 MiB; source must be a regular file |
| Definitions | At most 100 graphs and 100 recipes |
| Recipe | At most 512 steps; known operations and valid active flow |
| Input references | At most 512; `external` must be true; graph/input must exist |
| Run metadata | At most 1,000 entries; unique entry ID, RFC3339 time |
| Duration | Finite, nonnegative, at most 604,800,000 ms |
| Names | Nonempty, at most 256 UTF-8 bytes, no control characters |
| Selection | Selected graph/recipe must exist |
| Identity | Unique graph IDs, recipe names and graph/input reference pairs |

Each graph also passes [graph validation and budgets](../concepts/graphs.md).
Known sensitive arguments are checked even in disabled definitions and embedded
recipes. This is schema-based secret handling, not a detector for arbitrary
personal data in every string: review ordinary literal arguments before sharing.
External reference paths themselves can reveal local directory names.

## Libraries, commits and quotas

Browser local saves use IndexedDB transactions. Native Store graph/project
repositories use bounded validated files and cooperative writer locks. A save
that exceeds a count/byte budget must fail without replacing the previous valid
entry. A rename collision must preserve both existing definitions. Renaming a
storage key retains graph identity; duplicating creates a distinct identity.
Portable storage keys reject path traversal, Windows device names and case-only
collisions. These rules differ from legacy recipe-name sanitization.

The legacy recipe/variable Store's read-modify-write concurrency limitations do
not describe the newer graph/project repository locks. Locks coordinate
cooperating writers; path preflight checks do not close hostile same-owner
root-directory replacement races. Unix graph/project roots/files are tested for
0700/0600 modes. Native Windows owner-only ACL behavior remains unverified.

## Backup, transfer and recovery

1. Finish or cancel active work. Export each important project to a new file.
2. Keep input data separately with independently recorded byte lengths/hashes.
3. Validate exports using the destination version before removing any source.
4. Import/open in the destination client. Confirm graph/recipe selections and
   external-reference metadata. Rebind every required input deliberately.
5. Run a known fixture and compare exact output bytes before using real data.

For a denied/full browser store, export the in-memory session before closing it.
Close older tabs if a database upgrade is blocked. A malformed saved record may
prevent listing; back up site/profile data before clearing anything. An injected
quota regression test demonstrates error handling, not a disk-capacity guarantee.

For filesystem corruption, stop writers and copy the entire repository before
repair. Keep the failing original, validate a repaired copy, then replace it via
the repository API. Do not delete an existing entry merely to make a failed save
appear successful. File/export validation failures must leave prior entries intact.
