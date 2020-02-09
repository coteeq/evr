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

mod error;
mod rusage_ffi;
pub use rusage_ffi::Rusage;

pub use error::WaitError;

#[derive(Debug)]
pub struct WaitInfo {
    pub status: WaitStatus,
    pub usage: Rusage,
    pub wall_time: Duration
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
) -> Result<WaitInfo, WaitError> {
    let pid = Pid::from_raw(child.id() as i32);
    let (send, recv) = mpsc::channel();

    let thr = thread::spawn(move || wait4_pid(pid, send, timer));

    match recv.recv_timeout(timeout) {
        Ok(Ok(usg)) => Ok(usg),
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
