use crate::schema::NeuroQuitSessionRow;
use linfa::prelude::*;
use linfa_trees::RandomForest;
use ndarray::{Array2, Array1};
use std::collections::HashMap;

/// Build per-user, per-day features:
/// - mean craving_score
/// - mean frontal_theta_norm
/// - mean theta_coherence_fp
/// - mean heart_rate_bpm
/// - cigarettes_today (target)
pub fn build_daily_features(
    rows: &[NeuroQuitSessionRow],
) -> (Array2<f32>, Array1<f32>) {
    let mut by_user_day: HashMap<(String, String), Vec<&NeuroQuitSessionRow>> = HashMap::new();

    for row in rows {
        let day = row.timestamp_iso.date_naive().to_string();
        by_user_day
            .entry((row.user_id.clone(), day))
            .or_default()
            .push(row);
    }

    let n = by_user_day.len();
    let mut x = Array2::<f32>::zeros((n, 4));
    let mut y = Array1::<f32>::zeros(n);

    for (i, ((_user, _day), v)) in by_user_day.into_iter().enumerate() {
        let len = v.len() as f32;
        let mut sum_crave = 0.0;
        let mut sum_theta = 0.0;
        let mut sum_coh = 0.0;
        let mut sum_hr = 0.0;
        let mut last_cigs = 0.0;

        for r in v {
            sum_crave += r.craving_score;
            sum_theta += r.frontal_theta_norm;
            sum_coh += r.theta_coherence_fp;
            sum_hr += r.heart_rate_bpm;
            last_cigs = r.cigarettes_today as f32;
        }

        x[[i, 0]] = sum_crave / len;
        x[[i, 1]] = sum_theta / len;
        x[[i, 2]] = sum_coh / len;
        x[[i, 3]] = sum_hr / len;
        y[i] = last_cigs;
    }

    (x, y)
}

pub struct NeuroQuitModel {
    rf: RandomForest<f32>,
}

impl NeuroQuitModel {
    pub fn fit(rows: &[NeuroQuitSessionRow]) -> Option<Self> {
        if rows.is_empty() {
            return None;
        }
        let (x, y) = build_daily_features(rows);
        let ds = Dataset::new(x, y);

        let rf = RandomForest::params()
            .max_depth(Some(6))
            .n_trees(64)
            .fit(&ds)
            .ok()?;

        Some(Self { rf })
    }

    /// Predict expected cigarettes for a given feature vector.
    /// This is for analytic use only, never direct control.
    pub fn predict_cigarettes(&self, features: &[f32; 4]) -> f32 {
        let x = Array2::from_shape_vec((1, 4), features.to_vec()).unwrap();
        let ds = DatasetBase::from(x);
        let y_hat = self.rf.predict(&ds);
        y_hat[0].max(0.0)
    }
}
