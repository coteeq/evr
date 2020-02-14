use std::path::{ Path, PathBuf };
use std::env::temp_dir;
use lazy_static::lazy_static;
use std::io::{ Error, ErrorKind };
use crate::wait::ChildExitStatus;

pub mod python;
pub mod clang;

pub use python::PythonBackend;
pub use clang::ClangBackend;

pub mod run_error;
pub use run_error::RunError;


lazy_static! {
    static ref EVR_TMP_DIR: PathBuf = temp_dir().join("evr-tmp");
}

pub trait Backend {
    fn get_template(&self) -> Option<&str>;

    fn run(&self, fname: &Path) -> Result<ChildExitStatus, RunError>;
}

fn mk_tmp_dir() -> std::io::Result<&'static std::path::PathBuf> {
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
