use nih_plug::{prelude::*, nih_export_standalone};
use simple_reverb::MyReverb;

fn main() {
    nih_export_standalone::<MyReverb>();
}