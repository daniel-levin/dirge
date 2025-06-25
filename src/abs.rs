use std::{
    borrow::{Borrow, Cow},
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

use ref_cast::RefCast;

pub fn absolutize<'a, P: AsRef<Path> + 'a>(p: P) -> io::Result<Cow<'a, AbsPath>> {
    Ok(if p.as_ref().is_absolute() {
        // SAFETY: the lifetime is captured.
        unsafe { Cow::Borrowed(&*(p.as_ref() as *const Path as *const AbsPath)) }
    } else {
        Cow::Owned(AbsPathBuf::new(p)?)
    })
}

/// Equivalent to [PathBuf], but guaranteed to be absolute.
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct AbsPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be absolute.
#[derive(Debug, RefCast, PartialEq, Eq)]
#[repr(transparent)]
pub struct AbsPath(Path);

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

impl AbsPath {}
