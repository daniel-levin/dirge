use dirge::{RelPath, RelPathBuf};
use std::path::Path;

#[test]
fn basic() {
    let rel = RelPathBuf::new("src/main.rs").unwrap();
    let _: &RelPath = &rel;
    let _: &Path = &rel;
}

#[test]
fn deref_to_rel_path() {
    let rel = RelPathBuf::new("src/main.rs").unwrap();
    let rel_path: &RelPath = &rel;
    assert_eq!(rel_path.to_string_lossy(), "src/main.rs");
}

#[test]
fn deref_methods() {
    let rel = RelPathBuf::new("src/main.rs").unwrap();
    let _parent = rel.parent();
    let _file_name = rel.file_name();
    let _extension = rel.extension();
}

#[test]
fn no_overhead() {
    use std::mem;
    assert_eq!(
        mem::size_of::<RelPathBuf>(),
        mem::size_of::<std::path::PathBuf>()
    );
}

#[test]
fn rejects_absolute_paths() {
    assert!(RelPathBuf::new("/absolute/path").is_err());

    #[cfg(windows)]
    {
        assert!(RelPathBuf::new("C:\\absolute\\path").is_err());
        assert!(RelPathBuf::new("D:/absolute/path").is_err());
    }
}

#[test]
fn accepts_relative_paths() {
    assert!(RelPathBuf::new("relative/path").is_ok());
    assert!(RelPathBuf::new("./relative/path").is_ok());
    assert!(RelPathBuf::new("../relative/path").is_ok());
    assert!(RelPathBuf::new("file.txt").is_ok());
}
