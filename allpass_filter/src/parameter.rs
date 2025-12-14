use num_traits::Float;

/// 平滑化機能を持つパラメータ構造体
#[derive(Clone, Copy, Debug)]
pub struct SmoothedParam<T> {
    current_value: T,
    target_value: T,
    factor: T,
}

impl<T: Float> SmoothedParam<T> {
    /// 新しいSmoothedParamを作成
    /// `initial_value`: 初期値
    /// `smooth_factor`: 平滑化係数 (1.0で即時変化、0.0に近づくほど遅く変化)
    pub fn new(initial_value: T, smooth_factor: T) -> Self {
        Self {
            current_value: initial_value,
            target_value: initial_value,
            factor: smooth_factor,
        }
    }

    /// 目標値を設定
    pub fn set_target(&mut self, target: T) {
        self.target_value = target;
    }

    /// 現在値と目標値を即時に設定
    pub fn set_immediate(&mut self, value: T) {
        self.current_value = value;
        self.target_value = value;
    }

    /// 平滑化係数を設定
    pub fn set_factor(&mut self, factor: T) {
        self.factor = factor;
    }

    /// 次の平滑化ステップを計算し、現在値を更新して返す
    #[inline]
    pub fn next(&mut self) -> T {
        if self.current_value != self.target_value {
            let diff = self.target_value - self.current_value;

            // 十分に近い場合は目標値に直接設定
            if diff.abs() < T::epsilon() {
                self.current_value = self.target_value;
            } else {
                self.current_value = self.current_value + (diff * self.factor);
            }
        }

        self.current_value
    }

    /// 現在の値を取得
    pub fn current(&self) -> T {
        self.current_value
    }
}