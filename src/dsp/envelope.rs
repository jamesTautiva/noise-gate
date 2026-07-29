use crate::dsp::utils::flush_to_zero;

pub struct EnvelopeFollower {
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
    hold_counter: f32,
    hold_samples: f32,
    adaptive_release: bool,
    prev_target: f32,
}

impl EnvelopeFollower {
    pub fn new() -> Self {
        Self {
            envelope: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            hold_counter: 0.0,
            hold_samples: 0.0,
            adaptive_release: false,
            prev_target: 0.0,
        }
    }

    pub fn set_coefficients(&mut self, attack_coeff: f32, release_coeff: f32) {
        self.attack_coeff = attack_coeff;
        self.release_coeff = release_coeff;
    }

    pub fn set_hold(&mut self, hold_ms: f32, sample_rate: f32) {
        self.hold_samples = (hold_ms / 1000.0) * sample_rate;
    }

    pub fn set_adaptive_release(&mut self, adaptive: bool) {
        self.adaptive_release = adaptive;
    }

    pub fn process(&mut self, target: f32) -> f32 {
        let coeff = if target > self.envelope {
            self.attack_coeff
        } else {
            // Adaptive release: ajustar según la diferencia de target
            if self.adaptive_release {
                let target_diff = (self.prev_target - target).abs();
                // Si la diferencia es grande, release más rápido
                let adaptive_factor = (target_diff * 2.0).min(1.0);
                self.release_coeff * (1.0 + adaptive_factor * 4.0)
            } else {
                self.release_coeff
            }
        };
        
        self.prev_target = flush_to_zero(target);
        
        // Si el target es alto (gate abierto), resetear el contador de hold
        if target > 0.5 {
            self.hold_counter = self.hold_samples;
        }
        
        // Aplicar hold si el contador es positivo
        if self.hold_counter > 0.0 {
            self.hold_counter -= 1.0;
            // Durante hold, mantener el envelope alto (no decay)
        } else {
            self.envelope = target + coeff * (self.envelope - target);
        }
        
        flush_to_zero(self.envelope)
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.hold_counter = 0.0;
        self.prev_target = 0.0;
    }
}

impl Default for EnvelopeFollower {
    fn default() -> Self {
        Self::new()
    }
}
