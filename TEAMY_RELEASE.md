# Teamy Facet release preparation

These packages are maintained by TeamDman in this fork. They are independently
published packages with their own Rust type identities, rather than official
Facet releases. Original upstream contributors, copyright notices, and the
MIT OR Apache-2.0 licensing are retained.

The reviewed source checkpoint is
[345ebb7ba42f7c6644c1d89ef53288fe81204fa5](https://github.com/TeamDman/facet/commit/345ebb7ba42f7c6644c1d89ef53288fe81204fa5).
Its upstream baseline is
[65bae5c31a7ce401bc44630fb96250ea884cfd3e](https://github.com/facet-rs/facet/commit/65bae5c31a7ce401bc44630fb96250ea884cfd3e),
with [Cow ownership PR #2657](https://github.com/facet-rs/facet/pull/2657) and
[enum-alias PR #2664](https://github.com/facet-rs/facet/pull/2664).
Release preparation changes package names, dependency identities, and fork
documentation on top of that checkpoint. The active integration branch is
`teamy-main`; older `main` history is retained separately.

## Packages and publication order

All packages in this repository's release use version `0.50.0-rc.7`.
Exact internal requirements keep the cooperating reflection and macro APIs aligned.
Their `teamy-` package prefix distinguishes this release from official Facet.

Publish the groups in this order; packages in each group have their dependencies
available from earlier groups:

1. `teamy-facet-core`, `teamy-facet-macro-types`,
   `teamy-facet-testhelpers-macros`.
2. `teamy-facet-path`, `teamy-facet-macro-parse`,
   `teamy-facet-testhelpers`.
3. `teamy-facet-reflect`, `teamy-facet-macros-impl`.
4. `teamy-facet-solver`, `teamy-facet-pretty`, `teamy-facet-macros`.
5. `teamy-facet`.
6. `teamy-facet-error`, `teamy-facet-default`.

This is 11 runtime packages plus three default/test support packages. Internal
development dependencies use path-only package aliases so Cargo omits
them from normalized archives. This prevents development cycles from blocking
the first publication; complete local package/consumer validation before
publication. The source-tree-only
`facet-showcase`, `facet-testattrs`, and `facet-validate` packages are private
and excluded from release. `facet-macro-template` is an inactive, unpublished
helper outside the workspace.

The matching Format, Dessert, JSON, and Value forks are maintained in the sibling
[TeamDman/facet-format](https://github.com/TeamDman/facet-format) repository.
The matching Figue and attribute forks are maintained in
[TeamDman/figue](https://github.com/TeamDman/figue).
Use those Teamy packages throughout a consuming graph. Official JSON/Value
packages depend on official Facet identities and cannot replace them.

## Installation

Canonical dependency aliases and Rust library names remain unchanged because
derive macros and extension attributes generate paths such as `::facet`,
`::facet_error`, and `::figue`.

```toml
[dependencies]
facet = { package = "teamy-facet", version = "=0.50.0-rc.7" }
facet-reflect = { package = "teamy-facet-reflect", version = "=0.50.0-rc.7" }
facet-json = { package = "teamy-facet-json", version = "=0.50.0-rc.7" }
figue = { package = "teamy-figue", version = "=6.0.0-rc.1" }
```

The registry dependencies above become usable after publication. Local manifests
include versioned paths for development; Cargo removes those paths when packaging.
A downstream consumer must not need `[patch.crates-io]` to use the resulting
published Teamy dependency graph.

## Verification before publication

Historical converted-field defaults are retained by the current upstream scalar
conversion implementation. The focused regression compares reflected defaults
with generated Default values for an unsuffixed floating-point literal on f32,
a string literal on String, and an integer literal on u8; no older callback
implementation needs to be transplanted.

Run checks with the pinned Rust 1.96.0 toolchain. Check the selected libraries,
reflection/default behavior, enum aliases, retained Cow ownership behavior, and
package documentation. Inspect the normalized manifests and licenses in every
archive, and confirm that production dependencies contain registry versions and
Teamy package names. Validate a fresh consumer outside the fork workspaces with
one Facet-core identity, including its CLI attribute and serialization paths.

`cargo package --list --allow-dirty -p teamy-facet` previews archive contents.
`cargo package --allow-dirty -p teamy-facet` verifies an extracted archive.
Path-only internal development dependencies, including private helpers, are
omitted by Cargo during packaging. Complete source-tree tests are not an archive
test guarantee; source checkouts retain the full development graph.

Use the full source workspace for its existing tests, and run archive/consumer
checks against a disposable local registry before authorizing actual publication.
`--no-verify` alone does not demonstrate that an archive builds.

The upstream sponsors, website material, and project credits retained in this
repository describe upstream Facet. They do not imply upstream ownership,
endorsement, or sponsorship of this fork.

See [Cargo packaging](https://doc.rust-lang.org/cargo/commands/cargo-package.html)
and [dependency aliases](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml).
