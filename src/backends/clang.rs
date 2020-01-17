use serde_derive::{ Serialize, Deserialize };
use crate::backends::{ Backend, mk_tmp_dir };
use std::path::Path;
use std::process::Command;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use log::trace;

type Result<T> = std::io::Result<T>;

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


fn get_binary_by_filename(fname: &Path) -> Result<std::path::PathBuf> {
    let hashed_fname = {
        let mut hasher = DefaultHasher::new();
        fname.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    };
    
    Ok(mk_tmp_dir()?.join(hashed_fname))
}


impl ClangBackend {
    fn build(&self, fname: &Path) -> Result<std::path::PathBuf> {
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
                return Err(std::io::Error::new(std::io::ErrorKind::Other,
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

    fn run(&self, fname: &Path) -> std::io::Result<()> {
        let binary_fname = self.build(fname)?;

        Command::new(&binary_fname)
            .status()
            .map(|status| {
                trace!("{:#?}", status);
                ()
            })
    }
}
