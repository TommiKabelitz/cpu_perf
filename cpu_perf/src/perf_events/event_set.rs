use std::{ffi::c_void, io, mem};

use libc::ioctl;

use super::{
    EventIOState, EventType, PerfEvent, PerfEventAttr, SIZE_OF_U64, flags::PerfEventFlags,
};

/// Counts of each event for a single event period
#[derive(Default, Clone, Copy)]
pub struct EventCounts {
    /// Total CPU cycles. Be wary of what happens as the CPU frequency
    /// scales.
    pub num_cpu_cycles: u64,
    /// Retired instructions. Can be affected by various issues, most
    /// notable hardware interrupt counts.
    pub num_instructions: u64,
    /// Cache accesses, generally Last Level Cache but may vary
    /// depending on your CPU. May also include prefetches and coherency
    /// messages, again CPU dependent.
    pub num_cache_references: u64,
    /// Cache misses, as with `num_cache_references`, generally Last
    /// Level Cache, but CPU dependent. Intended for calculating miss
    /// rate with `num_cache_references`.
    pub num_cache_misses: u64,
    /// Retired branch instruction.
    pub num_branch_instructions: u64,
    /// Mispredicted branch instructions.
    pub num_branch_misses: u64,
    /// Bus cycles, not to be confused with total cycles.
    pub num_bus_cycles: u64,
    // pub num_stalled_cycles_frontend: u64,
    // pub num_stalled_cycles_backend: u64,
    /// Total cycles, not influenced by CPU frequency scaling.
    pub num_ref_cpu_cycles: u64,
}
pub const SIZE_OF_EVENT_COUNTS: usize = std::mem::size_of::<EventCounts>();

/// Struct that wraps a set of perf_event file descriptors
///
/// Can track any combination of [`EventType`], across
///
/// - All CPUs for a specific process.
/// - All processes on a specific CPU.
/// - A specific process on a specific CPU.
///
/// Event types are enabled and disabled using [`Self::enable`] and
/// [`Self::disable`].
///
/// Measurement starts and stops by calling [`Self::update_file_state`]
/// with [`EventIOState::Enable`] and [`EventIOState::Disable`].
///
/// The most recent set of counts between enable and disable are obtained
/// through [`Self::get_counts`].
///
/// As the different EventTypes are collected into a group,
/// one event must be the parent. [`EventType::CpuCycles`] has
/// been arbitrarily chosen as the parent. As such, it cannot
/// be disabled as that will disable the whole group.
///
/// The whole group can be disabled using [`Self::disable_group`].
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
        )
        .inspect_err(|_| eprintln!("Error initialising parent file descriptor: CpuCycles"))?;

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
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: Instructions"))?;
        let cache_references = PerfEvent::open(
            PerfEventAttr::new(EventType::CacheReferences)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: CacheReferences"))?;
        let cache_misses = PerfEvent::open(
            PerfEventAttr::new(EventType::CacheMisses)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: CacheMisses"))?;
        let branch_instructions = PerfEvent::open(
            PerfEventAttr::new(EventType::BranchInstructions)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: BranchInstructions"))?;
        let branch_misses = PerfEvent::open(
            PerfEventAttr::new(EventType::BranchMisses)
                .with_flags(attrs_flags)
                .with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: BranchMisses"))?;
        let bus_cycles = PerfEvent::open(
            PerfEventAttr::new(EventType::BusCycles).with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: BusCycles"))?;
        // let stalled_cycles_frontend = PerfEvent::open(
        //     PerfEventAttr::new(EventType::StalledCyclesFrontend).with_perf_format_group(),
        //     Some(parent_fd),
        //     process_id,
        //     cpu_id,
        //     flags,
        // ).inspect_err(|_| eprintln!("Error initialising file descriptor: StalledCyclesFrontend"))?;
        // let stalled_cycles_backend = PerfEvent::open(
        //     PerfEventAttr::new(EventType::StalledCyclesBackend).with_perf_format_group(),
        //     Some(parent_fd),
        //     process_id,
        //     cpu_id,
        //     flags,
        // ).inspect_err(|_| eprintln!("Error initialising file descriptor: StalledCyclesBackend"))?;
        let ref_cpu_cycles = PerfEvent::open(
            PerfEventAttr::new(EventType::RefCpuCycles).with_perf_format_group(),
            Some(parent_fd),
            process_id,
            cpu_id,
            flags,
        )
        .inspect_err(|_| eprintln!("Error initialising file descriptor: RefCpuCycles"))?;

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

    /// Enable tracking of a set of events.
    ///
    /// See [`Self::update_file_state`] for starting the actual counting.
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

    /// Disable tracking of a set of events.
    ///
    /// See [`Self::update_file_state`] for ending the actual counting.
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

    /// Update the state of the file which collects event counts.
    /// Use this to start and end counting by passing
    /// [`EventIOState::Enable`] and [`EventIOState::Disable`].
    pub fn update_file_state(&self, state: EventIOState) -> io::Result<i32> {
        let res = unsafe { ioctl(self.parent_fd, state as u64, 0) };
        if res < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }

    /// Get the counts currently in the count file. Calling this
    /// while counting is technically probably okay, but not advised.
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
        // buf[0] holds the number of events (types)
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
