use std::io;
use std::os::unix::io::RawFd;

use libc::{_IO, Ioctl, SYS_perf_event_open, pid_t, syscall};

pub mod flags;

pub const PERF_EVENT_IOC_ENABLE: Ioctl = _IO(b'$' as u32, 0);
pub const PERF_EVENT_IOC_DISABLE: Ioctl = _IO(b'$' as u32, 1);
pub const _PERF_EVENT_IOC_REFRESH: Ioctl = _IO(b'$' as u32, 2);
pub const PERF_EVENT_IOC_RESET: Ioctl = _IO(b'$' as u32, 3);

#[repr(C)]
#[derive(Debug, Default)]
pub struct PerfEventAttr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period_or_freq: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events_or_watermark: u32,
    pub bp_type: u32,
    pub config1: u64,
    pub config2: u64,
    pub branch_sample_type: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub clockid: i32,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
}

pub const PERF_TYPE_HARDWARE: u32 = 0;

pub const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES: u64 = 2;
pub const PERF_COUNT_HW_CACHE_MISSES: u64 = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u64 = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES: u64 = 5;
pub const PERF_COUNT_HW_BUS_CYCLES: u64 = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: u64 = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: u64 = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES: u64 = 9;

pub fn perf_event_open(
    attr: &mut PerfEventAttr,
    pid: pid_t,
    cpu: i32,
    group_fd: RawFd,
    flags: u64,
) -> io::Result<RawFd> {
    unsafe {
        let fd = syscall(
            SYS_perf_event_open,
            attr as *mut PerfEventAttr,
            pid,
            cpu,
            group_fd,
            flags,
        ) as RawFd;

        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(fd)
        }
    }
}
