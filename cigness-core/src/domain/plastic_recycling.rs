use serde::{Deserialize, Serialize};

/// Accepted plastic streams for Cigness device manufacturing.
/// These map directly to real-world resin codes and waste streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlasticType {
    PET,
    HDPE,
    PP,
    ABS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasticStream {
    /// Source identifier (e.g., MRF ID, city facility ID, or partner code).
    pub source_id: String,
    /// Type of plastic in this stream.
    pub plastic_type: PlasticType,
    /// Mass of recovered plastic in kilograms.
    pub mass_kg: f32,
}

/// Batch of recycled plastic allocated to device manufacturing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePlasticBatch {
    /// Unique batch identifier for traceability.
    pub batch_id: String,
    /// Total input mass in kilograms.
    pub total_input_mass_kg: f32,
    /// Expected number of devices producible from this batch.
    pub estimated_device_count: u32,
    /// Calculated recycled-content percentage (0.0–1.0).
    pub recycled_content_ratio: f32,
}

impl PlasticStream {
    pub fn mass_non_negative(&self) -> f32 {
        self.mass_kg.max(0.0)
    }
}

impl DevicePlasticBatch {
    /// Create a new batch assuming 100% recycled plastic input and a
    /// target plastic mass per device (kg).
    pub fn from_streams(
        batch_id: String,
        streams: &[PlasticStream],
        kg_per_device_shell: f32,
    ) -> Self {
        let total_input_mass_kg: f32 = streams.iter().map(|s| s.mass_non_negative()).sum();
        let kg_per_device_shell = kg_per_device_shell.max(0.001); // avoid division by zero

        let estimated_device_count =
            (total_input_mass_kg / kg_per_device_shell).floor().max(0.0) as u32;

        DevicePlasticBatch {
            batch_id,
            total_input_mass_kg,
            estimated_device_count,
            recycled_content_ratio: 1.0,
        }
    }
}
