use std::io;

use dirge::AbsPath;
use dirge::AbsPathBuf;
use dirge::ToAbsPathBuf;

#[test]
fn no_overhead() {
    assert_eq!(
        std::mem::size_of::<AbsPathBuf>(),
        std::mem::size_of::<std::path::PathBuf>()
    );
}

#[test]
fn basic() -> io::Result<()> {
    let c1 = "Cargo.toml".to_abs_path_buf()?;
    let c2 = AbsPathBuf::new("Cargo.toml")?;

    assert_eq!(c1, c2);

    Ok(())
}

#[test]
fn deref_to_abs_path() {
    let c1 = "Cargo.toml".to_abs_path_buf().unwrap();

    let _x: &AbsPath = &c1;
}

#[test]
fn deref_methods() {
    let c1 = "Cargo.toml".to_abs_path_buf().unwrap();

    assert!(c1.capacity() > 0);
}

#[test]
fn mutations_preserve_absolute_paths() {
    let mut path = AbsPathBuf::new(".").unwrap();
    for suffix in ["child", "../sibling", ""] {
        let expected = path.as_path().join(suffix);
        let joined = path.join(suffix).unwrap();
        assert!(joined.is_absolute());
        assert_eq!(joined.as_path(), expected);
        path.push(suffix).unwrap();
        assert_eq!(path, joined);
    }
    let replacement = AbsPathBuf::new("replacement").unwrap();
    path.push(&replacement).unwrap();
    assert_eq!(path, replacement);
}

#[cfg(windows)]
#[test]
fn drive_relative_mutations_resolve_absolute_paths() {
    let base = AbsPathBuf::new(r"C:\base\directory").unwrap();
    for suffix in ["C:child", "D:child", "C:", "D:"] {
        let expected = AbsPathBuf::new(suffix).unwrap();
        let joined = base.join(suffix).unwrap();
        assert!(joined.is_absolute(), "{suffix}");
        assert_eq!(joined, expected, "{suffix}");
        let mut pushed = base.clone();
        pushed.push(suffix).unwrap();
        assert_eq!(pushed, expected, "{suffix}");
    }
}

#[cfg(windows)]
#[test]
fn rooted_mutations_preserve_drive() {
    let mut path = AbsPathBuf::new(r"C:\base\directory").unwrap();
    let joined = path.join(r"\child").unwrap();
    assert_eq!(joined.as_os_str(), r"C:\child");
    path.push(r"\child").unwrap();
    assert_eq!(path, joined);
}
