use serde::{Deserialize, Serialize};

/// GeoGridNode represents a physical node in the Cigness grid:
/// - R-Node: recycling + butt collection
/// - M-Node: micro-manufacturing
/// - C-Node: circularity / device trade-in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    RNode, // Recycling node
    MNode, // Micro-manufacturing node
    CNode, // Circularity node
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoGridNode {
    pub node_id: String,
    pub node_type: NodeType,
    /// Latitude in WGS84.
    pub latitude: f64,
    /// Longitude in WGS84.
    pub longitude: f64,
    /// Daily average smoker visits (e.g., from retail and local data).
    pub avg_smoker_visits_per_day: u32,
    /// Daily average plastic intake (kg) at this node.
    pub avg_plastic_intake_kg_per_day: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoGridSummary {
    pub total_nodes: usize,
    pub total_r_nodes: usize,
    pub total_m_nodes: usize,
    pub total_c_nodes: usize,
    pub total_plastic_intake_kg_per_day: f32,
}

pub struct GeoGridAnalyzer;

impl GeoGridAnalyzer {
    pub fn summarize(nodes: &[GeoGridNode]) -> GeoGridSummary {
        let mut total_r = 0usize;
        let mut total_m = 0usize;
        let mut total_c = 0usize;
        let mut plastic_total = 0.0_f32;

        for n in nodes {
            match n.node_type {
                NodeType::RNode => total_r += 1,
                NodeType::MNode => total_m += 1,
                NodeType::CNode => total_c += 1,
            }
            plastic_total += n.avg_plastic_intake_kg_per_day.max(0.0);
        }

        GeoGridSummary {
            total_nodes: nodes.len(),
            total_r_nodes: total_r,
            total_m_nodes: total_m,
            total_c_nodes: total_c,
            total_plastic_intake_kg_per_day: plastic_total,
        }
    }
}
