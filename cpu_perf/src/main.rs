use std::io;
use std::thread::sleep;
use std::time::Duration;

use cpu_perf::{
    perf_events::{EventCounts, EventIOState, EventSet, EventType},
    plot::plot_data_from_buffer,
    sliding_window::SlidingBuffer,
    window::X11Window,
};
use two_dim_array::TwoDimensionalArray;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;

const NUM_TIME_SLICES: usize = 200;

fn main() -> io::Result<()> {
    let mut data_buffer = SlidingBuffer::new(EventCounts::default(), NUM_TIME_SLICES);

    let mut buffer: Vec<u32> = vec![0xff000000; (WIDTH * HEIGHT) as usize];

    let x11_window = X11Window::new(0, 0, WIDTH, HEIGHT, 1, 1, 0xffffff, &buffer)
        .expect("Failed to open window");
    x11_window.set_title("window").expect("Failed to set title");
    x11_window.show();
    x11_window.wait_map_notify();

    let cpu_id = 6;

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

    let mut two_dim_buffer_view =
        TwoDimensionalArray::new(&mut buffer, HEIGHT as usize, WIDTH as usize)
            .expect("Failed to init buffer as 2D");

    let mut t: usize = 0;
    loop {
        x11_window.update_window();

        event_set.update_file_state(EventIOState::Enable)?;
        sleep(Duration::from_millis(100));
        event_set.update_file_state(EventIOState::Disable)?;
        let counts = event_set.get_counts()?;
        println!(
            "{:^8} {:^20} {:^20} {:^20} {:^20}",
            t,
            counts.num_cache_references,
            counts.num_cache_misses,
            counts.num_branch_instructions,
            counts.num_branch_misses
        );

        data_buffer.set_next(counts);
        plot_data_from_buffer(
            data_buffer.get_current_window(),
            NUM_TIME_SLICES,
            &mut two_dim_buffer_view,
            6,
            WIDTH as usize,
            HEIGHT as usize,
            0xff00ff00,
        );

        t += 1;
        x11_window.show();
    }
}
