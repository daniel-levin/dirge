use dirge::{AbsPath, AbsPathBuf, ToAbsPathBuf};
use std::{
    ffi::OsStr,
    io,
    path::{Component, PathBuf},
};

#[test]
#[cfg(target_family = "unix")]
fn no_overhead() -> io::Result<()> {
    let dn1 = AbsPathBuf::new("/dev/null");

    let pb: PathBuf = unsafe { std::mem::transmute(dn1) };

    let mut comps = pb.components();

    assert_eq!(comps.next(), Some(Component::RootDir));
    assert_eq!(comps.next(), Some(Component::Normal(OsStr::new("dev"))));
    assert_eq!(comps.next(), Some(Component::Normal(OsStr::new("null"))));

    Ok(())
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
