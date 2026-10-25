use dirge::RelNormPath;
use dirge::RelNormPathBuf;
use dirge::ToRelNormPathBuf;
use dirge::Utf8Path;

fn absolute() -> &'static str {
    if cfg!(windows) { r"C:\a" } else { "/a" }
}

#[test]
fn normalization_and_borrowed_views() {
    for (input, expected) in [
        ("", ""),
        ("./", "."),
        ("a/./b/../c", "a/c"),
        ("a/..", "."),
        ("a/../../b", "../b"),
        ("../..", "../.."),
    ] {
        let norm = input.to_rel_norm_path_buf().unwrap();
        assert_eq!(norm.as_path(), Utf8Path::new(expected));
        let borrowed: &RelNormPath = &norm;
        assert!(borrowed.is_relative());
        assert_eq!(borrowed.as_rel_path().as_os_str(), norm.as_os_str());
        assert_eq!(borrowed.as_norm_path().as_os_str(), norm.as_os_str());
        assert_eq!(borrowed.to_owned(), norm);
        assert_eq!(RelNormPathBuf::new(&norm).unwrap(), norm);
        let std_path: &std::path::Path = norm.as_ref();
        assert_eq!(std_path.as_os_str(), borrowed.as_os_str());
    }
}

#[test]
fn rejects_absolute_inputs_without_mutating() {
    assert_eq!(
        RelNormPathBuf::new(absolute()).unwrap_err().kind(),
        std::io::ErrorKind::InvalidInput
    );
    let mut path = RelNormPathBuf::new("a/b").unwrap();
    let original = path.clone();
    assert!(path.join(absolute()).is_err());
    assert!(path.push(absolute()).is_err());
    assert_eq!(path, original);
    let joined = path.join("../../c/./d").unwrap();
    path.push("../../c/./d").unwrap();
    assert_eq!(path, joined);
    assert_eq!(path.as_path(), Utf8Path::new("c/d"));
}

#[test]
fn no_overhead() {
    assert_eq!(
        std::mem::size_of::<RelNormPathBuf>(),
        std::mem::size_of::<dirge::Utf8PathBuf>()
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_enforces_both_guarantees() {
    use serde_test::Token;
    use serde_test::assert_de_tokens;
    use serde_test::assert_de_tokens_error;
    use serde_test::assert_ser_tokens;
    use serde_test::assert_tokens;
    let path = RelNormPathBuf::new("a/c").unwrap();
    let text = if cfg!(windows) { r"a\c" } else { "a/c" };
    assert_tokens(&path, &[Token::Str(text)]);
    let borrowed: &RelNormPath = &path;
    assert_ser_tokens(&borrowed, &[Token::Str(text)]);
    assert_de_tokens(&path, &[Token::Str("a/./b/../c")]);
    assert_de_tokens_error::<RelNormPathBuf>(&[Token::Str(absolute())], "path must be relative");
}

#[cfg(windows)]
#[test]
fn windows_relative_paths() {
    for (input, expected) in [
        (r"C:a\..\..\b", r"C:..\b"),
        (r"C:a\..", "C:"),
        (r"\a\..\b", r"\b"),
        (r"a/b\..\c", r"a\c"),
    ] {
        let norm = RelNormPathBuf::new(input).unwrap();
        assert!(norm.is_relative());
        assert_eq!(norm.as_os_str(), expected);
    }
    for input in [r"\\server\share\a", r"\\?\C:\a", r"a\..\C:\b"] {
        assert!(RelNormPathBuf::new(input).is_err());
    }
}
