# teamy-facet-macros-impl

**TeamDman-maintained fork.** This fork uses the package name [teamy-facet-macros-impl](https://crates.io/crates/teamy-facet-macros-impl), independently of the upstream [facet-macros-impl](https://github.com/facet-rs/facet/tree/65bae5c31a7ce401bc44630fb96250ea884cfd3e/facet-macros-impl) package. It retains upstream MIT OR Apache-2.0 licensing and attribution. Use matching Teamy Facet packages throughout a dependency graph; official Facet traits and shapes are separate Rust types. See the [fork release notes](https://github.com/TeamDman/facet/blob/teamy-main/TEAMY_RELEASE.md) for provenance and installation.


<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo +nightly reedme --package teamy-facet-macros-impl

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->


[![crates.io](https://img.shields.io/crates/v/teamy-facet-macros-impl.svg)](https://crates.io/crates/teamy-facet-macros-impl)
[![documentation](https://docs.rs/teamy-facet-macros-impl/badge.svg)](https://docs.rs/teamy-facet-macros-impl)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/teamy-facet-macros-impl.svg)](./LICENSE-MIT)
[![Discord](https://img.shields.io/discord/1379550208551026748?logo=discord&label=discord)](https://discord.gg/JhD7CwCJ8F)

Implementation of facet derive macros, combining parsing and code generation.

This crate provides the internal implementation for `#[derive(Facet)]` and related procedural macros. It’s used by `facet-macros` (the proc-macro crate) and should not be used directly.

### Sponsors

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

…along with corporate sponsors:

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

…without whom this work could not exist.

### Special thanks

The facet logo was drawn by [Misiasart](https://misiasart.com/).

### License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/facet-rs/facet/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/facet-rs/facet/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

<!-- cargo-reedme: end -->
