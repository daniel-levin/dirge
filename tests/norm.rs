use dirge::NormPath;
use dirge::NormPathBuf;
use dirge::Path;

#[test]
fn basic() {
    let norm = NormPathBuf::new("path/to/file.txt").unwrap();
    let _: &NormPath = &norm;
    let _: &Path = &norm;
}

#[test]
fn deref_to_norm_path() {
    let norm = NormPathBuf::new("path/to/file.txt").unwrap();
    let norm_path: &NormPath = &norm;
    assert_eq!(&**norm_path, Path::new("path/to/file.txt"));
}

#[test]
fn deref_methods() {
    let norm = NormPathBuf::new("path/to/file.txt").unwrap();
    let _parent = norm.parent();
    let _file_name = norm.file_name();
    let _extension = norm.extension();
}

#[test]
fn no_overhead() {
    use std::mem;
    assert_eq!(
        mem::size_of::<NormPathBuf>(),
        mem::size_of::<std::path::PathBuf>()
    );
}

#[test]
fn normalizes_current_dir() {
    let norm = NormPathBuf::new("path/./to/file.txt").unwrap();
    assert_eq!(norm.as_path(), Path::new("path/to/file.txt"));
}

#[test]
fn normalizes_parent_dir() {
    let norm = NormPathBuf::new("path/to/../file.txt").unwrap();
    assert_eq!(norm.as_path(), Path::new("path/file.txt"));
}

#[test]
fn normalizes_complex_path() {
    let norm = NormPathBuf::new("path/./to/../other/./file.txt").unwrap();
    assert_eq!(norm.as_path(), Path::new("path/other/file.txt"));
}

#[test]
fn handles_multiple_parent_dirs() {
    let norm = NormPathBuf::new("path/to/deep/../../file.txt").unwrap();
    assert_eq!(norm.as_path(), Path::new("path/file.txt"));
}

#[test]
fn preserves_leading_parent_dirs() {
    let norm = NormPathBuf::new("../path/to/file.txt").unwrap();
    assert_eq!(norm.as_path(), Path::new("../path/to/file.txt"));
}

#[test]
fn handles_empty_becoming_current() {
    let norm = NormPathBuf::new("./").unwrap();
    assert_eq!(norm.as_path(), Path::new("."));
}

#[test]
fn handles_current_dir_only() {
    let norm = NormPathBuf::new(".").unwrap();
    assert_eq!(norm.as_path(), Path::new("."));
}

#[test]
fn preserves_absolute_paths() {
    let norm = NormPathBuf::new("/path/./to/../file.txt").unwrap();
    assert_eq!(norm.as_path(), Path::new("/path/file.txt"));
}

#[test]
fn preserves_unresolved_parents_and_stops_at_roots() {
    for (input, expected) in [
        ("", ""),
        ("../..", "../.."),
        ("../../file", "../../file"),
        ("../a/../../file", "../../file"),
        ("a/../../..", "../.."),
        ("a/..", "."),
        ("/../../file", "/file"),
        ("/a/../../..", "/"),
        ("a//b/./c/../", "a/b"),
    ] {
        let norm = NormPathBuf::new(input).unwrap();
        assert_eq!(norm.as_path(), Path::new(expected), "{input:?}");
        let again = NormPathBuf::new(&norm).unwrap();
        assert_eq!(again.as_os_str(), norm.as_os_str(), "{input:?}");
    }
}

#[cfg(windows)]
#[test]
fn windows_prefixes_and_separators() {
    for (input, expected) in [
        (r"C:/a\b/../file", r"C:\a\file"),
        (r"C:\..\..\file", r"C:\file"),
        (r"C:..\..\file", r"C:..\..\file"),
        (r"C:a\..\..\file", r"C:..\file"),
        (r"C:a\..", "C:"),
        (r"\a\..\..\file", r"\file"),
        (r"\\server\share\a\..\..\file", r"\\server\share\file"),
        (r"\\?\C:\a\..\..\file", r"\\?\C:\file"),
        (
            r"\\?\UNC\server\share\a\..\..\file",
            r"\\?\UNC\server\share\file",
        ),
        (r"\\.\device\a\..\file", r"\\.\device\file"),
        (r"dir\C:\file", r"dir\C:\file"),
        (r"\\?\C:\a/b\..\file", r"\\?\C:\file"),
    ] {
        let norm = NormPathBuf::new(input).unwrap();
        assert_eq!(norm.as_os_str(), expected, "{input:?}");
        let again = NormPathBuf::new(&norm).unwrap();
        assert_eq!(again.as_os_str(), norm.as_os_str(), "{input:?}");
    }
}
