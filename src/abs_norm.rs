use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::ops::Deref;

use ref_cast::RefCastCustom;
use ref_cast::ref_cast_custom;

use crate::Path;
use crate::PathBuf;

/// ```
/// use dirge::AbsNormPathBuf;
/// use dirge::Path;
///
/// let root = if cfg!(windows) { r"C:\" } else { "/" };
/// let input = Path::new(root).join("a/./b/../c");
/// let path = AbsNormPathBuf::new(input).unwrap();
/// assert_eq!(path.as_path(), Path::new(root).join("a/c"));
/// assert_eq!(path.is_absolute(), true);
/// assert_eq!(AbsNormPathBuf::new(&path).unwrap(), path);
///
/// let relative = AbsNormPathBuf::new("a/../b").unwrap();
/// assert_eq!(relative.is_absolute(), true);
/// assert_eq!(
///     relative.as_path(),
///     AbsNormPathBuf::new("b").unwrap().as_path()
/// );
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct AbsNormPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be absolute and normalized.
#[derive(RefCastCustom, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AbsNormPath(Path);

impl fmt::Debug for AbsNormPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for AbsNormPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl AbsNormPathBuf {
    pub fn new<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_abs_norm_path_buf()
    }
}

impl AsRef<Path> for AbsNormPathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for AbsNormPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for AbsNormPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for AbsNormPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

pub trait ToAbsNormPathBuf: AsRef<Path> {
    fn to_abs_norm_path_buf(&self) -> io::Result<AbsNormPathBuf>;
}

impl<P: AsRef<Path>> ToAbsNormPathBuf for P {
    fn to_abs_norm_path_buf(&self) -> io::Result<AbsNormPathBuf> {
        let absolute = std::path::absolute(self.as_ref())?;
        let normalized = crate::norm::normalize_path(&absolute);
        Ok(AbsNormPathBuf(crate::from_std_path_buf(normalized)?))
    }
}

impl Deref for AbsNormPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AbsNormPathBuf {
    type Target = AbsNormPath;

    fn deref(&self) -> &Self::Target {
        AbsNormPath::ref_cast(&self.0)
    }
}

impl Borrow<AbsNormPath> for AbsNormPathBuf {
    fn borrow(&self) -> &AbsNormPath {
        self
    }
}

impl ToOwned for AbsNormPath {
    type Owned = AbsNormPathBuf;

    fn to_owned(&self) -> Self::Owned {
        AbsNormPathBuf(self.0.to_owned())
    }
}

impl AbsNormPath {
    pub fn as_abs_path(&self) -> &crate::AbsPath {
        crate::AbsPath::ref_cast(&self.0)
    }

    pub fn as_norm_path(&self) -> &crate::NormPath {
        crate::NormPath::ref_cast(&self.0)
    }

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

impl AbsNormPathBuf {
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

impl AbsNormPathBuf {
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

    pub fn join<P: AsRef<Path>>(&self, path: P) -> io::Result<AbsNormPathBuf> {
        Self::new(self.0.join(path))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AbsNormPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AbsNormPathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path_buf = PathBuf::deserialize(deserializer)?;
        if path_buf.is_absolute() {
            Self::new(path_buf).map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("path must be absolute"))
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AbsNormPath {
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
        let path_buf = AbsNormPathBuf::new(ABS_PATH).unwrap();
        assert_tokens(&path_buf, &[Token::Str(ABS_PATH)]);
    }

    #[test]
    fn test_abs_path_buf_deserialize() {
        let path_buf = AbsNormPathBuf::new(ABS_PATH).unwrap();
        assert_tokens(&path_buf, &[Token::Str(ABS_PATH)]);
    }

    #[test]
    fn test_abs_path_buf_deserialize_invalid() {
        assert_de_tokens_error::<AbsNormPathBuf>(
            &[Token::Str("relative/path")],
            "path must be absolute",
        );
    }

    #[test]
    fn test_abs_path_serialize() {
        let path_buf = AbsNormPathBuf::new(ABS_PATH).unwrap();
        let abs_path: &AbsNormPath = &path_buf;
        use serde_test::Token;
        use serde_test::assert_ser_tokens;
        assert_ser_tokens(&abs_path, &[Token::Str(ABS_PATH)]);
    }
}

impl AbsNormPath {
    #[ref_cast_custom]
    pub(crate) fn ref_cast(path: &Path) -> &Self;

    /// ```
    /// use dirge::AbsNormPath;
    /// use dirge::AbsNormPathBuf;
    /// let owned = AbsNormPathBuf::new("a").unwrap();
    /// assert_eq!(AbsNormPath::new(owned.as_path()).unwrap(), &*owned);
    /// ```
    ///
    /// ```compile_fail
    /// use dirge::{AbsNormPath, Path};
    /// use ref_cast::RefCast;
    /// let _ = AbsNormPath::ref_cast(Path::new("a/.."));
    /// ```
    pub fn new<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<&Self> {
        let path = path.as_ref();
        if path.is_absolute() && crate::norm::is_normalized(path.as_ref()) {
            Ok(Self::ref_cast(path))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be absolute and normalized",
            ))
        }
    }
}

impl<'a> TryFrom<&'a Path> for &'a AbsNormPath {
    type Error = io::Error;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        AbsNormPath::new(path)
    }
}

impl AbsNormPathBuf {
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl From<AbsNormPathBuf> for PathBuf {
    fn from(path: AbsNormPathBuf) -> Self {
        path.0
    }
}

impl TryFrom<PathBuf> for AbsNormPathBuf {
    type Error = PathBuf;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if AbsNormPath::new(&path).is_ok() {
            Ok(Self(path))
        } else {
            Err(path)
        }
    }
}
