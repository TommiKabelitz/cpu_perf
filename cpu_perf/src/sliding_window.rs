pub struct SlidingBuffer<T> {
    window_size: usize,
    /// Current _starting_ index of the small buffer
    current_index: usize,
    buffer: Box<[T]>,
}

impl<T: Copy> SlidingBuffer<T> {
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
