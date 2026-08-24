use crate::params::NoiseGateParams;
use crate::dsp::filters::{HighPassFilter, LowPassFilter};
use crate::dsp::envelope::EnvelopeFollower;
use crate::dsp::metal_tools::{SmartGate, AdaptiveThreshold, PalmMuteDetector, PickDetector};
use nih_plug::prelude::*;
use std::sync::Arc;
use std::f32::consts::PI;
use std::collections::VecDeque;

pub struct NoiseGateProcessor {
    params: Arc<NoiseGateParams>,
    hpf: HighPassFilter,
    lpf: LowPassFilter,
    envelope: EnvelopeFollower,
    delay_buffer: VecDeque<f32>,
    smart_gate: SmartGate,
    adaptive_threshold: AdaptiveThreshold,
    palm_mute_detector: PalmMuteDetector,
    pick_detector: PickDetector,
    gui_waveform: Option<Arc<crate::gui::state::WaveformData>>,
}

impl NoiseGateProcessor {
    pub fn new(params: Arc<NoiseGateParams>) -> Self {
        Self {
            params,
            hpf: HighPassFilter::default(),
            lpf: LowPassFilter::default(),
            envelope: EnvelopeFollower::default(),
            delay_buffer: VecDeque::with_capacity(1024),
            smart_gate: SmartGate::default(),
            adaptive_threshold: AdaptiveThreshold::default(),
            palm_mute_detector: PalmMuteDetector::default(),
            pick_detector: PickDetector::default(),
            gui_waveform: None,
        }
    }

    pub fn set_gui_waveform(&mut self, waveform: Arc<crate::gui::state::WaveformData>) {
        self.gui_waveform = Some(waveform);
    }

    pub fn process(&mut self, buffer: &mut Buffer, sample_rate: f32) {
        let sample_rate = sample_rate.max(44100.0).min(192000.0);
        
        // Parámetros del gate - valores base
        let base_threshold_db = self.params.threshold_db.value();
        let range_db = self.params.range_db.value();
        let range_gain = util::db_to_gain(range_db);
        
        // Pick Detector: configurar sensibilidad si está habilitado
        if self.params.pick_detector_enabled.value() {
            self.pick_detector.set_sensitivity(self.params.pick_sensitivity.value());
        }
        
        // Palm Mute Detector: configurar si está habilitado
        if self.params.palm_mute_enabled.value() {
            self.palm_mute_detector.set_threshold(self.params.palm_mute_threshold.value());
            self.palm_mute_detector.set_sample_rate(sample_rate);
        }
        
        // Adaptive Threshold: configurar si está habilitado
        if self.params.adaptive_threshold_enabled.value() {
            self.adaptive_threshold.set_speed(self.params.adaptive_threshold_speed.value());
            self.adaptive_threshold.set_base_threshold(base_threshold_db);
        }
        
        // Attack/Release/Hold - valores base
        let base_attack_ms = self.params.attack_ms.value();
        let base_release_ms = self.params.release_ms.value();
        let hold_ms = self.params.hold_ms.value();
        
        // Smart Gate: ajustar attack/release dinámicamente si está habilitado
        let (attack_ms, release_ms) = if self.params.smart_gate_enabled.value() {
            // Smart Gate necesita el envelope, pero aún no lo tenemos
            // Usaremos valores base por ahora, ajustaremos después del envelope
            (base_attack_ms, base_release_ms)
        } else {
            (base_attack_ms, base_release_ms)
        };
        
        let attack_time = (attack_ms / 1000.0).max(0.0001);
        let release_time = (release_ms / 1000.0).max(0.0001);
        
        let attack_coeff = (-1.0 / (sample_rate * attack_time)).exp();
        let release_coeff = (-1.0 / (sample_rate * release_time)).exp();
        
        // Filtros sidechain con cálculo corregido
        let hp_cutoff = self.params.sidechain_hpf_hz.value().max(20.0); // Protección contra valores extremos
        let lp_cutoff = self.params.sidechain_lpf_hz.value().max(200.0); // Protección contra valores extremos
        
        // HPF: alpha = RC / (RC + 1/sample_rate)
        let hp_rc = 1.0 / (2.0 * PI * hp_cutoff);
        let hp_alpha = hp_rc / (hp_rc + 1.0 / sample_rate);
        
        // LPF: alpha = 1 / (1 + sample_rate * RC) - CORREGIDO
        let lp_rc = 1.0 / (2.0 * PI * lp_cutoff);
        let lp_alpha = 1.0 / (1.0 + sample_rate * lp_rc);
        
        // Lookahead con límite máximo de 20ms para evitar latencia excesiva
        let lookahead_ms = self.params.lookahead_ms.value().min(20.0);
        let lookahead_samples = ((lookahead_ms / 1000.0) * sample_rate) as usize;
        
        // Configurar envelope follower (con hold corregido)
        self.envelope.set_coefficients(attack_coeff, release_coeff);
        self.envelope.set_hold(hold_ms, sample_rate);
        
        // Actualizar filtros si cambian
        if hp_cutoff != self.hpf.get_cutoff() {
            self.hpf.set_cutoff(hp_cutoff);
            self.hpf.reset();
        }
        if lp_cutoff != self.lpf.get_cutoff() {
            self.lpf.set_cutoff(lp_cutoff);
            self.lpf.reset();
        }
        
        for mut sample_block in buffer.iter_samples() {
            for sample in sample_block.iter_mut() {
                let input_sample = *sample;
                
                // Si lookahead es 0, procesar directamente sin delay
                if lookahead_samples == 0 {
                    // Filtrar señal sidechain con cálculo corregido
                    let filtered = self.hpf.process(input_sample, hp_alpha);
                    let filtered = self.lpf.process(filtered, lp_alpha);
                    
                    // Obtener envelope suave
                    let envelope = self.envelope.process(filtered);
                    
                    // Smart Gate: ajustar attack/release dinámicamente basado en envelope
                    let (dynamic_attack_ms, dynamic_release_ms) = if self.params.smart_gate_enabled.value() {
                        self.smart_gate.process(envelope, sample_rate)
                    } else {
                        (base_attack_ms, base_release_ms)
                    };
                    
                    // Recalcular coeficientes con valores dinámicos
                    let dynamic_attack_time = (dynamic_attack_ms / 1000.0).max(0.0001);
                    let dynamic_release_time = (dynamic_release_ms / 1000.0).max(0.0001);
                    let dynamic_attack_coeff = (-1.0 / (sample_rate * dynamic_attack_time)).exp();
                    let dynamic_release_coeff = (-1.0 / (sample_rate * dynamic_release_time)).exp();
                    
                    // Actualizar envelope follower con coeficientes dinámicos
                    self.envelope.set_coefficients(dynamic_attack_coeff, dynamic_release_coeff);
                    
                    // Adaptive Threshold: ajustar threshold dinámicamente
                    let threshold_db = if self.params.adaptive_threshold_enabled.value() {
                        self.adaptive_threshold.process(envelope, base_threshold_db)
                    } else {
                        base_threshold_db
                    };
                    let threshold_linear = util::db_to_gain(threshold_db);
                    
                    // Comparar envelope vs threshold
                    let safe_threshold = threshold_linear.max(1e-6);
                    let ratio = (envelope / safe_threshold).clamp(0.0, 2.0);
                    
                    let gate_gain = if ratio < 0.5 {
                        range_gain
                    } else if ratio > 1.5 {
                        1.0
                    } else {
                        let t = (ratio - 0.5) / 1.0;
                        range_gain + (1.0 - range_gain) * t
                    };
                    
                    let output_sample = input_sample * gate_gain;
                    
                    // Escribir datos al waveform para visualización
                    if let Some(waveform) = &self.gui_waveform {
                        waveform.write_sample(input_sample, output_sample, gate_gain);
                    }
                    
                    *sample = output_sample;
                } else {
                    // Con lookahead: procesar sidechain con input actual, aplicar ganancia a input retrasado
                    
                    // Agregar sample al delay buffer
                    self.delay_buffer.push_back(input_sample);
                    
                    // Procesar sidechain con input actual (no retrasado)
                    let filtered = self.hpf.process(input_sample, hp_alpha);
                    let filtered = self.lpf.process(filtered, lp_alpha);
                    
                    // Obtener envelope suave
                    let envelope = self.envelope.process(filtered);
                    
                    // Smart Gate: ajustar attack/release dinámicamente basado en envelope
                    let (dynamic_attack_ms, dynamic_release_ms) = if self.params.smart_gate_enabled.value() {
                        self.smart_gate.process(envelope, sample_rate)
                    } else {
                        (base_attack_ms, base_release_ms)
                    };
                    
                    // Recalcular coeficientes con valores dinámicos
                    let dynamic_attack_time = (dynamic_attack_ms / 1000.0).max(0.0001);
                    let dynamic_release_time = (dynamic_release_ms / 1000.0).max(0.0001);
                    let dynamic_attack_coeff = (-1.0 / (sample_rate * dynamic_attack_time)).exp();
                    let dynamic_release_coeff = (-1.0 / (sample_rate * dynamic_release_time)).exp();
                    
                    // Actualizar envelope follower con coeficientes dinámicos
                    self.envelope.set_coefficients(dynamic_attack_coeff, dynamic_release_coeff);
                    
                    // Adaptive Threshold: ajustar threshold dinámicamente
                    let threshold_db = if self.params.adaptive_threshold_enabled.value() {
                        self.adaptive_threshold.process(envelope, base_threshold_db)
                    } else {
                        base_threshold_db
                    };
                    let threshold_linear = util::db_to_gain(threshold_db);
                    
                    // Comparar envelope vs threshold
                    let safe_threshold = threshold_linear.max(1e-6);
                    let ratio = (envelope / safe_threshold).clamp(0.0, 2.0);
                    
                    let gate_gain = if ratio < 0.5 {
                        range_gain
                    } else if ratio > 1.5 {
                        1.0
                    } else {
                        let t = (ratio - 0.5) / 1.0;
                        range_gain + (1.0 - range_gain) * t
                    };
                    
                    // Obtener sample retrasado del buffer
                    if self.delay_buffer.len() > lookahead_samples {
                        if let Some(delayed_sample) = self.delay_buffer.pop_front() {
                            // Aplicar ganancia al sample retrasado
                            let output_sample = delayed_sample * gate_gain;
                            
                            // Escribir datos al waveform para visualización
                            if let Some(waveform) = &self.gui_waveform {
                                waveform.write_sample(delayed_sample, output_sample, gate_gain);
                            }
                            
                            *sample = output_sample;
                        } else {
                            *sample = input_sample;
                        }
                    } else {
                        // Warm-up: pasar el sample sin procesamiento hasta que el buffer se llene
                        *sample = input_sample;
                    }
                }
            }
        }
    }

    pub fn meter(&self) -> Arc<crate::meter::SharedMeter> {
        crate::meter::SharedMeter::new()
    }
}
