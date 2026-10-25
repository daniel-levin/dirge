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
