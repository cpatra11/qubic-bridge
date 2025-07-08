// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/solana/programs/validator/src/state/mod.rs
pub mod validator_state;

pub struct ValidatorState {
    pub validators: Vec<ValidatorInfo>,
    pub performance_metrics: PerformanceMetrics,
}

pub struct ValidatorInfo {
    pub id: String,
    pub is_active: bool,
    pub stake: u64,
}

pub struct PerformanceMetrics {
    pub total_validations: u64,
    pub successful_validations: u64,
    pub failed_validations: u64,
}

impl ValidatorState {
    pub fn new() -> Self {
        ValidatorState {
            validators: Vec::new(),
            performance_metrics: PerformanceMetrics {
                total_validations: 0,
                successful_validations: 0,
                failed_validations: 0,
            },
        }
    }

    pub fn add_validator(&mut self, id: String, stake: u64) {
        self.validators.push(ValidatorInfo {
            id,
            is_active: true,
            stake,
        });
    }

    pub fn deactivate_validator(&mut self, id: &str) {
        if let Some(validator) = self.validators.iter_mut().find(|v| v.id == id) {
            validator.is_active = false;
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