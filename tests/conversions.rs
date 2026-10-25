use std::collections::BTreeSet;
use std::collections::HashSet;

use dirge::*;

macro_rules! check_type {
    ($test:ident, $owned:ident, $borrowed:ident) => {
        #[test]
        fn $test() {
            let initial = $owned::new("a").unwrap();
            let mut raw = initial.into_path_buf();
            raw.reserve(100);
            let pointer = raw.as_os_str().as_encoded_bytes().as_ptr();
            let capacity = raw.capacity();
            let borrowed = $borrowed::new(&raw).unwrap();
            assert_eq!(borrowed.as_os_str().as_encoded_bytes().as_ptr(), pointer);
            let converted: &$borrowed = raw.as_path().try_into().unwrap();
            assert_eq!(borrowed, converted);
            let owned = $owned::try_from(raw).unwrap();
            assert_eq!(owned.as_os_str().as_encoded_bytes().as_ptr(), pointer);
            assert_eq!(owned.capacity(), capacity);
            let hash_set = HashSet::from([owned.clone()]);
            let tree_set = BTreeSet::from([owned.clone()]);
            let borrowed: &$borrowed = &owned;
            assert!(hash_set.contains(borrowed));
            assert!(tree_set.contains(borrowed));
            let raw: PathBuf = owned.into();
            assert_eq!(raw.as_os_str().as_encoded_bytes().as_ptr(), pointer);
            assert_eq!(raw.capacity(), capacity);
        }
    };
}
check_type!(absolute, AbsPathBuf, AbsPath);
check_type!(relative, RelPathBuf, RelPath);
check_type!(normalized, NormPathBuf, NormPath);
check_type!(absolute_normalized, AbsNormPathBuf, AbsNormPath);
check_type!(relative_normalized, RelNormPathBuf, RelNormPath);

#[test]
fn validation_rejects_without_transforming_or_consuming() {
    let absolute = if cfg!(windows) { r"C:\a" } else { "/a" };
    assert!(AbsPath::new("a").is_err());
    assert!(RelPath::new(absolute).is_err());
    assert!(AbsNormPath::new("a").is_err());
    assert!(RelNormPath::new(absolute).is_err());
    for text in ["a/..", "a/./b", "a//b", "./", "a/"] {
        assert!(NormPath::new(text).is_err(), "{text:?}");
        assert!(RelNormPath::new(text).is_err(), "{text:?}");
        let raw = PathBuf::from(text);
        let pointer = raw.as_os_str().as_encoded_bytes().as_ptr();
        let rejected = NormPathBuf::try_from(raw).unwrap_err();
        assert_eq!(rejected.as_os_str().as_encoded_bytes().as_ptr(), pointer);
        assert_eq!(rejected.as_os_str(), text);
    }
    for text in ["", ".", ".."] {
        assert!(NormPath::new(text).is_ok());
    }
}

#[test]
fn weakening_moves_the_allocation() {
    macro_rules! check {
        ($strong:ident, $weak:ident) => {
            let strong = $strong::new("a").unwrap();
            let pointer = strong.as_os_str().as_encoded_bytes().as_ptr();
            let capacity = strong.capacity();
            let weak: $weak = strong.into();
            assert_eq!(weak.as_os_str().as_encoded_bytes().as_ptr(), pointer);
            assert_eq!(weak.capacity(), capacity);
        };
    }
    check!(AbsNormPathBuf, AbsPathBuf);
    check!(AbsNormPathBuf, NormPathBuf);
    check!(RelNormPathBuf, RelPathBuf);
    check!(RelNormPathBuf, NormPathBuf);
}

#[test]
fn borrowed_validation_matches_normalizer() {
    let parts = ["", ".", "..", "a", "b"];
    for first in parts {
        for second in parts {
            for third in parts {
                for separator in ["/", "//", std::path::MAIN_SEPARATOR_STR] {
                    let text = format!("{first}{separator}{second}{separator}{third}");
                    let normalized = NormPathBuf::new(&text).unwrap();
                    let unchanged = normalized.as_os_str() == Path::new(&text).as_os_str();
                    assert_eq!(NormPath::new(&text).is_ok(), unchanged, "{text:?}");
                    assert!(NormPath::new(normalized.as_path()).is_ok(), "{text:?}");
                }
            }
        }
    }
}

#[cfg(windows)]
#[test]
fn windows_representation_validation() {
    for text in [
        r"C:\a",
        r"C:a",
        r"C:..\a",
        r"\a",
        r"\\server\share\a",
        r"\\?\C:\a",
        r"\\?\UNC\server\share\a",
        r"\\.\device\a",
    ] {
        assert!(NormPath::new(text).is_ok(), "{text:?}");
    }
    for text in [
        r"C:/a",
        r"C:\a\..",
        r"C:\\a",
        r"\\server\share",
        r"\\?\C:\a\.\b",
    ] {
        assert!(NormPath::new(text).is_err(), "{text:?}");
    }
}
