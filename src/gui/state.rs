use crate::presets::Preset;
use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

#[derive(Clone)]
pub struct GuiState {
    pub waveform: Arc<WaveformData>,
    pub current_preset: Arc<AtomicCell<Preset>>,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            waveform: Arc::new(WaveformData::new()),
            current_preset: Arc::new(AtomicCell::new(Preset::Default)),
        }
    }
}

pub struct WaveformData {
    pub input_samples: Arc<AtomicCell<[f32; 512]>>,
    pub output_samples: Arc<AtomicCell<[f32; 512]>>,
    pub gain_reduction: Arc<AtomicCell<[f32; 512]>>,
    pub write_index: Arc<AtomicCell<usize>>,
}

impl WaveformData {
    pub fn new() -> Self {
        Self {
            input_samples: Arc::new(AtomicCell::new([0.0; 512])),
            output_samples: Arc::new(AtomicCell::new([0.0; 512])),
            gain_reduction: Arc::new(AtomicCell::new([0.0; 512])),
            write_index: Arc::new(AtomicCell::new(0)),
        }
    }

    pub fn write_sample(&self, input: f32, output: f32, gain: f32) {
        let idx = self.write_index.fetch_add(1);
        let idx = idx % 512;
        
        let mut input_buf = self.input_samples.load();
        let mut output_buf = self.output_samples.load();
        let mut gain_buf = self.gain_reduction.load();
        
        input_buf[idx] = input;
        output_buf[idx] = output;
        gain_buf[idx] = gain;
        
        self.input_samples.store(input_buf);
        self.output_samples.store(output_buf);
        self.gain_reduction.store(gain_buf);
    }
}
