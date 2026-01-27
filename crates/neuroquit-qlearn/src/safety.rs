use crate::schema::NeuroQuitSessionRow;
use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum NeuroQuitError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("CSV parse error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Value out of allowed range: {0}")]
    Range(String),
    #[error("Schema violation: {0}")]
    Schema(String),
}

/// Enforce basic bioscale safety for each row.
/// - craving_score, theta, coherence in [0,1]
/// - non-negative cigarettes_today
/// - event_type whitelisted
pub fn enforce_row_safety(row: &NeuroQuitSessionRow) -> Result<(), NeuroQuitError> {
    if !(0.0..=1.0).contains(&row.craving_score) {
        return Err(NeuroQuitError::Range("craving_score out of [0,1]".into()));
    }
    if !(0.0..=1.0).contains(&row.frontal_theta_norm) {
        return Err(NeuroQuitError::Range("frontal_theta_norm out of [0,1]".into()));
    }
    if !(0.0..=1.0).contains(&row.theta_coherence_fp) {
        return Err(NeuroQuitError::Range("theta_coherence_fp out of [0,1]".into()));
    }
    if row.cigarettes_today < 0 {
        return Err(NeuroQuitError::Range("cigarettes_today < 0".into()));
    }

    let allowed = [
        "CravingDetected",
        "CravingResolved",
        "Slip",
        "CigaretteFreeDay",
        "EcoMilestone",
    ];
    if !allowed.contains(&row.event_type.as_str()) {
        return Err(NeuroQuitError::Schema("unexpected event_type".into()));
    }

    Ok(())
}
