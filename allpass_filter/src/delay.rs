use num_traits::Float;
use alloc::vec::Vec;
use alloc::vec;
use crate::interpolation::{Interpolator, Linear};

pub struct DelayLine<T, I> {
    buffer: Vec<T>,    // 遅延バッファ
    writer_ptr: usize, // 書き込みポインタ
    interpolator: I,   // 補間方法
}

impl<T: Float, I: Interpolator<T>> DelayLine<T, I> {
    /// 新しいDelayLineを作成
    /// `max_delay`: 最大遅延サンプル数
    /// `interpolator`: 補間方法
    pub fn new(max_delay: usize, interpolator: I) -> Self {
        let buffer = vec![T::zero(); max_delay];
        Self {
            buffer,
            writer_ptr: 0,
            interpolator,
        }
    }

    /// 入力サンプルを遅延線にプッシュ
    /// `input`: 入力サンプル
    pub fn push(&mut self, input: T) {
        self.buffer[self.writer_ptr] = input;
        self.writer_ptr = (self.writer_ptr + 1) % self.buffer.len();
    }

    /// 補間付きで遅延線からサンプルを読み出す
    /// `delay`: 遅延時間（サンプル単位）
    /// 戻り値: 読み出したサンプル
    pub fn read_interpolated(&self, delay: T) -> T {
        let delay_float = delay.to_f64().unwrap();
        let buffer_len = self.buffer.len() as f64;
        let writer_pos = self.writer_ptr as f64;

        // 読み出し位置の計算
        let mut read_pos = writer_pos - delay_float;
        while read_pos < 0.0 {
            read_pos += buffer_len;
        }

        // 補間を使ってサンプルを取得
        self.interpolator.interpolate(&self.buffer, read_pos)
    }
}