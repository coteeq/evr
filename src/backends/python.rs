use serde_derive::Deserialize;
use crate::backends::{ Backend, RunError };
use std::process::{ Command };
use std::path::Path;
use crate::wait::{ wait_child, ChildExitStatus };
use crate::serde_duration::deserialize_duration;
use std::time::Duration;

#[derive(Debug, Deserialize, Default)]
pub struct PythonBackend {
    template: Option<String>,
    version: Option<String>,

    #[serde(default = "default_timeout", deserialize_with = "deserialize_duration")]
    timeout: Duration,
}


fn default_timeout() -> Duration {
    Duration::from_secs(1)
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

    fn run(&self, fname: &Path) -> Result<ChildExitStatus, RunError> {
        let timer = std::time::Instant::now();

        let child = Command::new(self.get_interpreter())
            .arg(fname.as_os_str())
            .spawn()?;

        Ok(wait_child(child, self.timeout, timer)?)
    }
}