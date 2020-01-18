use std::path::{ Path, PathBuf };
use std::env::temp_dir;
use lazy_static::lazy_static;
use std::io::{ Error, ErrorKind, Result };
use nix::sys::signal::Signal as NixSignal;

pub mod python;
pub mod clang;

pub use python::PythonBackend;
pub use clang::ClangBackend;

lazy_static! {
    static ref EVR_TMP_DIR: PathBuf = temp_dir().join("evr-tmp");
}

pub enum RunStatus {
    Success,
    ErrorCode(i32),
    TimedOut(std::time::Duration),
    Signal(NixSignal, bool),
}

pub trait Backend {
    fn get_template(&self) -> Option<&str>;

    fn run(&self, fname: &Path) -> Result<RunStatus>;

    fn try_guess_test_file(&self, fname: &Path) -> Option<PathBuf> {
        let maybe_test = fname.with_extension("txt");
        if maybe_test.exists() {
            return Some(maybe_test);
        }
        None
    }
}

fn mk_tmp_dir() -> Result<&'static std::path::PathBuf> {
    if !EVR_TMP_DIR.exists() {
        std::fs::create_dir(&*EVR_TMP_DIR)?;
    } else {
        if !EVR_TMP_DIR.is_dir() {
            return Err(Error::new(ErrorKind::AlreadyExists,
                "tmp dir already exists and not a directory"))
        }
    }
    Ok(&*EVR_TMP_DIR)
}
