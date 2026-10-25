use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::ops::Deref;

use ref_cast::RefCastCustom;
use ref_cast::ref_cast_custom;

use crate::Path;
use crate::PathBuf;

/// Equivalent to [PathBuf], but guaranteed to be absolute.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct AbsPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be absolute.
#[derive(RefCastCustom, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl AsRef<std::path::Path> for AbsPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

impl AsRef<std::path::Path> for AbsPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

pub trait ToAbsPathBuf: AsRef<Path> {
    fn to_abs_path_buf(&self) -> io::Result<AbsPathBuf>;
}

impl<P: AsRef<Path>> ToAbsPathBuf for P {
    fn to_abs_path_buf(&self) -> io::Result<AbsPathBuf> {
        Ok(AbsPathBuf(crate::from_std_path_buf(std::path::absolute(
            self.as_ref(),
        )?)?))
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

impl AbsPath {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_str(&self) -> &str {
        self.as_str()
    }
}

impl AbsPathBuf {
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl AbsPathBuf {
    pub fn as_path(&self) -> &Path {
        self
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        *self = self.join(path)?;
        Ok(())
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> io::Result<AbsPathBuf> {
        let joined = self.0.join(path);
        if joined.is_absolute() {
            Ok(Self(joined))
        } else {
            Self::new(joined)
        }
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
    use serde_test::Token;
    use serde_test::assert_de_tokens_error;
    use serde_test::assert_tokens;

    use super::*;

    const ABS_PATH: &str = if cfg!(windows) {
        r"C:\home\user\file.txt"
    } else {
        "/home/user/file.txt"
    };

    #[test]
    fn test_abs_path_buf_serialize() {
        let path_buf = AbsPathBuf::new(ABS_PATH).unwrap();
        assert_tokens(&path_buf, &[Token::Str(ABS_PATH)]);
    }

    #[test]
    fn test_abs_path_buf_deserialize() {
        let path_buf = AbsPathBuf::new(ABS_PATH).unwrap();
        assert_tokens(&path_buf, &[Token::Str(ABS_PATH)]);
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
        let path_buf = AbsPathBuf::new(ABS_PATH).unwrap();
        let abs_path: &AbsPath = &path_buf;
        use serde_test::Token;
        use serde_test::assert_ser_tokens;
        assert_ser_tokens(&abs_path, &[Token::Str(ABS_PATH)]);
    }
}

impl AbsPath {
    #[ref_cast_custom]
    pub(crate) fn ref_cast(path: &Path) -> &Self;

    /// ```
    /// use dirge::AbsPath;
    /// use dirge::AbsPathBuf;
    /// let owned = AbsPathBuf::new("a").unwrap();
    /// assert_eq!(AbsPath::new(owned.as_path()).unwrap(), &*owned);
    /// ```
    ///
    /// ```compile_fail
    /// use dirge::{AbsPath, Path};
    /// use ref_cast::RefCast;
    /// let _ = AbsPath::ref_cast(Path::new("a/.."));
    /// ```
    pub fn new<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<&Self> {
        let path = path.as_ref();
        if path.is_absolute() {
            Ok(Self::ref_cast(path))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be absolute",
            ))
        }
    }
}

impl<'a> TryFrom<&'a Path> for &'a AbsPath {
    type Error = io::Error;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        AbsPath::new(path)
    }
}

impl AbsPathBuf {
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl From<AbsPathBuf> for PathBuf {
    fn from(path: AbsPathBuf) -> Self {
        path.0
    }
}

impl TryFrom<PathBuf> for AbsPathBuf {
    type Error = PathBuf;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if AbsPath::new(&path).is_ok() {
            Ok(Self(path))
        } else {
            Err(path)
        }
    }
}

impl From<crate::AbsNormPathBuf> for AbsPathBuf {
    fn from(path: crate::AbsNormPathBuf) -> Self {
        Self(path.into_path_buf())
    }
}
