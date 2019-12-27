use serde_derive::{ Serialize, Deserialize };
use crate::backends::Backend;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClangBackend {
    template: Option<String>
}

impl Backend for ClangBackend {
    fn get_template(&self) -> Option<&str> {
        match self.template {
            Some(ref t) => Some(t),
            None => None
        }
    }

    fn run(&self, _fname: &Path) -> std::io::Result<()> {
        todo!();
    }
}