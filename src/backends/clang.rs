use super::get_binary_by_filename;
use crate::backends::{Backend, RunError};
use crate::serde_duration::deserialize_duration;
use crate::wait::{wait_child, ChildExitStatus};
use serde_derive::Deserialize;
use std::io::{Error, ErrorKind, Result as IoResult};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Deserialize, Default)]
pub struct ClangBackend {
    template: Option<String>,

    #[serde(default)]
    args: Vec<String>,

    #[serde(default = "default_cc")]
    cc: String,

    #[serde(default = "default_timeout", deserialize_with = "deserialize_duration")]
    timeout: Duration,
}

fn default_cc() -> String {
    "clang++".to_string()
}

fn default_timeout() -> Duration {
    Duration::from_secs(1)
}

impl ClangBackend {
    fn build(&self, fname: &Path) -> IoResult<PathBuf> {
        let binary_fname = get_binary_by_filename(fname)?;
        let get_mtime = |file| std::fs::metadata(file)?.modified();
        let src_mod = get_mtime(fname);
        let binary_mod = get_mtime(&binary_fname);

        if src_mod.is_err() || binary_mod.is_err() || src_mod.unwrap() > binary_mod.unwrap() {
            let clang_status = Command::new(&self.cc)
                .arg("-x")
                .arg("c++")
                .arg(fname.as_os_str())
                .arg("-o")
                .arg(&binary_fname)
                .args(&self.args)
                .status()?;

            if !clang_status.success() {
                return Err(Error::new(ErrorKind::Other, "could not compile"));
            }
        }

        Ok(binary_fname)
    }
}

impl Backend for ClangBackend {
    fn get_template(&self) -> Option<&str> {
        match self.template {
            Some(ref t) => Some(t),
            None => None,
        }
    }

    fn run(&self, fname: &Path) -> Result<ChildExitStatus, RunError> {
        let binary_fname = self.build(fname)?;

        let proc = Command::new(&binary_fname).spawn()?;

        Ok(wait_child(proc, self.timeout, std::time::Instant::now())?)
    }
}
