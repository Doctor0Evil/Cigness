use serde::{Deserialize, Serialize};

use crate::domain::plastic_recycling::DevicePlasticBatch;

/// Abstraction for a non-nicotine, non-combustion calm-break device.
/// This layer is hardware-agnostic and focuses on usage and material impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroDeviceModel {
    pub model_id: String,
    /// Mass of recycled plastic used in one device shell (kg).
    pub shell_mass_kg: f32,
    /// Estimated average daily sessions per user.
    pub avg_sessions_per_day: u32,
    /// True if the device firmware is explicitly non-addictive (no dark patterns).
    pub non_addictive_design: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroDeviceImpact {
    /// Estimated number of smoker users supported.
    pub supported_users: u32,
    /// Total recycled plastic embodied in devices (kg).
    pub total_recycled_plastic_kg: f32,
    /// Estimated number of cigarettes avoided per day across all users.
    pub cigarettes_avoided_per_day: f32,
}

impl NeuroDeviceModel {
    /// Compute impact from a recycled plastic batch and per-user avoidance factor.
    pub fn compute_impact(
        &self,
        batch: &DevicePlasticBatch,
        cigarettes_avoided_per_user_per_day: f32,
        cigarettes_per_pack: u32,
    ) -> NeuroDeviceImpact {
        let shell_mass_kg = self.shell_mass_kg.max(0.001);
        let supported_users = (batch.total_input_mass_kg / shell_mass_kg)
            .floor()
            .max(0.0) as u32;

        let cigs_avoided_per_user =
            cigarettes_avoided_per_user_per_day.max(0.0) * (cigarettes_per_pack.max(1) as f32);

        NeuroDeviceImpact {
            supported_users,
            total_recycled_plastic_kg: batch.total_input_mass_kg,
            cigarettes_avoided_per_day: cigs_avoided_per_user * supported_users as f32,
        }
    }
}
