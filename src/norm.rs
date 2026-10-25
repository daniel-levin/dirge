use std::borrow::Borrow;
use std::ffi::OsString;
use std::fmt;
use std::io;
use std::ops::Deref;
use std::path::Component;
use std::path::MAIN_SEPARATOR_STR;

use ref_cast::RefCast;

use crate::Path;
use crate::PathBuf;

/// ```
/// use dirge::NormPathBuf;
/// use dirge::Path;
///
/// assert_eq!(NormPathBuf::new("").unwrap().as_path(), Path::new(""));
/// assert_eq!(NormPathBuf::new(".").unwrap().as_path(), Path::new("."));
/// assert_eq!(NormPathBuf::new("./").unwrap().as_path(), Path::new("."));
/// assert_eq!(NormPathBuf::new("a/..").unwrap().as_path(), Path::new("."));
/// ```
///
/// ```
/// use dirge::NormPathBuf;
/// use dirge::Path;
///
/// assert_eq!(NormPathBuf::new("./a").unwrap().as_path(), Path::new("a"));
/// assert_eq!(
///     NormPathBuf::new("a//b///").unwrap().as_path(),
///     Path::new("a/b")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/./b").unwrap().as_path(),
///     Path::new("a/b")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/b/../c").unwrap().as_path(),
///     Path::new("a/c")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/b/../../c").unwrap().as_path(),
///     Path::new("c")
/// );
/// ```
///
/// ```
/// use dirge::NormPathBuf;
/// use dirge::Path;
///
/// assert_eq!(NormPathBuf::new("..").unwrap().as_path(), Path::new(".."));
/// assert_eq!(
///     NormPathBuf::new("../..").unwrap().as_path(),
///     Path::new("../..")
/// );
/// assert_eq!(
///     NormPathBuf::new("../../a").unwrap().as_path(),
///     Path::new("../../a")
/// );
/// assert_eq!(
///     NormPathBuf::new("a/../../b").unwrap().as_path(),
///     Path::new("../b")
/// );
/// assert_eq!(
///     NormPathBuf::new("../a/../../b").unwrap().as_path(),
///     Path::new("../../b")
/// );
/// ```
///
/// ```
/// use dirge::NormPathBuf;
/// use dirge::Path;
///
/// assert_eq!(
///     NormPathBuf::new("/a/../b").unwrap().as_path(),
///     Path::new("/b")
/// );
/// assert_eq!(
///     NormPathBuf::new("/../../a").unwrap().as_path(),
///     Path::new("/a")
/// );
/// assert_eq!(
///     NormPathBuf::new("/a/../../..").unwrap().as_path(),
///     Path::new("/")
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
#[derive(PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct NormPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be normalized.
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

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for NormPathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

#[cfg(feature = "camino")]
impl AsRef<std::path::Path> for NormPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_std_path()
    }
}

pub trait ToNormPathBuf: AsRef<Path> {
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

impl<P: AsRef<Path>> ToNormPathBuf for P {
    fn to_norm_path_buf(&self) -> io::Result<NormPathBuf> {
        let normalized = normalize_path(self.as_ref().as_ref());
        Ok(NormPathBuf(crate::from_std_path_buf(normalized)?))
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

impl NormPath {
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

impl NormPathBuf {
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
