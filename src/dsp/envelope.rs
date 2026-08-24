use crate::dsp::utils::flush_to_zero;

pub struct EnvelopeFollower {
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
    hold_counter: f32,
    hold_samples: f32,
}

impl EnvelopeFollower {
    pub fn new() -> Self {
        Self {
            envelope: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            hold_counter: 0.0,
            hold_samples: 0.0,
        }
    }

    pub fn set_coefficients(&mut self, attack_coeff: f32, release_coeff: f32) {
        self.attack_coeff = attack_coeff;
        self.release_coeff = release_coeff;
    }

    pub fn set_hold(&mut self, hold_ms: f32, sample_rate: f32) {
        self.hold_samples = (hold_ms / 1000.0) * sample_rate;
    }

    pub fn process(&mut self, target: f32) -> f32 {
        let target_level = target.abs();
        
        // Si el target es alto (attack), resetear el contador de hold
        if target_level > self.envelope {
            self.hold_counter = self.hold_samples;
        }
        
        // Determinar si estamos en attack o release
        let coeff = if target_level > self.envelope {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        
        // Aplicar hold: durante hold, solo permitir attack, no release
        if self.hold_counter > 0.0 {
            self.hold_counter -= 1.0;
            // Durante hold, solo actualizar si estamos en attack
            if target_level > self.envelope {
                self.envelope = target_level + coeff * (self.envelope - target_level);
            }
            // Si estamos en release durante hold, mantener el envelope actual
        } else {
            // Después del hold, seguir la señal con attack/release normales
            self.envelope = target_level + coeff * (self.envelope - target_level);
        }
        
        flush_to_zero(self.envelope)
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.hold_counter = 0.0;
    }
}

impl Default for EnvelopeFollower {
    fn default() -> Self {
        Self::new()
    }
}
