use nix::libc::{self, c_int};
use nix::{
    errno::Errno,
    sys::wait::WaitStatus,
    unistd::Pid
};
use std::time::{ Instant, Duration };
use std::process::Child;
use std::thread;
use std::sync::mpsc;
use std::convert::{ TryFrom, TryInto };

mod error;
mod rusage_ffi;
pub use rusage_ffi::Rusage;

pub use error::WaitError;
use error::ProcessSignalInfo;

#[derive(Debug)]
struct WaitInfo {
    pub status: WaitStatus,
    pub usage: Rusage,
    pub wall_time: Duration
}

#[derive(Debug)]
pub struct ChildExitStatus {
    pub usage: Rusage,
    pub wall_time: Duration
}

impl TryFrom<WaitInfo> for ChildExitStatus {
    type Error = WaitError;

    fn try_from(info: WaitInfo) -> Result<Self, Self::Error> {
        match info.status {
            WaitStatus::Exited(pid, ret) =>
                match ret {
                    0 => Ok(ChildExitStatus { usage: info.usage, wall_time: info.wall_time }),
                    _ => Err(WaitError::ReturnNonZero(ret, pid))
                },
            WaitStatus::Signaled(pid, signal, coredump) =>
                Err(WaitError::Signaled(ProcessSignalInfo { pid, signal, coredump })),
            _ => Err(WaitError::NotExited)
        }
    }
}


fn wait4_pid(
    pid: Pid,
    chan: mpsc::Sender<std::result::Result<WaitInfo, nix::Error>>,
    timer: Instant
) {
    let mut status: c_int = 0;
    let mut usg: libc::rusage;
    let wait_ret;

    unsafe {
        usg = std::mem::zeroed();
        wait_ret = libc::wait4(pid.as_raw(), &mut status, 0 as c_int, &mut usg);
    }

    #[allow(unused_must_use)] {
        chan.send(match wait_ret {
            -1 => Err(nix::Error::Sys(Errno::last())),
            _ => WaitStatus::from_raw(pid, status).map(|nix_status| {
                    WaitInfo {
                        status: nix_status,
                        usage: usg.into(),
                        wall_time: timer.elapsed()
                    }
                }
            )
        });
    };
}


pub fn wait_child(
    mut child: Child,
    timeout: Duration,
    timer: Instant
) -> Result<ChildExitStatus, WaitError> {
    let pid = Pid::from_raw(child.id() as i32);
    let (send, recv) = mpsc::channel();

    let thr = thread::spawn(move || wait4_pid(pid, send, timer));

    match recv.recv_timeout(timeout) {
        Ok(Ok(wait_info)) => wait_info.try_into(),
        Ok(Err(err)) => Err(err.into()),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            drop(recv);
            drop(thr);

            #[allow(unused_must_use)] {
                child.kill();
            }

            Err(WaitError::TimedOut(timeout))
        },
        Err(mpsc::RecvTimeoutError::Disconnected) => unreachable!()
    }
}
