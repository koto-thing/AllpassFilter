use allpass_filter::{AllPassFilter, Linear, Nearest, Cubic};

fn main() {
    let mut allpass_filter = AllPassFilter::new_default(1000, 10.5, 0.5);

    let input_signal = vec![1.0, 0.0, 0.0, 0.0, 0.0];

    println!("Output: ");
    for input in input_signal {
        let output = allpass_filter.process(input);
        println!("{:.4}", output);
    }
}