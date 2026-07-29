use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
};

/// Información compartida entre DSP y GUI.
///
/// El DSP escribe.
/// La GUI únicamente lee.
pub struct SharedMeter {
    input_peak: AtomicU32,
    output_peak: AtomicU32,
    gain_reduction: AtomicU32,
    gate_open: AtomicU32,
    waveform: Mutex<Vec<f32>>,
    gate_history: Mutex<Vec<bool>>,
    fft_spectrum: Mutex<Vec<f32>>,
}

impl SharedMeter {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            input_peak: AtomicU32::new(0),
            output_peak: AtomicU32::new(0),
            gain_reduction: AtomicU32::new(0),
            gate_open: AtomicU32::new(0),
            waveform: Mutex::new(vec![0.0; 512]),
            gate_history: Mutex::new(vec![false; 256]),
            fft_spectrum: Mutex::new(vec![0.0; 256]),
        })
    }

    //----------------------------------------------------
    // INPUT
    //----------------------------------------------------

    pub fn set_input_peak(&self, value: f32) {
        self.input_peak
            .store(value.to_bits(), Ordering::Relaxed);
    }

    pub fn input_peak(&self) -> f32 {
        f32::from_bits(
            self.input_peak.load(Ordering::Relaxed),
        )
    }

    //----------------------------------------------------
    // OUTPUT
    //----------------------------------------------------

    pub fn set_output_peak(&self, value: f32) {
        self.output_peak
            .store(value.to_bits(), Ordering::Relaxed);
    }

    pub fn output_peak(&self) -> f32 {
        f32::from_bits(
            self.output_peak.load(Ordering::Relaxed),
        )
    }

    //----------------------------------------------------
    // GAIN REDUCTION
    //----------------------------------------------------

    pub fn set_gain_reduction(&self, value: f32) {
        self.gain_reduction
            .store(value.to_bits(), Ordering::Relaxed);
    }

    pub fn gain_reduction(&self) -> f32 {
        f32::from_bits(
            self.gain_reduction.load(Ordering::Relaxed),
        )
    }

    //----------------------------------------------------
    // GATE
    //----------------------------------------------------

    pub fn set_gate_open(&self, open: bool) {
        self.gate_open.store(
            if open { 1.0f32 } else { 0.0f32 }.to_bits(),
            Ordering::Relaxed,
        );
        
        // Actualizar historial del gate
        if let Ok(mut history) = self.gate_history.lock() {
            history.remove(0);
            history.push(open);
        }
    }

    pub fn gate_open(&self) -> bool {
        f32::from_bits(
            self.gate_open.load(Ordering::Relaxed),
        ) > 0.5
    }

    pub fn get_gate_history(&self) -> Vec<bool> {
        self.gate_history.lock()
            .map(|h| h.clone())
            .unwrap_or_default()
    }

    //----------------------------------------------------
    // WAVEFORM
    //----------------------------------------------------

    pub fn push_waveform_sample(&self, sample: f32) {
        if let Ok(mut waveform) = self.waveform.lock() {
            waveform.remove(0);
            waveform.push(sample);
        }
    }

    pub fn get_waveform(&self) -> Vec<f32> {
        self.waveform.lock()
            .map(|w| w.clone())
            .unwrap_or_default()
    }

    //----------------------------------------------------
    // FFT SPECTRUM
    //----------------------------------------------------

    pub fn set_fft_spectrum(&self, spectrum: Vec<f32>) {
        if let Ok(mut fft) = self.fft_spectrum.lock() {
            *fft = spectrum;
        }
    }

    pub fn get_fft_spectrum(&self) -> Vec<f32> {
        self.fft_spectrum.lock()
            .map(|f| f.clone())
            .unwrap_or_default()
    }
}