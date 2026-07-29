use std::sync::Arc;

use crate::{
    meter::SharedMeter,
    params::NoiseGateParams,
};

pub struct GuiState {
    pub params: Arc<NoiseGateParams>,
    pub meter: Arc<SharedMeter>,
}

impl GuiState {
    pub fn new(
        params: Arc<NoiseGateParams>,
        meter: Arc<SharedMeter>,
    ) -> Self {
        Self {
            params,
            meter,
        }
    }
}