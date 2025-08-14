use std::io;
use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;

mod perf_events;

use libc::{close, ioctl, read};
use perf_events::{
    PERF_EVENT_IOC_DISABLE, PERF_EVENT_IOC_ENABLE, PERF_EVENT_IOC_RESET, PerfEventAttr,
    flags::PerfEventFlags, perf_event_open,
};

use crate::perf_events::EventType;

fn main() -> io::Result<()> {
    let cpu_id = 0;

    let mut cache_ref_attrs = PerfEventAttr::new(EventType::CacheReferences)
        .with_flags(PerfEventFlags::DISABLED | PerfEventFlags::EXCLUDE_KERNEL);

    let cache_ref_fd = perf_event_open(&mut cache_ref_attrs, -1, cpu_id, -1, 0)?;
    let parent_fd = cache_ref_fd;

    let mut cache_miss_attrs = PerfEventAttr::new(EventType::CacheMisses)
        .with_flags(PerfEventFlags::DISABLED | PerfEventFlags::EXCLUDE_KERNEL);

    let cache_miss_fd = perf_event_open(&mut cache_miss_attrs, -1, cpu_id, parent_fd, 0)?;

    let mut branch_retired_attrs = PerfEventAttr::new(EventType::BranchInstructions)
        .with_flags(PerfEventFlags::DISABLED | PerfEventFlags::EXCLUDE_KERNEL);

    let branch_retired_fd = perf_event_open(&mut branch_retired_attrs, -1, cpu_id, parent_fd, 0)?;

    let mut branch_miss_attrs = PerfEventAttr::new(EventType::BranchMisses)
        .with_flags(PerfEventFlags::DISABLED | PerfEventFlags::EXCLUDE_KERNEL);

    let branch_miss_fd = perf_event_open(&mut branch_miss_attrs, -1, cpu_id, parent_fd, 0)?;

    unsafe {
        ioctl(parent_fd, PERF_EVENT_IOC_RESET, 0);
        ioctl(parent_fd, PERF_EVENT_IOC_ENABLE, 0);
    }

    sleep(Duration::from_secs(1));

    unsafe {
        ioctl(parent_fd, PERF_EVENT_IOC_DISABLE, 0);

        let mut cache_ref_count: i64 = 0;
        let mut cache_miss_count: i64 = 0;
        let mut branch_retired_count: i64 = 0;
        let mut branch_miss_count: i64 = 0;
        let cache_ref_read_res = read(
            cache_ref_fd,
            &mut cache_ref_count as *mut _ as *mut _,
            size_of::<i64>(),
        );
        let cache_miss_read_res = read(
            cache_miss_fd,
            &mut cache_miss_count as *mut _ as *mut _,
            size_of::<i64>(),
        );
        let branch_retired_read_res = read(
            branch_retired_fd,
            &mut branch_retired_count as *mut _ as *mut _,
            size_of::<i64>(),
        );
        let branch_miss_read_res = read(
            branch_miss_fd,
            &mut branch_miss_count as *mut _ as *mut _,
            size_of::<i64>(),
        );

        if cache_ref_read_res != size_of::<i64>() as isize {
            eprintln!("Failed to read cache_ref counter");
        } else {
            println!("Cache references: {}", cache_ref_count);
        }
        if cache_miss_read_res != size_of::<i64>() as isize {
            eprintln!("Failed to read cache_miss counter");
        } else {
            println!("Cache misses: {}", cache_miss_count);
        }
        if branch_retired_read_res != size_of::<i64>() as isize {
            eprintln!("Failed to read branch_retired counter");
        } else {
            println!("Branch instructions: {}", branch_retired_count);
        }
        if branch_miss_read_res != size_of::<i64>() as isize {
            eprintln!("Failed to read branch_miss counter");
        } else {
            println!("Branch misses: {}", branch_miss_count);
        }
    }

    unsafe {
        close(cache_ref_fd);
        close(cache_miss_fd);
        close(branch_retired_fd);
        close(branch_miss_fd);
    }

    Ok(())
}
