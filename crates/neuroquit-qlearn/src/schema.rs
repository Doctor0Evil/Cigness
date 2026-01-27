use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NeuroQuitSessionRow {
    pub shard_id: String,
    pub user_id: String,            // must be pseudonymous
    pub timestamp_iso: DateTime<Utc>,
    pub event_type: String,         // CravingDetected|CravingResolved|Slip|...
    pub craving_score: f32,         // 0–1
    pub frontal_theta_norm: f32,    // 0–1
    pub theta_coherence_fp: f32,    // 0–1
    pub heart_rate_bpm: f32,
    pub hrv_index: f32,
    pub cigarettes_today: i32,
    pub ecosystem_region: String,
}

#[derive(Debug, Deserialize)]
pub struct TobaccoFootprintRow {
    pub region: String,
    pub co2_kg_per_cig: f32,
    pub water_l_per_cig: f32,
    pub butt_kg_per_cig: f32,
    pub source_ref: String,
}
