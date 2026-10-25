use dirge::AbsNormPath;
use dirge::AbsNormPathBuf;
use dirge::ToAbsNormPathBuf;
use dirge::Utf8Path;

fn root() -> &'static str {
    if cfg!(windows) { r"C:\" } else { "/" }
}

#[test]
fn normalizes_absolute_and_relative_inputs() {
    for (input, expected) in [("a/./b/../c", "a/c"), ("a/../../b", "b"), ("../../..", "")] {
        let input = Utf8Path::new(root()).join(input);
        let norm = input.to_abs_norm_path_buf().unwrap();
        assert_eq!(norm.as_path(), Utf8Path::new(root()).join(expected));
        let borrowed: &AbsNormPath = &norm;
        assert!(borrowed.is_absolute());
        assert_eq!(borrowed.as_abs_path().as_os_str(), norm.as_os_str());
        assert_eq!(borrowed.as_norm_path().as_os_str(), norm.as_os_str());
        assert_eq!(borrowed.to_owned(), norm);
        assert_eq!(AbsNormPathBuf::new(&norm).unwrap(), norm);
    }
    let relative = AbsNormPathBuf::new("a/../b").unwrap();
    assert_eq!(relative, AbsNormPathBuf::new("b").unwrap());
    assert!(relative.is_absolute());
    assert!(AbsNormPathBuf::new("").is_err());
}

#[test]
fn mutation_preserves_both_guarantees() {
    let mut path = AbsNormPathBuf::new(Utf8Path::new(root()).join("a/b")).unwrap();
    let joined = path.join("../c/./d").unwrap();
    path.push("../c/./d").unwrap();
    assert_eq!(path, joined);
    assert_eq!(path.as_path(), Utf8Path::new(root()).join("a/c/d"));
    path.push(Utf8Path::new(root()).join("x/../y")).unwrap();
    assert_eq!(path.as_path(), Utf8Path::new(root()).join("y"));
}

#[test]
fn no_overhead() {
    assert_eq!(
        std::mem::size_of::<AbsNormPathBuf>(),
        std::mem::size_of::<dirge::Utf8PathBuf>()
    );
}

#[cfg(feature = "serde")]
#[test]
fn deserialization_normalizes() {
    let expected = AbsNormPathBuf::new(Utf8Path::new(root()).join("a/c")).unwrap();
    let input = if cfg!(windows) {
        r"C:\a\.\b\..\c"
    } else {
        "/a/./b/../c"
    };
    serde_test::assert_de_tokens(&expected, &[serde_test::Token::Str(input)]);
}

#[cfg(windows)]
#[test]
fn windows_prefixes() {
    for (input, expected) in [
        (r"C:\a\..\..\b", r"C:\b"),
        (r"\\server\share\a\..\..\b", r"\\server\share\b"),
        (r"\\?\C:\a\..\b", r"\\?\C:\b"),
    ] {
        let path = AbsNormPathBuf::new(input).unwrap();
        assert_eq!(path.as_os_str(), expected);
        assert!(path.is_absolute());
    }
}
