use std::io;
use std::thread::sleep;
use std::time::Duration;

use cpu_perf::perf_events::{EventIOState, EventSet};

fn main() -> io::Result<()> {
    let cpu_id = 0;

    let mut event_set = EventSet::new(cpu_id)?;
    event_set.cache_references.enable();
    event_set.cache_misses.enable();
    event_set.branch_instructions.enable();
    event_set.branch_misses.enable();

    println!("CPU performance for cpu = {}", cpu_id);
    println!(
        "{:^8} {:^20} {:^20} {:^20} {:^20}",
        "t", "cache accesses", "cache misses", "branch instructions", "branch misses"
    );

    let mut t = 0;
    loop {
        event_set.update_file_state(EventIOState::Enable)?;
        sleep(Duration::from_secs(1));
        event_set.update_file_state(EventIOState::Disable)?;
        let counts = event_set.get_counts()?;

        let cache_accesses = counts.num_cache_references;
        let cache_misses = counts.num_cache_misses;
        let branch_instructions = counts.num_branch_instructions;
        let branch_misses = counts.num_branch_misses;

        println!(
            "{:^8} {:^20} {:^20} {:^20} {:^20}",
            t, cache_accesses, cache_misses, branch_instructions, branch_misses
        );
        t += 1;
    }
}
