use dirge::AbsPathBuf;
use dirge::NormPathBuf;
use dirge::RelPathBuf;

macro_rules! check_strings {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            let path = <$ty>::new("données/文件.txt").unwrap();
            let expected = path.as_os_str().to_str().unwrap().to_owned();
            #[cfg(feature = "camino")]
            {
                let borrowed: &str = path.as_str();
                let converted: &str = path.to_str();
                assert_eq!(borrowed, expected);
                assert_eq!(converted, expected);
                let owned: String = path.into_string();
                assert_eq!(owned, expected);
            }
            #[cfg(not(feature = "camino"))]
            {
                let borrowed: Option<&str> = path.as_str();
                let converted: Option<&str> = path.to_str();
                assert_eq!(borrowed, Some(expected.as_str()));
                assert_eq!(converted, Some(expected.as_str()));
                let owned: Result<String, $ty> = path.into_string();
                assert_eq!(owned.unwrap(), expected);
            }
        }
    };
}

check_strings!(absolute_strings, AbsPathBuf);
check_strings!(relative_strings, RelPathBuf);
check_strings!(normalized_strings, NormPathBuf);

#[cfg(all(unix, not(feature = "camino")))]
#[test]
fn invalid_utf8_is_recoverable() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let input = OsStr::from_bytes(b"dir/\xff");
    macro_rules! check_invalid {
        ($ty:ty) => {
            let path = <$ty>::new(input).unwrap();
            assert_eq!(path.as_str(), None);
            assert_eq!(path.to_str(), None);
            let original = path.clone();
            let recovered = path.into_string().unwrap_err();
            assert_eq!(recovered.as_os_str(), original.as_os_str());
            assert_eq!(recovered, original);
        };
    }
    check_invalid!(AbsPathBuf);
    check_invalid!(RelPathBuf);
    check_invalid!(NormPathBuf);
}
