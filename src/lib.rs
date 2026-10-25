//! [<img alt="crates.io" src="https://img.shields.io/crates/v/dirge.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dirge)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dirge?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dirge)
//! [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/daniel-levin/dirge/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/daniel-levin/dirge/actions)
//!
//! **Strongly-typed** path types.
//!
//! ## Examples
//!
//! ### Absolute paths are just paths
//! ```
//! # fn example() -> AbsPathBuf { AbsPathBuf::new(".").unwrap() }
//! use std::path::Path;
//!
//! use dirge::AbsPath;
//! use dirge::AbsPathBuf;
//!
//! let a: AbsPathBuf = example();
//! let b: &AbsPath = &a;
//! let c: &Path = &a;
//! let d: &Path = &b;
//!
//! /// Precondition enforced by type system
//! fn needs_abs_path(p: &AbsPath) {
//! # // ...
//! }
//!
//! /// But they go anywhere the standard library's paths do!
//! let _ = std::fs::read_to_string(&a);
//! ```
//!
//! ### Relative paths provide similar guarantees
//! ```
//! use std::path::Path;
//!
//! use dirge::RelPath;
//! use dirge::RelPathBuf;
//!
//! let rel = RelPathBuf::new("src/main.rs").unwrap();
//! let rel_ref: &RelPath = &rel;
//! let path_ref: &Path = &rel;
//!
//! /// Type system enforces relative path requirement
//! fn needs_relative_path(p: &RelPath) {
//! # // ...
//! }
//!
//! needs_relative_path(&rel);
//! ```
//!
//! ### Normalized paths remove redundant components
//! ```
//! use std::path::Path;
//!
//! use dirge::NormPath;
//! use dirge::NormPathBuf;
//!
//! let norm = NormPathBuf::new("path/./to/../file.txt").unwrap();
//! assert_eq!(norm.as_path(), Path::new("path/file.txt"));
//!
//! let norm_ref: &NormPath = &norm;
//! let path_ref: &Path = &norm;
//!
//! /// Type system guarantees normalized paths
//! fn needs_normalized_path(p: &NormPath) {
//! # // ...
//! }
//!
//! needs_normalized_path(&norm);
//! ```
//!
//! ## Background
//! This crate provides portable extensions to the standard library's path
//! functionality. Our types have specific usages while incurring no storage
//! overhead. For example, [AbsPathBuf] and [std::path::PathBuf] have identical
//! in-memory representations, but the former is guaranteed to be an absolute
//! path.
//!
//! This crate has several goals:
//!
//! - Enhance correctness through specific types.
//! - Be conducive to re-exporting.
//! - Be portable.

#![deny(unsafe_code)]

#[cfg(feature = "camino")]
pub(crate) type PathBuf = camino::Utf8PathBuf;

#[cfg(not(feature = "camino"))]
pub(crate) type PathBuf = std::path::PathBuf;

mod abs;
mod norm;
mod rel;

pub use abs::AbsPath;
pub use abs::AbsPathBuf;
pub use abs::ToAbsPathBuf;
pub use norm::NormPath;
pub use norm::NormPathBuf;
pub use norm::ToNormPathBuf;
pub use rel::RelPath;
pub use rel::RelPathBuf;
pub use rel::ToRelPathBuf;
