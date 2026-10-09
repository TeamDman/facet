# teamy-facet-macro-types

**TeamDman-maintained fork.** This fork uses the package name [teamy-facet-macro-types](https://crates.io/crates/teamy-facet-macro-types), independently of the upstream [facet-macro-types](https://github.com/facet-rs/facet/tree/65bae5c31a7ce401bc44630fb96250ea884cfd3e/facet-macro-types) package. It retains upstream MIT OR Apache-2.0 licensing and attribution. Use matching Teamy Facet packages throughout a dependency graph; official Facet traits and shapes are separate Rust types. See the [fork release notes](https://github.com/TeamDman/facet/blob/teamy-main/TEAMY_RELEASE.md) for provenance and installation.


[![crates.io](https://img.shields.io/crates/v/teamy-facet-macro-types.svg)](https://crates.io/crates/teamy-facet-macro-types)
[![documentation](https://docs.rs/teamy-facet-macro-types/badge.svg)](https://docs.rs/teamy-facet-macro-types)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/teamy-facet-macro-types.svg)](./LICENSE-MIT)
[![Discord](https://img.shields.io/discord/1379550208551026748?logo=discord&label=discord)](https://discord.gg/JhD7CwCJ8F)

Defines the unsynn grammar and type definitions used throughout the facet macro ecosystem.

This crate provides the foundational AST types that represent Rust type declarations (structs, enums, unions) as parsed from token streams, enabling the derive macro infrastructure.

## Sponsors

Thanks to all individual sponsors:

<p> <a href="https://github.com/sponsors/fasterthanlime">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/github-dark.svg">
<img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/github-light.svg" height="40" alt="GitHub Sponsors">
</picture>
</a> <a href="https://patreon.com/fasterthanlime">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/patreon-dark.svg">
    <img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/patreon-light.svg" height="40" alt="Patreon">
    </picture>
</a> </p>

...along with corporate sponsors:

<p> <a href="https://aws.amazon.com">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/aws-dark.svg">
<img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/aws-light.svg" height="40" alt="AWS">
</picture>
</a> <a href="https://zed.dev">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/zed-dark.svg">
<img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/zed-light.svg" height="40" alt="Zed">
</picture>
</a> <a href="https://depot.dev?utm_source=facet">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/depot-dark.svg">
<img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v3/depot-light.svg" height="40" alt="Depot">
</picture>
</a> </p>

...without whom this work could not exist.

## Special thanks

The facet logo was drawn by [Misiasart](https://misiasart.com/).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/facet-rs/facet/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/facet-rs/facet/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
