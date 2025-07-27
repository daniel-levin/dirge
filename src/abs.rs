use std::{
    borrow::Borrow,
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

use std::fmt;

use ref_cast::RefCast;

/// Equivalent to [PathBuf], but guaranteed to be absolute.
#[derive(PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct AbsPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be absolute.
#[derive(RefCast, PartialEq, Eq)]
#[repr(transparent)]
pub struct AbsPath(Path);

impl fmt::Debug for AbsPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for AbsPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl AbsPathBuf {
    pub fn new<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_abs_path_buf()
    }
}

impl AsRef<Path> for AbsPathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for AbsPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

pub trait ToAbsPathBuf: AsRef<Path> {
    fn to_abs_path_buf(&self) -> io::Result<AbsPathBuf>;
}

impl<P: AsRef<Path>> ToAbsPathBuf for P {
    fn to_abs_path_buf(&self) -> io::Result<AbsPathBuf> {
        Ok(AbsPathBuf(std::path::absolute(self)?))
    }
}

impl Deref for AbsPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AbsPathBuf {
    type Target = AbsPath;

    fn deref(&self) -> &Self::Target {
        AbsPath::ref_cast(&self.0)
    }
}

impl Borrow<AbsPath> for AbsPathBuf {
    fn borrow(&self) -> &AbsPath {
        self
    }
}

impl ToOwned for AbsPath {
    type Owned = AbsPathBuf;

    fn to_owned(&self) -> Self::Owned {
        AbsPathBuf(self.0.to_owned())
    }
}

impl AbsPathBuf {
    pub fn as_path(&self) -> &Path {
        self
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AbsPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AbsPathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path_buf = PathBuf::deserialize(deserializer)?;
        if path_buf.is_absolute() {
            Ok(AbsPathBuf(path_buf))
        } else {
            Err(serde::de::Error::custom("path must be absolute"))
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AbsPath {
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
    fn test_abs_path_buf_serialize() {
        let path_buf = AbsPathBuf::new("/home/user/file.txt").unwrap();
        assert_tokens(&path_buf, &[Token::Str("/home/user/file.txt")]);
    }

    #[test]
    fn test_abs_path_buf_deserialize() {
        let path_buf = AbsPathBuf::new("/home/user/file.txt").unwrap();
        assert_tokens(&path_buf, &[Token::Str("/home/user/file.txt")]);
    }

    #[test]
    fn test_abs_path_buf_deserialize_invalid() {
        assert_de_tokens_error::<AbsPathBuf>(
            &[Token::Str("relative/path")],
            "path must be absolute",
        );
    }

    #[test]
    fn test_abs_path_serialize() {
        let path_buf = AbsPathBuf::new("/home/user/file.txt").unwrap();
        let abs_path: &AbsPath = &path_buf;
        use serde_test::{Token, assert_ser_tokens};
        assert_ser_tokens(&abs_path, &[Token::Str("/home/user/file.txt")]);
    }
}
