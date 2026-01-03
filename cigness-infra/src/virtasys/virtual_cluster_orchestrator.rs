use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// VirtualCluster represents a logical Virta-Sys cluster dedicated
/// to Cigness workloads (cessation analytics, geo-grid, plastic loops).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCluster {
    pub cluster_id: String,
    /// Virtual CPUs available.
    pub vcpus: u32,
    /// Memory in GB.
    pub memory_gb: u32,
    /// Storage in TB.
    pub storage_tb: u32,
    /// Network capacity in Gbps.
    pub network_gbps: f32,
    /// Preferred for high-intensity Cigness tasks (e.g., geo-grid recompute).
    pub preferred_for_cigness: bool,
}

/// Energy-aware scheduling window, aligned with Virta-Sys energy scheduling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBlock {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    /// Target power budget in watts for this block.
    pub target_watts: f32,
    /// Target performance index (0.0–1.0) for Cigness workloads.
    pub target_performance_index: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergySchedule {
    pub schedule_id: String,
    pub time_blocks: Vec<TimeBlock>,
}

/// High-level configuration for Cigness virtual cluster orchestration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CignessVirtualClusterOrchestrator {
    pub clusters: HashMap<String, VirtualCluster>,
    pub energy_schedule: EnergySchedule,
    /// Factor that encodes the "computationally-superior" bias for
    /// Cigness workloads (>= 1.0).
    pub computational_superiority_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CignessWorkloadDemand {
    pub required_compute_index: f32,
    pub required_memory_gb: u32,
    pub required_storage_tb: u32,
    pub required_network_gbps: f32,
}

/// Result of placing a Cigness workload on a virtual cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementDecision {
    pub cluster_id: String,
    pub effective_compute_capacity_index: f32,
    pub energy_block_id: String,
}

impl CignessVirtualClusterOrchestrator {
    /// Select the most suitable cluster for a given workload demand,
    /// under the current energy schedule.
    pub fn select_cluster(
        &self,
        demand: &CignessWorkloadDemand,
        now: DateTime<Utc>,
    ) -> Option<PlacementDecision> {
        let block = match self
            .energy_schedule
            .time_blocks
            .iter()
            .find(|b| now >= b.start_time && now < b.end_time)
        {
            Some(b) => b,
            None => return None,
        };

        let mut best_cluster: Option<&VirtualCluster> = None;
        let mut best_score: f32 = f32::MIN;

        for cluster in self.clusters.values() {
            if cluster.memory_gb < demand.required_memory_gb {
                continue;
            }
            if cluster.storage_tb < demand.required_storage_tb {
                continue;
            }
            if cluster.network_gbps < demand.required_network_gbps {
                continue;
            }

            let base_compute_index =
                cluster.vcpus as f32 * self.computational_superiority_factor.max(1.0);
            if base_compute_index < demand.required_compute_index {
                continue;
            }

            let preference_bonus = if cluster.preferred_for_cigness { 1.2 } else { 1.0 };
            let energy_alignment = block.target_performance_index.max(0.1);

            let score = base_compute_index * preference_bonus * energy_alignment;

            if score > best_score {
                best_score = score;
                best_cluster = Some(cluster);
            }
        }

        best_cluster.map(|cluster| PlacementDecision {
            cluster_id: cluster.cluster_id.clone(),
            effective_compute_capacity_index: best_score,
            energy_block_id: self.energy_schedule.schedule_id.clone(),
        })
    }
}
