# Changelog

All notable changes to the facet project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.42.0](https://github.com/TeamDman/facet/compare/facet-core-v0.41.0...facet-core-v0.42.0) - 2026-01-03

### Fixed

- *(soundness)* make OxRef::new and OxMut::new unsafe

### Other

- Add Facet implementation for smol_str::SmolStr
- Set up release-plz with synchronized versions and trusted publishing
- Add `facet_no_doc` cfg for global doc string stripping
- Fix facet-pretty to respect skip_serializing_if and add HTML roundtrip tests
- Add html::text attribute for enum variants and comprehensive roundtrip test
- Fix inconsistent Shape hash (issue #1574)
- Fix soundness issue: Attr can contain non-Sync data
- Require 'static for Opaque Facet impl
- *(facet-core)* simplify Ox API by requiring T: Facet
- fix broken intra-doc link to Peek in facet-core
- Improve AGENTS.md, closes #1551
