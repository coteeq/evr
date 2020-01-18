use serde_derive::{ Serialize, Deserialize };
use crate::backends::{ Backend, mk_tmp_dir, RunStatus };
use std::path::{ Path, PathBuf };
use std::io::{ Result, Error, ErrorKind };
use std::process::Command;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use log::trace;


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClangBackend {
    template: Option<String>,

    #[serde(default)]
    args: Vec<String>,

    #[serde(default = "default_cc")]
    cc: String,
}


fn default_cc() -> String {
    "clang++".to_string()
}


fn get_binary_by_filename(fname: &Path) -> Result<PathBuf> {
    let hashed_fname = {
        let mut hasher = DefaultHasher::new();
        fname.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    };
    
    Ok(mk_tmp_dir()?.join(hashed_fname))
}


impl ClangBackend {
    fn build(&self, fname: &Path) -> Result<PathBuf> {
        let binary_fname = get_binary_by_filename(fname)?;
        let get_mtime = |file| {
            std::fs::metadata(file)?
                .modified()
        };
        let src_mod = get_mtime(fname);
        let binary_mod = get_mtime(&binary_fname);

        if src_mod.is_err() || binary_mod.is_err() || src_mod.unwrap() > binary_mod.unwrap() {
            let clang_status = Command::new(&self.cc)
                .args(&self.args)
                .arg("-x").arg("c++")
                .arg(fname.as_os_str())
                .arg("-o").arg(&binary_fname)
                .arg("-lstdc++")
                .status()?;
            
            trace!("{:#?}", clang_status);
            if !clang_status.success() {
                return Err(Error::new(ErrorKind::Other,
                    "could not compile"));
            }
        }

        Ok(binary_fname)
    }
}


impl Backend for ClangBackend {
    fn get_template(&self) -> Option<&str> {
        match self.template {
            Some(ref t) => Some(t),
            None => None
        }
    }

    fn run(&self, fname: &Path) -> Result<RunStatus> {
        let binary_fname = self.build(fname)?;

        let binary_proc = Command::new(&binary_fname).spawn()?;
        get_status(binary_proc)
    }
}

use nix::sys::wait;

#[cfg(unix)]
fn get_status(proc: std::process::Child) -> Result<RunStatus> {
    let id = proc.id() as i32; // for fuck sake, why this emits u32?

    loop {
        let status_result = wait::waitpid(Some(nix::unistd::Pid::from_raw(id)), None)
            .map_err(|err| Error::new(ErrorKind::Other, err));
    
        let status = status_result?;
        match status {
            wait::WaitStatus::Exited(pid, code) => {
                assert_eq!(pid.as_raw(), id);

                if code == 0 {
                    return Ok(RunStatus::Success);
                } else {
                    return Ok(RunStatus::ErrorCode(code));
                }
            },
            wait::WaitStatus::Signaled(pid, sig, coredump) => {
                assert_eq!(pid.as_raw(), id);

                return Ok(RunStatus::Signal(sig, coredump));
            }
            _ => continue,
        }
    }
}

#[cfg(not(unix))]
fn get_status(proc: std::process::Child) -> Result<RunStatus> {
    compile_error!("currently only unix supported");
}