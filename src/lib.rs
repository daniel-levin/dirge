//! ## Examples
//!
//! ### Absolute paths are just paths
//! ```
//! # fn example() -> AbsPathBuf { AbsPathBuf::new(".").unwrap() }
//!
//! use dirge::{AbsPath, AbsPathBuf};
//! use std::path::Path;
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
//! let x = std::fs::read_to_string(&a);
//! ```
//!
//!
//! ## Background
//! This crate provides portable extensions to the standard library's path functionality.
//! Our types have specific usages while incurring no storage overhead. For example, [AbsPathBuf]
//! and [std::path::PathBuf] have identical in-memory representations, but the former is guaranteed
//! to be an absolute path.
//!
//! This crate has several goals:
//!
//! - Enhance correctness through specific types.
//! - Be conducive to re-exporting.
//! - Be portable.

mod abs;
mod rel;

pub use abs::{AbsPath, AbsPathBuf, ToAbsPathBuf, absolutize};
