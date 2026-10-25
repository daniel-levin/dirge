use dirge::AbsNormPathBuf;
use dirge::AbsPathBuf;
use dirge::NormPathBuf;
use dirge::RelNormPathBuf;
use dirge::RelPathBuf;

macro_rules! check_strings {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            let path = <$ty>::new("données/文件.txt").unwrap();
            let expected = path.as_os_str().to_str().unwrap().to_owned();
            {
                let borrowed: &str = path.as_str();
                let converted: &str = path.to_str();
                assert_eq!(borrowed, expected);
                assert_eq!(converted, expected);
                let owned: String = path.into_string();
                assert_eq!(owned, expected);
            }
        }
    };
}

check_strings!(absolute_strings, AbsPathBuf);
check_strings!(absolute_normalized_strings, AbsNormPathBuf);
check_strings!(relative_strings, RelPathBuf);
check_strings!(relative_normalized_strings, RelNormPathBuf);
check_strings!(normalized_strings, NormPathBuf);
