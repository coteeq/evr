use serde_derive::Deserialize;
use std::path::{Path, PathBuf};
use toml::de;

type Error = std::io::Error;
use std::io::ErrorKind;

use crate::backends::{Backend, ClangBackend, ClangCBackend, PythonBackend};

#[derive(Debug, Deserialize)]
pub struct Conf {
    #[serde(skip)]
    path: Option<PathBuf>,

    #[serde(default)]
    python: PythonBackend,

    #[serde(default)]
    clang: ClangBackend,

    #[serde(default)]
    clang_c: ClangCBackend,
}

impl Conf {
    pub fn get_template(&self, fname: &Path) -> &str {
        self.get_backend(fname)
            .and_then(|backend| backend.get_template())
            .unwrap_or("")
    }

    pub fn get_backend(&self, fname: &Path) -> Option<Box<&dyn Backend>> {
        let ext = fname.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        match ext {
            "py" => Some(Box::new(&self.python)),
            "cc" | "cpp" | "cxx" => Some(Box::new(&self.clang)),
            "c" => Some(Box::new(&self.clang_c)),
            _ => None,
        }
    }
}

pub fn get_conf() -> Result<Conf, Error> {
    let mut current = std::env::current_dir()?;
    let path = loop {
        let candidate = current.join(".evr");
        if candidate.exists() {
            break candidate;
        }

        if !current.pop() {
            return Err(Error::new(ErrorKind::NotFound, "Not a evr subtree"));
        }
    };

    let raw_buf = std::fs::read_to_string(path.as_path())?;

    let buf_result: Result<Conf, de::Error> = de::from_str(&raw_buf);

    buf_result
        .map(|mut buf| {
            buf.path = Some(path.clone());
            buf
        })
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))
}
