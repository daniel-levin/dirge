use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::ops::Deref;

use ref_cast::RefCastCustom;
use ref_cast::ref_cast_custom;

use crate::Utf8Path;
use crate::Utf8PathBuf;

/// ```
/// use dirge::RelNormPathBuf;
/// use dirge::Utf8Path;
///
/// assert_eq!(
///     RelNormPathBuf::new("a/./b/../c").unwrap().as_path(),
///     Utf8Path::new("a/c")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("a/../../b").unwrap().as_path(),
///     Utf8Path::new("../b")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("../..").unwrap().as_path(),
///     Utf8Path::new("../..")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("a/..").unwrap().as_path(),
///     Utf8Path::new(".")
/// );
/// assert_eq!(
///     RelNormPathBuf::new("").unwrap().as_path(),
///     Utf8Path::new("")
/// );
/// let absolute = if cfg!(windows) { r"C:\a" } else { "/a" };
/// assert_eq!(RelNormPathBuf::new(absolute).is_err(), true);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct RelNormPathBuf(Utf8PathBuf);

/// Equivalent to [Utf8Path], but guaranteed to be relative and normalized.
#[derive(RefCastCustom, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RelNormPath(Utf8Path);

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
    pub fn new<P: AsRef<Utf8Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_rel_norm_path_buf()
    }
}

impl AsRef<Utf8Path> for RelNormPathBuf {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<Utf8Path> for RelNormPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<std::path::Path> for RelNormPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

impl AsRef<std::path::Path> for RelNormPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

pub trait ToRelNormPathBuf: AsRef<Utf8Path> {
    fn to_rel_norm_path_buf(&self) -> io::Result<RelNormPathBuf>;
}

impl<P: AsRef<Utf8Path>> ToRelNormPathBuf for P {
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
    type Target = Utf8Path;

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

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_str(&self) -> &str {
        self.as_str()
    }
}

impl RelNormPathBuf {
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl RelNormPathBuf {
    pub fn as_path(&self) -> &Utf8Path {
        self
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn push<P: AsRef<Utf8Path>>(&mut self, path: P) -> io::Result<()> {
        *self = self.join(path)?;
        Ok(())
    }

    pub fn join<P: AsRef<Utf8Path>>(&self, path: P) -> io::Result<RelNormPathBuf> {
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
        let path_buf = Utf8PathBuf::deserialize(deserializer)?;
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

impl RelNormPath {
    #[ref_cast_custom]
    pub(crate) fn ref_cast(path: &Utf8Path) -> &Self;

    /// ```
    /// use dirge::RelNormPath;
    /// use dirge::RelNormPathBuf;
    /// let owned = RelNormPathBuf::new("a").unwrap();
    /// assert_eq!(RelNormPath::new(owned.as_path()).unwrap(), &*owned);
    /// ```
    ///
    /// ```compile_fail
    /// use dirge::{RelNormPath, Utf8Path};
    /// use ref_cast::RefCast;
    /// let _ = RelNormPath::ref_cast(Utf8Path::new("a/.."));
    /// ```
    pub fn new<P: AsRef<Utf8Path> + ?Sized>(path: &P) -> io::Result<&Self> {
        let path = path.as_ref();
        if path.is_relative() && crate::norm::is_normalized(path.as_ref()) {
            Ok(Self::ref_cast(path))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be relative and normalized",
            ))
        }
    }
}

impl<'a> TryFrom<&'a Utf8Path> for &'a RelNormPath {
    type Error = io::Error;

    fn try_from(path: &'a Utf8Path) -> Result<Self, Self::Error> {
        RelNormPath::new(path)
    }
}

impl RelNormPathBuf {
    pub fn into_path_buf(self) -> Utf8PathBuf {
        self.0
    }
}

impl From<RelNormPathBuf> for Utf8PathBuf {
    fn from(path: RelNormPathBuf) -> Self {
        path.0
    }
}

impl TryFrom<Utf8PathBuf> for RelNormPathBuf {
    type Error = Utf8PathBuf;

    fn try_from(path: Utf8PathBuf) -> Result<Self, Self::Error> {
        if RelNormPath::new(&path).is_ok() {
            Ok(Self(path))
        } else {
            Err(path)
        }
    }
}
