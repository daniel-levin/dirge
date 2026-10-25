//! [<img alt="crates.io" src="https://img.shields.io/crates/v/dirge.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dirge)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dirge?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dirge)
//! [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/daniel-levin/dirge/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/daniel-levin/dirge/actions)
//!
//! **Strongly-typed** path types.
//!
//! This crate has several goals:
//!
//! - Enhance correctness through specific types.
//! - Be conducive to re-exporting.
//! - Be portable.

#![deny(unsafe_code)]

pub use camino::Utf8Path as Path;
pub use camino::Utf8PathBuf as PathBuf;

mod abs;
mod abs_norm;
mod norm;
mod rel;
mod rel_norm;

pub use abs::AbsPath;
pub use abs::AbsPathBuf;
pub use abs::ToAbsPathBuf;
pub use abs_norm::AbsNormPath;
pub use abs_norm::AbsNormPathBuf;
pub use abs_norm::ToAbsNormPathBuf;
pub use norm::NormPath;
pub use norm::NormPathBuf;
pub use norm::ToNormPathBuf;
pub use rel::RelPath;
pub use rel::RelPathBuf;
pub use rel::ToRelPathBuf;
pub use rel_norm::RelNormPath;
pub use rel_norm::RelNormPathBuf;
pub use rel_norm::ToRelNormPathBuf;

fn from_std_path_buf(path: std::path::PathBuf) -> std::io::Result<PathBuf> {
    PathBuf::from_path_buf(path).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "path is not valid UTF-8")
    })
}

#[cfg(all(test, unix))]
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
