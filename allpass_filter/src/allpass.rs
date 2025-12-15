use num_traits::Float;
use crate::delay::DelayLine;
use crate::interpolation::{Interpolator, Linear};
use crate::parameter::SmoothedParam;

pub struct AllPassFilter<T, I> {
    delay_line: DelayLine<T, I>,    // 遅延線
    delay_length: SmoothedParam<T>, // 遅延時間（サンプル単位）
    g: SmoothedParam<T>,            // フィードバックゲイン
}

impl<T: Float> AllPassFilter<T, Linear> {
    pub fn new_default(max_delay_samples: usize, initial_delay: T, gain: T) -> Self {
        Self::new(max_delay_samples, initial_delay, gain, Linear)
    }
}

impl<T: Float, I: Interpolator<T>> AllPassFilter<T, I> {
    /// 新しいAllPassFilterを作成
    /// `max_delay_samples`: 最大遅延サンプル数
    /// `initial_delay`: 初期遅延時間（サンプル単位）
    /// `gain`: フィードバックゲイン
    /// `interpolator`: 補間方法
    pub fn new(max_delay_samples: usize, initial_delay: T, gain: T, interpolator: I) -> Self {
        let default_smooth = T::from(0.01).unwrap();

        Self {
            delay_line: DelayLine::new(max_delay_samples, interpolator),
            delay_length: SmoothedParam::new(initial_delay, default_smooth),
            g: SmoothedParam::new(gain, default_smooth),
        }
    }

    /// 平滑化係数を設定
    /// `factor`: 平滑化係数 (1.0で即時変化、0.0に近づくほど遅く変化)
    pub fn set_smoothing(&mut self, factor: T) {
        self.delay_length.set_factor(factor);
        self.g.set_factor(factor);
    }

    /// オーディオサンプルを処理
    /// `input`: 入力サンプル
    /// 戻り値: 出力サンプル
    #[inline]
    pub fn process(&mut self, input: T) -> T {
        let current_delay = self.delay_length.next();
        let current_g = self.g.next();

        let delayed_value = self.delay_line.read_interpolated(current_delay);
        let v_n = input + (current_g * delayed_value);
        let output = delayed_value - (current_g * v_n);

        self.delay_line.push(v_n);

        output
    }

    /// ブロック単位でオーディオサンプルを処理
    /// `input`: 入力サンプルのスライス
    /// `output`: 出力サンプルのスライス
    pub fn process_block(&mut self, input: &[T], output: &mut [T]) {
        for (in_sample, out_sample) in input.iter().zip(output.iter_mut()) {
            *out_sample = self.process(*in_sample);
        }
    }

    /// ブロック単位でオーディオサンプルをインプレース処理
    /// `buffer`: 入出力サンプルのスライス
    pub fn process_block_inplace(&mut self, buffer: &mut [T]) {
        for sample in buffer.iter_mut() {
            *sample = self.process(*sample);
        }
    }

    /// 遅延時間を設定
    /// `delay`: 遅延時間（サンプル単位）
    pub fn set_delay(&mut self, delay: T) {
        self.delay_length.set_target(delay);
    }

    /// フィードバックゲインを設定
    /// `gain`: フィードバックゲイン
    pub fn set_gain(&mut self, gain: T) {
        self.g.set_target(gain);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpolation::Interpolator;

    #[test]
    fn test_delay_behaviour_at_zero_gain() {
        let mut allpass_filter = AllPassFilter::new(100, 10.0, 0.0, Linear);

        let out0 = allpass_filter.process(0.0);

        for _ in 0..9 {
            let out = allpass_filter.process(0.0);
            assert_eq!(out, 0.0, "Output should be zero before delay is filled");
        }

        let out_delayed = allpass_filter.process(0.0);
        assert_eq!(out_delayed, 0.0, "Output should still be zero after delay is filled");
    }

    #[test]
    fn test_block_processing_consistency() {
        let mut allpass_filter_single = AllPassFilter::new_default(100, 5.5, 0.5);
        let mut allpass_filter_block = AllPassFilter::new_default(100, 5.5, 0.5);

        let input = vec![1.0, -0.5, 0.2, 0.0, 0.0, 0.0];
        let mut output_block = vec![0.0; input.len()];

        allpass_filter_block.process_block(&input, &mut output_block);

        for (i, &sample) in input.iter().enumerate() {
            let out_single = allpass_filter_single.process(sample);
            assert!((out_single - output_block[i]).abs() < 1e-6, "Block and single sample outputs should match");
        }
    }
}