use num_traits::Float;
use alloc::vec::Vec;
use alloc::vec;
use crate::interpolation::{Interpolator, Linear};

pub struct DelayLine<T, I> {
    buffer: Vec<T>,
    writer_ptr: usize,
    interpolator: I,
}

impl<T: Float, I: Interpolator<T>> DelayLine<T, I> {
    pub fn new(max_delay: usize, interpolator: I) -> Self {
        let buffer = vec![T::zero(); max_delay];
        Self {
            buffer,
            writer_ptr: 0,
            interpolator,
        }
    }

    pub fn push(&mut self, input: T) {
        self.buffer[self.writer_ptr] = input;
        self.writer_ptr = (self.writer_ptr + 1) % self.buffer.len();
    }

    pub fn read_interpolated(&self, delay: T) -> T {
        let delay_float = delay.to_f64().unwrap();
        let buffer_len = self.buffer.len() as f64;
        let writer_pos = self.writer_ptr as f64;

        let mut read_pos = writer_pos - delay_float;
        while read_pos < 0.0 {
            read_pos += buffer_len;
        }

        self.interpolator.interpolate(&self.buffer, read_pos)
    }
}