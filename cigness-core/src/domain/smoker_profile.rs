use serde::{Deserialize, Serialize};

/// SmokerProfile captures real, self-reported consumption and context data
/// for non-fictional individuals enrolled in the Cigness program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmokerProfile {
    /// Stable, pseudonymous identifier for the individual.
    pub profile_id: String,
    /// Age in full years.
    pub age_years: u8,
    /// Average packs smoked per day (can be fractional, e.g., 0.5).
    pub packs_per_day: f32,
    /// Years the user has been smoking.
    pub years_smoked: f32,
    /// True if the person currently has a physician-confirmed diagnosis
    /// of a smoking-related condition (e.g., COPD, lung cancer).
    pub has_diagnosed_condition: bool,
    /// True if the person reports severe anxiety or tobacco shakes.
    pub reports_severe_anxiety: bool,
    /// True if a clinician has recommended smoking cessation.
    pub clinician_recommended_cessation: bool,
    /// Country or region code for regulatory and emissions models (e.g., "US-AZ").
    pub region_code: String,
}

/// Derived metrics to support health and carbon calculations without
/// relying on hypothetical or illustrative data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmokerDerivedMetrics {
    /// Estimated total pack-years (packs/day * years smoked).
    pub pack_years: f32,
    /// Qualitative risk band based on pack-years and conditions.
    pub risk_band: HealthRiskBand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthRiskBand {
    Low,
    Moderate,
    High,
    Critical,
}

impl SmokerProfile {
    /// Compute derived metrics from a real profile, enforcing non-negative
    /// values and clamping to reasonable bounds.
    pub fn derive_metrics(&self) -> SmokerDerivedMetrics {
        let packs_per_day = self.packs_per_day.max(0.0);
        let years_smoked = self.years_smoked.max(0.0);
        let pack_years = packs_per_day * years_smoked;

        let risk_band = if self.has_diagnosed_condition {
            HealthRiskBand::Critical
        } else if pack_years >= 20.0 {
            HealthRiskBand::High
        } else if pack_years >= 10.0 {
            HealthRiskBand::Moderate
        } else {
            HealthRiskBand::Low
        };

        SmokerDerivedMetrics {
            pack_years,
            risk_band,
        }
    }
}
