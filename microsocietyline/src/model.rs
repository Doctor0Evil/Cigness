#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HabitState {
    /// Dimensionless habit load, 0–1 (0 = none, 1 = fully entrenched).
    pub habit: f64,
    /// Habit decay rate per tick, before FEAR / CHURCH modulation.
    pub base_decay: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PollutionState {
    /// Local pollution stock, normalized to a biophysical ceiling.
    pub stock: f64,
    /// Maximum sustainable local pollution stock (like capacity in BiophysicalState).
    pub capacity: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExposureState {
    /// Instantaneous exposure dose this tick (e.g., from local stock and habits).
    pub dose: f64,
    /// Cumulative exposure over the agent’s life, normalized to a ceiling.
    pub cumulative: f64,
    /// Maximum allowed cumulative exposure before collapse / forced repair.
    pub max_cumulative: f64,
}
