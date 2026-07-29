pub mod params;
pub mod dsp;
mod meter;
mod plugin;
mod presets;
mod gui;

use nih_plug::prelude::*;
use plugin::NoiseGate;

nih_export_vst3!(NoiseGate);