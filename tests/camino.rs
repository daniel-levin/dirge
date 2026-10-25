#![cfg(feature = "camino")]

use camino::Utf8Path;
use camino::Utf8PathBuf;
use dirge::AbsPath;
use dirge::AbsPathBuf;
use dirge::NormPath;
use dirge::NormPathBuf;
use dirge::RelPath;
use dirge::RelPathBuf;

#[test]
fn utf8_api_and_standard_path_interop() {
    let input = Utf8PathBuf::from("données/文件.txt");
    let rel = RelPathBuf::new(&input).unwrap();
    let norm = NormPathBuf::new(&input).unwrap();
    let abs = AbsPathBuf::new(&input).unwrap();

    let rel_ref: &RelPath = &rel;
    let norm_ref: &NormPath = &norm;
    let abs_ref: &AbsPath = &abs;
    let paths: [&Utf8Path; 3] = [rel_ref, norm_ref, abs_ref];
    for path in paths {
        let filename: &str = path.file_name().unwrap();
        assert_eq!(filename, "文件.txt");
    }
    fn std_path(p: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        p.as_ref().to_path_buf()
    }
    assert_eq!(std_path(&rel), std_path(rel_ref));
    assert_eq!(std_path(&norm), std_path(norm_ref));
    assert_eq!(std_path(&abs), std_path(abs_ref));
    assert_eq!(rel_ref.to_owned(), rel);
    assert_eq!(norm_ref.to_owned(), norm);
    assert_eq!(abs_ref.to_owned(), abs);
    assert!(abs.is_absolute());
}

#[test]
fn normalization_preserves_unicode() {
    let norm = NormPathBuf::new("données/临时/../文件.txt").unwrap();
    assert_eq!(norm.as_path(), Utf8Path::new("données/文件.txt"));
    assert_eq!(norm.file_name(), Some("文件.txt"));
}

#[test]
fn absolute_mutation_accepts_utf8_paths() {
    let mut abs = AbsPathBuf::new(".").unwrap();
    let joined = abs.join(Utf8Path::new("文件.txt"));
    abs.push(Utf8Path::new("文件.txt"));
    assert_eq!(abs, joined);
    assert_eq!(abs.file_name(), Some("文件.txt"));
}
