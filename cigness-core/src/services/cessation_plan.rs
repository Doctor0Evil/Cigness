use serde::{Deserialize, Serialize};

use crate::domain::health_risk::HealthRiskReport;
use crate::domain::smoker_profile::{HealthRiskBand, SmokerProfile};

/// High-level cessation strategy aligned with real-world clinical practice.
/// This does not prescribe drugs or specific therapies; it structures
/// intensity, monitoring, and referral flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CessationPlan {
    pub profile_id: String,
    /// Target reduction in packs/day over the next 90 days.
    pub target_packs_per_day_90d: f32,
    /// Whether immediate clinician or counseling referral is recommended.
    pub referral_recommended: bool,
    /// Recommended monitoring cadence in days.
    pub monitoring_interval_days: u32,
}

pub struct CessationPlanner;

impl CessationPlanner {
    pub fn build_plan(profile: &SmokerProfile, risk: &HealthRiskReport) -> CessationPlan {
        let baseline_ppd = profile.packs_per_day.max(0.0);

        // For high/critical risk, push toward rapid reduction and referral.
        let (target_ppd, referral, interval) = match risk.risk_band {
            HealthRiskBand::Critical => {
                (0.0_f32, true, 7) // aim for full cessation with weekly monitoring
            }
            HealthRiskBand::High => {
                (baseline_ppd * 0.25, true, 14) // 75% reduction over 90 days
            }
            HealthRiskBand::Moderate => {
                (baseline_ppd * 0.5, false, 21) // 50% reduction
            }
            HealthRiskBand::Low => {
                (baseline_ppd * 0.75, false, 30) // 25% reduction
            }
        };

        CessationPlan {
            profile_id: profile.profile_id.clone(),
            target_packs_per_day_90d: target_ppd.max(0.0),
            referral_recommended: referral || profile.clinician_recommended_cessation,
            monitoring_interval_days: interval,
        }
    }
}
