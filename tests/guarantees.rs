use dirge::*;

#[test]
fn guarantee_implementations() {
    fn absolute<T: AbsolutePath + ?Sized>() {}
    fn relative<T: RelativePath + ?Sized>() {}
    fn normalized<T: NormalizedPath + ?Sized>() {}
    fn absolute_normalized<T: AbsolutePath + NormalizedPath + ?Sized>() {}
    fn relative_normalized<T: RelativePath + NormalizedPath + ?Sized>() {}

    absolute::<AbsPath>();
    absolute::<AbsPathBuf>();
    relative::<RelPath>();
    relative::<RelPathBuf>();
    normalized::<NormPath>();
    normalized::<NormPathBuf>();
    absolute_normalized::<AbsNormPath>();
    absolute_normalized::<AbsNormPathBuf>();
    relative_normalized::<RelNormPath>();
    relative_normalized::<RelNormPathBuf>();
}
