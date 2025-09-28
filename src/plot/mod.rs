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
