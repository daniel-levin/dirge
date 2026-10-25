#![cfg(windows)]

use dirge::*;

fn check_normalization(input: &str, expected: &str) {
    let normalized = NormPathBuf::new(input).unwrap();
    // Path equality ignores some spelling differences; compare the actual text.
    assert_eq!(normalized.as_str(), expected, "{input:?}");
    assert_eq!(
        NormPathBuf::new(&normalized).unwrap().as_str(),
        expected,
        "{input:?}"
    );
    assert!(NormPath::new(expected).is_ok(), "{input:?} -> {expected:?}");
    assert_eq!(NormPath::new(input).is_ok(), input == expected, "{input:?}");
    assert_eq!(
        NormPathBuf::try_from(Utf8PathBuf::from(input)).is_ok(),
        input == expected,
        "{input:?}"
    );
}

#[test]
fn mundane_paths() {
    for (input, expected) in [
        ("", ""),
        (".", "."),
        (r".\", "."),
        ("./", "."),
        ("report.txt", "report.txt"),
        (r"src\main.rs", r"src\main.rs"),
        ("src/main.rs", r"src\main.rs"),
        (r"src\\main.rs", r"src\main.rs"),
        (r"src\.\main.rs", r"src\main.rs"),
        (r"src\temp\..\main.rs", r"src\main.rs"),
        (r"src\", "src"),
        (r"C:\", r"C:\"),
        ("C:/", r"C:\"),
        (r"c:\Users\Ada\file.txt", r"c:\Users\Ada\file.txt"),
        (r"C:/Users\Ada//file.txt", r"C:\Users\Ada\file.txt"),
        (r"C:\Program Files\App\", r"C:\Program Files\App"),
        (r"C:\数据\café\🦀.txt", r"C:\数据\café\🦀.txt"),
        (r"C:\a\..", r"C:\"),
        (r"\", r"\"),
        ("/", r"\"),
    ] {
        check_normalization(input, expected);
    }
}

#[test]
fn parents_roots_and_drive_relative_paths() {
    for (input, expected) in [
        (r"..\..\a", r"..\..\a"),
        (r"..\a\..\..\b", r"..\..\b"),
        (r"a\b\..\..\..", ".."),
        (r"C:\..\..\..\a", r"C:\a"),
        (r"\..\..\a", r"\a"),
        ("C:", "C:"),
        ("c:", "c:"),
        ("C:.", "C:"),
        (r"C:.\a", "C:a"),
        (r"C:a\..", "C:"),
        (r"C:..\..\a", r"C:..\..\a"),
        (r"C:a\..\..\b", r"C:..\b"),
        (r"C:a\b\..\..\..", "C:.."),
        (r"C:../a", r"C:..\a"),
    ] {
        check_normalization(input, expected);
        let path = Utf8Path::new(expected);
        assert_eq!(AbsPath::new(path).is_ok(), path.is_absolute());
        assert_eq!(RelPath::new(path).is_ok(), path.is_relative());
        assert_eq!(AbsNormPath::new(path).is_ok(), path.is_absolute());
        assert_eq!(RelNormPath::new(path).is_ok(), path.is_relative());
    }
}

#[test]
fn unc_device_and_verbatim_prefixes() {
    for (input, expected) in [
        (r"\\server\share", r"\\server\share\"),
        (r"\\server\share\", r"\\server\share\"),
        (r"\\server\share\..\..\file", r"\\server\share\file"),
        (r"\\server\share\a\.\b\..", r"\\server\share\a"),
        (r"\\服务器\共享\a\..\文件", r"\\服务器\共享\文件"),
        (r"\\.\COM42", r"\\.\COM42\"),
        (r"\\.\device\a\..\..\file", r"\\.\device\file"),
        (r"\\?\C:\", r"\\?\C:\"),
        (r"\\?\C:\a\.\b\..\c", r"\\?\C:\a\c"),
        (r"\\?\C:\..\..\file", r"\\?\C:\file"),
        (r"\\?\UNC\server\share", r"\\?\UNC\server\share"),
        (
            r"\\?\UNC\server\share\a\..\..\file",
            r"\\?\UNC\server\share\file",
        ),
        (
            r"\\?\Volume{01234567-89ab-cdef-0123-456789abcdef}\a\..\b",
            r"\\?\Volume{01234567-89ab-cdef-0123-456789abcdef}\b",
        ),
        // Forward slashes are literal characters in a verbatim component.
        (r"\\?\C:\a/b", r"\\?\C:\a/b"),
        (r"\\?\C:\a/b\..\c", r"\\?\C:\c"),
        (r"\\?\C:\a/../b", r"\\?\C:\a/../b"),
        (r"\\?\C:\a/./b", r"\\?\C:\a/./b"),
    ] {
        check_normalization(input, expected);
        assert!(AbsNormPath::new(expected).is_ok(), "{expected:?}");
        assert!(RelNormPathBuf::new(input).is_err(), "{input:?}");
    }
}

#[test]
fn lexical_normalization_is_not_filename_validation() {
    for name in [
        "CON",
        "NUL",
        "PRN.txt",
        "AUX",
        "COM1",
        "LPT9",
        "con.txt",
        "file:stream",
        "file::$DATA",
        "file.",
        "file ",
        "...",
        ".. ",
        "a?b",
        "a*b",
        "a<b",
        "a>b",
        "a|b",
        "a\"b",
        "a\0b",
        "é",
        "e\u{301}",
        "数据",
        "🦀",
        "a\u{202e}txt",
    ] {
        let input = format!(r"C:\folder\{name}");
        check_normalization(&input, &input);
        assert!(AbsNormPath::new(&input).is_ok(), "{input:?}");
    }
}

#[test]
fn components_that_look_like_prefixes() {
    for (input, expected) in [
        (r"dir\C:\file", r"dir\C:\file"),
        (r"dir\D:child", r"dir\D:child"),
        (r"dir\C:\..\file", r"dir\file"),
        (r"a\..\C:\file", r"C:\file"),
        (r"a\..\C:child", "C:child"),
    ] {
        check_normalization(input, expected);
    }
    let input = r"a\..\C:\file";
    assert!(RelPath::new(input).is_ok());
    assert!(RelNormPathBuf::new(input).is_err());
    let mut path = RelNormPathBuf::new("a").unwrap();
    let before = path.clone();
    assert!(path.push(r"..\C:\file").is_err());
    assert_eq!(path, before);
}

#[test]
fn joins_replace_roots_and_prefixes_without_losing_guarantees() {
    for (base, suffix, expected) in [
        (r"C:\base", "child", r"C:\base\child"),
        (r"C:\base", r"\child", r"C:\child"),
        (r"C:\base", "/child", r"C:\child"),
        (r"C:\base", r"D:\child", r"D:\child"),
        (r"C:\base", r"\\server\share\child", r"\\server\share\child"),
        (r"\\server\share\base", r"\child", r"\\server\share\child"),
        (r"\\?\C:\base", r"..\child", r"\\?\C:\child"),
    ] {
        let mut path = AbsPathBuf::try_from(Utf8PathBuf::from(base)).unwrap();
        let joined = path.join(suffix).unwrap();
        assert_eq!(joined.as_str(), expected, "{base:?} + {suffix:?}");
        path.push(suffix).unwrap();
        assert_eq!(path, joined);
        assert!(AbsPath::new(path.as_path()).is_ok());

        let mut norm = AbsNormPathBuf::try_from(Utf8PathBuf::from(base)).unwrap();
        let joined = norm.join(suffix).unwrap();
        norm.push(suffix).unwrap();
        assert_eq!(norm.as_str(), expected, "{base:?} + {suffix:?}");
        assert_eq!(norm, joined);
        assert!(AbsNormPath::new(norm.as_path()).is_ok());
    }
}

#[test]
fn failed_relative_push_keeps_buffer_and_contents() {
    for suffix in [
        r"C:\file",
        r"\\server\share\file",
        r"\\?\C:\file",
        r"\\.\device\file",
    ] {
        let mut path = RelNormPathBuf::new(r"a\b").unwrap();
        let pointer = path.as_str().as_ptr();
        let capacity = path.capacity();
        assert!(path.join(suffix).is_err(), "{suffix:?}");
        assert!(path.push(suffix).is_err(), "{suffix:?}");
        assert_eq!(path.as_str(), r"a\b");
        assert_eq!(path.as_str().as_ptr(), pointer);
        assert_eq!(path.capacity(), capacity);
    }
}

#[test]
fn long_paths_and_deep_parent_chains() {
    let component = "x".repeat(300);
    let input = format!(r"\\?\C:\{component}\{}leaf", r"a\..\".repeat(1024));
    check_normalization(&input, &format!(r"\\?\C:\{component}\leaf"));
    let parents = format!("{}leaf", r"..\".repeat(1024));
    check_normalization(&parents, &parents);
    check_normalization(&format!(r"C:\{parents}"), r"C:\leaf");
}

#[cfg(feature = "serde")]
#[test]
fn serde_checks_prefixes_and_normalizes() {
    use serde_test::Token;
    use serde_test::assert_de_tokens;
    use serde_test::assert_de_tokens_error;
    use serde_test::assert_tokens;
    for (input, expected) in [
        (r"C:/a/../b", r"C:\b"),
        (r"\\server\share\a\..\b", r"\\server\share\b"),
        (r"\\?\C:\a/b\..\c", r"\\?\C:\c"),
    ] {
        let path = AbsNormPathBuf::try_from(Utf8PathBuf::from(expected)).unwrap();
        assert_tokens(&path, &[Token::Str(expected)]);
        assert_de_tokens(&path, &[Token::Str(input)]);
    }
    for input in ["C:child", r"\child", "child", ""] {
        assert_de_tokens_error::<AbsNormPathBuf>(&[Token::Str(input)], "path must be absolute");
    }
    for input in [r"C:\child", r"\\server\share\child", r"a\..\C:\child"] {
        assert_de_tokens_error::<RelNormPathBuf>(&[Token::Str(input)], "path must be relative");
    }
}
