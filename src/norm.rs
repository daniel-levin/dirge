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

/// Equivalent to [PathBuf], but guaranteed to be normalized.
///
/// Normalization is lexical: redundant separators and `.` components are
/// removed, and `..` cancels a preceding normal component. Unresolved leading
/// `..` components are preserved in relative paths; traversal above a root is
/// ignored. A nonempty path that cancels out becomes `.`. Windows prefixes are
/// preserved, and component separators use the platform's main separator.
///
/// This does not access the filesystem or resolve symbolic links, so the result
/// may refer to a different file when a removed component is a symbolic link.
#[derive(PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct NormPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be normalized.
///
/// See [NormPathBuf] for the lexical normalization rules.
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

fn normalize_path(path: &std::path::Path) -> std::path::PathBuf {
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

    // Append raw component text: PathBuf::push can reinterpret a normal Windows
    // component such as "C:" as a prefix, or normalize verbatim paths again.
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

    // If the path is now empty and the original wasn't, use "."
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
    /// Borrows the path as UTF-8. With `camino`, this always returns `&str`;
    /// otherwise it returns `None` when the path is not valid UTF-8.
    #[cfg(feature = "camino")]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Borrows the path as UTF-8. With `camino`, this always returns `&str`;
    /// otherwise it returns `None` when the path is not valid UTF-8.
    #[cfg(not(feature = "camino"))]
    pub fn as_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    /// Equivalent to [`Self::as_str`], including its feature-dependent return type.
    #[cfg(feature = "camino")]
    pub fn to_str(&self) -> &str {
        self.as_str()
    }

    /// Equivalent to [`Self::as_str`], including its feature-dependent return type.
    #[cfg(not(feature = "camino"))]
    pub fn to_str(&self) -> Option<&str> {
        self.as_str()
    }
}

impl NormPathBuf {
    /// Consumes the path into a UTF-8 string. With `camino`, this is infallible;
    /// otherwise an invalid UTF-8 path is returned unchanged in `Err`.
    #[cfg(feature = "camino")]
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    /// Consumes the path into a UTF-8 string. With `camino`, this is infallible;
    /// otherwise an invalid UTF-8 path is returned unchanged in `Err`.
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
        // Always normalize during deserialization
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
