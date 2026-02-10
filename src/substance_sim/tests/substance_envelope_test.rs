#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_caffeine_scenario() {
        let envelope = SubstanceEnvelopeSpec::new(
            4.2,   // caffeine
            0.0,   // nicotine
            0.0,   // thc
            0.15,  // d2
            0.0,   // cb1
            75.0,  // adenosine
            0.8,   // da
            0.1,   // ser
            0.05,  // gaba
            0.3,   // glu
        ).unwrap();

        assert!((envelope.power - 0.45).abs() < 0.01);
        assert!((envelope.decay - 0.18).abs() < 0.01);
        assert!((envelope.lifeforce - 0.37).abs() < 0.01);
        assert!(envelope.is_diagnostic_only);
        assert!(!envelope.is_actuating);
    }

    #[test]
    fn test_invalid_caffeine_high() {
        let result = SubstanceEnvelopeSpec::new(
            12.0,  // out of bounds
            0.0,
            0.0,
            0.15,
            0.0,
            75.0,
            0.8,
            0.1,
            0.05,
            0.3,
        );
        assert!(matches!(result, Err(SubstanceEnvelopeError::OutOfBounds { .. })));
    }

    #[test]
    fn test_substance_envelope_to_snapshot() {
        let envelope = SubstanceEnvelopeSpec::new(
            2.5,
            0.2,
            50.0,
            0.2,
            0.4,
            45.0,
            1.2,
            -0.1,
            -0.2,
            0.4,
        ).unwrap();

        let snapshot = envelope.to_biophysical_snapshot();
        assert_eq!(snapshot.source, "substance_sim::virtual_envelope");
        assert!(snapshot.is_virtual);
        assert!(snapshot.is_diagnostic_only);
        assert!(snapshot.is_non_reward);
        assert_eq!(snapshot.roh_bound, 0.3);
        assert_eq!(snapshot.capability_tier, "CapModelOnly");
    }
}
