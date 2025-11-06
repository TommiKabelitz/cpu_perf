pub mod colours;
mod digits;

use two_dim_array::TwoDimensionalArray;

use crate::{
    perf_events::EventCounts,
    plot::{
        colours::Colour,
        digits::{DECIMAL_POINT, ONE, ORDERED_DIGITS, ZERO},
    },
};

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

#[allow(clippy::too_many_arguments)]
pub fn decorate_plot(
    window_buffer: &mut TwoDimensionalArray<u32>,
    plot_x: usize,
    plot_y: usize,
    plot_width: usize,
    plot_height: usize,
    time_extent: f64,
    text_colour: Colour,
    background_colour: Colour,
) {
    window_buffer.as_mut_slice().fill(background_colour as u32);
    // y-axis
    let x_start = plot_x - 5 * 8;
    let vertical_separation = plot_height / 10;
    // Render 0.x
    for (i, digit) in ORDERED_DIGITS.iter().enumerate() {
        render_digit(
            window_buffer,
            x_start,
            plot_y + plot_height - i * vertical_separation - 8,
            &ZERO,
            text_colour,
        );
        render_digit(
            window_buffer,
            x_start + 9,
            plot_y + plot_height - i * vertical_separation - 8,
            &DECIMAL_POINT,
            text_colour,
        );
        render_digit(
            window_buffer,
            x_start + 18,
            plot_y + plot_height - i * vertical_separation - 8,
            digit,
            text_colour,
        );
        // Render tick mark
        window_buffer
            .get_mut_panic(
                plot_y + plot_height - i * vertical_separation,
                plot_x - 8..plot_x,
            )
            .fill(text_colour as u32);
        window_buffer
            .get_mut_panic(
                plot_y + plot_height - i * vertical_separation - 1,
                plot_x - 8..plot_x,
            )
            .fill(text_colour as u32);
    }

    // Render 1.0
    render_digit(window_buffer, x_start, plot_y - 8, &ONE, text_colour);
    render_digit(
        window_buffer,
        x_start + 9,
        plot_y - 8,
        &DECIMAL_POINT,
        text_colour,
    );
    render_digit(window_buffer, x_start + 18, plot_y - 8, &ZERO, text_colour);

    // Render tick mark
    window_buffer
        .get_mut_panic(plot_y, plot_x - 8..plot_x)
        .fill(text_colour as u32);
    window_buffer
        .get_mut_panic(plot_y - 1, plot_x - 8..plot_x)
        .fill(text_colour as u32);

    // x-axis
    let tick_spacing = 2;
    let label_separation = (plot_width as f64 / time_extent * tick_spacing as f64) as usize;
    let n_labels = plot_width / label_separation + 1;
    let mut label = 0;
    for i in 0..n_labels {
        let mut x = plot_x + plot_width - i * label_separation - 1;
        // Render tick mark
        for i in 0..8 {
            window_buffer
                .get_mut_panic(plot_y + plot_height + i, x..x + 2)
                .fill(text_colour as u32);
        }
        let mut num = label;
        loop {
            let digit = num % 10;
            render_digit(
                window_buffer,
                x,
                plot_y + plot_height + 18,
                &ORDERED_DIGITS[digit],
                text_colour,
            );
            x -= 9;
            num /= 10;
            if num == 0 {
                break;
            }
        }
        label += tick_spacing;
    }
}

fn render_digit(
    window_buffer: &mut TwoDimensionalArray<u32>,
    x: usize,
    y: usize,
    digit: &[u8; 128],
    colour: Colour,
) {
    let mut y = y;
    for row in digit.chunks_exact(8) {
        let slice = window_buffer.get_mut_panic(y, x..x + 8);
        for i in 0..8 {
            // Could rewrite branchless, but leave it for now
            if row[i] == 1 {
                slice[i] = colour as u32;
            }
        }
        y += 1;
    }
}
