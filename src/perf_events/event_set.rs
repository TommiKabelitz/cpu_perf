use std::{ffi::c_void, io, mem};

use libc::ioctl;

use super::{
    EventIOState, EventType, PerfEvent, PerfEventAttr, SIZE_OF_U64, flags::PerfEventFlags,
};

#[derive(Default, Clone, Copy)]
pub struct EventCounts {
    pub num_cpu_cycles: u64,
    pub num_instructions: u64,
    pub num_cache_references: u64,
    pub num_cache_misses: u64,
    pub num_branch_instructions: u64,
    pub num_branch_misses: u64,
    pub num_bus_cycles: u64,
    // pub num_stalled_cycles_frontend: u64,
    // pub num_stalled_cycles_backend: u64,
    pub num_ref_cpu_cycles: u64,
}
pub const SIZE_OF_EVENT_COUNTS: usize = std::mem::size_of::<EventCounts>();

pub struct EventSet {
    parent_fd: i32,
    cpu_cycles: PerfEvent,
    instructions: PerfEvent,
    cache_references: PerfEvent,
    cache_misses: PerfEvent,
    branch_instructions: PerfEvent,
    branch_misses: PerfEvent,
    bus_cycles: PerfEvent,
    // stalled_cycles_frontend: PerfEvent,
    // stalled_cycles_backend: PerfEvent,
    ref_cpu_cycles: PerfEvent,
}

impl EventSet {
    pub fn new(cpu_id: Option<u32>, process_id: Option<u32>) -> io::Result<Self> {
        if cpu_id.is_none() && process_id.is_none() {
            return Err(io::Error::other(
                "Process Id and CPU Id cannot both be None",
            ));
        }
        let cpu_id = cpu_id.map_or(-1, |id| id as i32);
        let process_id = process_id.map_or(-1, |id| id as i32);

        let flags = 0;
        let attrs_flags = PerfEventFlags::EXCLUDE_HV;

        // Group parent, do not disable
        let cpu_cycles = PerfEvent::open(
            PerfEventAttr::new(EventType::CpuCycles)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            None,
            process_id,
            cpu_id,
            flags,
        )?;

        // TODO: Consider a macro for this
        let parent_fd = cpu_cycles.fd;
        let instructions = PerfEvent::open(
            PerfEventAttr::new(EventType::Instructions)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;
        let cache_references = PerfEvent::open(
            PerfEventAttr::new(EventType::CacheReferences)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;
        let cache_misses = PerfEvent::open(
            PerfEventAttr::new(EventType::CacheMisses)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;
        let branch_instructions = PerfEvent::open(
            PerfEventAttr::new(EventType::BranchInstructions)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;
        let branch_misses = PerfEvent::open(
            PerfEventAttr::new(EventType::BranchMisses)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;
        let bus_cycles = PerfEvent::open(
            PerfEventAttr::new(EventType::BusCycles).with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;
        // let stalled_cycles_frontend = PerfEvent::open(
        //     PerfEventAttr::new(EventType::StalledCyclesFrontend).with_perf_format_group(),
        //     Some(parent_fd),
        //     process_id,
        //     cpu_id,
        //     flags,
        // )?;
        // let stalled_cycles_backend = PerfEvent::open(
        //     PerfEventAttr::new(EventType::StalledCyclesBackend).with_perf_format_group(),
        //     Some(parent_fd),
        //     process_id,
        //     cpu_id,
        //     flags,
        // )?;
        let ref_cpu_cycles = PerfEvent::open(
            PerfEventAttr::new(EventType::RefCpuCycles).with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )?;

        Ok(Self {
            parent_fd,
            cpu_cycles,
            instructions,
            cache_references,
            cache_misses,
            branch_instructions,
            branch_misses,
            bus_cycles,
            // stalled_cycles_frontend,
            // stalled_cycles_backend,
            ref_cpu_cycles,
        })
    }

    pub fn enable(&mut self, events: &[EventType]) {
        for event in events {
            match event {
                EventType::BranchInstructions => self.branch_instructions.enable(),
                EventType::BranchMisses => self.branch_misses.enable(),
                EventType::BusCycles => self.bus_cycles.enable(),
                EventType::CacheMisses => self.cache_misses.enable(),
                EventType::CacheReferences => self.cache_references.enable(),
                EventType::Instructions => self.instructions.enable(),
                EventType::RefCpuCycles => self.ref_cpu_cycles.enable(),
                EventType::CpuCycles => self.cpu_cycles.enable(),
            }
        }
    }

    pub fn disable(&mut self, events: &[EventType]) {
        for event in events {
            match event {
                EventType::BranchInstructions => self.branch_instructions.disable(),
                EventType::BranchMisses => self.branch_misses.disable(),
                EventType::BusCycles => self.bus_cycles.disable(),
                EventType::CacheMisses => self.cache_misses.disable(),
                EventType::CacheReferences => self.cache_references.disable(),
                EventType::Instructions => self.instructions.disable(),
                EventType::RefCpuCycles => self.ref_cpu_cycles.disable(),
                EventType::CpuCycles => {
                    eprintln!("Note: CpuCycles cannot be disabled as it is the parent of the group")
                }
            }
        }
    }

    pub fn update_file_state(&self, state: EventIOState) -> io::Result<i32> {
        let res = unsafe { ioctl(self.parent_fd, state as u64, 0) };
        if res < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }

    pub fn get_counts(&self) -> io::Result<EventCounts> {
        let mut buf: [u64; SIZE_OF_EVENT_COUNTS / SIZE_OF_U64 + 1] =
            [0; SIZE_OF_EVENT_COUNTS / SIZE_OF_U64 + 1];

        let res = unsafe {
            libc::read(
                self.parent_fd,
                buf.as_mut_ptr() as *mut c_void,
                mem::size_of_val(&buf),
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(EventCounts {
            num_cpu_cycles: buf[1],
            num_instructions: buf[2],
            num_cache_references: buf[3],
            num_cache_misses: buf[4],
            num_branch_instructions: buf[5],
            num_branch_misses: buf[6],
            num_bus_cycles: buf[7],
            // num_stalled_cycles_frontend: buf[],
            // num_stalled_cycles_backend: buf[],
            num_ref_cpu_cycles: buf[8],
        })
    }
}
