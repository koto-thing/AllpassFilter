use num_traits::Float;

pub trait Interpolator<T> {
    fn interpolate(&self, buffer: &[T], read_pos: f64) -> T;
}

pub struct Linear;

impl<T: Float> Interpolator<T> for Linear {
    /// 線形補間を行う
    /// `buffer`: 補間対象のリングバッファ
    /// `read_pos`: 読み出し位置（小数点以下を含む）
    /// 戻り値: 補間されたサンプル値
    fn interpolate(&self, buffer: &[T], read_pos: f64) -> T {
        let len = buffer.len();
        
        // 整数部と小数部の分離
        let index_i = read_pos.floor() as usize;
        let frac = read_pos - (index_i as f64);
        
        // リングバッファのインデックス計算
        let index0 = index_i % len;
        let index1 = (index_i + 1) % len;
        
        let value0 = buffer[index0];
        let value1 = buffer[index1];
        
        // 線形補間
        let frac_t = T::from(frac).unwrap();
        value0 * (T::one() - frac_t) + value1 * frac_t
    }
}

pub struct Nearest;

impl<T: Float> Interpolator<T> for Nearest {
    /// 最近傍補間を行う
    /// `buffer`: 補間対象のリングバッファ
    /// `read_pos`: 読み出し位置（小数点以下を含む）
    /// 戻り値: 補間されたサンプル値
    fn interpolate(&self, buffer: &[T], read_pos: f64) -> T {
        let index = (read_pos.round() as usize) % buffer.len();
        buffer[index]
    }
}

pub struct Cubic;

impl<T: Float> Interpolator<T> for Cubic {
    /// 3次補間（Catmull-Romスプライン）を行う
    /// `buffer`: 補間対象のリングバッファ
    /// `read_pos`: 読み出し位置（小数点以下を含む）
    /// 戻り値: 補間されたサンプル値
    fn interpolate(&self, buffer: &[T], read_pos: f64) -> T {
        let len = buffer.len();
        
        // 整数部と小数部の分離
        let index_i = read_pos.floor() as usize;
        let frac = read_pos - (index_i as f64);
        let frac_t = T::from(frac).unwrap();
        
        // 4つの点のインデックス計算
        // p0: 一つ前の点, p1: 現在の点, p2: 次の点, p3: 次の次の点
        let index0 = (index_i + len - 1) % len;
        let index1 = index_i % len;
        let index2 = (index_i + 1) % len;
        let index3 = (index_i + 2) % len;
        
        let p0 = buffer[index0];
        let p1 = buffer[index1];
        let p2 = buffer[index2];
        let p3 = buffer[index3];
        
        // Catmull-Romのスプライン補間
        let c0 = p1;
        let c1 = (p2 - p0) * T::from(0.5).unwrap();
        let c2 = (p0 * T::from(2.0).unwrap())
            - (p1 * T::from(5.0).unwrap())
            + (p2 * T::from(4.0).unwrap())
            - p3;
        let c2 = c2 * T::from(0.5).unwrap();
        let c3 = (p0 * T::from(-1.0).unwrap())
            + (p1 * T::from(3.0).unwrap())
            - (p2 * T::from(3.0).unwrap())
            + p3;
        let c3 = c3 * T::from(0.5).unwrap();
        
        // ホーナー法による多項式評価
        ((c3 * frac_t + c2) * frac_t + c1) * frac_t + c0
    }
}