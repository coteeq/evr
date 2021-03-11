use crate::wait::ChildExitStatus;
use lazy_static::lazy_static;
use std::collections::hash_map::DefaultHasher;
use std::env::temp_dir;
use std::hash::{Hash, Hasher};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub mod clang;
pub mod clang_c;
pub mod python;

pub use clang::ClangBackend;
pub use clang_c::ClangCBackend;
pub use python::PythonBackend;

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
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "tmp dir already exists and not a directory",
            ));
        }
    }
    Ok(&*EVR_TMP_DIR)
}

pub fn get_binary_by_filename(fname: &Path) -> std::io::Result<PathBuf> {
    let hashed_fname = {
        let mut hasher = DefaultHasher::new();
        fname.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    };

    Ok(mk_tmp_dir()?.join(hashed_fname))
}
