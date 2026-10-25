use std::borrow::Borrow;
use std::ffi::OsString;
use std::fmt;
use std::io;
use std::ops::Deref;
use std::path::Component;
use std::path::MAIN_SEPARATOR_STR;

use ref_cast::RefCastCustom;
use ref_cast::ref_cast_custom;

use crate::Utf8Path;
use crate::Utf8PathBuf;

/// ```
/// use dirge::NormPathBuf;
/// use dirge::Utf8Path;
///
/// assert_eq!(NormPathBuf::new("").unwrap().as_path(), Utf8Path::new(""));
/// assert_eq!(NormPathBuf::new(".").unwrap().as_path(), Utf8Path::new("."));
/// assert_eq!(
///     NormPathBuf::new("./").unwrap().as_path(),
///     Utf8Path::new(".")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/..").unwrap().as_path(),
///     Utf8Path::new(".")
/// );
/// ```
///
/// ```
/// use dirge::NormPathBuf;
/// use dirge::Utf8Path;
///
/// assert_eq!(
///     NormPathBuf::new("./a").unwrap().as_path(),
///     Utf8Path::new("a")
/// );
/// assert_eq!(
///     NormPathBuf::new("a//b///").unwrap().as_path(),
///     Utf8Path::new("a/b")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/./b").unwrap().as_path(),
///     Utf8Path::new("a/b")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/b/../c").unwrap().as_path(),
///     Utf8Path::new("a/c")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/b/../../c").unwrap().as_path(),
///     Utf8Path::new("c")
/// );
/// ```
///
/// ```
/// use dirge::NormPathBuf;
/// use dirge::Utf8Path;
///
/// assert_eq!(
///     NormPathBuf::new("..").unwrap().as_path(),
///     Utf8Path::new("..")
/// );
/// assert_eq!(
///     NormPathBuf::new("../..").unwrap().as_path(),
///     Utf8Path::new("../..")
/// );
/// assert_eq!(
///     NormPathBuf::new("../../a").unwrap().as_path(),
///     Utf8Path::new("../../a")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/../../b").unwrap().as_path(),
///     Utf8Path::new("../b")
/// );
/// assert_eq!(
///     NormPathBuf::new("../a/../../b").unwrap().as_path(),
///     Utf8Path::new("../../b")
/// );
/// ```
///
/// ```
/// use dirge::NormPathBuf;
/// use dirge::Utf8Path;
///
/// assert_eq!(
///     NormPathBuf::new("/a/../b").unwrap().as_path(),
///     Utf8Path::new("/b")
/// );
/// assert_eq!(
///     NormPathBuf::new("/../../a").unwrap().as_path(),
///     Utf8Path::new("/a")
/// );
/// assert_eq!(
///     NormPathBuf::new("/a/../../..").unwrap().as_path(),
///     Utf8Path::new("/")
/// );
///
/// let normalized = NormPathBuf::new("a/./b/../c").unwrap();
/// assert_eq!(
///     NormPathBuf::new(&normalized).unwrap().as_os_str(),
///     normalized.as_os_str()
/// );
/// ```
///
/// ```
/// #[cfg(windows)]
/// {
///     use dirge::NormPathBuf;
///
///     assert_eq!(
///         NormPathBuf::new(r"C:/a\b/../c").unwrap().as_os_str(),
///         r"C:\a\c"
///     );
///     assert_eq!(
///         NormPathBuf::new(r"C:\..\..\a").unwrap().as_os_str(),
///         r"C:\a"
///     );
///     assert_eq!(
///         NormPathBuf::new(r"C:a\..\..\b").unwrap().as_os_str(),
///         r"C:..\b"
///     );
///     assert_eq!(NormPathBuf::new(r"C:a\..").unwrap().as_os_str(), "C:");
///     assert_eq!(NormPathBuf::new(r"\a\..\..\b").unwrap().as_os_str(), r"\b");
///     assert_eq!(
///         NormPathBuf::new(r"\\server\share\a\..\..\b")
///             .unwrap()
///             .as_os_str(),
///         r"\\server\share\b"
///     );
///     assert_eq!(
///         NormPathBuf::new(r"\\?\C:\a\..\b").unwrap().as_os_str(),
///         r"\\?\C:\b"
///     );
///     assert_eq!(
///         NormPathBuf::new(r"\\?\UNC\server\share\a\..\b")
///             .unwrap()
///             .as_os_str(),
///         r"\\?\UNC\server\share\b"
///     );
/// }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct NormPathBuf(Utf8PathBuf);

/// Equivalent to [Utf8Path], but guaranteed to be normalized.
#[derive(RefCastCustom, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NormPath(Utf8Path);

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
    pub fn new<P: AsRef<Utf8Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_norm_path_buf()
    }
}

impl AsRef<Utf8Path> for NormPathBuf {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<Utf8Path> for NormPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<std::path::Path> for NormPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

impl AsRef<std::path::Path> for NormPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

pub trait ToNormPathBuf: AsRef<Utf8Path> {
    fn to_norm_path_buf(&self) -> io::Result<NormPathBuf>;
}

pub(crate) fn normalize_path(path: &std::path::Path) -> std::path::PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(components.last(), Some(Component::Normal(_))) {
                    components.pop();
                } else if !path.has_root() {
                    components.push(component);
                }
            }
            _ => components.push(component),
        }
    }

    let mut normalized = OsString::new();
    let mut needs_separator = false;
    for component in components {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(MAIN_SEPARATOR_STR),
            _ => {
                if needs_separator {
                    normalized.push(MAIN_SEPARATOR_STR);
                }
                normalized.push(component.as_os_str());
            }
        }
        needs_separator = matches!(
            component,
            Component::Normal(_) | Component::ParentDir | Component::CurDir
        );
    }

    if normalized.is_empty() && !path.as_os_str().is_empty() {
        normalized.push(".");
    }

    std::path::PathBuf::from(normalized)
}

impl<P: AsRef<Utf8Path>> ToNormPathBuf for P {
    fn to_norm_path_buf(&self) -> io::Result<NormPathBuf> {
        let normalized = normalize_path(self.as_ref().as_ref());
        Ok(NormPathBuf(crate::from_std_path_buf(normalized)?))
    }
}

impl Deref for NormPath {
    type Target = Utf8Path;

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

impl NormPath {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_str(&self) -> &str {
        self.as_str()
    }
}

impl NormPathBuf {
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl NormPathBuf {
    pub fn as_path(&self) -> &Utf8Path {
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
        let path_buf = Utf8PathBuf::deserialize(deserializer)?;
        path_buf
            .to_norm_path_buf()
            .map_err(serde::de::Error::custom)
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
    use serde_test::Token;
    use serde_test::assert_de_tokens;
    use serde_test::assert_tokens;

    use super::*;

    #[test]
    fn test_norm_path_buf_serialize() {
        let path_buf = NormPathBuf::new("path/to/file.txt").unwrap();
        assert_tokens(
            &path_buf,
            &[Token::Str(if cfg!(windows) {
                r"path\to\file.txt"
            } else {
                "path/to/file.txt"
            })],
        );
    }

    #[test]
    fn test_norm_path_buf_deserialize() {
        let path_buf = NormPathBuf::new("path/to/file.txt").unwrap();
        assert_tokens(
            &path_buf,
            &[Token::Str(if cfg!(windows) {
                r"path\to\file.txt"
            } else {
                "path/to/file.txt"
            })],
        );
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
        use serde_test::Token;
        use serde_test::assert_ser_tokens;
        assert_ser_tokens(
            &norm_path,
            &[Token::Str(if cfg!(windows) {
                r"path\to\file.txt"
            } else {
                "path/to/file.txt"
            })],
        );
    }
}

impl NormPath {
    #[ref_cast_custom]
    pub(crate) fn ref_cast(path: &Utf8Path) -> &Self;

    /// ```
    /// use dirge::NormPath;
    /// use dirge::NormPathBuf;
    /// let owned = NormPathBuf::new("a").unwrap();
    /// assert_eq!(NormPath::new(owned.as_path()).unwrap(), &*owned);
    /// ```
    ///
    /// ```compile_fail
    /// use dirge::{NormPath, Utf8Path};
    /// use ref_cast::RefCast;
    /// let _ = NormPath::ref_cast(Utf8Path::new("a/.."));
    /// ```
    pub fn new<P: AsRef<Utf8Path> + ?Sized>(path: &P) -> io::Result<&Self> {
        let path = path.as_ref();
        if crate::norm::is_normalized(path.as_ref()) {
            Ok(Self::ref_cast(path))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be normalized",
            ))
        }
    }
}

impl<'a> TryFrom<&'a Utf8Path> for &'a NormPath {
    type Error = io::Error;

    fn try_from(path: &'a Utf8Path) -> Result<Self, Self::Error> {
        NormPath::new(path)
    }
}

impl NormPathBuf {
    pub fn into_path_buf(self) -> Utf8PathBuf {
        self.0
    }
}

impl From<NormPathBuf> for Utf8PathBuf {
    fn from(path: NormPathBuf) -> Self {
        path.0
    }
}

impl TryFrom<Utf8PathBuf> for NormPathBuf {
    type Error = Utf8PathBuf;

    fn try_from(path: Utf8PathBuf) -> Result<Self, Self::Error> {
        if NormPath::new(&path).is_ok() {
            Ok(Self(path))
        } else {
            Err(path)
        }
    }
}

impl From<crate::AbsNormPathBuf> for NormPathBuf {
    fn from(path: crate::AbsNormPathBuf) -> Self {
        Self(path.into_path_buf())
    }
}

impl From<crate::RelNormPathBuf> for NormPathBuf {
    fn from(path: crate::RelNormPathBuf) -> Self {
        Self(path.into_path_buf())
    }
}

pub(crate) fn is_normalized(path: &std::path::Path) -> bool {
    let mut remaining = path.as_os_str().as_encoded_bytes();
    let mut needs_separator = false;
    let mut normal_seen = false;
    for component in path.components() {
        let text = match component {
            Component::CurDir => return path.as_os_str() == ".",
            Component::ParentDir if path.has_root() || normal_seen => return false,
            Component::RootDir => MAIN_SEPARATOR_STR.as_bytes(),
            _ => component.as_os_str().as_encoded_bytes(),
        };
        if needs_separator {
            let Some(rest) = remaining.strip_prefix(MAIN_SEPARATOR_STR.as_bytes()) else {
                return false;
            };
            remaining = rest;
        }
        let Some(rest) = remaining.strip_prefix(text) else {
            return false;
        };
        remaining = rest;
        normal_seen |= matches!(component, Component::Normal(_));
        needs_separator = matches!(component, Component::Normal(_) | Component::ParentDir);
    }
    remaining.is_empty()
}
