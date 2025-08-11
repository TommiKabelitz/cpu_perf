use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;
use std::{io, time};

mod perf_events;

use libc::{close, ioctl, read};
use perf_events::{
    PERF_COUNT_HW_BRANCH_INSTRUCTIONS, PERF_COUNT_HW_BRANCH_MISSES, PERF_COUNT_HW_CACHE_MISSES,
    PERF_COUNT_HW_CACHE_REFERENCES, PERF_EVENT_IOC_DISABLE, PERF_EVENT_IOC_ENABLE,
    PERF_EVENT_IOC_RESET, PERF_TYPE_HARDWARE, PerfEventAttr, flags::PerfEventFlags,
    perf_event_open,
};

fn main() -> io::Result<()> {
    let cpu_id = 0;

    let mut cache_ref_attrs = PerfEventAttr::default();
    cache_ref_attrs.type_ = PERF_TYPE_HARDWARE;
    cache_ref_attrs.size = size_of::<PerfEventAttr>() as u32;
    cache_ref_attrs.config = PERF_COUNT_HW_CACHE_REFERENCES;
    cache_ref_attrs.flags = (PerfEventFlags::DISABLED | PerfEventFlags::EXCLUDE_KERNEL).bits();

    let cache_ref_fd = perf_event_open(&mut cache_ref_attrs, -1, cpu_id, -1, 0)?;
    let parent_fd = cache_ref_fd;

    let mut cache_miss_attrs = PerfEventAttr::default();
    cache_miss_attrs.type_ = PERF_TYPE_HARDWARE;
    cache_miss_attrs.size = size_of::<PerfEventAttr>() as u32;
    cache_miss_attrs.config = PERF_COUNT_HW_CACHE_MISSES;
    cache_miss_attrs.flags = PerfEventFlags::EXCLUDE_KERNEL.bits();

    let cache_miss_fd = perf_event_open(&mut cache_miss_attrs, -1, cpu_id, parent_fd, 0)?;

    let mut branch_retired_attrs = PerfEventAttr::default();
    branch_retired_attrs.type_ = PERF_TYPE_HARDWARE;
    branch_retired_attrs.size = size_of::<PerfEventAttr>() as u32;
    branch_retired_attrs.config = PERF_COUNT_HW_BRANCH_INSTRUCTIONS;
    branch_retired_attrs.flags = PerfEventFlags::EXCLUDE_KERNEL.bits();

    let branch_retired_fd = perf_event_open(&mut branch_retired_attrs, -1, cpu_id, parent_fd, 0)?;

    let mut branch_miss_attrs = PerfEventAttr::default();
    branch_miss_attrs.type_ = PERF_TYPE_HARDWARE;
    branch_miss_attrs.size = size_of::<PerfEventAttr>() as u32;
    branch_miss_attrs.config = PERF_COUNT_HW_BRANCH_MISSES;
    branch_miss_attrs.flags = PerfEventFlags::EXCLUDE_KERNEL.bits();

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
