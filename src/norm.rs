use std::{
    borrow::Borrow,
    io,
    ops::Deref,
    path::{Component, Path, PathBuf},
};

use std::fmt;

use ref_cast::RefCast;

/// Equivalent to [PathBuf], but guaranteed to be normalized.
///
/// A normalized path has no `.` or `..` components and uses canonical separators.
#[derive(PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct NormPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be normalized.
///
/// A normalized path has no `.` or `..` components and uses canonical separators.
#[derive(RefCast, PartialEq, Eq)]
#[repr(transparent)]
pub struct NormPath(Path);

impl fmt::Debug for NormPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for NormPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl NormPathBuf {
    pub fn new<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_norm_path_buf()
    }
}

impl AsRef<Path> for NormPathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for NormPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

pub trait ToNormPathBuf: AsRef<Path> {
    fn to_norm_path_buf(&self) -> io::Result<NormPathBuf>;
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {
                // Skip "." components
            }
            Component::ParentDir => {
                // Handle ".." by popping the last component if possible
                if !normalized.pop() {
                    // If we can't pop (empty path), keep the ".."
                    normalized.push("..");
                }
            }
            _ => {
                normalized.push(component);
            }
        }
    }

    // If the path is now empty and the original wasn't, use "."
    if normalized.as_os_str().is_empty() && !path.as_os_str().is_empty() {
        normalized.push(".");
    }

    normalized
}

impl<P: AsRef<Path>> ToNormPathBuf for P {
    fn to_norm_path_buf(&self) -> io::Result<NormPathBuf> {
        let normalized = normalize_path(self.as_ref());
        Ok(NormPathBuf(normalized))
    }
}

impl Deref for NormPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for NormPathBuf {
    type Target = NormPath;

    fn deref(&self) -> &Self::Target {
        NormPath::ref_cast(&self.0)
    }
}

impl Borrow<NormPath> for NormPathBuf {
    fn borrow(&self) -> &NormPath {
        self
    }
}

impl ToOwned for NormPath {
    type Owned = NormPathBuf;

    fn to_owned(&self) -> Self::Owned {
        NormPathBuf(self.0.to_owned())
    }
}

impl NormPathBuf {
    pub fn as_path(&self) -> &Path {
        self
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for NormPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for NormPathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path_buf = PathBuf::deserialize(deserializer)?;
        // Always normalize during deserialization
        Ok(NormPathBuf(normalize_path(&path_buf)))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for NormPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use serde_test::{Token, assert_de_tokens, assert_tokens};

    #[test]
    fn test_norm_path_buf_serialize() {
        let path_buf = NormPathBuf::new("path/to/file.txt").unwrap();
        assert_tokens(&path_buf, &[Token::Str("path/to/file.txt")]);
    }

    #[test]
    fn test_norm_path_buf_deserialize() {
        let path_buf = NormPathBuf::new("path/to/file.txt").unwrap();
        assert_tokens(&path_buf, &[Token::Str("path/to/file.txt")]);
    }

    #[test]
    fn test_norm_path_buf_deserialize_normalizes() {
        let expected = NormPathBuf::new("path/file.txt").unwrap();
        assert_de_tokens(&expected, &[Token::Str("path/./to/../file.txt")]);
    }

    #[test]
    fn test_norm_path_serialize() {
        let path_buf = NormPathBuf::new("path/to/file.txt").unwrap();
        let norm_path: &NormPath = &path_buf;
        use serde_test::{Token, assert_ser_tokens};
        assert_ser_tokens(&norm_path, &[Token::Str("path/to/file.txt")]);
    }
}
