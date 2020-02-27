use nix::libc;
use std::time::Duration;

#[derive(Debug)]
pub struct Rusage {
    pub ru_utime: Duration,
    pub ru_stime: Duration,
    pub ru_maxrss: i64,
    pub ru_ixrss: i64,
    pub ru_idrss: i64,
    pub ru_isrss: i64,
    pub ru_minflt: i64,
    pub ru_majflt: i64,
    pub ru_nswap: i64,
    pub ru_inblock: i64,
    pub ru_oublock: i64,
    pub ru_msgsnd: i64,
    pub ru_msgrcv: i64,
    pub ru_nsignals: i64,
    pub ru_nvcsw: i64,
    pub ru_nivcsw: i64,
}

impl From<libc::rusage> for Rusage {
    fn from(usg: libc::rusage) -> Rusage {
        const MICROS_IN_SEC: u64 = 1_000_000;
        let convert_timeval = |tv: libc::timeval| {
            Duration::from_micros(
                tv.tv_sec as u64 * MICROS_IN_SEC +
                tv.tv_usec as u64
            )
        };

        Rusage {
            ru_utime: convert_timeval(usg.ru_utime),
            ru_stime: convert_timeval(usg.ru_stime),
            ru_maxrss: usg.ru_maxrss,
            ru_ixrss: usg.ru_ixrss,
            ru_idrss: usg.ru_idrss,
            ru_isrss: usg.ru_isrss,
            ru_minflt: usg.ru_minflt,
            ru_majflt: usg.ru_majflt,
            ru_nswap: usg.ru_nswap,
            ru_inblock: usg.ru_inblock,
            ru_oublock: usg.ru_oublock,
            ru_msgsnd: usg.ru_msgsnd,
            ru_msgrcv: usg.ru_msgrcv,
            ru_nsignals: usg.ru_nsignals,
            ru_nvcsw: usg.ru_nvcsw,
            ru_nivcsw: usg.ru_nivcsw,
        }
    }
}

impl Rusage {
    pub fn get_rss_bytes(&self) -> i64 {
        if cfg!(target_os = "macos") {
            self.ru_maxrss
        } else if cfg!(target_os = "linux") {
            self.ru_maxrss * 1000 // on linux ru_maxrss is in kilobytes
        } else {
            unimplemented!();
        }
    }
}