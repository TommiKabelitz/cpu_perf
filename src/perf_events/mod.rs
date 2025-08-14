use std::io;
use std::os::unix::io::RawFd;

use libc::{_IO, Ioctl, SYS_perf_event_open, ioctl, pid_t, read, syscall};

use crate::perf_events::flags::PerfEventFlags;

pub mod flags;

pub const PERF_EVENT_IOC_ENABLE: Ioctl = _IO(b'$' as u32, 0);
pub const PERF_EVENT_IOC_DISABLE: Ioctl = _IO(b'$' as u32, 1);
pub const PERF_EVENT_IOC_REFRESH: Ioctl = _IO(b'$' as u32, 2);
pub const PERF_EVENT_IOC_RESET: Ioctl = _IO(b'$' as u32, 3);

#[repr(u64)]
pub enum EventIOState {
    Enable = PERF_EVENT_IOC_ENABLE,
    Disable = PERF_EVENT_IOC_DISABLE,
    Refresh = PERF_EVENT_IOC_REFRESH,
    _Reset = PERF_EVENT_IOC_RESET,
}

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

const SIZE_OF_U64: usize = size_of::<u64>();
const SIZE_OF_U64_AS_ISIZE: isize = size_of::<u64>() as isize;

pub struct PerfEvent {
    _attrs: PerfEventAttr,
    pub fd: i32,
}

impl PerfEvent {
    pub fn open(
        mut attrs: PerfEventAttr,
        parent_fd: Option<RawFd>,
        pid: pid_t,
        cpu_id: i32,
        flags: u64,
    ) -> io::Result<Self> {
        // perf_event_open convention is to DISABLE parent and ENABLE
        // children who inherit the FD
        let group_fd = if let Some(group_fd) = parent_fd {
            attrs.flags &= (!PerfEventFlags::DISABLED).bits();
            group_fd
        } else {
            attrs.flags &= (PerfEventFlags::DISABLED).bits();
            -1
        };

        let fd = unsafe {
            let fd = syscall(
                SYS_perf_event_open,
                &mut attrs as *mut PerfEventAttr,
                pid,
                cpu_id,
                group_fd,
                flags,
            ) as RawFd;

            if fd < 0 {
                return Err(io::Error::last_os_error());
            } else {
                fd
            }
        };
        Ok(Self { _attrs: attrs, fd })
    }

    pub fn update_file_state(&self, state: EventIOState) {
        unsafe { ioctl(self.fd, state as u64, 0) };
    }

    pub fn get_count(&self) -> io::Result<u64> {
        let mut count: u64 = 0;
        let read_res = unsafe { read(self.fd, &mut count as *mut _ as *mut _, SIZE_OF_U64) };
        match read_res {
            -1 => Err(io::Error::last_os_error()),
            0 => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "EOF while reading perf counter",
            )),
            SIZE_OF_U64_AS_ISIZE => Ok(count),
            n if n < SIZE_OF_U64 as isize => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("short read: got {} bytes, expected {}", n, SIZE_OF_U64),
            )),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Somehow wrote too many bytes"),
            )),
        }
    }
}

impl Drop for PerfEvent {
    fn drop(&mut self) {
        // Avoid double close
        if self.fd >= 0 {
            unsafe {
                let _ = libc::close(self.fd);
            }
            self.fd = -1;
        }
    }
}
