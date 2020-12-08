use crate::wait::WaitError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum RunError {
    IoError(std::io::Error),
    WaitError(WaitError)
}

impl From<IoError> for RunError {
    fn from(err: IoError) -> Self {
        RunError::IoError(err)
    }
}

impl From<WaitError> for RunError {
    fn from(err: WaitError) -> Self {
        RunError::WaitError(err)
    }
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            RunError::IoError(e) => write!(f, "I/O error: {}", e),
            RunError::WaitError(e) => write!(f, "Wait error: {}", e),
        }
    }
}

impl std::error::Error for RunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RunError::IoError(ref e) => Some(e),
            RunError::WaitError(ref e) => Some(e)
        }
    }
}