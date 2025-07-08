// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/solana/programs/validator/src/state/validator_state.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ValidatorState {
    pub validator_id: String,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PerformanceMetrics {
    pub total_validations: u64,
    pub successful_validations: u64,
    pub failed_validations: u64,
}

impl ValidatorState {
    pub fn new(validator_id: String) -> Self {
        Self {
            validator_id,
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    pub fn record_validation(&mut self, success: bool) {
        self.performance_metrics.total_validations += 1;
        if success {
            self.performance_metrics.successful_validations += 1;
        } else {
            self.performance_metrics.failed_validations += 1;
        }
    }
}