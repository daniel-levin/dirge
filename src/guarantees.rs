use crate::AbsNormPath;
use crate::AbsNormPathBuf;
use crate::AbsPath;
use crate::AbsPathBuf;
use crate::NormPath;
use crate::NormPathBuf;
use crate::RelNormPath;
use crate::RelNormPathBuf;
use crate::RelPath;
use crate::RelPathBuf;
use crate::Utf8Path;

mod sealed {
    pub trait Sealed {}
}

/// ```
/// use dirge::AbsNormPathBuf;
/// use dirge::AbsolutePath;
/// use dirge::NormalizedPath;
/// use dirge::Utf8Path;
///
/// fn absolute_normalized<P: AbsolutePath + NormalizedPath + ?Sized>(p: &P) -> &Utf8Path {
///     p.as_ref()
/// }
///
/// let path = AbsNormPathBuf::new("a/../b").unwrap();
/// assert_eq!(absolute_normalized(&path), path.as_path());
/// assert_eq!(absolute_normalized(&*path), path.as_path());
/// ```
///
/// ```compile_fail
/// use dirge::{AbsolutePath, Utf8Path};
/// struct Unchecked;
/// impl AsRef<Utf8Path> for Unchecked {
///     fn as_ref(&self) -> &Utf8Path { Utf8Path::new("relative") }
/// }
/// impl AbsolutePath for Unchecked {}
/// ```
pub trait AbsolutePath: sealed::Sealed + AsRef<Utf8Path> {}

/// ```
/// use dirge::NormalizedPath;
/// use dirge::RelNormPathBuf;
/// use dirge::RelativePath;
/// use dirge::Utf8Path;
///
/// fn relative_normalized<P: RelativePath + NormalizedPath + ?Sized>(p: &P) -> &Utf8Path {
///     p.as_ref()
/// }
///
/// let path = RelNormPathBuf::new("a/../b").unwrap();
/// assert_eq!(relative_normalized(&path), Utf8Path::new("b"));
/// assert_eq!(relative_normalized(&*path), Utf8Path::new("b"));
/// ```
///
/// ```compile_fail
/// use dirge::{RelativePath, Utf8Path};
/// struct Unchecked;
/// impl AsRef<Utf8Path> for Unchecked {
///     fn as_ref(&self) -> &Utf8Path { Utf8Path::new("/absolute") }
/// }
/// impl RelativePath for Unchecked {}
/// ```
pub trait RelativePath: sealed::Sealed + AsRef<Utf8Path> {}

/// ```
/// use dirge::NormPathBuf;
/// use dirge::NormalizedPath;
/// use dirge::Utf8Path;
///
/// fn normalized<P: NormalizedPath + ?Sized>(p: &P) -> &Utf8Path {
///     p.as_ref()
/// }
///
/// let path = NormPathBuf::new("a/../b").unwrap();
/// assert_eq!(normalized(&path), Utf8Path::new("b"));
/// assert_eq!(normalized(&*path), Utf8Path::new("b"));
/// ```
///
/// ```compile_fail
/// use dirge::{NormalizedPath, Utf8Path};
/// struct Unchecked;
/// impl AsRef<Utf8Path> for Unchecked {
///     fn as_ref(&self) -> &Utf8Path { Utf8Path::new("a/..") }
/// }
/// impl NormalizedPath for Unchecked {}
/// ```
///
/// ```compile_fail
/// use dirge::{AbsolutePath, NormalizedPath, AbsPathBuf};
/// fn both<P: AbsolutePath + NormalizedPath + ?Sized>(_: &P) {}
/// both(&AbsPathBuf::new("a/../b").unwrap());
/// ```
///
/// ```compile_fail
/// use dirge::{RelativePath, NormalizedPath, RelPathBuf};
/// fn both<P: RelativePath + NormalizedPath + ?Sized>(_: &P) {}
/// both(&RelPathBuf::new("a/../b").unwrap());
/// ```
pub trait NormalizedPath: sealed::Sealed + AsRef<Utf8Path> {}

impl sealed::Sealed for AbsPath {}
impl sealed::Sealed for AbsPathBuf {}
impl sealed::Sealed for RelPath {}
impl sealed::Sealed for RelPathBuf {}
impl sealed::Sealed for NormPath {}
impl sealed::Sealed for NormPathBuf {}
impl sealed::Sealed for AbsNormPath {}
impl sealed::Sealed for AbsNormPathBuf {}
impl sealed::Sealed for RelNormPath {}
impl sealed::Sealed for RelNormPathBuf {}

impl AbsolutePath for AbsPath {}
impl AbsolutePath for AbsPathBuf {}
impl AbsolutePath for AbsNormPath {}
impl AbsolutePath for AbsNormPathBuf {}

impl RelativePath for RelPath {}
impl RelativePath for RelPathBuf {}
impl RelativePath for RelNormPath {}
impl RelativePath for RelNormPathBuf {}

impl NormalizedPath for NormPath {}
impl NormalizedPath for NormPathBuf {}
impl NormalizedPath for AbsNormPath {}
impl NormalizedPath for AbsNormPathBuf {}
impl NormalizedPath for RelNormPath {}
impl NormalizedPath for RelNormPathBuf {}
