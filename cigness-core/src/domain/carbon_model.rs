use serde::{Deserialize, Serialize};

/// CarbonModel calculates per-smoker and aggregated CO₂-equivalent emissions
/// from real smoking patterns, using configurable factors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonFactors {
    /// CO₂e per cigarette (kg), from environmental LCA data sources.
    pub kg_co2e_per_cigarette: f32,
    /// Number of cigarettes per pack (typically 20).
    pub cigarettes_per_pack: u32,
    /// CO₂e per disposed cigarette butt (kg), including cleanup logistics.
    pub kg_co2e_per_butt_waste: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmokerCarbonFootprint {
    /// Annual CO₂e from combustion.
    pub annual_kg_co2e_combustion: f32,
    /// Annual CO₂e from butt waste and cleanup.
    pub annual_kg_co2e_waste: f32,
    /// Total annual CO₂e footprint from smoking.
    pub annual_kg_co2e_total: f32,
}

impl CarbonFactors {
    /// Compute annual footprint from average packs per day.
    pub fn compute_annual_footprint(&self, packs_per_day: f32) -> SmokerCarbonFootprint {
        let packs_per_day = packs_per_day.max(0.0);
        let cigarettes_per_pack = self.cigarettes_per_pack.max(1) as f32;
        let cigarettes_per_day = packs_per_day * cigarettes_per_pack;
        let days_per_year = 365.0;

        let annual_cigarettes = cigarettes_per_day * days_per_year;

        let annual_kg_co2e_combustion = annual_cigarettes * self.kg_co2e_per_cigarette;
        let annual_kg_co2e_waste = annual_cigarettes * self.kg_co2e_per_butt_waste;
        let annual_kg_co2e_total = annual_kg_co2e_combustion + annual_kg_co2e_waste;

        SmokerCarbonFootprint {
            annual_kg_co2e_combustion,
            annual_kg_co2e_waste,
            annual_kg_co2e_total,
        }
    }
}
