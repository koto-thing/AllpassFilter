use nih_plug::prelude::*;
use nih_plug::params::{FloatParam};
use std::sync::Arc;
use allpass_filter::{AllPassFilter, DelayLine, Linear};

/// コムフィルタ
/// 式: y[n] = x[n] + feedback * y[n - D]
struct CombFilter {
    delay_line: DelayLine<f32, Linear>,
    delay_samples: f32,
    feedback: f32,
}

impl CombFilter {
    fn new(sample_rate: f32, delay_ms: f32, feedback: f32) -> Self {
        let delay_samples = (sample_rate * (delay_ms / 1000.0)).round();
        Self {
            delay_line: DelayLine::new((delay_samples as usize) + 1000, Linear),
            delay_samples,
            feedback,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read_interpolated(self.delay_samples);
        let output = input + (delayed * self.feedback);
        self.delay_line.push(output); // IIR型
        output
    }
}

/// シュレーダー・リバーブ本体
struct SchroederReverb {
    combs: Vec<CombFilter>,
    apfs: Vec<AllPassFilter<f32, Linear>>,
}

impl SchroederReverb {
    fn new(sample_rate: f32) -> Self {
        // 並列コムフィルタ
        let comb_params = [
            (29.7, 0.95), (37.1, 0.93), (41.1, 0.91), (43.7, 0.89)
        ];
        // 直列オールパスフィルタ
        let apf_params = [
            (5.0, 0.7), (1.7, 0.7)
        ];

        let combs = comb_params.iter()
            .map(|(ms, fb)| CombFilter::new(sample_rate, *ms, *fb))
            .collect();

        let apfs = apf_params.iter()
            .map(|(ms, g)| {
                let s = (sample_rate as f64 * (ms / 1000.0)) as f32;
                AllPassFilter::new((s as usize) + 500, s, *g, Linear)
            })
            .collect();

        Self { combs, apfs }
    }

    fn process(&mut self, input: f32) -> f32 {
        // コムフィルタを並列で処理
        let mut wet = 0.0;
        for comb in &mut self.combs {
            wet += comb.process(input);
        }

        // オールパスフィルタを直列で処理
        for apf in &mut self.apfs {
            wet = apf.process(wet);
        }

        wet * 0.2
    }
}

pub struct MyReverb {
    params: Arc<MyReverbParams>,
    reverb: Option<SchroederReverb>,
}

#[derive(Params)]
struct MyReverbParams {
    #[id = "dry_wet"]
    pub dry_wet: FloatParam,

    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for MyReverb {
    fn default() -> Self {
        Self {
            params: Arc::new(MyReverbParams {
                dry_wet: FloatParam::new(
                    "Dry/Wet",
                    0.5,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                ),
                gain: FloatParam::new(
                    "Output Gain",
                    1.0,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                ).with_smoother(SmoothingStyle::Linear(50.0)),
            }),
            reverb: None,
        }
    }
}

impl Plugin for MyReverb {
    const NAME: &'static str = "Simple Rust Reverb";
    const VENDOR: &'static str = "Kenta Goto";
    const URL: &'static str = "https://github.com/koto-thing";
    const EMAIL: &'static str = "gotoukenta62@gmail.com";
    const VERSION: &'static str = "0.0.1";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // サンプルレートに合わせてリバーブ生成
        self.reverb = Some(SchroederReverb::new(buffer_config.sample_rate));
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let reverb = match &mut self.reverb {
            Some(r) => r,
            None => return ProcessStatus::Normal,
        };

        for mut channel_samples in buffer.iter_samples() {
            if channel_samples.len() == 0 {
                continue;
            }
            let dry_wet = self.params.dry_wet.value();
            let gain = self.params.gain.value();

            // 左チャンネルの入力取得
            let in_l = *channel_samples.get_mut(0).unwrap();

            // 右チャンネルがあれば取得、なければ左と同じにする
            let in_r = if channel_samples.len() > 1 {
                *channel_samples.get_mut(1).unwrap()
            } else {
                in_l
            };

            // モノラル入力としてリバーブ計算
            let mono_in = (in_l + in_r) * 0.5;
            let wet_signal = reverb.process(mono_in);

            // ステレオ出力
            let out_l = (in_l * (1.0 - dry_wet)) + (wet_signal * dry_wet);
            let out_r = (in_r * (1.0 - dry_wet)) + (wet_signal * dry_wet);

            *channel_samples.get_mut(0).unwrap() = out_l * gain;
            if channel_samples.len() > 1 {
                *channel_samples.get_mut(1).unwrap() = out_r * gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MyReverb {
    const CLAP_ID: &'static str = "com.koto-thing.simple-reverb";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Simple Rust Reverb");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("https://github.com/koto-thing");
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("mailto:gotoukenta62@gmail.com");
    const CLAP_FEATURES: &'static [ClapFeature] = &[];
}

impl Vst3Plugin for MyReverb {
    const VST3_CLASS_ID: [u8; 16] = *b"SimpleReverbRust";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Reverb];
}

nih_export_clap!(MyReverb);
nih_export_vst3!(MyReverb);