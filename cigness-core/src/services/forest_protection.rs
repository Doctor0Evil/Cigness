use serde::{Deserialize, Serialize};

/// Configuration rule for sawmills that participate in the Cigness ecosystem.
/// This maps to real policy statements: e.g., "no virgin-wood packaging".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SawmillPolicy {
    pub sawmill_id: String,
    /// Must use recycled-plastic or recycled-paper pallets and crates.
    pub require_recycled_packaging: bool,
    /// If true, virgin wood packaging is disallowed.
    pub forbid_virgin_wood_packaging: bool,
    /// Percentage of packaging that must be traceable to PCR/PIR sources (0.0–1.0).
    pub min_recycled_content_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SawmillComplianceStatus {
    pub sawmill_id: String,
    pub compliant: bool,
    pub effective_recycled_content_ratio: f32,
}

pub struct ForestProtectionService;

impl ForestProtectionService {
    pub fn evaluate(
        policy: &SawmillPolicy,
        observed_recycled_content_ratio: f32,
    ) -> SawmillComplianceStatus {
        let ratio = observed_recycled_content_ratio.clamp(0.0, 1.0);
        let compliant = ratio >= policy.min_recycled_content_ratio;

        SawmillComplianceStatus {
            sawmill_id: policy.sawmill_id.clone(),
            compliant,
            effective_recycled_content_ratio: ratio,
        }
    }
}
