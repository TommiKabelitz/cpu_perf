/// A simple sliding window that ensures contiguous data
/// without having to copy all the time by maintaining
/// two contigous copies of the data.
///
/// The most recently added value will always be the **last**
/// value in the slice when calling [`Self::get_current_window`].
pub struct SlidingBuffer<T> {
    window_size: usize,
    /// Current _starting_ index of the small buffer
    current_index: usize,
    buffer: Box<[T]>,
}

impl<T: Copy> SlidingBuffer<T> {
    // Implementation has current_index range from 0 to window_size.
    // When we access the buffer, we step ahead of current size which
    // recently wrote a value and read the remaining buffer.
    // To update, we increment current_index, or set it to zero if it
    // would cross into the copy of the buffer.

    pub fn new(initial_value: T, window_size: usize) -> Self
    where
        T: Copy,
    {
        let buffer = vec![initial_value; window_size * 2].into_boxed_slice();

        Self {
            window_size,
            current_index: 0,
            buffer,
        }
    }

    pub fn get_current_window(&self) -> &[T] {
        unsafe {
            self.buffer
                .get_unchecked(self.current_index + 1..self.current_index + 1 + self.window_size)
        }
    }

    pub fn set_next(&mut self, value: T) {
        if self.current_index + self.window_size + 1 == self.window_size * 2 {
            self.current_index = 0
        } else {
            self.current_index += 1;
        }
        self.buffer[self.current_index] = value;
        self.buffer[self.current_index + self.window_size] = value;
    }
}
