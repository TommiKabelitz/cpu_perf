use two_dim_array::TwoDimensionalArray;

use crate::{
    perf_events::EventCounts,
    plot::digits::{
        DECIMAL_POINT, EIGHT, FIVE, FOUR, MINUS, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO,
    },
};
mod digits;

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

pub fn decorate_plot(
    window_buffer: &mut TwoDimensionalArray<u32>,
    plot_x: usize,
    plot_y: usize,
    plot_width: usize,
    plot_height: usize,
    time_extent: f64,
) {
    let mut x = plot_x + 10;
    let y = plot_y + plot_height + 10;

    render_digit(window_buffer, x, y, &MINUS);
    x += 9;
    render_digit(window_buffer, x, y, &ZERO);
    x += 9;
    render_digit(window_buffer, x, y, &ONE);
    x += 9;
    render_digit(window_buffer, x, y, &TWO);
    x += 9;
    render_digit(window_buffer, x, y, &DECIMAL_POINT);
    x += 9;
    render_digit(window_buffer, x, y, &THREE);
    x += 9;
    render_digit(window_buffer, x, y, &FOUR);
    x += 9;
    render_digit(window_buffer, x, y, &FIVE);
    x += 9;
    render_digit(window_buffer, x, y, &SIX);
    x += 9;
    render_digit(window_buffer, x, y, &SEVEN);
    x += 9;
    render_digit(window_buffer, x, y, &EIGHT);
    x += 9;
    render_digit(window_buffer, x, y, &NINE);
    x += 9;
}

fn render_digit(
    window_buffer: &mut TwoDimensionalArray<u32>,
    x: usize,
    y: usize,
    digit: &[u8; 128],
) {
    let mut y = y;
    for row in digit.chunks_exact(8) {
        let slice = window_buffer.get_mut_panic(y, x..x + 8);
        for i in 0..8 {
            slice[i] = row[i] as u32 * 0xffffffff;
        }
        y += 1;
    }
}
