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
//! # #[cfg(not(feature = "camino"))]
//! use std::path::Path;
//!
//! # #[cfg(feature = "camino")]
//! # use camino::Utf8Path as Path;
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
//! # #[cfg(not(feature = "camino"))]
//! use std::path::Path;
//!
//! # #[cfg(feature = "camino")]
//! # use camino::Utf8Path as Path;
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
//! # #[cfg(not(feature = "camino"))]
//! use std::path::Path;
//!
//! # #[cfg(feature = "camino")]
//! # use camino::Utf8Path as Path;
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
//! ## Features
//! With `camino` enabled, all owned and borrowed path types wrap
//! `camino::Utf8PathBuf` and `camino::Utf8Path` instead of the standard
//! library path types. Constructors accept UTF-8 paths, and dereferencing
//! exposes Camino's string-based API. `AsRef<std::path::Path>` remains
//! available for filesystem APIs. Resolving an absolute path returns an error
//! if the resulting path (including the current directory) is not valid UTF-8.
//!
//! The `serde` feature supports either backend, including when both features
//! are enabled.
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
pub type Path = camino::Utf8Path;

#[cfg(not(feature = "camino"))]
pub type Path = std::path::Path;

#[cfg(feature = "camino")]
pub type PathBuf = camino::Utf8PathBuf;

#[cfg(not(feature = "camino"))]
pub type PathBuf = std::path::PathBuf;

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

fn from_std_path_buf(path: std::path::PathBuf) -> std::io::Result<PathBuf> {
    #[cfg(feature = "camino")]
    {
        camino::Utf8PathBuf::from_path_buf(path).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "path is not valid UTF-8")
        })
    }
    #[cfg(not(feature = "camino"))]
    {
        Ok(path)
    }
}

#[cfg(all(test, feature = "camino", unix))]
mod tests {
    #[test]
    fn rejects_non_utf8_resolved_paths() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let path = std::path::PathBuf::from(OsString::from_vec(b"/tmp/\xff".to_vec()));
        let error = super::from_std_path_buf(path).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }
}
