use std::path::{ Path, PathBuf };

pub mod python;
pub mod clang;

pub use python::PythonBackend;
pub use clang::ClangBackend;

pub trait Backend {
    fn get_template(&self) -> Option<&str>;

    fn run(&self, fname: &Path) -> std::io::Result<()>;

    fn try_guess_test_file(&self, fname: &Path) -> Option<PathBuf> {
        let maybe_test = fname.with_extension("txt");
        if maybe_test.exists() {
            return Some(maybe_test);
        }
        None
    }
}
