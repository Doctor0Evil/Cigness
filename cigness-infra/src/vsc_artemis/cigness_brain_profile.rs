use serde::{Deserialize, Serialize};

/// CignessBrainProfile describes how VSC-ARTEMIS treats Cigness as a
/// high-impact, health/environment workload within its system-brain layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CignessBrainProfile {
    /// Stable identifier under the VSC-ARTEMIS system namespace.
    pub system_id: String,
    /// Whether Cigness instructions must be logged via InstructionAudit.
    pub require_blockchain_audit: bool,
    /// Whether Cigness data flows must pass through Data-Lake sanitization.
    pub require_datalake_sanitization: bool,
    /// Whether Cigness XR or neuro-stimulation interfaces must be
    /// guarded by Dream-Catcher no-harm policy.
    pub require_dreamcatcher_no_harm: bool,
}

impl Default for CignessBrainProfile {
    fn default() -> Self {
        CignessBrainProfile {
            system_id: "CIGNESS-SYSTEM".to_string(),
            require_blockchain_audit: true,
            require_datalake_sanitization: true,
            require_dreamcatcher_no_harm: true,
        }
    }
}
