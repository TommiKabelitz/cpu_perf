use std::thread::sleep;
use std::time::Duration;
use std::{io, time::SystemTime};

use cpu_perf::{
    perf_events::{EventCounts, EventIOState, EventSet, EventType},
    plot::{decorate_plot, plot_data_from_buffer},
    sliding_window::SlidingBuffer,
    window::X11Window,
};
use two_dim_array::TwoDimensionalArray;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const PLOT_BUFFER_WIDTH: usize = WIDTH * 2 / 3;
const PLOT_BUFFER_HEIGHT: usize = HEIGHT * 2 / 3;
const PLOT_X: usize = WIDTH * 8 / 25;
const PLOT_Y: usize = HEIGHT * 5 / 20;

const SAMPLE_RATE: u64 = 60;
const SLEEP_TIME: f64 = 1.0 / SAMPLE_RATE as f64;
const NUM_TIME_SLICES: usize = PLOT_BUFFER_WIDTH;
const PLOT_TIME_EXTENT: f64 = SLEEP_TIME * NUM_TIME_SLICES as f64;

fn main() -> io::Result<()> {
    let print_out = false;
    println!("WIDTH = {}, HEIGHT = {}", WIDTH, HEIGHT);
    println!(
        "PLOT_WIDTH = {}, PLOT_HEIGHT = {}",
        PLOT_BUFFER_WIDTH, PLOT_BUFFER_HEIGHT
    );
    println!("PLOT_X = {}, PLOT_Y = {}", PLOT_X, PLOT_Y);
    println!(
        "PLOT (X + WIDTH) = {}, PLOT (Y + HEIGHT) = {}",
        PLOT_X + PLOT_BUFFER_WIDTH,
        PLOT_Y + PLOT_BUFFER_HEIGHT
    );
    println!("SAMPLE_RATE = {}", SAMPLE_RATE);
    println!("SLEEP_TIME = {}", SLEEP_TIME);
    println!("PLOT_TIME_EXTENT = {}", PLOT_TIME_EXTENT);
    if PLOT_X + PLOT_BUFFER_WIDTH > WIDTH || PLOT_Y + PLOT_BUFFER_HEIGHT > HEIGHT {
        panic!("FIX DIMENSIONS");
    }

    let mut data_buffer = SlidingBuffer::new(EventCounts::default(), NUM_TIME_SLICES);

    let mut window_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut plot_buffer: Vec<u32> = vec![0; PLOT_BUFFER_WIDTH * PLOT_BUFFER_HEIGHT];

    let x11_window = X11Window::new(0, 0, WIDTH as u32, HEIGHT as u32, 1, 1, 0, &window_buffer)
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

    if print_out {
        println!("CPU performance for cpu = {}", cpu_id);
        println!(
            "{:^8} {:^20} {:^20} {:^20} {:^20}",
            "t", "cache accesses", "cache misses", "branch instructions", "branch misses"
        );
    }

    let mut two_dim_window_buffer = TwoDimensionalArray::new(&mut window_buffer, HEIGHT, WIDTH)
        .expect("Failed to init buffer as 2D");

    let mut two_dim_plot_buffer =
        TwoDimensionalArray::new(&mut plot_buffer, PLOT_BUFFER_HEIGHT, PLOT_BUFFER_WIDTH)
            .expect("Failed to init buffer as 2D");

    decorate_plot(
        &mut two_dim_window_buffer,
        PLOT_X,
        PLOT_Y,
        PLOT_BUFFER_WIDTH,
        PLOT_BUFFER_HEIGHT,
        PLOT_TIME_EXTENT,
        Colour::WHITE,
        Colour::GREY,
    );

    let sleep_duration = Duration::from_secs_f64(SLEEP_TIME);
    let mut t: usize = 0;
    #[allow(unused_assignments)]
    let mut started_harvesting = SystemTime::now();
    loop {
        let counts = event_set.get_counts()?;
        event_set.update_file_state(EventIOState::Enable)?;
        started_harvesting = SystemTime::now();

        x11_window.update_window();

        if print_out {
            println!(
                "{:^8} {:^20} {:^20} {:^20} {:^20}",
                t,
                counts.num_cache_references,
                counts.num_cache_misses,
                counts.num_branch_instructions,
                counts.num_branch_misses
            );
        }

        data_buffer.set_next(counts);
        plot_data_from_buffer(
            data_buffer.get_current_window(),
            NUM_TIME_SLICES,
            &mut two_dim_plot_buffer,
            2,
            PLOT_BUFFER_WIDTH,
            PLOT_BUFFER_HEIGHT,
            0xff00ff00,
        );

        // Copy plot data into main buffer
        for (window_row, plot_row) in two_dim_window_buffer
            .rows_mut()
            .skip(PLOT_Y)
            .zip(two_dim_plot_buffer.rows())
        {
            window_row[PLOT_X..PLOT_X + PLOT_BUFFER_WIDTH].copy_from_slice(plot_row);
        }

        t += 1;
        x11_window.show();
        sleep(
            sleep_duration.saturating_sub(
                SystemTime::now()
                    .duration_since(started_harvesting)
                    .unwrap_or(sleep_duration),
            ),
        );
        event_set.update_file_state(EventIOState::Disable)?;
    }
}
