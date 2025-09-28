use std::slice::SliceIndex;

pub struct TwoDimensionalArray<'a, T> {
    buffer: &'a mut [T],
    num_rows: usize,
    num_cols: usize,
}

impl<'a, T> TwoDimensionalArray<'a, T> {
    pub fn new(buffer: &'a mut [T], num_rows: usize, num_cols: usize) -> Option<Self> {
        if buffer.len() != num_cols * num_rows {
            None
        } else {
            Some(Self {
                buffer,
                num_rows,
                num_cols,
            })
        }
    }

    /// Returns a reference to an element or row subslice, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get(index).unwrap_unchecked()`.  It's UB
    /// to call `.get_unchecked(len)`, even if you immediately convert to a
    /// pointer.  And it's UB to call `.get_unchecked(..len + 1)`,
    /// `.get_unchecked(..=len)`, or similar.
    ///
    /// [`get`]: slice::get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use cpu_perf::two_dim_array::TwoDimensionalArray;
    /// let mut a = [1,2,3,4];
    /// let x = TwoDimensionalArray::new(&mut a, 2, 2).unwrap();
    ///
    /// unsafe {
    ///     assert_eq!(*x.get_unchecked(0,1), 2);
    ///     assert_eq!(*x.get_unchecked(1,0..2), [3,4])
    /// }
    /// ```
    pub unsafe fn get_unchecked<I>(&self, row_idx: usize, col_idx: I) -> &I::Output
    where
        I: SliceIndex<[T]>,
    {
        unsafe {
            self.buffer
                .get_unchecked(row_idx * self.num_cols..row_idx * self.num_cols + self.num_cols)
                .get_unchecked(col_idx)
        }
    }

    /// Returns a mutable reference to an element or row subslice, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get_mut(index).unwrap_unchecked()`.  It's UB
    /// to call `.get_unchecked_mut(len)`, even if you immediately convert to a
    /// pointer.  And it's UB to call `.get_unchecked_mut(..len + 1)`,
    /// `.get_unchecked_mut(..=len)`, or similar.
    ///
    /// [`get_mut`]: slice::get_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use cpu_perf::two_dim_array::TwoDimensionalArray;
    /// let mut a = [1,2,3,4];
    /// let mut x = TwoDimensionalArray::new(&mut a, 2, 2).unwrap();
    ///
    /// unsafe {
    ///     assert_eq!(*x.get_unchecked_mut(0,1), 2);
    ///     assert_eq!(*x.get_unchecked_mut(1,0..2), [3,4])
    /// }
    /// ```
    pub unsafe fn get_unchecked_mut<I>(&mut self, row_idx: usize, col_idx: I) -> &I::Output
    where
        I: SliceIndex<[T]>,
    {
        unsafe {
            self.buffer
                .get_unchecked_mut(row_idx * self.num_cols..row_idx * self.num_cols + self.num_cols)
                .get_unchecked_mut(col_idx)
        }
    }

    pub fn get<I>(&self, row_idx: usize, col_idx: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.buffer
            .get(row_idx * self.num_cols..row_idx * self.num_cols + self.num_cols)?
            .get(col_idx)
    }

    pub fn get_mut<I>(&mut self, row_idx: usize, col_idx: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.buffer
            .get_mut(row_idx * self.num_cols..row_idx * self.num_cols + self.num_cols)?
            .get_mut(col_idx)
    }

    pub fn get_panic<I>(&self, row_idx: usize, col_idx: I) -> &I::Output
    where
        I: SliceIndex<[T]>,
    {
        &self.buffer[row_idx * self.num_cols..row_idx * self.num_cols + self.num_cols][col_idx]
    }

    pub fn get_mut_panic<I>(&mut self, row_idx: usize, col_idx: I) -> &mut I::Output
    where
        I: SliceIndex<[T]>,
    {
        &mut self.buffer[row_idx * self.num_cols..row_idx * self.num_cols + self.num_cols][col_idx]
    }
}

pub enum TwoDimensionalArrayIndexError {
    RowIndexOutOfBounds,
    ColumnIndexOutOfBounds,
}
