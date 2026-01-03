use serde::{Deserialize, Serialize};

use crate::domain::smoker_profile::{HealthRiskBand, SmokerDerivedMetrics};

/// Structured health risk report aligned with real-world clinical concerns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthRiskReport {
    pub pack_years: f32,
    pub risk_band: HealthRiskBand,
    /// Binary flags for urgent escalation.
    pub escalation_required: bool,
}

impl HealthRiskReport {
    pub fn from_derived(metrics: &SmokerDerivedMetrics) -> Self {
        let escalation_required = matches!(
            metrics.risk_band,
            HealthRiskBand::High | HealthRiskBand::Critical
        );

        HealthRiskReport {
            pack_years: metrics.pack_years,
            risk_band: metrics.risk_band.clone(),
            escalation_required,
        }
    }
}
