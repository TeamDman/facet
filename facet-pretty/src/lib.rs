//! TeamDman-maintained fork of [facet-pretty](https://github.com/facet-rs/facet/tree/65bae5c31a7ce401bc44630fb96250ea884cfd3e/facet-pretty), published as
//! [teamy-facet-pretty](https://crates.io/crates/teamy-facet-pretty).
//! This fork uses its own Facet crate identities; use matching Teamy packages.
//! Upstream MIT OR Apache-2.0 licensing and attribution are retained.
//! See the [release notes](https://github.com/TeamDman/facet/blob/teamy-main/TEAMY_RELEASE.md).
//!
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
//!
//! [![crates.io](https://img.shields.io/crates/v/teamy-facet-pretty.svg)](https://crates.io/crates/teamy-facet-pretty)
//! [![documentation](https://docs.rs/teamy-facet-pretty/badge.svg)](https://docs.rs/teamy-facet-pretty)
//! [![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/teamy-facet-pretty.svg)](./LICENSE-MIT)
//! [![Discord](https://img.shields.io/discord/1379550208551026748?logo=discord&label=discord)](https://discord.gg/JhD7CwCJ8F)
//!
//! Provides pretty-printing capabilities for Facet types.
//!
//! Example:
//!
//! ```rust
//! use facet::Facet;
//! use facet_pretty::FacetPretty;
//!
//! #[derive(Debug, Facet)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let person = Person {
//!     name: "Alice".to_string(),
//!     age: 30,
//! };
//! println!("Default pretty-printing:");
//! println!("{}", person.pretty());
//! ```
//!
//! Produces the output:
//!
//! ```text
//! Person {
//!   name: "Alice",
//!   age: 30,
//! }
//! ```
#![doc = include_str!("../readme-footer.md")]

extern crate alloc;

mod color;
mod display;
mod printer;
mod shape;

pub use color::*;
pub use display::*;
pub use printer::*;
pub use shape::*;
