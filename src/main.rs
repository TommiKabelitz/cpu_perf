use std::io;
use std::thread::sleep;
use std::time::Duration;

use cpu_perf::{
    perf_events::{EventIOState, EventSet, EventType},
    plot::plot_square,
    window::X11Window,
};

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;

fn main() -> io::Result<()> {
    let mut buffer: Vec<u32> = vec![0xff000000; (WIDTH * HEIGHT) as usize];

    let x11_window = X11Window::new(0, 0, WIDTH, HEIGHT, 1, 1, 0xffffff, &buffer)
        .expect("Failed to open window");
    x11_window.set_title("window").expect("Failed to set title");
    x11_window.show();
    x11_window.wait_map_notify();

    let cpu_id = 0;

    let mut event_set = EventSet::new(Some(cpu_id), None)?;
    event_set.enable(&[
        EventType::CacheReferences,
        EventType::CacheMisses,
        EventType::BranchInstructions,
        EventType::BranchMisses,
    ]);

    println!("CPU performance for cpu = {}", cpu_id);
    println!(
        "{:^8} {:^20} {:^20} {:^20} {:^20}",
        "t", "cache accesses", "cache misses", "branch instructions", "branch misses"
    );

    let mut t: usize = 0;
    loop {
        x11_window.update_window();

        // for v in &mut buffer[t * 100..(t + 1) * 100] {
        //     *v = 0xff00ff00
        // }

        event_set.update_file_state(EventIOState::Enable)?;
        sleep(Duration::from_millis(100));
        event_set.update_file_state(EventIOState::Disable)?;
        let counts = event_set.get_counts()?;

        let cache_accesses = counts.num_cache_references;
        let cache_misses = counts.num_cache_misses;
        let branch_instructions = counts.num_branch_instructions;
        let branch_misses = counts.num_branch_misses;

        let y = HEIGHT as u64 * cache_misses / cache_accesses;
        let x = WIDTH as u64 - t as u64 * 5;
        let index = ((HEIGHT as u64 - y) * WIDTH as u64 - x) as usize;
        plot_square(&mut buffer, index, 10, WIDTH as usize, 0xff00ff00);

        let y = HEIGHT as u64 * branch_misses / branch_instructions;
        let x = WIDTH as u64 - t as u64 * 5;
        let index = ((HEIGHT as u64 - y) * WIDTH as u64 - x) as usize;
        plot_square(&mut buffer, index, 10, WIDTH as usize, 0xff0000ff);

        println!(
            "{:^8} {:^20} {:^20} {:^20} {:^20}",
            t, cache_accesses, cache_misses, branch_instructions, branch_misses
        );
        t += 1;
        x11_window.show();
    }
}
