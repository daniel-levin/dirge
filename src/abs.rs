use std::{
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

use ref_cast::RefCast;

/// Equivalent to [PathBuf], but guaranteed to be absolute.
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct AbsPathBuf(PathBuf);

/// Equivalent to [Path], but guaranteed to be absolute.
#[derive(Debug, RefCast)]
#[repr(transparent)]
pub struct AbsPath(Path);

impl AbsPathBuf {
    pub fn absolutize<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        p.as_ref().to_abs_path_buf()
    }
}

impl AsRef<Path> for AbsPathBuf {
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
