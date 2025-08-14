use std::io;
use std::os::unix::io::RawFd;

use libc::{_IO, Ioctl, SYS_perf_event_open, pid_t, syscall};

use crate::perf_events::flags::PerfEventFlags;

pub mod flags;

pub const PERF_EVENT_IOC_ENABLE: Ioctl = _IO(b'$' as u32, 0);
pub const PERF_EVENT_IOC_DISABLE: Ioctl = _IO(b'$' as u32, 1);
pub const _PERF_EVENT_IOC_REFRESH: Ioctl = _IO(b'$' as u32, 2);
pub const PERF_EVENT_IOC_RESET: Ioctl = _IO(b'$' as u32, 3);

#[allow(dead_code)]
pub enum EventType {
    CpuCycles = 0,
    Instructions = 1,
    CacheReferences = 2,
    CacheMisses = 3,
    BranchInstructions = 4,
    BranchMisses = 5,
    BusCycles = 6,
    StalledCyclesFrontend = 7,
    StalledCyclesBackend = 8,
    RefCpuCycles = 9,
}

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

impl PerfEventAttr {
    pub fn new(event: EventType) -> Self {
        Self {
            type_: PERF_TYPE_HARDWARE,
            size: size_of::<Self>() as u32,
            config: event as u64,
            sample_period_or_freq: 0,
            sample_type: 0,
            read_format: 0,
            flags: 0,
            wakeup_events_or_watermark: 0,
            bp_type: 0,
            config1: 0,
            config2: 0,
            branch_sample_type: 0,
            sample_regs_user: 0,
            sample_stack_user: 0,
            clockid: 0,
            sample_regs_intr: 0,
            aux_watermark: 0,
            sample_max_stack: 0,
            __reserved_2: 0,
        }
    }

    pub fn with_flags(self, flags: PerfEventFlags) -> Self {
        Self {
            flags: flags.bits(),
            ..self
        }
    }
}

pub const PERF_TYPE_HARDWARE: u32 = 0;

// pub const _PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
// pub const _PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
// pub const PERF_COUNT_HW_CACHE_REFERENCES: u64 = 2;
// pub const PERF_COUNT_HW_CACHE_MISSES: u64 = 3;
// pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u64 = 4;
// pub const PERF_COUNT_HW_BRANCH_MISSES: u64 = 5;
// pub const _PERF_COUNT_HW_BUS_CYCLES: u64 = 6;
// pub const _PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: u64 = 7;
// pub const _PERF_COUNT_HW_STALLED_CYCLES_BACKEND: u64 = 8;
// pub const _PERF_COUNT_HW_REF_CPU_CYCLES: u64 = 9;

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
