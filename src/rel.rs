use std::{
    borrow::Borrow,
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

use std::fmt;

use ref_cast::RefCast;

/// Equivalent to [PathBuf], but guaranteed to be relative.
#[derive(PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct RelPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be relative.
#[derive(RefCast, PartialEq, Eq)]
#[repr(transparent)]
pub struct RelPath(Path);

impl fmt::Debug for RelPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for RelPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl RelPathBuf {
    pub fn new<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_rel_path_buf()
    }
}

impl AsRef<Path> for RelPathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

pub trait ToRelPathBuf: AsRef<Path> {
    fn to_rel_path_buf(&self) -> io::Result<RelPathBuf>;
}

impl<P: AsRef<Path>> ToRelPathBuf for P {
    fn to_rel_path_buf(&self) -> io::Result<RelPathBuf> {
        let path = self.as_ref();
        if path.is_relative() {
            Ok(RelPathBuf(path.to_path_buf()))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be relative",
            ))
        }
    }
}

impl Deref for RelPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RelPathBuf {
    type Target = RelPath;

    fn deref(&self) -> &Self::Target {
        RelPath::ref_cast(&self.0)
    }
}

impl Borrow<RelPath> for RelPathBuf {
    fn borrow(&self) -> &RelPath {
        self
    }
}

impl ToOwned for RelPath {
    type Owned = RelPathBuf;

    fn to_owned(&self) -> Self::Owned {
        RelPathBuf(self.0.to_owned())
    }
}

impl RelPathBuf {
    pub fn as_path(&self) -> &Path {
        self
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RelPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RelPathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path_buf = PathBuf::deserialize(deserializer)?;
        if path_buf.is_relative() {
            Ok(RelPathBuf(path_buf))
        } else {
            Err(serde::de::Error::custom("path must be relative"))
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RelPath {
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
    use serde_test::{Token, assert_de_tokens_error, assert_tokens};

    #[test]
    fn test_rel_path_buf_serialize() {
        let path_buf = RelPathBuf::new("relative/path.txt").unwrap();
        assert_tokens(&path_buf, &[Token::Str("relative/path.txt")]);
    }

    #[test]
    fn test_rel_path_buf_deserialize() {
        let path_buf = RelPathBuf::new("relative/path.txt").unwrap();
        assert_tokens(&path_buf, &[Token::Str("relative/path.txt")]);
    }

    #[test]
    fn test_rel_path_buf_deserialize_invalid() {
        assert_de_tokens_error::<RelPathBuf>(
            &[Token::Str("/absolute/path")],
            "path must be relative",
        );
    }

    #[test]
    fn test_rel_path_serialize() {
        let path_buf = RelPathBuf::new("relative/path.txt").unwrap();
        let rel_path: &RelPath = &path_buf;
        use serde_test::{Token, assert_ser_tokens};
        assert_ser_tokens(&rel_path, &[Token::Str("relative/path.txt")]);
    }
}
