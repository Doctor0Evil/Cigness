pub mod schema;
pub mod loader;
pub mod model;
pub mod safety;

/// This crate is for offline, bioscale-compatible analysis of NeuroQuit shards.
/// It MUST NOT be used to trigger automatic medical decisions, diagnoses,
/// or coercive interventions. Outputs are advisory and should be combined
/// with human judgment, informed consent, and clear opt-out paths.
