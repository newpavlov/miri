//@ignore-target-windows: File handling is not implemented yet
//@ignore-host-windows: Only supported for UNIX hosts
//@compile-flags: -Zmiri-disable-isolation

#[path = "../utils/mod.rs"]
mod utils;

/// Test that the [`tempfile`] crate is compatible with miri for UNIX hosts and targets
fn main() {
    test_tempfile();
    test_tempfile_in();
    test_tempfile_ext().unwrap();
}

fn test_tempfile() {
    tempfile::tempfile().unwrap();
}

fn test_tempfile_in() {
    let dir_path = utils::tmp();
    tempfile::tempfile_in(dir_path).unwrap();
}

fn test_tempfile_ext() -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::FileExt;

    let mut f = tempfile::tempfile()?;
    f.write_all(b"hello world")?;
    let mut buf = [0u8; 3];
    f.read_exact_at(&mut buf, 2)?;
    assert_eq!(&buf, b"llo");
    f.read_exact_at(&mut buf, 6)?;
    assert_eq!(&buf, b"wor");

    f.write_all_at(b" mo", 6)?;

    let mut buf = [0u8; 11];
    f.read_exact_at(&mut buf, 0)?;
    assert_eq!(&buf, b"hello  mold");

    Ok(())
}
