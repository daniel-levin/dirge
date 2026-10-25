use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::ops::Deref;

use ref_cast::RefCast;

use crate::Path;
use crate::PathBuf;

/// ```
/// use dirge::Path;
/// use dirge::RelNormPathBuf;
///
/// assert_eq!(
///     RelNormPathBuf::new("a/./b/../c").unwrap().as_path(),
///     Path::new("a/c")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("a/../../b").unwrap().as_path(),
///     Path::new("../b")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("../..").unwrap().as_path(),
///     Path::new("../..")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("a/..").unwrap().as_path(),
///     Path::new(".")
/// );
/// assert_eq!(RelNormPathBuf::new("").unwrap().as_path(), Path::new(""));
/// let absolute = if cfg!(windows) { r"C:\a" } else { "/a" };
/// assert_eq!(RelNormPathBuf::new(absolute).is_err(), true);
/// ```
#[derive(PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct RelNormPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be relative and normalized.
#[derive(RefCast, PartialEq, Eq)]
#[repr(transparent)]
pub struct RelNormPath(Path);

impl fmt::Debug for RelNormPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for RelNormPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl RelNormPathBuf {
    pub fn new<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_rel_norm_path_buf()
    }
}

impl AsRef<Path> for RelNormPathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for RelNormPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for RelNormPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for RelNormPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

pub trait ToRelNormPathBuf: AsRef<Path> {
    fn to_rel_norm_path_buf(&self) -> io::Result<RelNormPathBuf>;
}

impl<P: AsRef<Path>> ToRelNormPathBuf for P {
    fn to_rel_norm_path_buf(&self) -> io::Result<RelNormPathBuf> {
        let path = self.as_ref();
        if path.is_absolute() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be relative",
            ));
        }
        let normalized = crate::norm::normalize_path(path.as_ref());
        if normalized.is_absolute() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be relative",
            ));
        }
        Ok(RelNormPathBuf(crate::from_std_path_buf(normalized)?))
    }
}

impl Deref for RelNormPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RelNormPathBuf {
    type Target = RelNormPath;

    fn deref(&self) -> &Self::Target {
        RelNormPath::ref_cast(&self.0)
    }
}

impl Borrow<RelNormPath> for RelNormPathBuf {
    fn borrow(&self) -> &RelNormPath {
        self
    }
}

impl ToOwned for RelNormPath {
    type Owned = RelNormPathBuf;

    fn to_owned(&self) -> Self::Owned {
        RelNormPathBuf(self.0.to_owned())
    }
}

impl RelNormPath {
    pub fn as_rel_path(&self) -> &crate::RelPath {
        crate::RelPath::ref_cast(&self.0)
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

impl RelNormPathBuf {
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

impl RelNormPathBuf {
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

    pub fn join<P: AsRef<Path>>(&self, path: P) -> io::Result<RelNormPathBuf> {
        Self::new(self.0.join(path))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RelNormPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RelNormPathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path_buf = PathBuf::deserialize(deserializer)?;
        Self::new(path_buf).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RelNormPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
