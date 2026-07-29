use crate::params::NoiseGateParams;
use crate::dsp::filters::{HighPassFilter, LowPassFilter};
use crate::dsp::detector::PeakDetector;
use crate::dsp::envelope::EnvelopeFollower;
use crate::dsp::gain::GainStage;
use crate::dsp::metal_tools::{PickDetector, PalmMuteDetector, SmartGate, AdaptiveThreshold, NoiseClassifier};
use nih_plug::prelude::*;
use std::sync::Arc;
use std::f32::consts::PI;
use std::collections::VecDeque;

pub struct NoiseGateProcessor {
    params: Arc<NoiseGateParams>,
    hpf: HighPassFilter,
    lpf: LowPassFilter,
    detector: PeakDetector,
    envelope: EnvelopeFollower,
    gain: GainStage,
    delay_buffer: VecDeque<f32>,
    pick_detector: PickDetector,
    palm_mute_detector: PalmMuteDetector,
    smart_gate: SmartGate,
    adaptive_threshold: AdaptiveThreshold,
    noise_classifier: NoiseClassifier,
    gui_waveform: Option<Arc<crate::gui::state::WaveformData>>,
}

impl NoiseGateProcessor {
    pub fn new(params: Arc<NoiseGateParams>) -> Self {
        Self {
            params,
            hpf: HighPassFilter::default(),
            lpf: LowPassFilter::default(),
            detector: PeakDetector::default(),
            envelope: EnvelopeFollower::default(),
            gain: GainStage::new(),
            delay_buffer: VecDeque::with_capacity(512),
            pick_detector: PickDetector::default(),
            palm_mute_detector: PalmMuteDetector::default(),
            smart_gate: SmartGate::default(),
            adaptive_threshold: AdaptiveThreshold::default(),
            noise_classifier: NoiseClassifier::default(),
            gui_waveform: None,
        }
    }

    pub fn set_gui_waveform(&mut self, waveform: Arc<crate::gui::state::WaveformData>) {
        self.gui_waveform = Some(waveform);
    }

    pub fn process(&mut self, buffer: &mut Buffer, sample_rate: f32) {
        let sample_rate = sample_rate.max(44100.0).min(192000.0);
        
        let open_threshold = util::db_to_gain(self.params.threshold_db.value());
        let close_threshold = open_threshold * util::db_to_gain(-3.0);

        let hp_cutoff = self.params.sidechain_hpf_hz.value();
        let hp_alpha = 1.0 / (1.0 + (2.0 * PI * hp_cutoff / sample_rate));
        
        let lp_cutoff = self.params.sidechain_lpf_hz.value();
        let lp_alpha = 1.0 / (1.0 + (2.0 * PI * lp_cutoff / sample_rate));

        let attack_time = (self.params.attack_ms.value() / 1000.0).max(0.0001);
        let release_time = (self.params.release_ms.value() / 1000.0).max(0.0001);

        let attack_coeff = (-1.0 / (sample_rate * attack_time)).exp();
        let release_coeff = (-1.0 / (sample_rate * release_time)).exp();

        let hold_ms = self.params.hold_ms.value();
        let _hold_samples = ((hold_ms / 1000.0) * sample_rate) as usize;

        // Lookahead delay en samples
        let lookahead_samples = ((self.params.lookahead_ms.value() / 1000.0) * sample_rate) as usize;
        let lookahead_samples = lookahead_samples.min(512); // Limitar tamaño del buffer

        self.detector.set_thresholds(open_threshold, close_threshold);
        self.envelope.set_coefficients(attack_coeff, release_coeff);
        self.envelope.set_hold(hold_ms, sample_rate);
        self.envelope.set_adaptive_release(self.params.adaptive_release.value());
        self.gain.set_range(self.params.range_db.value());

        if self.params.pick_detector_enabled.value() {
            self.pick_detector.set_sensitivity(self.params.pick_sensitivity.value());
        }
        if self.params.palm_mute_enabled.value() {
            self.palm_mute_detector.set_threshold(self.params.palm_mute_threshold.value());
        }
        if self.params.adaptive_threshold_enabled.value() {
            self.adaptive_threshold.set_speed(self.params.adaptive_threshold_speed.value());
        }

        // Pre-calculate enabled features to avoid branches in hot loop
        let pick_enabled = self.params.pick_detector_enabled.value();
        let palm_mute_enabled = self.params.palm_mute_enabled.value();
        let smart_gate_enabled = self.params.smart_gate_enabled.value();
        let adaptive_threshold_enabled = self.params.adaptive_threshold_enabled.value();
        let noise_classification_enabled = self.params.noise_classification_enabled.value();
        let has_waveform = self.gui_waveform.is_some();

        for mut sample_block in buffer.iter_samples() {
            for sample in sample_block.iter_mut() {
                // Guardar sample en delay buffer para lookahead
                self.delay_buffer.push_back(*sample);
                
                if self.delay_buffer.len() > lookahead_samples {
                    if let Some(delayed_sample) = self.delay_buffer.pop_front() {
                        let filtered = self.hpf.process(delayed_sample, hp_alpha);
                        let filtered = self.lpf.process(filtered, lp_alpha);
                        
                        // Metal tools (pre-calculated branches)
                        let _is_pick = if pick_enabled {
                            self.pick_detector.process(filtered)
                        } else {
                            false
                        };
                        
                        let _is_palm_mute = if palm_mute_enabled {
                            self.palm_mute_detector.process(filtered)
                        } else {
                            false
                        };
                        
                        let mut envelope = self.envelope.process(filtered);
                        
                        if smart_gate_enabled {
                            envelope = self.smart_gate.process(envelope);
                        }
                        
                        if adaptive_threshold_enabled {
                            envelope = self.adaptive_threshold.process(envelope, open_threshold);
                        }
                        
                        let _noise_type = if noise_classification_enabled {
                            self.noise_classifier.process(envelope)
                        } else {
                            crate::dsp::metal_tools::NoiseType::Silence
                        };
                        
                        let gate_open = self.detector.process(filtered);
                        let gain = if gate_open { 1.0 } else { self.gain.process(envelope) };
                        let output_sample = *sample * gain;
                        
                        // Write to waveform visualization if available
                        if has_waveform {
                            if let Some(waveform) = &self.gui_waveform {
                                waveform.write_sample(delayed_sample, output_sample, gain);
                            }
                        }
                        
                        *sample = output_sample;
                    } else {
                        *sample = *sample; // Passthrough si buffer vacío
                    }
                } else {
                    *sample = *sample; // Passthrough mientras buffer se llena
                }
            }
        }
    }

    pub fn meter(&self) -> Arc<crate::meter::SharedMeter> {
        crate::meter::SharedMeter::new()
    }
}
