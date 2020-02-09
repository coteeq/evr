use serde_derive::{ Serialize, Deserialize };
use crate::backends::{ Backend, RunError };
use std::process::{ Command };
use std::path::Path;
use crate::wait::{ wait_child, WaitInfo };

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PythonBackend {
    template: Option<String>,
    version: Option<String>,
    timeout: Option<f32>,
}


impl PythonBackend {
    fn get_interpreter(&self) -> String {
        format!(
            "python{}",
            self.version
                .as_ref()
                .unwrap_or(&String::new())
        )
    }
}


impl Backend for PythonBackend {
    fn get_template(&self) -> Option<&str> {
        match self.template {
            Some(ref t) => Some(t),
            None => None
        }
    }

    fn run(&self, fname: &Path) -> Result<WaitInfo, RunError> {
        let timer = std::time::Instant::now();

        let child = Command::new(self.get_interpreter())
            .arg(fname.as_os_str())
            .spawn()?;

        let timeout = std::time::Duration::from_secs_f32(self.timeout.unwrap_or(1.0));
        Ok(wait_child(child, timeout, timer)?)
    }
}