use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::ops::Deref;

use ref_cast::RefCast;

use crate::Path;
use crate::PathBuf;

/// Equivalent to [PathBuf], but guaranteed to be relative.
#[derive(PartialEq, Eq, Clone, Hash)]
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

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for RelPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for RelPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
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

impl RelPath {
    #[cfg(feature = "camino")]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[cfg(not(feature = "camino"))]
    pub fn as_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    #[cfg(feature = "camino")]
    pub fn to_str(&self) -> &str {
        self.as_str()
    }

    #[cfg(not(feature = "camino"))]
    pub fn to_str(&self) -> Option<&str> {
        self.as_str()
    }
}

impl RelPathBuf {
    #[cfg(feature = "camino")]
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    #[cfg(not(feature = "camino"))]
    pub fn into_string(self) -> Result<String, Self> {
        self.0
            .into_os_string()
            .into_string()
            .map_err(|path| Self(path.into()))
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
    use serde_test::Token;
    use serde_test::assert_de_tokens_error;
    use serde_test::assert_tokens;

    use super::*;

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
            &[Token::Str(if cfg!(windows) {
                r"C:\absolute\path"
            } else {
                "/absolute/path"
            })],
            "path must be relative",
        );
    }

    #[test]
    fn test_rel_path_serialize() {
        let path_buf = RelPathBuf::new("relative/path.txt").unwrap();
        let rel_path: &RelPath = &path_buf;
        use serde_test::Token;
        use serde_test::assert_ser_tokens;
        assert_ser_tokens(&rel_path, &[Token::Str("relative/path.txt")]);
    }
}
