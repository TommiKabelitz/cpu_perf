use two_dim_array::TwoDimensionalArray;

use crate::perf_events::EventCounts;

pub fn plot_square(
    buffer: &mut TwoDimensionalArray<u32>,
    centre_x: usize,
    centre_y: usize,
    width: usize,
    colour: u32,
) {
    let row_start = centre_y.saturating_sub(width / 2);
    let row_end = (centre_y + width / 2).min(buffer.num_rows() - 1);
    let col_start = centre_x.saturating_sub(width / 2);
    let col_end = (centre_x + width / 2).min(buffer.num_cols() - 1);
    for (_i, row) in buffer
        .rows_mut()
        .enumerate()
        .skip(row_start)
        .take_while(|(i, _)| *i <= row_end)
    {
        row[col_start..col_end].fill(colour);
    }

    // let top_left = centre - width / 2 - width / 2 * buffer_width;
    // for i in 0..width {
    //     buffer[top_left + i * buffer_width..top_left + width + i * buffer_width].fill(colour);
    // }
}

pub fn plot_data_from_buffer(
    value_buffer: &[EventCounts],
    num_counts: usize,
    plot_buffer: &mut TwoDimensionalArray<u32>,
    width: usize,
    buffer_width: usize,
    buffer_height: usize,
    colour: u32,
) {
    plot_buffer.as_mut_slice().fill(0xff000000);

    let point_separation = buffer_width / num_counts;
    let mut x = 0;
    for counts in value_buffer.iter() {
        if counts.num_cache_references > 0 {
            let y = buffer_height
                - buffer_height * counts.num_cache_misses as usize
                    / counts.num_cache_references as usize;
            plot_square(plot_buffer, x, y, width, colour);
        }
        if counts.num_branch_instructions > 0 {
            let y = buffer_height
                - buffer_height * counts.num_branch_misses as usize
                    / counts.num_branch_instructions as usize;
            plot_square(plot_buffer, x, y, width, 0xff0000ff);
        }
        x += point_separation;
    }
}
