use crate::{AllPassFilter, Linear};
use alloc::boxed::Box;
use core::slice;

pub type CAllPass = AllPassFilter<f32, Linear>;

/// AllPassFilterインスタンスの生成
/// `max_delay`: 最大遅延長（サンプル単位）
/// `initial_delay`: 初期遅延時間（サンプル単位）
/// `gain`: フィードバックゲイン
/// 戻り値: 生成されたAllPassFilterインスタンスへのポインタ
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_create(max_delay: usize, initial_delay: f32, gain: f32) -> *mut CAllPass {
    let apf = AllPassFilter::new_default(max_delay, initial_delay, gain);
    Box::into_raw(Box::new(apf))
}

/// インスタンスの破棄
/// `ptr`: 破棄するAllPassFilterインスタンスのポインタ
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_destroy(ptr: *mut CAllPass) {
    if ptr.is_null() {
        return;
    }
    let _ = Box::from_raw(ptr);
}

/// 単一サンプル処理
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_process(ptr: *mut CAllPass, input: f32) -> f32 {
    let apf = &mut *ptr;
    apf.process(input)
}

/// ブロック単位処理
/// `input`: 入力サンプルのポインタ
/// `output`: 出力サンプルのポインタ
/// `len`: サンプル数
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_process_block(
    ptr: *mut CAllPass,
    input: *const f32,
    output: *mut f32,
    len: usize,
) {
    let apf = &mut *ptr;
    let in_slice = slice::from_raw_parts(input, len);
    let out_slice = slice::from_raw_parts_mut(output, len);

    apf.process_block(in_slice, out_slice);
}

/// 遅延時間の設定
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_set_delay(ptr: *mut CAllPass, delay: f32) {
    let apf = &mut *ptr;
    apf.set_delay(delay);
}

/// ゲインの設定
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_set_gain(ptr: *mut CAllPass, gain: f32) {
    let apf = &mut *ptr;
    apf.set_gain(gain);
}

/// 平滑化係数の設定
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allpass_set_smoothing(ptr: *mut CAllPass, factor: f32) {
    let apf = &mut *ptr;
    apf.set_smoothing(factor);
}
