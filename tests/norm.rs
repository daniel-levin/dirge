use dirge::{NormPath, NormPathBuf};
use std::path::Path;

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
    assert_eq!(norm_path.to_string_lossy(), "path/to/file.txt");
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
    assert_eq!(norm.to_string_lossy(), "path/to/file.txt");
}

#[test]
fn normalizes_parent_dir() {
    let norm = NormPathBuf::new("path/to/../file.txt").unwrap();
    assert_eq!(norm.to_string_lossy(), "path/file.txt");
}

#[test]
fn normalizes_complex_path() {
    let norm = NormPathBuf::new("path/./to/../other/./file.txt").unwrap();
    assert_eq!(norm.to_string_lossy(), "path/other/file.txt");
}

#[test]
fn handles_multiple_parent_dirs() {
    let norm = NormPathBuf::new("path/to/deep/../../file.txt").unwrap();
    assert_eq!(norm.to_string_lossy(), "path/file.txt");
}

#[test]
fn preserves_leading_parent_dirs() {
    let norm = NormPathBuf::new("../path/to/file.txt").unwrap();
    assert_eq!(norm.to_string_lossy(), "../path/to/file.txt");
}

#[test]
fn handles_empty_becoming_current() {
    let norm = NormPathBuf::new("./").unwrap();
    assert_eq!(norm.to_string_lossy(), ".");
}

#[test]
fn handles_current_dir_only() {
    let norm = NormPathBuf::new(".").unwrap();
    assert_eq!(norm.to_string_lossy(), ".");
}

#[test]
fn preserves_absolute_paths() {
    let norm = NormPathBuf::new("/path/./to/../file.txt").unwrap();
    assert_eq!(norm.to_string_lossy(), "/path/file.txt");
}
