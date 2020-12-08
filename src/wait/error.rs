use nix;
use std::time::Duration;

#[derive(Debug)]
pub struct ProcessSignalInfo {
    pub pid: nix::unistd::Pid,
    pub signal: nix::sys::signal::Signal,
    pub coredump: bool,
}

#[derive(Debug)]
pub enum WaitError {
    TimedOut(Duration),
    ReturnNonZero(i32, nix::unistd::Pid),
    Signaled(ProcessSignalInfo),
    OsError(nix::Error),
    NotExited,
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
            WaitError::ReturnNonZero(ret, pid) => write!(
                f,
                "process exited with non-zero exit code ({}). was {}",
                ret, pid
            ),
            WaitError::Signaled(ref info) => write!(
                f,
                "process killed by {} {}. was {}",
                info.signal,
                if info.coredump { "(core dumped)" } else { "" },
                info.pid
            ),
            WaitError::OsError(err) => err.fmt(f),
            WaitError::NotExited => write!(f, "process signaled, but not exited"),
        }
    }
}

impl std::error::Error for WaitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            WaitError::TimedOut(_)
            | WaitError::ReturnNonZero(_, _)
            | WaitError::Signaled(_)
            | WaitError::NotExited => None,
            WaitError::OsError(ref e) => Some(e),
        }
    }
}
