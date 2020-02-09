use nix;
use std::time::Duration;

#[derive(Debug)]
pub enum WaitError {
    TimedOut(Duration),
    OsError(nix::Error)
}

impl From<nix::Error> for WaitError {
    fn from(err: nix::Error) -> Self {
        WaitError::OsError(err)
    }
}

impl std::fmt::Display for WaitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            WaitError::TimedOut(dur) => write!(f, "process timed out in {:?}", dur),
            WaitError::OsError(err) => err.fmt(f)
        }
    }
}

impl std::error::Error for WaitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            WaitError::TimedOut(_) => None,
            WaitError::OsError(ref e) => Some(e)
        }
    }
}