use std::io;
use std::thread::sleep;
use std::time::Duration;

mod perf_events;

use perf_events::{EventIOState, PerfEvent, PerfEventAttr, flags::PerfEventFlags};

use crate::perf_events::EventType;

fn main() -> io::Result<()> {
    let cpu_id = 0;
    println!("CPU performance for cpu = {}", cpu_id);
    println!(
        "{:^8} {:^20} {:^20} {:^20} {:^20}",
        "t", "cache accesses", "cache misses", "branch instructions", "branch misses"
    );

    let mut t = 0;
    loop {
        let attrs_flags = PerfEventFlags::EXCLUDE_HV;
        let flags = 0;

        let cache_access_events = PerfEvent::open(
            PerfEventAttr::new(EventType::CacheReferences).with_flags(attrs_flags),
            None,
            -1,
            cpu_id,
            flags,
        )?;
        let parent_fd = cache_access_events.fd;
        let cache_miss_events = PerfEvent::open(
            PerfEventAttr::new(EventType::CacheMisses).with_flags(attrs_flags),
            Some(parent_fd),
            -1,
            cpu_id,
            flags,
        )?;
        let branch_retired_events = PerfEvent::open(
            PerfEventAttr::new(EventType::BranchInstructions).with_flags(attrs_flags),
            Some(parent_fd),
            -1,
            cpu_id,
            flags,
        )?;
        let branch_miss_events = PerfEvent::open(
            PerfEventAttr::new(EventType::BranchMisses).with_flags(attrs_flags),
            Some(parent_fd),
            -1,
            cpu_id,
            flags,
        )?;

        cache_access_events.update_file_state(EventIOState::Refresh);
        cache_access_events.update_file_state(EventIOState::Enable);

        sleep(Duration::from_secs(1));

        cache_access_events.update_file_state(EventIOState::Disable);

        let cache_accesses = cache_access_events.get_count()?;
        let cache_misses = cache_miss_events.get_count()?;
        let branch_instructions = branch_retired_events.get_count()?;
        let branch_misses = branch_miss_events.get_count()?;

        println!(
            "{:^8} {:^20} {:^20} {:^20} {:^20}",
            t, cache_accesses, cache_misses, branch_instructions, branch_misses
        );
        t += 1;
    }
}
