use crate::perf_events::EventCounts;

pub fn plot_square(
    buffer: &mut [u32],
    centre: usize,
    width: usize,
    buffer_width: usize,
    colour: u32,
) {
    let top_left = centre - width / 2 - width / 2 * buffer_width;
    for i in 0..width {
        buffer[top_left + i * buffer_width..top_left + width + i * buffer_width].fill(colour);
    }
}

pub fn plot_data_from_buffer(
    value_buffer: &[EventCounts],
    num_counts: usize,
    plot_buffer: &mut [u32],
    width: usize,
    buffer_width: usize,
    buffer_height: usize,
    colour: u32,
) {
    plot_buffer.fill(0xff000000);

    let point_separation = (85 * buffer_width / 100) / num_counts;
    let mut x = 15 * point_separation / 2;
    for counts in value_buffer.iter() {
        if counts.num_cache_references > 0 {
            let y = buffer_height * counts.num_cache_misses as usize
                / counts.num_cache_references as usize;
            let index = (buffer_height - y) * buffer_width + x;
            plot_square(plot_buffer, index, width, buffer_width, 0xff00ff00);
        }
        if counts.num_branch_instructions > 0 {
            let y = buffer_height * counts.num_branch_misses as usize
                / counts.num_branch_instructions as usize;
            let index = (buffer_height - y) * buffer_width + x;

            plot_square(plot_buffer, index, width, buffer_width, 0xff0000ff);
        }
        x += point_separation;
    }
}
